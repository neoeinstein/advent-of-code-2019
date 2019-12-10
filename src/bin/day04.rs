use advent_of_code_2019::day04;

fn main() {
    env_logger::init();
    let range = day04::parse_input();

    let result = day04::find_valid_passwords_part_1(range.clone());
    println!("Valid passwords: {}", result);

    let result = day04::find_valid_passwords_part_2(range);
    println!("Valid passwords (part 2): {}", result);
}
