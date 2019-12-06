use std::{
    collections::VecDeque, 
    convert::TryFrom, 
    fmt, 
    ops,
};
use snafu::{ResultExt, Snafu};
use thiserror::Error;
use super::{
    decoder::{InvalidOpCode, OpCode, Operation, ParameterMode, ParameterModes},
    Address, 
    Program,
};

#[derive(Clone, Copy, Debug)]
struct ProgramCounter(usize);

impl fmt::Display for ProgramCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "position {}", self.0)
    }
}

impl Default for ProgramCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgramCounter {
    const START: Self = Self(0);

    const fn new() -> Self {
        Self(0)
    }

    const fn param(self, idx: u8) -> Address {
        Address::new(self.0 + (idx as usize) + 1)
    }

    fn advance(&mut self, cnt: usize) {
        self.0 += cnt;
    }

    fn jump(&mut self, address: Address) {
        self.0 = address.value()
    }

    const fn address(self) -> Address {
        Address::new(self.0)
    }
}

#[derive(Debug)]
pub struct Executable {
    program: Program,
    pc: ProgramCounter,
    input: VecDeque<isize>,
    output: Vec<isize>,
}

impl From<Program> for Executable {
    fn from(program: Program) -> Self {
        Self {
            program,
            pc: ProgramCounter::START,
            input: VecDeque::new(),
            output: Vec::new(),
        }
    }
}

impl Executable {
    pub fn set_input(&mut self, input: impl IntoIterator<Item = isize>) {
        self.input.clear();
        self.input.extend(input);
    }

    pub fn output(&self) -> &[isize] {
        &self.output
    }

    pub fn memory(&mut self) -> &Program {
        &self.program
    }

    pub fn memory_mut(&mut self) -> &mut Program {
        &mut self.program
    }

    fn exec_read(&self, address: Address) -> Result<isize, ExecutionErrorInner> {
        self.program.try_read(address).ok_or_else(|| ExecutionErrorInner::OutOfBoundsAccess { address, position: self.pc })
    }

    fn read_param(&self, modes: ParameterModes, idx: u8) -> Result<isize, ExecutionErrorInner> {
        let mode = modes.for_param(idx);
        let value = self.exec_read(self.pc.param(idx))?;

        match mode {
            ParameterMode::Position => {
                let address = Address::try_from_value(value)
                    .ok_or_else(|| ExecutionErrorInner::InvalidAddress { value, position: self.pc })?;
                self.exec_read(address)
            }
            ParameterMode::Immediate => Ok(value),
        }
    }

    fn exec_write(&mut self, address: Address, value: isize) -> Result<isize, ExecutionErrorInner> {
        self.program.try_write(address, value)
            .ok_or_else(|| ExecutionErrorInner::OutOfBoundsAccess { address, position: self.pc })
    }

    pub fn execute(&mut self) -> Result<(), ExecutionError> {
        while self.pc.address() < self.program.max_address() {
            let opcode = OpCode::try_from(self.exec_read(self.pc.address())?)
                .context(InvalidOperation { position: self.pc })?;
    
            match opcode.op() {
                Operation::Halt => break,
                Operation::Add => self.execute_binary_op(opcode.param_modes(), ops::Add::add)?,
                Operation::Mul => self.execute_binary_op(opcode.param_modes(), ops::Mul::mul)?,
                Operation::Input => self.execute_input()?,
                Operation::Output => self.execute_output(opcode.param_modes())?,
                Operation::JumpNonZero => self.execute_jump_cond(opcode.param_modes(), true)?,
                Operation::JumpZero => self.execute_jump_cond(opcode.param_modes(), false)?,
                Operation::LessThan => self.execute_cmp(opcode.param_modes(), isize::lt)?,
                Operation::Equal => self.execute_cmp(opcode.param_modes(), isize::eq)?,
            };
        }

        Ok(())
    }

    fn get_binary_operands(&self, modes: ParameterModes) -> Result<BinOperands, ExecutionErrorInner> {
        if self.pc.param(2) > self.program.max_address() {
            return Err(ExecutionErrorInner::OutOfBoundsAccess {
                position: self.pc,
                address: self.pc.param(2),
            });
        }

        let destination = self.exec_read(self.pc.param(2))?;

        Ok(BinOperands {
            values: (self.read_param(modes, 0)?, self.read_param(modes, 1)?),
            destination: Address::try_from_value(destination)
                .ok_or_else(|| ExecutionErrorInner::InvalidAddress { value: destination, position: self.pc })?,
        })
    }    

    fn execute_binary_op(&mut self, modes: ParameterModes, f: fn(isize, isize) -> isize) -> Result<(), ExecutionErrorInner> {
        let operands = self.get_binary_operands(modes)?;
        let result = f(operands.values.0, operands.values.1);

        self.exec_write(operands.destination, result)?;

        self.pc.advance(4);
        Ok(())
    }
    
    fn execute_input(&mut self) -> Result<(), ExecutionErrorInner> {
        let sloc = self.exec_read(self.pc.param(0))?;
        let address = Address::try_from_value(sloc)
            .ok_or_else(|| ExecutionErrorInner::InvalidAddress { value: sloc, position: self.pc })?;
        let in_value = self.input.pop_front().expect("unexpected end of input");
        self.exec_write(address, in_value)?;
        self.pc.advance(2);
        Ok(())
    }

    fn execute_output(&mut self, modes: ParameterModes) -> Result<(), ExecutionErrorInner>  {
        let value = self.read_param(modes, 0)?;
        self.output.push(value);
        self.pc.advance(2);
        Ok(())
    }

    fn execute_jump_cond(&mut self, modes: ParameterModes, jump_if_non_zero: bool) -> Result<(), ExecutionErrorInner> {
        let value = self.read_param(modes, 0)?;

        if (value != 0) == jump_if_non_zero {
            let target = self.read_param(modes, 1)?;
            let address = Address::try_from_value(target)
                .ok_or_else(|| ExecutionErrorInner::InvalidAddress { value: target, position: self.pc })?;
            
            self.pc.jump(address);
        } else {
            self.pc.advance(3);
        }

        Ok(())
    }

    fn execute_cmp(&mut self, modes: ParameterModes, f: fn(&isize, &isize) -> bool) -> Result<(), ExecutionErrorInner> {
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

#[derive(Error, Debug)]
#[error("{0}")]
pub struct ExecutionError(#[from] ExecutionErrorInner);

#[derive(Snafu, Debug)]
enum ExecutionErrorInner {
    #[snafu(display("execution error: invalid opcode at {}", position))]
    InvalidOperation {
        source: InvalidOpCode,
        position: ProgramCounter,
    },
    #[snafu(display("execution error: attempted out of bounds access to {} at {}", address, position))]
    OutOfBoundsAccess {
        position: ProgramCounter,
        address: Address,
    },
    #[snafu(display("execution error: attempted to create an address from a negative value {} at {}", value, position))]
    InvalidAddress {
        position: ProgramCounter,
        value: isize,
    },
}

#[derive(Clone, Copy, Debug)]
struct BinOperands {
    values: (isize, isize),
    destination: Address,
}

