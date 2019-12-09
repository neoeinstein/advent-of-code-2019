use super::{
    decode::{decode, BinaryOperands, Decoded, InputOperands, JumpIfOperands, OutputOperands},
    error,
    ops::Instruction,
    Address, Buffer, Memory, Relative, Word,
};
use snafu::{ResultExt, Snafu};
use std::{
    convert::TryFrom,
    fmt, ops,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{channel, Receiver, RecvError, SendError, Sender},
    },
};
use thiserror::Error;

/// A counter which keeps track of the currently executing instruction
/// in the Intcode executor
#[derive(Clone, Copy, Debug)]
pub struct ProgramCounter(usize);

impl fmt::Display for ProgramCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Default for ProgramCounter {
    fn default() -> Self {
        Self::START
    }
}

impl ProgramCounter {
    /// A program counter initialized to start from the first address in memory
    const START: Self = Self(0);

    #[inline]
    /// Obtains the address of parameter `idx` for the current instruction
    pub const fn param(self, idx: u8) -> Address {
        Address::new(self.0 + (idx as usize) + 1)
    }

    #[inline]
    /// Advances the program counter by the specified offset
    pub fn advance(&mut self, cnt: usize) {
        self.0 += cnt;
    }

    #[inline]
    /// Jumps to the specified address
    pub fn jump(&mut self, address: Address) {
        self.0 = address.value()
    }

    #[inline]
    /// Gets the address of the currently executing instruction
    pub const fn address(self) -> Address {
        Address::new(self.0)
    }
}

static NEXT_EXECUTABLE_ID: AtomicUsize = AtomicUsize::new(0);

/// The Intcode interpreter
///
/// Executes programs, keeps track of current position, and relays input and
/// output during execution.
#[derive(Debug)]
pub struct Executable {
    id: usize,
    memory: Memory,
    pc: ProgramCounter,
    rel: Address,
    input: Receiver<Word>,
    output: Sender<Word>,
}

impl From<Memory> for Executable {
    fn from(memory: Memory) -> Self {
        Self {
            id: NEXT_EXECUTABLE_ID.fetch_add(1, Ordering::AcqRel),
            memory,
            pc: ProgramCounter::START,
            rel: Address::new(0),
            input: channel().1,
            output: channel().0,
        }
    }
}

impl Executable {
    pub fn single_input(&mut self, value: Word) {
        let (tx, rx) = channel();
        self.input = rx;
        tx.send(value).unwrap();
    }

    pub fn pipe_to(&mut self, target: &mut Executable) -> Sender<Word> {
        let (tx, rx) = channel();
        self.output = tx.clone();
        target.input = rx;
        tx
    }

    pub fn buffer_to(&mut self, target: &mut Executable) -> Buffer {
        Buffer::between(self, target)
    }

    pub(super) fn pipe_outputs_to(&mut self, target: Sender<Word>) {
        self.output = target;
    }

    pub(super) fn pipe_inputs_from(&mut self, source: Receiver<Word>) {
        self.input = source;
    }

    pub fn drain(&mut self) -> OutputDrain {
        let (tx, rx) = channel();
        self.pipe_outputs_to(tx);
        OutputDrain(rx)
    }

    fn exec_read(&self, address: Address) -> Result<Word, ExecutionErrorInner> {
        Ok(self
            .memory
            .try_read(address)
            .context(OutOfBoundsAccess { pc: self.pc })?)
    }

    fn exec_write(&mut self, address: Address, value: Word) -> Result<Word, ExecutionErrorInner> {
        self.memory
            .try_write(address, value)
            .context(OutOfBoundsAccess { pc: self.pc })
    }

    pub fn execute_in_thread(self) -> std::thread::JoinHandle<Result<Memory, ExecutionError>> {
        std::thread::spawn(move || self.execute())
    }

    /// Executes the Intcode program in memory until a halt instruction is
    /// encountered or an invalid operation causes termination due to an
    /// `ExecutionError`
    pub fn execute(mut self) -> Result<Memory, ExecutionError> {
        while self.step()? {}

        Ok(self.memory)
    }

    pub fn step(&mut self) -> Result<bool, ExecutionError> {
        let i = Instruction::try_from(self.exec_read(self.pc.address())?)
            .context(InvalidInstruction { pc: self.pc })?;

        let decoded = decode(i, self.pc, &self.memory)?;

        Ok(self.execute_op(decoded)?)
    }

    fn execute_op(&mut self, op: Decoded) -> Result<bool, ExecutionErrorInner> {
        match op {
            Decoded::Add(params) => self.execute_binary_op(params, ops::Add::add),
            Decoded::Mul(params) => self.execute_binary_op(params, ops::Mul::mul),
            Decoded::Input(params) => self.execute_input(params),
            Decoded::Output(params) => self.execute_output(params),
            Decoded::JumpNonZero(params) => self.execute_jump_if(params, true),
            Decoded::JumpZero(params) => self.execute_jump_if(params, false),
            Decoded::LessThan(params) => self.execute_cmp(params, Word::lt),
            Decoded::Equal(params) => self.execute_cmp(params, Word::eq),
            Decoded::AddRel(params) => self.execute_add_rel(params),
            Decoded::Halt => {
                log::debug!("{}@{}: halt", self.id, self.pc);
                return Ok(false);
            }
        }?;

        Ok(true)
    }
    fn execute_binary_op(
        &mut self,
        operands: BinaryOperands,
        f: fn(Word, Word) -> Word,
    ) -> Result<(), ExecutionErrorInner> {
        let left = operands
            .left
            .load(self.rel, &self.memory)
            .context(OutOfBoundsAccess { pc: self.pc })?;
        let right = operands
            .right
            .load(self.rel, &self.memory)
            .context(OutOfBoundsAccess { pc: self.pc })?;
        let result = f(left, right);

        log::debug!("{}@{}: {} {} = {}", self.id, self.pc, left, right, result);

        self.exec_write(operands.target, result)?;

        self.pc.advance(4);
        Ok(())
    }

