use std::{collections::VecDeque, convert::TryFrom, fmt, io, mem, ops};
use snafu::{Snafu, ResultExt};
use thiserror::Error;

#[derive(Clone, Copy, Debug)]
struct BinOperands {
    values: (usize, usize),
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
pub struct InvalidOpCode(usize);

impl TryFrom<usize> for OpCode {
    type Error = InvalidOpCode;
    fn try_from(op: usize) -> Result<OpCode, Self::Error> {
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
}

#[derive(Clone, Debug)]
pub struct Program {
    data: Vec<usize>,
}

impl Program {
    pub fn from_vec(data: Vec<usize>) -> Self {
        Self { data }
    }

    pub fn from_reader(input: &mut dyn io::Read) -> io::Result<Program> {
        let mut raw_data = String::new();
        input.read_to_string(&mut raw_data)?;
    
        let data = raw_data.split(',')
            .filter(|op| !op.is_empty())
            .map(|op| op.trim().parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e)))
            .collect::<io::Result<Vec<usize>>>()?;

        Ok(Self::from_vec(data))
    }

    fn max_address(&self) -> Address {
        Address::new(self.data.len())
    }

    pub fn try_read(&self, address: Address) -> Option<usize> {
        self.data.get(address.0).copied()
    }

    pub fn try_write(&mut self, address: Address, value: usize) -> Option<usize> {
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
}

#[derive(Clone, Copy, Debug)]
struct ParameterModes(usize);

impl ParameterModes {
    fn for_param(self, idx: usize) -> Result<ParameterMode, InvalidParameterMode> {
        ParameterMode::try_from((self.0 / (10usize.pow(idx as u32))) % 10)
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

impl TryFrom<usize> for ParameterMode {
    type Error = InvalidParameterMode;
    fn try_from(mode: usize) -> Result<Self, Self::Error> {
        match mode {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            _ => Err(InvalidParameterMode(mode)),
        }
    }
}

#[derive(Error, Debug)]
#[error("invalid parameter mode {0}")]
pub struct InvalidParameterMode(usize);

#[derive(Debug)]
pub struct Executable {
    program: Program,
    pc: ProgramCounter,
    input: VecDeque<usize>,
    output: Vec<usize>,
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
    pub fn set_input(&mut self, input: impl IntoIterator<Item = usize>) {
        self.input.clear();
        self.input.extend(input);
    }

    pub fn output(&self) -> &[usize] {
        &self.output
    }

    pub fn memory(&mut self) -> &Program {
        &self.program
    }

    pub fn memory_mut(&mut self) -> &mut Program {
        &mut self.program
    }

    fn exec_read(&self, address: Address) -> Result<usize, ExecutionErrorInner> {
        self.program.try_read(address).ok_or_else(|| ExecutionErrorInner::OutOfBoundsAccess { address, position: self.pc })
    }

    fn read_param(&self, modes: ParameterModes, idx: usize) -> Result<usize, ExecutionErrorInner> {
        let mode = modes.for_param(idx).context(BadParameterMode { position: self.pc, index: idx })?;
        let parameter = self.exec_read(self.pc.param(idx))?;

        match mode {
            ParameterMode::Position => {
                let address = Address::new(parameter);
                self.exec_read(address)
            }
            ParameterMode::Immediate => Ok(parameter),
        }
    }

    fn exec_write(&mut self, address: Address, value: usize) -> Result<usize, ExecutionErrorInner> {
        self.program.try_write(address, value).ok_or_else(|| ExecutionErrorInner::OutOfBoundsAccess { address, position: self.pc })
    }

    pub fn execute(&mut self) -> Result<(), ExecutionError> {
        while self.pc.address() < self.program.max_address() {
            let opcode = OpCode::try_from(self.exec_read(self.pc.address())?).context(Operation { position: self.pc })?;
    
            match opcode {
                OpCode::Halt => break,
                OpCode::Add => self.execute_binary_op(ops::Add::add)?,
                OpCode::Mul => self.execute_binary_op(ops::Mul::mul)?,
                OpCode::Input => self.execute_io_op(Self::execute_input)?,
                OpCode::Output => self.execute_io_op(Self::execute_output)?,
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

        Ok(BinOperands {
            values: (self.read_param(modes, 0)?, self.read_param(modes, 1)?),
            destination: Address::new(self.exec_read(self.pc.param(2))?),
        })
    }    

    fn execute_binary_op(&mut self, f: fn(usize, usize) -> usize) -> Result<(), ExecutionErrorInner> {
        let operands = self.get_binary_operands()?;
        let result = f(operands.values.0, operands.values.1);

        self.exec_write(operands.destination, result)?;

        Ok(self.pc.advance(4))
    }

    fn execute_io_op(&mut self, f: fn(&mut Executable, Address) -> Result<(), ExecutionErrorInner>) -> Result<(), ExecutionErrorInner> {
        let address = Address::new(self.exec_read(self.pc.param(0))?);
        f(self, address)?;
        Ok(self.pc.advance(2))
    }
    
    fn execute_input(&mut self, address: Address) -> Result<(), ExecutionErrorInner> {
        let value = self.input.pop_front().expect("unexpected end of input");
        self.exec_write(address, value)?;
        Ok(())
    }

    fn execute_output(&mut self, address: Address) -> Result<(), ExecutionErrorInner>  {
        let value = self.exec_read(address)?;
        self.output.push(value);
        Ok(())
    }
}