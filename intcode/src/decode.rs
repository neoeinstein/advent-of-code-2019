use super::{
    error,
    execute::{self, ExecutionErrorInner},
    ops::{Instruction, OpCode, ParameterMode},
    Address, Memory, ProgramCounter, Relative, Word,
};
use arrayvec::ArrayVec;
use snafu::{ResultExt, Snafu};
use std::{convert::TryFrom, fmt};

macro_rules! decode_impl {
    ($pc:expr, $modes:expr, $memory:expr) => {{
        ensure_params($pc, Self::READ_PARAMS + Self::WRITE_PARAMS, $memory)
            .context(execute::OutOfBoundsAccess { pc: $pc })?;

        let mut pmodes: ArrayVec<
            [ParameterMode; (Self::READ_PARAMS + Self::WRITE_PARAMS) as usize],
        > = ArrayVec::new();
        $modes.fill_params(&mut pmodes);

        let addr_ctx = execute::DecodeError { pc: $pc };

        let mut inputs: ArrayVec<[Parameter; Self::READ_PARAMS as usize]> = ArrayVec::new();
        for i in 0..Self::READ_PARAMS {
            let param_addr = $pc.param(i);
            let value = $memory
                .try_read(param_addr)
                .context(execute::OutOfBoundsAccess { pc: $pc })?;
            let param = Parameter::interpret(pmodes[i as usize], value).context(addr_ctx)?;
            inputs.push(param);
        }

        let mut outputs: ArrayVec<[Output; Self::WRITE_PARAMS as usize]> = ArrayVec::new();
        for i in Self::READ_PARAMS..(Self::READ_PARAMS + Self::WRITE_PARAMS) {
            let param_addr = $pc.param(i);
            let value = $memory
                .try_read(param_addr)
                .context(execute::OutOfBoundsAccess { pc: $pc })?;
            let addr = Output::interpret(pmodes[i as usize], value).context(addr_ctx)?;
            outputs.push(addr);
        }

        debug_assert!(inputs.is_full() && outputs.is_full());

        (inputs.into_inner().unwrap(), outputs.into_inner().unwrap())
    }};
}

#[inline(always)]
fn ensure_params(
    pc: ProgramCounter,
    params: u8,
    memory: &Memory,
) -> Result<(), error::OutOfBoundsAccess> {
    let last_param = pc.param(params - 1);
    if last_param <= memory.max_address() {
        Ok(())
    } else {
        Err(error::OutOfBoundsAccess::new(last_param))
    }
}

trait Decodable: Sized {
    const WRITE_PARAMS: u8;
    const READ_PARAMS: u8;
    //fn decode2(instruction: Instruction, pc: ProgramCounter, memory: &Memory) ->
    // Result<(ArrayVec<[Parameter; Self::READ_PARAMS]>, ArrayVec<[Address;
    // Self::WRITE_PARAMS]>), ExecutionErrorInner>;
    fn decode(
        i: Instruction,
        pc: ProgramCounter,
        memory: &Memory,
    ) -> Result<Self, ExecutionErrorInner>;
}

/// Operands for an instruction that has two inputs and one output
#[derive(Debug, PartialEq, Eq)]
pub struct BinaryOperands {
    pub left: Parameter,
    pub right: Parameter,
    pub target: Output,
}

impl Decodable for BinaryOperands {
    const READ_PARAMS: u8 = 2;
    const WRITE_PARAMS: u8 = 1;
    fn decode(
        i: Instruction,
        pc: ProgramCounter,
        memory: &Memory,
    ) -> Result<BinaryOperands, ExecutionErrorInner> {
        let (inputs, outputs) = decode_impl!(pc, i.param_modes(), memory);

        Ok(BinaryOperands {
            left: inputs[0],
            right: inputs[1],
            target: outputs[0],
        })
    }
}

impl fmt::Display for BinaryOperands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {} => {}", self.left, self.right, self.target)
    }
}

