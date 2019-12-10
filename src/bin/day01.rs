use advent_of_code_2019::day01;

fn main() {
    env_logger::init();
    let input = day01::parse_input();

    let fuel_requirement: usize = input.iter().copied().map(day01::calculate_fuel).sum();
    println!("Fuel required: {}", fuel_requirement);

    let total_fuel_requirement: usize = input
        .iter()
        .copied()
        .map(day01::calculate_module_fuel)
        .sum();
    println!(
        "Total fuel required (including fuel weight): {}",
        total_fuel_requirement
    );
}
