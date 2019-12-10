use advent_of_code_2019::day05;
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    let memory = intcode::Memory::from_str(day05::PUZZLE_INPUT)?;

    let sys1 = day05::run_system_1_diagnostic(memory.clone())?;

    println!("System 1 diagnostic: {:?}", sys1);

    let sys5 = day05::run_system_5_diagnostic(memory)?;

    println!("System 5 diagnostic: {}", sys5);

    Ok(())
}
