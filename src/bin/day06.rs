use advent_of_code_2019::day06;
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    let orbits = day06::parse_input(day06::PUZZLE_INPUT)?;

    let tree = day06::OrbitTree::from_orbits(orbits);

    let checksum = tree.checksum().expect("orbit tree to have a checksum");

    println!("Orbital checksum: {}", checksum);

    let minimal_transfers = tree.find_minimal_orbital_transfers("YOU", "SAN");

    println!(
        "Transfers to move between YOU and SAN: {:?}",
        minimal_transfers
    );

    Ok(())
}
