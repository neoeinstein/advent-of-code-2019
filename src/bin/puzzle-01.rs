use std::io::BufRead;
use advent_of_code_2019::get_input_reader;

fn calculate_fuel(mass: usize) -> usize {
    (mass / 3).saturating_sub(2)
}

fn calculate_module_fuel(mut mass: usize) -> usize {
    let mut total_fuel = 0;
    
    while mass > 0 {
        mass = calculate_fuel(mass);
        total_fuel += mass;
    }

    total_fuel
}

fn parse_input() -> Vec<usize> {
    let in_fd = get_input_reader();
    in_fd.lines()
        .map(|l| l.expect("error reading line"))
        .filter(|l| !l.is_empty())
        .map(|mass_str| mass_str.parse().expect("data must be a valid integer"))
        .collect()
}

fn main() {
    let input = parse_input();
    let total_fuel_requirement: usize = input.into_iter()
        .map(calculate_module_fuel)
        .sum();
    
    println!("{}", total_fuel_requirement);
}
