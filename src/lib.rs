use std::{env, fs, io, path::PathBuf};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;

mod grid;
mod orientation;
mod position;

use grid::Grid;
use orientation::{Orientation, Turn};
use position::{GridPosition, Position2D};

fn get_input_filename() -> Option<PathBuf> {
    let in_file = env::args().nth(1)?;
    if in_file == "-" {
        None
    } else {
        Some(PathBuf::from(in_file))
    }
}

/// Creates an input reader from the file specified on the command line (STDIN
/// if not provided)
pub fn get_input_reader() -> Box<dyn io::BufRead> {
    match get_input_filename() {
        Some(in_file) => Box::new(io::BufReader::new(
            fs::File::open(&in_file).expect("file should be openable"),
        )),
        None => Box::new(io::BufReader::new(io::stdin())),
    }
}

pub fn run_intcode_program_single_in(
    program: intcode::Memory,
    input: intcode::Word,
) -> Result<Vec<intcode::Word>, intcode::ExecutionError> {
    let mut exe = intcode::Executable::from(program);

    exe.single_input(input);
    let drain = exe.drain();

    exe.execute()?;

    Ok(drain.to_vec())
}

pub fn run_intcode_program_single_in_single_out(
    program: intcode::Memory,
    input: intcode::Word,
) -> Result<intcode::Word, intcode::ExecutionError> {
    let results = run_intcode_program_single_in(program, input)?;

    Ok(results.last().copied().expect("one output"))
}

#[cfg(test)]
fn init_logging() {
    let _ = env_logger::builder().is_test(true).try_init();
}
