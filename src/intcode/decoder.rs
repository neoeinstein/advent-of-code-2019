use super::ProgramValue;
use snafu::Snafu;
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug)]
pub struct OpCode {
    op: Operation,
    modes: ParameterModes,
}

impl OpCode {
    pub const fn op(self) -> Operation {
        self.op
    }

    pub const fn param_modes(self) -> ParameterModes {
        self.modes
    }
}

impl TryFrom<ProgramValue> for OpCode {
    type Error = InvalidOpCode;
    fn try_from(value: ProgramValue) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(InvalidOpCode::NegativeValue { opcode: value });
        }
        let opcode = value as usize;

        let op = Operation::try_from(opcode)?;
        let modes = ParameterModes::try_from(opcode)?;

        Ok(Self { op, modes })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Halt,
    Add,
    Mul,
    Input,
    Output,
    JumpNonZero,
    JumpZero,
    LessThan,
    Equal,
}

impl TryFrom<usize> for Operation {
    type Error = InvalidOpCode;
    fn try_from(opcode: usize) -> Result<Self, Self::Error> {
        let code = opcode % 100;
        let op = match code {
            1 => Operation::Add,
            2 => Operation::Mul,
            3 => Operation::Input,
            4 => Operation::Output,
            5 => Operation::JumpNonZero,
            6 => Operation::JumpZero,
            7 => Operation::LessThan,
            8 => Operation::Equal,
            99 => Operation::Halt,
            _ => return Err(InvalidOpCode::UnknownOpcode { opcode }),
        };

        Ok(op)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ParameterModes(usize);

impl ParameterModes {
    pub fn for_param(self, idx: u8) -> ParameterMode {
        ParameterMode::from_value((self.0 / (10usize.pow(idx as u32))) % 10)
            .expect("invalid parameter mode in pre-validated context")
    }
}

impl TryFrom<usize> for ParameterModes {
    type Error = InvalidOpCode;
    fn try_from(opcode: usize) -> Result<Self, Self::Error> {
        let modes = opcode / 100;
        let mut m = modes;
        let mut idx = 0;
        while m != 0 {
            if ParameterMode::from_value(m % 10).is_none() {
                return Err(InvalidOpCode::InvalidParameterMode { opcode, index: idx });
            }
            m /= 10;
            idx += 1;
        }
        Ok(Self(modes))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ParameterMode {
    Position,
    Immediate,
}

impl Default for ParameterMode {
    fn default() -> Self {
        ParameterMode::Position
    }
}

impl ParameterMode {
    fn from_value(mode: usize) -> Option<Self> {
        match mode {
            0 => Some(ParameterMode::Position),
            1 => Some(ParameterMode::Immediate),
            _ => None,
        }
    }
}

#[derive(Snafu, Debug)]
pub enum InvalidOpCode {
    #[snafu(display("opcode cannot be negative (opcode = {})", opcode))]
    NegativeValue { opcode: ProgramValue },
    #[snafu(display("unknown opcode (opcode = {})", opcode))]
    UnknownOpcode { opcode: usize },
    #[snafu(display(
        "unknown parameter mode (opcode = {}, parameter index = {})",
        opcode,
        index
    ))]
    InvalidParameterMode { opcode: usize, index: usize },
}
