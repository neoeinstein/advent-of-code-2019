use advent_of_code_2019::{intcode,get_input_reader};
use anyhow::Result;

fn main() -> Result<()> {
    let program = intcode::Program::from_reader(&mut get_input_reader()).expect("error parsing program");

    let mut exe = intcode::Executable::from(program);

    exe.set_input(vec![1]);

    exe.execute()?;

    println!("Result: {:?}", exe.output());

    Ok(())
}
