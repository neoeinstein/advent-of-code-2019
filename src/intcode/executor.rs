use super::{
    decoder::{Instruction, OpCode, ParameterMode, ParameterModes},
    Address, Program, ProgramValue,
};
use snafu::{ResultExt, Snafu};
use std::{
    convert::TryFrom,
    fmt, ops,
    sync::mpsc::{channel, Receiver, RecvError, SendError, Sender},
};
use thiserror::Error;

/// A counter which keeps track of the currently executing instruction
/// in the Intcode executor
#[derive(Clone, Copy, Debug)]
struct ProgramCounter(usize);

impl fmt::Display for ProgramCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "position {}", self.0)
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

    /// Obtains the address of parameter `idx` for the current instruction
    const fn param(self, idx: u8) -> Address {
        Address::new(self.0 + (idx as usize) + 1)
    }

    /// Advances the program counter by the specified offset
    fn advance(&mut self, cnt: usize) {
        self.0 += cnt;
    }

    /// Jumps to the specified address
    fn jump(&mut self, address: Address) {
        self.0 = address.value()
    }

    /// Gets the address of the currently executing instruction
    const fn address(self) -> Address {
        Address::new(self.0)
    }
}

/// The Intcode interpreter
///
/// Executes programs, keeps track of current position, and relays input and
/// output during execution.
#[derive(Debug)]
pub struct Executable {
    program: Program,
    pc: ProgramCounter,
    input: Receiver<ProgramValue>,
    output: Sender<ProgramValue>,
}

impl From<Program> for Executable {
    fn from(program: Program) -> Self {
        Self {
            program,
            pc: ProgramCounter::START,
            input: channel().1,
            output: channel().0,
        }
    }
}

impl Executable {
    pub fn single_input(&mut self, value: ProgramValue) {
        let (tx, rx) = channel();
        self.input = rx;
        tx.send(value).unwrap();
    }

    pub fn pipe_outputs_to(&mut self, target: Sender<ProgramValue>) {
        self.output = target;
    }

    pub fn pipe_inputs_from(&mut self, source: Receiver<ProgramValue>) {
        self.input = source;
    }

    pub fn drain(&mut self) -> OutputDrain {
        let (tx, rx) = channel();
        self.pipe_outputs_to(tx);
        OutputDrain(rx)
    }

    /// Access to the program data in the executable's memory
    pub fn memory(&mut self) -> &Program {
        &self.program
    }

    /// Mutable access to the program data in the executable's memory
    pub fn memory_mut(&mut self) -> &mut Program {
        &mut self.program
    }

    fn exec_read(&self, address: Address) -> Result<ProgramValue, ExecutionErrorInner> {
        self.program
            .try_read(address)
            .ok_or_else(|| ExecutionErrorInner::OutOfBoundsAccess {
                address,
                position: self.pc,
            })
    }

    fn read_param(
        &self,
        modes: ParameterModes,
        idx: u8,
    ) -> Result<ProgramValue, ExecutionErrorInner> {
        let mode = modes.for_param(idx);
        let value = self.exec_read(self.pc.param(idx))?;

        match mode {
            ParameterMode::Position => {
                let address = Address::try_from_value(value).ok_or_else(|| {
                    ExecutionErrorInner::InvalidAddress {
                        value,
                        position: self.pc,
                    }
                })?;
                self.exec_read(address)
            }
            ParameterMode::Immediate => Ok(value),
        }
    }

    fn exec_write(
        &mut self,
        address: Address,
        value: ProgramValue,
    ) -> Result<ProgramValue, ExecutionErrorInner> {
        self.program.try_write(address, value).ok_or_else(|| {
            ExecutionErrorInner::OutOfBoundsAccess {
                address,
                position: self.pc,
            }
        })
    }

    /// Executes the Intcode program until a halt instruction is encountered or
    /// an invalid operation causes termination due to an `ExecutionError`
    pub fn execute(&mut self) -> Result<(), ExecutionError> {
        let r = self.execute_impl();
        self.output = channel().0;
        r
    }

    fn execute_impl(&mut self) -> Result<(), ExecutionError> {
        while self.pc.address() <= self.program.max_address() {
            let instruction = Instruction::try_from(self.exec_read(self.pc.address())?)
                .context(InvalidInstruction { position: self.pc })?;

            match instruction.opcode() {
                OpCode::Halt => return Ok(()),
                OpCode::Add => self.execute_binary_op(instruction.param_modes(), ops::Add::add)?,
                OpCode::Mul => self.execute_binary_op(instruction.param_modes(), ops::Mul::mul)?,
                OpCode::Input => self.execute_input()?,
                OpCode::Output => self.execute_output(instruction.param_modes())?,
                OpCode::JumpNonZero => self.execute_jump_cond(instruction.param_modes(), true)?,
                OpCode::JumpZero => self.execute_jump_cond(instruction.param_modes(), false)?,
                OpCode::LessThan => {
                    self.execute_cmp(instruction.param_modes(), ProgramValue::lt)?
                }
                OpCode::Equal => self.execute_cmp(instruction.param_modes(), ProgramValue::eq)?,
            };
        }

        Err(ExecutionErrorInner::OutOfBoundsAccess {
            position: self.pc,
            address: self.pc.address(),
        })?
    }

