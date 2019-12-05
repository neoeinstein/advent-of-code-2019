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
    JumpNonZero,
    JumpZero,
    LessThan,
    Equal
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
            5 => OpCode::JumpNonZero,
            6 => OpCode::JumpZero,
            7 => OpCode::LessThan,
            8 => OpCode::Equal,
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

    fn jump(&mut self, address: Address) {
        self.0 = address.0
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
                OpCode::JumpNonZero => self.execute_jump_cond(true)?,
                OpCode::JumpZero => self.execute_jump_cond(false)?,
                OpCode::LessThan => self.execute_cmp(isize::lt)?,
                OpCode::Equal => self.execute_cmp(isize::eq)?,
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

    fn execute_jump_cond(&mut self, jump_if_non_zero: bool) -> Result<(), ExecutionErrorInner> {
        let modes = ParameterModes(self.exec_read(self.pc.address())? / 100);
        let value = self.read_param(modes, 0)?;

        if (value != 0) == jump_if_non_zero {
            let target = self.read_param(modes, 1)?;
            let address = Address::try_from_value(target).ok_or_else(|| ExecutionErrorInner::InvalidAddress { value: target, position: self.pc })?;
            
            self.pc.jump(address);
        } else {
            self.pc.advance(3);
        }

        Ok(())
    }

    fn execute_cmp(&mut self, f: fn(&isize, &isize) -> bool) -> Result<(), ExecutionErrorInner> {
        let operands = self.get_binary_operands()?;

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

#[cfg(test)]
mod tests {
    use std::io;
    use pretty_assertions::assert_eq;
    use super::{Program, Executable};
    use anyhow::Result;

    const POS_IS_INPUT_EQUAL_TO_8: &str = "3,9,8,9,10,9,4,9,99,-1,8";
    const POS_IS_INPUT_LESS_THAN_8: &str = "3,9,7,9,10,9,4,9,99,-1,8";
    const IMM_IS_INPUT_EQUAL_TO_8: &str = "3,3,1108,-1,8,3,4,3,99";
    const IMM_IS_INPUT_LESS_THAN_8: &str = "3,3,1107,-1,8,3,4,3,99";
    
    const TRUE: &[isize] = &[1];
    const ONE: &[isize] = &[1];
    const FALSE: &[isize] = &[0];
    const ZERO: &[isize] = &[0];

    #[test]
    fn position_mode_1_is_equal_to_8() -> Result<()> {
        let input = vec![1];

        run_program_test(POS_IS_INPUT_EQUAL_TO_8, input, FALSE)
    }

    #[test]
    fn position_mode_8_is_equal_to_8() -> Result<()> {
        let input = vec![8];

        run_program_test(POS_IS_INPUT_EQUAL_TO_8, input, TRUE)
    }

    #[test]
    fn position_mode_1_is_less_than_8() -> Result<()> {
        let input = vec![1];

        run_program_test(POS_IS_INPUT_LESS_THAN_8, input, TRUE)
    }

    #[test]
    fn position_mode_8_is_less_than_8() -> Result<()> {
        let input = vec![8];

        run_program_test(POS_IS_INPUT_LESS_THAN_8, input, FALSE)
    }

    #[test]
    fn immediate_mode_1_is_equal_to_8() -> Result<()> {
        let input = vec![1];

        run_program_test(IMM_IS_INPUT_EQUAL_TO_8, input, FALSE)
    }

    #[test]
    fn immediate_mode_8_is_equal_to_8() -> Result<()> {
        let input = vec![8];

        run_program_test(IMM_IS_INPUT_EQUAL_TO_8, input, TRUE)
    }

    #[test]
    fn immediate_mode_1_is_less_than_8() -> Result<()> {
        let input = vec![1];

        run_program_test(IMM_IS_INPUT_LESS_THAN_8, input, TRUE)
    }

    #[test]
    fn immediate_mode_8_is_less_than_8() -> Result<()> {
        let input = vec![8];

        run_program_test(IMM_IS_INPUT_LESS_THAN_8, input, FALSE)
    }

    const POS_JUMP_INPUT_WAS_ZERO: &str = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    const IMM_JUMP_INPUT_WAS_ZERO: &str = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";

    #[test]
    fn position_mode_jump_if_0_is_0() -> Result<()> {
        let input = vec![0];

        run_program_test(POS_JUMP_INPUT_WAS_ZERO, input, ZERO)
    }

    #[test]
    fn position_mode_jump_if_1_is_0() -> Result<()> {
        let input = vec![1];

        run_program_test(POS_JUMP_INPUT_WAS_ZERO, input, ONE)
    }

    #[test]
    fn immediate_mode_jump_if_0_is_0() -> Result<()> {
        let input = vec![0];

        run_program_test(IMM_JUMP_INPUT_WAS_ZERO, input, ZERO)
    }

    #[test]
    fn immediate_mode_jump_if_1_is_0() -> Result<()> {
        let input = vec![1];

        run_program_test(IMM_JUMP_INPUT_WAS_ZERO, input, ONE)
    }

    const PUZ_5_PART_2_EXAMPLE: &str = "
        3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

    #[test]
    fn compare_neg2_against_8() -> Result<()> {
        let input = vec![-2];

        run_program_test(PUZ_5_PART_2_EXAMPLE, input, &[999])
    }

    #[test]
    fn compare_8_against_8() -> Result<()> {
        let input = vec![8];

        run_program_test(PUZ_5_PART_2_EXAMPLE, input, &[1000])
    }

    #[test]
    fn compare_99_against_8() -> Result<()> {
        let input = vec![99];

        run_program_test(PUZ_5_PART_2_EXAMPLE, input, &[1001])
    }

    #[test]
    fn run_system_1_diagnostics() -> Result<()> {
        let exe = run_diagnostics(1)?;

        assert!(exe.output()[..exe.output().len()-2].iter().copied().all(|i| i == 0isize));

        Ok(())
    }

    #[test]
    fn run_system_5_diagnostics() -> Result<()> {
        let exe = run_diagnostics(5)?;

        assert_eq!(1, exe.output().len());

        Ok(())
    }

    fn run_diagnostics(system: isize) -> Result<Executable> {
        const PROGRAM: &str = include_str!("../inputs/input-05");

        let program = Program::from_reader(&mut io::Cursor::new(PROGRAM))?;

        let mut exe = Executable::from(program);

        exe.set_input(vec![system]);

        exe.execute()?;
        
        println!("system {} diagnostic code = {}", system, exe.output()[exe.output().len() - 1]);

        Ok(exe)
    }

    fn run_program_test(program_data: &str, input: Vec<isize>, expected: &[isize]) -> Result<()> {
        let program = Program::from_reader(&mut io::Cursor::new(program_data))?;

        let mut exe = Executable::from(program);

        exe.set_input(input);

        exe.execute()?;

        assert_eq!(expected, exe.output());

        Ok(())
    }
}