    fn execute_input(&mut self, operands: InputOperands) -> Result<(), ExecutionErrorInner> {
        let value = self
            .input
            .recv()
            .context(UnexpectedEndOfInput { pc: self.pc })?;

        log::debug!("{}@{}: {} =>", self.id, self.pc, value);

        self.exec_write(operands.target, value)?;
        self.pc.advance(2);
        Ok(())
    }

    fn execute_output(&mut self, operands: OutputOperands) -> Result<(), ExecutionErrorInner> {
        let value = operands
            .source
            .load(self.rel, &self.memory)
            .context(OutOfBoundsAccess { pc: self.pc })?;

        log::debug!("{}@{}: => {}", self.id, self.pc, value);

        self.output
            .send(value)
            .context(OutputPipeClosed { pc: self.pc })?;
        self.pc.advance(2);
        Ok(())
    }

    fn execute_add_rel(&mut self, operands: OutputOperands) -> Result<(), ExecutionErrorInner> {
        let value = operands
            .source
            .load(self.rel, &self.memory)
            .context(OutOfBoundsAccess { pc: self.pc })?;

        let next = (self.rel + Relative::from(value)).context(InvalidAddress {
            pc: self.pc
        })?;

        log::debug!("{}@{}: {} {} => {}", self.id, self.pc, self.rel, value, next);

        self.rel = next;
        self.pc.advance(2);
        Ok(())
    }

    fn execute_jump_if(
        &mut self,
        operands: JumpIfOperands,
        non_zero: bool,
    ) -> Result<(), ExecutionErrorInner> {
        let value = operands
            .value
            .load(self.rel, &self.memory)
            .context(OutOfBoundsAccess { pc: self.pc })?;
        let target_raw = operands
            .jump_target
            .load(self.rel, &self.memory)
            .context(OutOfBoundsAccess { pc: self.pc })?;
        let target = Address::try_from(target_raw).context(InvalidAddress { pc: self.pc })?;

        if (value != 0) == non_zero {
            log::debug!("{}@{}: {} ~> {}", self.id, self.pc, value, target);

            self.pc.jump(target);
        } else {
            log::debug!("{}@{}: {} !~>", self.id, self.pc, value);

            self.pc.advance(3);
        }

        Ok(())
    }

    fn execute_cmp(
        &mut self,
        operands: BinaryOperands,
        f: fn(&Word, &Word) -> bool,
    ) -> Result<(), ExecutionErrorInner> {
        let left = operands
            .left
            .load(self.rel, &self.memory)
            .context(OutOfBoundsAccess { pc: self.pc })?;
        let right = operands
            .right
            .load(self.rel, &self.memory)
            .context(OutOfBoundsAccess { pc: self.pc })?;

        let result = if f(&left, &right) { 1 } else { 0 };

        log::debug!("{}@{}: {} {} = {}", self.id, self.pc, left, right, result);

        self.exec_write(operands.target, result)?;

        self.pc.advance(4);
        Ok(())
    }
}

pub struct OutputDrain(Receiver<Word>);

impl OutputDrain {
    /// Blocks until the executable has halted
    pub fn to_vec(&self) -> Vec<Word> {
        let mut outputs = Vec::new();

        loop {
            match self.0.recv() {
                Ok(x) => outputs.push(x),
                Err(_) => return outputs,
            }
        }
    }
}

/// An error during execution
///
/// Possible errors include:
///
/// * Execution of an invalid instruction
/// * Access to an address beyond the memory limit
/// * Attempt to interpret a negative value as an address
#[derive(Error, Debug)]
#[error("{0}")]
pub struct ExecutionError(#[from] ExecutionErrorInner);

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum ExecutionErrorInner {
    #[snafu(display("invalid instruction; pc = {}", pc))]
    InvalidInstruction {
        source: super::ops::InvalidInstruction,
        pc: ProgramCounter,
    },
    #[snafu(display("attempted out of bounds access; pc = {}", pc))]
    OutOfBoundsAccess {
        source: error::OutOfBoundsAccess,
        pc: ProgramCounter,
    },
    #[snafu(display("invalid address; pc = {}", pc))]
    InvalidAddress {
        source: error::InvalidAddress,
        pc: ProgramCounter,
    },
    #[snafu(display("unexpected end of input; pc = {}", pc))]
    UnexpectedEndOfInput {
        source: RecvError,
        pc: ProgramCounter,
    },
    #[snafu(display("attempted to write on a closed output pipe; pc = {}", pc))]
    OutputPipeClosed {
        source: SendError<Word>,
        pc: ProgramCounter,
    },
}

#[derive(Clone, Copy, Debug)]
struct BinOperands {
    values: (Word, Word),
    destination: Address,
}
