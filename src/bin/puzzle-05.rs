use advent_of_code_2019::{intcode,get_input_reader};
use anyhow::Result;

fn main() -> Result<()> {
    let program = intcode::Program::from_reader(&mut get_input_reader())?;

    let mut exe = intcode::Executable::from(program);

    let input = if cfg!(feature = "part-1") {
        1
    } else {
        5
    };

    exe.set_input(vec![input]);

    exe.execute()?;

    println!("Result: {:?}", exe.output());

    Ok(())
}
