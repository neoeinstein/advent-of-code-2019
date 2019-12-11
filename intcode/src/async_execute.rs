use super::{
    decode::{decode, BinaryOperands, Decoded, InputOperands, JumpIfOperands, OutputOperands},
    execute::*,
    ops::Instruction,
    Address, Memory, Relative, Word,
};
use snafu::ResultExt;
use std::{
    convert::TryFrom,
    ops,
    sync::atomic::{AtomicUsize, Ordering},
};
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct AsyncBuffer {
    last_output: Option<Word>,
    rx: Receiver<Word>,
    tx: Sender<Word>,
}

impl AsyncBuffer {
    pub fn between(source: &mut AsyncExecutable, target: &mut AsyncExecutable) -> Self {
        let (tx, buf_out) = channel(1);
        let (buf_in, rx) = channel(1);

        source.pipe_outputs_to(buf_in);
        target.pipe_inputs_from(buf_out);

        AsyncBuffer {
            last_output: None,
            rx,
            tx,
        }
    }

    pub fn injector(&self) -> Sender<Word> {
        self.tx.clone()
    }

    pub async fn execute(mut self) -> Option<Word> {
        loop {
            let value = match self.rx.recv().await {
                Some(v) => v,
                None => break,
            };

            self.last_output = Some(value);

            // Ignore if the listener has stopped listening
            let _ = self.tx.send(value).await;
        }

        self.last_output
    }
}

static NEXT_EXECUTABLE_ID: AtomicUsize = AtomicUsize::new(0);

/// The Intcode interpreter
///
/// Executes programs, keeps track of current position, and relays input and
/// output during execution.
#[derive(Debug)]
pub struct AsyncExecutable {
    id: usize,
    memory: Memory,
    pc: ProgramCounter,
    rel: Address,
    input: Receiver<Word>,
    output: Sender<Word>,
    steps: usize,
}

impl From<Memory> for AsyncExecutable {
    fn from(memory: Memory) -> Self {
        Self {
            id: NEXT_EXECUTABLE_ID.fetch_add(1, Ordering::AcqRel),
            memory,
            pc: ProgramCounter::START,
            rel: Address::new(0),
            input: channel(1).1,
            output: channel(1).0,
            steps: 0,
        }
    }
}

impl AsyncExecutable {
    pub fn single_input(&mut self, value: Word) {
        let (mut tx, rx) = channel(1);
        self.input = rx;
        tokio::spawn(async move { tx.send(value).await });
    }

    pub fn pipe_to(&mut self, target: &mut AsyncExecutable) -> Sender<Word> {
        let (tx, rx) = channel(1);
        self.output = tx.clone();
        target.input = rx;
        tx
    }

    pub fn buffer_to(&mut self, target: &mut AsyncExecutable) -> AsyncBuffer {
        AsyncBuffer::between(self, target)
    }

    pub fn pipe_outputs_to(&mut self, target: Sender<Word>) {
        self.output = target;
    }

    pub fn pipe_inputs_from(&mut self, source: Receiver<Word>) {
        self.input = source;
    }

    pub fn drain(&mut self) -> AsyncOutputDrain {
        let (tx, rx) = channel(1);
        self.pipe_outputs_to(tx);
        AsyncOutputDrain(rx)
    }

    fn read_instruction(&self) -> Result<Decoded, ExecutionErrorInner> {
        let op = self
            .memory
            .try_read(self.pc.address())
            .context(UnexpectedEndOfProgram { pc: self.pc })?;

        let i = Instruction::try_from(op).context(InvalidInstruction { pc: self.pc })?;

        decode(i, self.pc, &self.memory)
    }

    /// Executes the Intcode program in memory until a halt instruction is
    /// encountered or an invalid operation causes termination due to an
    /// `ExecutionError`
    pub async fn execute(mut self) -> Result<Memory, ExecutionError> {
        while self.step().await? {}

        Ok(self.memory)
    }

    pub async fn step(&mut self) -> Result<bool, ExecutionError> {
        Ok(self.execute_op(self.read_instruction()?).await?)
    }

