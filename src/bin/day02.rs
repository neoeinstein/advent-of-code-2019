use advent_of_code_2019::day02;
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    let memory = intcode::Memory::from_str(day02::PUZZLE_INPUT)?;

    let output = day02::run_with_specific_state(memory.clone(), 12, 2)?;
    println!("Diagnostic: (12, 02): {}", output);

    const TARGET: intcode::Word = 19_690_720;
    let (noun, verb) = day02::search_for_noun_and_verb(memory, TARGET)?;
    println!("Searching: ({:02}, {:02}): {}", noun, verb, TARGET);

    Ok(())
}
