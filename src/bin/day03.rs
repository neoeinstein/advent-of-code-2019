use advent_of_code_2019::day03::{self, CandidatePoint};

fn main() {
    env_logger::init();
    let wires = day03::parse_input();

    let manhattan = day03::find_nearest_intersection(
        &wires.0,
        &wires.1,
        CandidatePoint::manhattan_distance_from_origin,
    );
    println!(
        "Nearest intersection by manhattan distance: {:?}",
        manhattan
    );

    let wire_delay =
        day03::find_nearest_intersection(&wires.0, &wires.1, CandidatePoint::wire_delay_to_point);
    println!("Nearest intersection by wire delay: {:?}", wire_delay);
}
