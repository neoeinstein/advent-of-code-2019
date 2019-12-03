use std::{env, fs, io::{self, BufRead}, path::{Path, PathBuf}};

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

fn read_input(filename: Option<&Path>) -> Vec<usize> {
    let in_fd: Box<dyn io::BufRead> = match filename {
        Some(in_file) => Box::new(io::BufReader::new(fs::File::open(in_file).expect("file should be openable"))),
        None => Box::new(io::BufReader::new(io::stdin())),
    };
    in_fd.lines()
        .map(|l| l.expect("error reading line"))
        .filter(|l| !l.is_empty())
        .map(|mass_str| mass_str.parse().expect("data must be a valid integer"))
        .collect()
}

fn get_input_filename() -> Option<PathBuf> {
    let in_file = env::args().skip(1).next()?;
    if in_file == "-" {
        None
    } else {
        Some(PathBuf::from(in_file))
    }
}

fn main() {
    let in_file = get_input_filename();
    let input = read_input(in_file.as_ref().map(|p| p.as_ref()));
    let total_fuel_requirement: usize = input.into_iter()
        .map(calculate_module_fuel)
        .sum();
    
    println!("{}", total_fuel_requirement);
}
