use std::{
    convert::TryFrom,
    io::Read,
};
use advent_of_code_2019::get_input_reader;

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

fn execute_binary_op(data: &mut [usize], pc: usize, f: fn(&mut[usize], BinOperands)) -> usize {
    let operands = get_binary_operands(&*data, pc + 1);
    f(data, operands);
    pc + 4
}

fn execute_add(data: &mut [usize], operands: BinOperands) {
    data[operands.destination] = data[operands.sources.0] + data[operands.sources.1];
}

fn execute_mul(data: &mut [usize], operands: BinOperands) {
    data[operands.destination] = data[operands.sources.0] * data[operands.sources.1];
}

fn execute_program(initial: &[usize], noun: usize, verb: usize) -> usize {
    let mut data = Vec::from(initial);
    data[1] = noun;
    data[2] = verb;

    let mut pc: usize = 0;

    while pc < data.len() {
        let opcode = OpCode::try_from(data[pc]).expect(&format!("invalid opcode {} at position {}", data[pc], pc));

        pc = match opcode {
            OpCode::Halt => break,
            OpCode::Add => execute_binary_op(&mut data, pc, execute_add),
            OpCode::Mul => execute_binary_op(&mut data, pc, execute_mul),
        };
    }

    data[0]
}

fn parse_input() -> Vec<usize> {
    let mut in_fd = get_input_reader();
    let mut data = String::new();
    in_fd.read_to_string(&mut data).expect("error reading data");

    data.split(',')
        .filter(|op| !op.is_empty())
        .map(|op| op.trim().parse().expect("data must be a valid integer"))
        .collect()
}

fn main() {
    let input = parse_input();

    for noun in 0..99 {
        for verb in 0..99 {
            let output = execute_program(&input, noun, verb);
            if output == 19690720 {
                println!("({}, {}) = {}", noun, verb, noun * 100 + verb);
                return;
            }
        }
    }
}
