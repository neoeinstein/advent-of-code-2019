use super::Word;
use arrayvec::{Array, ArrayVec};
use snafu::Snafu;
use std::{convert::TryFrom, fmt};

/// Describes the instruction, including the operation to be executed as well as
/// how to interpret any parameters that have been provided
#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    opcode: OpCode,
    modes: ParameterModes,
}

impl Instruction {
    /// Gets the instruction's `OpCode`
    pub const fn opcode(self) -> OpCode {
        self.opcode
    }

    /// Gets a structure with information on how to interpret the instruction's
    /// parameters
    pub const fn param_modes(self) -> ParameterModes {
        self.modes
    }
}

impl TryFrom<Word> for Instruction {
    type Error = InvalidInstruction;
    fn try_from(raw: Word) -> Result<Self, Self::Error> {
        if raw < 0 {
            return Err(InvalidInstruction::NegativeValue { opcode: raw });
        }
        let raw = raw as usize;
        let opcode = OpCode::try_from(raw)?;
        let modes = ParameterModes::try_from(raw)?;

        Ok(Self { opcode, modes })
    }
}

/// Describes the operation to execute
///
/// The number in the description refers to the instruction stem (`instruction %
/// 100`), which specifies the operation to execute as well as implying the
/// quantity of parameters required.
#[derive(Clone, Copy, Debug)]
pub enum OpCode {
    /// (`99`) Halts the program
    Halt,
    /// (`01`) Adds params 0 and 1, stores result in param 2
    Add,
    /// (`02`) Multiplies params 0 and 1, stores result in param 2
    Mul,
    /// (`03`) Stores next input value in param 0
    Input,
    /// (`04`) Stores param 0 as next output value
    Output,
    /// (`05`) If param 0 is non-zero, jumps the program counter to param 1
    JumpNonZero,
    /// (`06`) If param 0 is zero, jumps the program counter to param 1
    JumpZero,
    /// (`07`) If param 0 is less than param 1, stores `1` in param 2, otherwise
    /// stores `0`
    LessThan,
    /// (`08`) If param 0 is equal to param 1, stores `1` in param 2, otherwise
    /// stores `0`
    Equal,
    /// (`09`) Adjusts the relative base register by the value in param 0
    AddRel,
}

impl TryFrom<usize> for OpCode {
    type Error = InvalidInstruction;
    fn try_from(opcode: usize) -> Result<Self, Self::Error> {
        let code = opcode % 100;
        let op = match code {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpNonZero,
            6 => OpCode::JumpZero,
            7 => OpCode::LessThan,
            8 => OpCode::Equal,
            9 => OpCode::AddRel,
            99 => OpCode::Halt,
            _ => return Err(InvalidInstruction::UnknownOpcode { opcode }),
        };

        Ok(op)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ParameterModes(usize);

impl ParameterModes {
    pub fn fill_params<A: Array<Item = ParameterMode>>(mut self, buf: &mut ArrayVec<A>) {
        while !buf.is_full() {
            let mode = ParameterMode::from_value(self.0 % 10)
                .expect("invalid parameter mode in pre-validated context");
            buf.push(mode);
            self.0 /= 10;
        }
    }
}

impl TryFrom<usize> for ParameterModes {
    type Error = InvalidInstruction;
    fn try_from(opcode: usize) -> Result<Self, Self::Error> {
        let modes = opcode / 100;
        let mut m = modes;
        let mut idx = 0;
        while m != 0 {
            if ParameterMode::from_value(m % 10).is_none() {
                return Err(InvalidInstruction::InvalidParameterMode { opcode, index: idx });
            }
            m /= 10;
            idx += 1;
        }
        Ok(Self(modes))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ParameterMode {
    /// The parameter is an address reference; the actual parameter value should
    /// be retrieved from that address
    Position,
    /// The parameter is the value to be used for the operation
    Immediate,
    /// The parameter is at an address relative to the current relative address
    /// base
    Relative,
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
            2 => Some(ParameterMode::Relative),
            _ => None,
        }
    }
}

impl fmt::Display for ParameterMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mode = match self {
            ParameterMode::Position => "position",
            ParameterMode::Immediate => "immediate",
            ParameterMode::Relative => "relative",
        };

        f.write_str(mode)
    }
}

#[derive(Snafu, Debug)]
pub enum InvalidInstruction {
    #[snafu(display("opcode cannot be negative (opcode = {})", opcode))]
    NegativeValue { opcode: Word },
    #[snafu(display("unknown opcode (opcode = {})", opcode))]
    UnknownOpcode { opcode: usize },
    #[snafu(display(
        "unknown parameter mode (opcode = {}, parameter index = {})",
        opcode,
        index
    ))]
    InvalidParameterMode { opcode: usize, index: usize },
}
