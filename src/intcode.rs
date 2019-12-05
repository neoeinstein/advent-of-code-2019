use std::{collections::VecDeque, convert::TryFrom, fmt, io, mem, ops};
use snafu::{Snafu, ResultExt};
use thiserror::Error;

#[derive(Clone, Copy, Debug)]
struct BinOperands {
    values: (isize, isize),
    destination: Address,
}

#[derive(Clone, Copy, Debug)]
enum OpCode {
    Halt,
    Add,
    Mul,
    Input,
    Output,
}

#[derive(Error, Debug)]
#[error("unknown operation {0}")]
pub struct InvalidOpCode(isize);

impl TryFrom<isize> for OpCode {
    type Error = InvalidOpCode;
    fn try_from(op: isize) -> Result<OpCode, Self::Error> {
        let code = match op % 100 {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            3 => OpCode::Input,
            4 => OpCode::Output,
            99 => OpCode::Halt,
            _ => Err(InvalidOpCode(op))?,
        };

        Ok(code)
    }
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct ExecutionError(#[from] ExecutionErrorInner);

#[derive(Snafu, Debug)]
enum ExecutionErrorInner {
    #[snafu(display("execution error: unknown opcode at {}", position))]
    Operation {
        source: InvalidOpCode,
        position: ProgramCounter,
    },
    #[snafu(display("execution error: unknown parameter mode {} at {}", index, position))]
    BadParameterMode {
        source: InvalidParameterMode,
        position: ProgramCounter,
        index: usize,
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

#[derive(Clone, Debug)]
pub struct Program {
    data: Vec<isize>,
}

impl Program {
    pub fn from_vec(data: Vec<isize>) -> Self {
        Self { data }
    }

    pub fn from_reader(input: &mut dyn io::Read) -> io::Result<Program> {
        let mut raw_data = String::new();
        input.read_to_string(&mut raw_data)?;
    
        let data = raw_data.split(',')
            .filter(|op| !op.is_empty())
            .map(|op| op.trim().parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e)))
            .collect::<io::Result<Vec<isize>>>()?;

        Ok(Self::from_vec(data))
    }

    fn max_address(&self) -> Address {
        Address::new(self.data.len())
    }

    pub fn try_read(&self, address: Address) -> Option<isize> {
        self.data.get(address.0).copied()
    }

    pub fn try_write(&mut self, address: Address, value: isize) -> Option<isize> {
        let sloc = self.data.get_mut(address.0)?;
        Some(mem::replace(sloc, value))
    }
}

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

    const fn param(self, idx: usize) -> Address {
        Address(self.0 + idx + 1)
    }

    fn advance(&mut self, cnt: usize) {
        self.0 += cnt;
    }

    const fn address(self) -> Address {
        Address(self.0)
    }
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Address(usize);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "address {}", self.0)
    }
}

impl Address {
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    pub fn try_from_value(value: isize) -> Option<Self> {
        if value >= 0 {
            Some(Self::new(value as usize))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ParameterModes(isize);

impl ParameterModes {
    fn for_param(self, idx: usize) -> Result<ParameterMode, InvalidParameterMode> {
        ParameterMode::try_from((self.0 / (10isize.pow(idx as u32))) % 10)
    }
}

#[derive(Clone, Copy, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl Default for ParameterMode {
    fn default() -> Self {
        ParameterMode::Position
    }
}

impl TryFrom<isize> for ParameterMode {
    type Error = InvalidParameterMode;
    fn try_from(mode: isize) -> Result<Self, Self::Error> {
        match mode {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            _ => Err(InvalidParameterMode(mode)),
        }
    }
}

#[derive(Error, Debug)]
#[error("invalid parameter mode {0}")]
pub struct InvalidParameterMode(isize);

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

    fn read_param(&self, modes: ParameterModes, idx: usize) -> Result<isize, ExecutionErrorInner> {
        let mode = modes.for_param(idx).context(BadParameterMode { position: self.pc, index: idx })?;
        let value = self.exec_read(self.pc.param(idx))?;

        match mode {
            ParameterMode::Position => {
                let address = Address::try_from_value(value).ok_or_else(|| ExecutionErrorInner::InvalidAddress { value, position: self.pc })?;
                self.exec_read(address)
            }
            ParameterMode::Immediate => Ok(value),
        }
    }

    fn exec_write(&mut self, address: Address, value: isize) -> Result<isize, ExecutionErrorInner> {
        self.program.try_write(address, value).ok_or_else(|| ExecutionErrorInner::OutOfBoundsAccess { address, position: self.pc })
    }

    pub fn execute(&mut self) -> Result<(), ExecutionError> {
        while self.pc.address() < self.program.max_address() {
            let opcode = OpCode::try_from(self.exec_read(self.pc.address())?).context(Operation { position: self.pc })?;
    
            match opcode {
                OpCode::Halt => break,
                OpCode::Add => self.execute_binary_op(ops::Add::add)?,
                OpCode::Mul => self.execute_binary_op(ops::Mul::mul)?,
                OpCode::Input => self.execute_input()?,
                OpCode::Output => self.execute_output()?,
            };
        }

        Ok(())
    }

    fn get_binary_operands(&self) -> Result<BinOperands, ExecutionErrorInner> {
        if self.pc.param(2) >= self.program.max_address() {
            return Err(ExecutionErrorInner::OutOfBoundsAccess {
                position: self.pc,
                address: self.pc.param(2),
            });
        }

        let modes = ParameterModes(self.exec_read(self.pc.address())? / 100);
        let destination = self.exec_read(self.pc.param(2))?;

        Ok(BinOperands {
            values: (self.read_param(modes, 0)?, self.read_param(modes, 1)?),
            destination: Address::try_from_value(destination).ok_or_else(|| ExecutionErrorInner::InvalidAddress { value: destination, position: self.pc })?,
        })
    }    

    fn execute_binary_op(&mut self, f: fn(isize, isize) -> isize) -> Result<(), ExecutionErrorInner> {
        let operands = self.get_binary_operands()?;
        let result = f(operands.values.0, operands.values.1);

        self.exec_write(operands.destination, result)?;

        self.pc.advance(4);
        Ok(())
    }
    
    fn execute_input(&mut self) -> Result<(), ExecutionErrorInner> {
        let sloc = self.exec_read(self.pc.param(0))?;
        let address = Address::try_from_value(sloc).ok_or_else(|| ExecutionErrorInner::InvalidAddress { value: sloc, position: self.pc })?;
        let in_value = self.input.pop_front().expect("unexpected end of input");
        self.exec_write(address, in_value)?;
        self.pc.advance(2);
        Ok(())
    }

    fn execute_output(&mut self) -> Result<(), ExecutionErrorInner>  {
        let modes = ParameterModes(self.exec_read(self.pc.address())? / 100);
        let value = self.read_param(modes, 0)?;
        self.output.push(value);
        self.pc.advance(2);
        Ok(())
    }
}