    fn get_binary_operands(
        &self,
        modes: ParameterModes,
    ) -> Result<BinOperands, ExecutionErrorInner> {
        if self.pc.param(2) > self.program.max_address() {
            return Err(ExecutionErrorInner::OutOfBoundsAccess {
                position: self.pc,
                address: self.pc.param(2),
            });
        }

        let destination = self.exec_read(self.pc.param(2))?;

        Ok(BinOperands {
            values: (self.read_param(modes, 0)?, self.read_param(modes, 1)?),
            destination: Address::try_from_value(destination).ok_or_else(|| {
                ExecutionErrorInner::InvalidAddress {
                    value: destination,
                    position: self.pc,
                }
            })?,
        })
    }

    fn execute_binary_op(
        &mut self,
        modes: ParameterModes,
        f: fn(ProgramValue, ProgramValue) -> ProgramValue,
    ) -> Result<(), ExecutionErrorInner> {
        let operands = self.get_binary_operands(modes)?;
        let result = f(operands.values.0, operands.values.1);

        self.exec_write(operands.destination, result)?;

        self.pc.advance(4);
        Ok(())
    }

    fn execute_input(&mut self) -> Result<(), ExecutionErrorInner> {
        let sloc = self.exec_read(self.pc.param(0))?;
        let address =
            Address::try_from_value(sloc).ok_or_else(|| ExecutionErrorInner::InvalidAddress {
                value: sloc,
                position: self.pc,
            })?;
        let in_value = self
            .input
            .recv()
            .context(UnexpectedEndOfInput { position: self.pc })?;
        self.exec_write(address, in_value)?;
        self.pc.advance(2);
        Ok(())
    }

    fn execute_output(&mut self, modes: ParameterModes) -> Result<(), ExecutionErrorInner> {
        let value = self.read_param(modes, 0)?;
        self.output
            .send(value)
            .context(OutputPipeClosed { position: self.pc })?;
        self.pc.advance(2);
        Ok(())
    }

    fn execute_jump_cond(
        &mut self,
        modes: ParameterModes,
        jump_if_non_zero: bool,
    ) -> Result<(), ExecutionErrorInner> {
        let value = self.read_param(modes, 0)?;

        if (value != 0) == jump_if_non_zero {
            let target = self.read_param(modes, 1)?;
            let address = Address::try_from_value(target).ok_or_else(|| {
                ExecutionErrorInner::InvalidAddress {
                    value: target,
                    position: self.pc,
                }
            })?;

            self.pc.jump(address);
        } else {
            self.pc.advance(3);
        }

        Ok(())
    }

    fn execute_cmp(
        &mut self,
        modes: ParameterModes,
        f: fn(&ProgramValue, &ProgramValue) -> bool,
    ) -> Result<(), ExecutionErrorInner> {
        let operands = self.get_binary_operands(modes)?;

        let result = if f(&operands.values.0, &operands.values.1) {
            1
        } else {
            0
        };

        self.exec_write(operands.destination, result)?;

        self.pc.advance(4);
        Ok(())
    }
}

pub struct OutputDrain(Receiver<ProgramValue>);

impl OutputDrain {
    /// Blocks until the executable has halted
    pub fn to_vec(&self) -> Vec<ProgramValue> {
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
enum ExecutionErrorInner {
    #[snafu(display("execution error: invalid instruction at {}", position))]
    InvalidInstruction {
        source: super::decoder::InvalidInstruction,
        position: ProgramCounter,
    },
    #[snafu(display(
        "execution error: attempted out of bounds access to {} at {}",
        address,
        position
    ))]
    OutOfBoundsAccess {
        position: ProgramCounter,
        address: Address,
    },
    #[snafu(display(
        "execution error: attempted to create an address from a negative value {} at {}",
        value,
        position
    ))]
    InvalidAddress {
        position: ProgramCounter,
        value: ProgramValue,
    },
    #[snafu(display("execution error: unexpected end of input at {}", position))]
    UnexpectedEndOfInput {
        source: RecvError,
        position: ProgramCounter,
    },
    #[snafu(display(
        "execution error: attempted to write on a closed output pipe at {}",
        position
    ))]
    OutputPipeClosed {
        source: SendError<ProgramValue>,
        position: ProgramCounter,
    },
}

#[derive(Clone, Copy, Debug)]
struct BinOperands {
    values: (ProgramValue, ProgramValue),
    destination: Address,
}
