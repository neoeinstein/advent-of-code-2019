use std::{convert::TryFrom, io, mem};
use thiserror::Error;

#[derive(Clone, Copy, Debug)]
struct BinOperands {
    sources: (usize, usize),
    destination: usize,
}

fn get_binary_operands(data: &[usize], op_start: usize) -> BinOperands {
    assert!(op_start + 2 < data.len());
    BinOperands {
        sources: (data[op_start], data[op_start + 1]),
        destination: data[op_start + 2],
    }
}

#[derive(Clone, Copy, Debug)]
enum OpCode {
    Halt,
    Add,
    Mul,
}

#[derive(Clone, Copy, Debug)]
struct InvalidOpCode;

impl TryFrom<usize> for OpCode {
    type Error = InvalidOpCode;
    fn try_from(op: usize) -> Result<OpCode, Self::Error> {
        match op {
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Mul),
            99 => Ok(OpCode::Halt),
            _ => Err(InvalidOpCode),
        }
    }
}

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("invalid opcode {opcode} at position {position}")]
    InvalidOpCode {
        opcode: usize,
        position: usize,
    }
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

    pub fn try_read(&self, address: usize) -> Option<usize> {
        self.data.get(address).copied()
    }

    pub fn try_write(&mut self, address: usize, value: usize) -> Option<usize> {
        let sloc = self.data.get_mut(address)?;
        Some(mem::replace(sloc, value))
    }

    pub fn execute(&self) -> Result<Program, ExecutionError> {
        let mut p = self.clone();
        p.execute_in_place()?;
        Ok(p)
    }

    pub fn execute_in_place(&mut self) -> Result<(), ExecutionError> {
        let mut pc: usize = 0;

        while pc < self.data.len() {
            let opcode = OpCode::try_from(self.data[pc]).map_err(|_| ExecutionError::InvalidOpCode { opcode: self.data[pc], position: pc })?;
    
            pc = match opcode {
                OpCode::Halt => break,
                OpCode::Add => self.execute_binary_op(pc, Self::execute_add),
                OpCode::Mul => self.execute_binary_op(pc, Self::execute_mul),
            };
        }

        Ok(())
    }

    fn execute_binary_op(&mut self, pc: usize, f: fn(&mut Program, BinOperands)) -> usize {
        let operands = get_binary_operands(&self.data, pc + 1);
        f(self, operands);
        pc + 4
    }
    
    fn execute_add(&mut self, operands: BinOperands) {
        self.data[operands.destination] = self.data[operands.sources.0] + self.data[operands.sources.1];
    }
    
    fn execute_mul(&mut self, operands: BinOperands) {
        self.data[operands.destination] = self.data[operands.sources.0] * self.data[operands.sources.1];
    }
    
}
