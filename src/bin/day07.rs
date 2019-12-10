use advent_of_code_2019::day07;
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    let memory = intcode::Memory::from_str(day07::PUZZLE_INPUT)?;

    let (best_sequence, max) = day07::find_best_phase_sequence(&memory)?;

    println!("Best sequence: {:?}, end value = {}", best_sequence, max);

    let (best_sequence, max) = day07::find_best_phase_sequence_with_feedback(&memory)?;

    println!("Best sequence: {:?}, end value = {}", best_sequence, max);

    Ok(())
}
