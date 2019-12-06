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
    let calc = if cfg!(feature = "part-1") {
        calculate_fuel
    } else {
        calculate_module_fuel
    };

    let input = parse_input();
    let total_fuel_requirement: usize = input.into_iter()
        .map(calc)
        .sum();
    
    println!("{}", total_fuel_requirement);
}

#[cfg(test)]
mod tests {
    use super::{calculate_fuel, calculate_module_fuel};
    use pretty_assertions::assert_eq;

    #[test]
    fn calculate_fuel_12() {
        const MASS: usize = 12;
        const EXPECTED: usize = 2;

        let actual = calculate_fuel(MASS);

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn calculate_fuel_14() {
        const MASS: usize = 14;
        const EXPECTED: usize = 2;

        let actual = calculate_fuel(MASS);

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn calculate_fuel_1969() {
        const MASS: usize = 1969;
        const EXPECTED: usize = 654;

        let actual = calculate_fuel(MASS);

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn calculate_fuel_100756() {
        const MASS: usize = 100756;
        const EXPECTED: usize = 33583;

        let actual = calculate_fuel(MASS);

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn calculate_module_fuel_12() {
        const MASS: usize = 12;
        const EXPECTED: usize = 2;

        let actual = calculate_module_fuel(MASS);

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn calculate_module_fuel_14() {
        const MASS: usize = 14;
        const EXPECTED: usize = 2;

        let actual = calculate_module_fuel(MASS);

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn calculate_module_fuel_1969() {
        const MASS: usize = 1969;
        const EXPECTED: usize = 966;

        let actual = calculate_module_fuel(MASS);

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn calculate_module_fuel_100756() {
        const MASS: usize = 100756;
        const EXPECTED: usize = 50346;

        let actual = calculate_module_fuel(MASS);

        assert_eq!(EXPECTED, actual);
    }
}