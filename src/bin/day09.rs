use advent_of_code_2019::day09;
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    let memory = intcode::Memory::from_str(day09::PUZZLE_INPUT)?;

    let diagnostic = day09::run_diagnostic(memory.clone())?;

    println!("Diagnostic result: {}", diagnostic);

    let boost = day09::run_boost(memory)?;

    println!("BOOST result: {}", boost);

    Ok(())
}