/// Operands for an instruction that has two inputs and no outputs
#[derive(Debug, PartialEq, Eq)]
pub struct JumpIfOperands {
    pub value: Parameter,
    pub jump_target: Parameter,
}

impl Decodable for JumpIfOperands {
    const READ_PARAMS: u8 = 2;
    const WRITE_PARAMS: u8 = 0;
    fn decode(
        i: Instruction,
        pc: ProgramCounter,
        memory: &Memory,
    ) -> Result<JumpIfOperands, ExecutionErrorInner> {
        let (inputs, _) = decode_impl!(pc, i.param_modes(), memory);

        Ok(JumpIfOperands {
            value: inputs[0],
            jump_target: inputs[1],
        })
    }
}

impl fmt::Display for JumpIfOperands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ~> {}", self.value, self.jump_target)
    }
}

/// Operands for an instruction that has only one output
#[derive(Debug, PartialEq, Eq)]
pub struct InputOperands {
    pub target: Output,
}

impl Decodable for InputOperands {
    const READ_PARAMS: u8 = 0;
    const WRITE_PARAMS: u8 = 1;
    fn decode(
        i: Instruction,
        pc: ProgramCounter,
        memory: &Memory,
    ) -> Result<InputOperands, ExecutionErrorInner> {
        let (_, outputs) = decode_impl!(pc, i.param_modes(), memory);

        Ok(InputOperands { target: outputs[0] })
    }
}

impl fmt::Display for InputOperands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.target.fmt(f)
    }
}

/// Operands for an instruction that has only one input
#[derive(Debug, PartialEq, Eq)]
pub struct OutputOperands {
    pub source: Parameter,
}

impl Decodable for OutputOperands {
    const READ_PARAMS: u8 = 1;
    const WRITE_PARAMS: u8 = 0;
    fn decode(
        i: Instruction,
        pc: ProgramCounter,
        memory: &Memory,
    ) -> Result<OutputOperands, ExecutionErrorInner> {
        let (inputs, _) = decode_impl!(pc, i.param_modes(), memory);

        Ok(OutputOperands { source: inputs[0] })
    }
}

impl fmt::Display for OutputOperands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.source.fmt(f)
    }
}

/// Operands for an instruction that has one input and one output
#[derive(Debug, PartialEq, Eq)]
pub struct UnaryOperands {
    pub value: Parameter,
    pub target: Output,
}

impl Decodable for UnaryOperands {
    const READ_PARAMS: u8 = 1;
    const WRITE_PARAMS: u8 = 1;
    fn decode(
        i: Instruction,
        pc: ProgramCounter,
        memory: &Memory,
    ) -> Result<UnaryOperands, ExecutionErrorInner> {
        let (inputs, outputs) = decode_impl!(pc, i.param_modes(), memory);

        Ok(UnaryOperands {
            value: inputs[0],
            target: outputs[0],
        })
    }
}

impl fmt::Display for UnaryOperands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", self.value, self.target)
    }
}

/// A decoded instruction with parameters
#[derive(Debug, PartialEq, Eq)]
pub enum Decoded {
    Halt,
    Add(BinaryOperands),
    Mul(BinaryOperands),
    Input(InputOperands),
    Output(OutputOperands),
    JumpNonZero(JumpIfOperands),
    JumpZero(JumpIfOperands),
    LessThan(BinaryOperands),
    Equal(BinaryOperands),
    AddRel(OutputOperands),
}

impl fmt::Display for Decoded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Decoded::Halt => "halt".fmt(f),
            Decoded::Add(ops) => write!(f, "add {}", ops),
            Decoded::Mul(ops) => write!(f, "mul {}", ops),
            Decoded::Input(ops) => write!(f, "read => {}", ops),
            Decoded::Output(ops) => write!(f, "write {} =>", ops),
            Decoded::JumpNonZero(ops) => write!(f, "jnz {}", ops),
            Decoded::JumpZero(ops) => write!(f, "jz {}", ops),
            Decoded::LessThan(ops) => write!(f, "lt {}", ops),
            Decoded::Equal(ops) => write!(f, "eq {}", ops),
            Decoded::AddRel(ops) => write!(f, "add rel, {} => rel", ops),
        }
    }
}