    async fn execute_op(&mut self, op: Decoded) -> Result<bool, ExecutionErrorInner> {
        self.steps += 1;
        match op {
            Decoded::Add(params) => self.execute_binary_op(params, ops::Add::add),
            Decoded::Mul(params) => self.execute_binary_op(params, ops::Mul::mul),
            Decoded::Input(params) => self.execute_input(params).await,
            Decoded::Output(params) => self.execute_output(params).await,
            Decoded::JumpNonZero(params) => self.execute_jump_if(params, true),
            Decoded::JumpZero(params) => self.execute_jump_if(params, false),
            Decoded::LessThan(params) => self.execute_cmp(params, Word::lt),
            Decoded::Equal(params) => self.execute_cmp(params, Word::eq),
            Decoded::AddRel(params) => self.execute_add_rel(params),
            Decoded::Halt => {
                log::trace!("{}@{}: halt", self.id, self.pc);
                log::debug!(
                    "halted (steps = {}; memory size = {})",
                    self.steps,
                    self.memory.max_address().value() + 1
                );
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
            .context(InvalidAddress { pc: self.pc })?;
        let right = operands
            .right
            .load(self.rel, &self.memory)
            .context(InvalidAddress { pc: self.pc })?;
        let result = f(left, right);

        log::trace!("{}@{}: {} {} = {}", self.id, self.pc, left, right, result);

        operands.target.store(self.rel, &mut self.memory, result);

        self.pc.advance(4);
        Ok(())
    }

    async fn execute_input(&mut self, operands: InputOperands) -> Result<(), ExecutionErrorInner> {
        let value = self
            .input
            .recv()
            .await
            .ok_or(ExecutionErrorInner::UnexpectedEndOfInput {
                source: std::sync::mpsc::RecvError,
                pc: self.pc,
            })?;

        log::trace!("{}@{}: {} =>", self.id, self.pc, value);

        operands.target.store(self.rel, &mut self.memory, value);

        self.pc.advance(2);
        Ok(())
    }

    async fn execute_output(
        &mut self,
        operands: OutputOperands,
    ) -> Result<(), ExecutionErrorInner> {
        let value = operands
            .source
            .load(self.rel, &self.memory)
            .context(InvalidAddress { pc: self.pc })?;

        log::trace!("{}@{}: => {}", self.id, self.pc, value);

        self.output
            .send(value)
            .await
            .map_err(|e| ExecutionErrorInner::OutputPipeClosed {
                source: std::sync::mpsc::SendError(e.0),
                pc: self.pc,
            })?;
        self.pc.advance(2);
        Ok(())
    }

    fn execute_add_rel(&mut self, operands: OutputOperands) -> Result<(), ExecutionErrorInner> {
        let value = operands
            .source
            .load(self.rel, &self.memory)
            .context(InvalidAddress { pc: self.pc })?;

        let next = (self.rel + Relative::from(value)).context(InvalidAddress { pc: self.pc })?;

        log::trace!(
            "{}@{}: {} {} => {}",
            self.id,
            self.pc,
            self.rel,
            value,
            next
        );

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
            .context(InvalidAddress { pc: self.pc })?;
        let target_raw = operands
            .jump_target
            .load(self.rel, &self.memory)
            .context(InvalidAddress { pc: self.pc })?;
        let target = Address::try_from(target_raw).context(InvalidAddress { pc: self.pc })?;

        if (value != 0) == non_zero {
            log::trace!("{}@{}: {} ~> {}", self.id, self.pc, value, target);

            self.pc.jump(target);
        } else {
            log::trace!("{}@{}: {} !~>", self.id, self.pc, value);

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
            .context(InvalidAddress { pc: self.pc })?;
        let right = operands
            .right
            .load(self.rel, &self.memory)
            .context(InvalidAddress { pc: self.pc })?;

        let result = if f(&left, &right) { 1 } else { 0 };

        log::trace!("{}@{}: {} {} = {}", self.id, self.pc, left, right, result);

        operands.target.store(self.rel, &mut self.memory, result);

        self.pc.advance(4);
        Ok(())
    }
}

pub struct AsyncOutputDrain(Receiver<Word>);

impl AsyncOutputDrain {
    /// Blocks until the executable has halted
    pub fn to_vec(
        mut self,
    ) -> impl std::future::Future<Output = Result<Vec<Word>, tokio::task::JoinError>> {
        tokio::spawn(async move {
            let mut outputs = Vec::new();

            loop {
                match self.0.recv().await {
                    Some(x) => outputs.push(x),
                    None => break,
                }
            }

            outputs
        })
    }
}
