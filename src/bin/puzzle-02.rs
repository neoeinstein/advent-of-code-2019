use advent_of_code_2019::{intcode,get_input_reader};
use anyhow::{anyhow, Result};

fn run_with_specific_state(program: intcode::Program, noun: usize, verb: usize) -> Result<usize> {
    let mut p = program.clone();
    p.try_write(1, noun);
    p.try_write(2, verb);

    p.execute_in_place()?;

    let output = p.try_read(0).ok_or_else(|| anyhow!("No data in location 0"))?;

    Ok(output)
} 

fn search_for_noun_and_verb(program: intcode::Program, target: usize) -> Result<()> {

    for noun in 0..99 {
        for verb in 0..99 {
            let output = run_with_specific_state(program.clone(), noun, verb)?;

            if output == target {
                println!("({}, {}) = {}", noun, verb, noun * 100 + verb);
                return Ok(());
            }
        }
    }

    return Err(anyhow!("Unable to find (noun, verb) pair that outputs {}", target));
}

fn main() -> Result<()> {
    let program = intcode::Program::from_reader(&mut get_input_reader()).expect("error parsing program");

    if cfg!(feature = "part-1") {
        let output = run_with_specific_state(program, 12, 2)?;
        println!("Result: {}", output);
    } else {
        search_for_noun_and_verb(program, 19690720)?;
    }

    Ok(())
}