pub(crate) fn decode(
    i: Instruction,
    pc: ProgramCounter,
    memory: &Memory,
) -> Result<Decoded, ExecutionErrorInner> {
    let decoded = match i.opcode() {
        OpCode::Halt => Decoded::Halt,
        OpCode::Add => Decoded::Add(Decodable::decode(i, pc, memory)?),
        OpCode::Mul => Decoded::Mul(Decodable::decode(i, pc, memory)?),
        OpCode::Input => Decoded::Input(Decodable::decode(i, pc, memory)?),
        OpCode::Output => Decoded::Output(Decodable::decode(i, pc, memory)?),
        OpCode::JumpNonZero => Decoded::JumpNonZero(Decodable::decode(i, pc, memory)?),
        OpCode::JumpZero => Decoded::JumpZero(Decodable::decode(i, pc, memory)?),
        OpCode::LessThan => Decoded::LessThan(Decodable::decode(i, pc, memory)?),
        OpCode::Equal => Decoded::Equal(Decodable::decode(i, pc, memory)?),
        OpCode::AddRel => Decoded::AddRel(Decodable::decode(i, pc, memory)?),
    };

    log::trace!("@{}: {}", pc, decoded);

    Ok(decoded)
}

/// An instruction input parameter
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parameter {
    Position(Address),
    Immediate(Word),
    Relative(Relative),
}

impl Parameter {
    fn interpret(mode: ParameterMode, value: Word) -> Result<Self, DecodeError> {
        match mode {
            ParameterMode::Position => Ok(Parameter::Position(
                Address::try_from(value).context(InvalidAddress)?,
            )),
            ParameterMode::Immediate => Ok(Parameter::Immediate(value)),
            ParameterMode::Relative => Ok(Parameter::Relative(Relative::from(value))),
        }
    }

    /// Loads value from memory
    pub fn load(
        self,
        relative_base: Address,
        memory: &Memory,
    ) -> Result<Word, error::InvalidAddress> {
        match self {
            Parameter::Position(addr) => Ok(memory.read_or_default(addr)),
            Parameter::Immediate(value) => Ok(value),
            Parameter::Relative(offset) => Ok(memory.read_or_default((relative_base + offset)?)),
        }
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Parameter::Position(addr) => write!(f, "({})", addr),
            Parameter::Immediate(value) => write!(f, "${}", value),
            Parameter::Relative(offset) => write!(f, "(rel{})", offset),
        }
    }
}

/// An instruction output parameter
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Output {
    Position(Address),
    Relative(Relative),
}

impl Output {
    fn interpret(mode: ParameterMode, value: Word) -> Result<Self, DecodeError> {
        match mode {
            ParameterMode::Position => Ok(Output::Position(
                Address::try_from(value).context(InvalidAddress)?,
            )),
            ParameterMode::Immediate => Err(DecodeError::InvalidOutputMode { mode }),
            ParameterMode::Relative => Ok(Output::Relative(Relative::from(value))),
        }
    }

    /// Stores a value to memory
    pub fn store(self, relative_base: Address, memory: &mut Memory, value: Word) -> Word {
        match self {
            Output::Position(addr) => memory.write_arbitrary(addr, value),
            Output::Relative(offset) => {
                memory.write_arbitrary((relative_base + offset).expect("valid address"), value)
            }
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Output::Position(addr) => write!(f, "({})", addr),
            Output::Relative(offset) => write!(f, "(rel{})", offset),
        }
    }
}

#[derive(Snafu, Debug)]
pub enum DecodeError {
    #[snafu(display("instruction refers to illegal address"))]
    InvalidAddress { source: error::InvalidAddress },
    #[snafu(display("invalid mode for instruction output (mode = {})", mode,))]
    InvalidOutputMode { mode: ParameterMode },
}
