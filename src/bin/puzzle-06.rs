use std::{cell::RefCell, collections::HashMap, io::BufRead, rc::Rc};
use anyhow::Result;
use advent_of_code_2019::get_input_reader;

#[derive(Clone, Debug)]
struct Orbit {
    satelite: String,
    inner: String,
}

#[derive(Clone, Debug, Default)]
struct OrbitTree {
    inner: Option<Rc<RefCell<OrbitTree>>>,
}

impl OrbitTree {
    fn from_orbits(orbits: impl IntoIterator<Item = Orbit>) -> HashMap<String, Rc<RefCell<Self>>> {
        let mut non_leaves = Vec::new();
        let mut orbit_map = HashMap::<String, Rc<RefCell<OrbitTree>>>::new();
        for o in orbits {
            let name = o.inner.clone();
            let inner = Rc::clone(orbit_map.entry(o.inner).or_default());
            let satelite = orbit_map.entry(o.satelite).or_default();
            satelite.borrow_mut().inner = Some(inner);
            non_leaves.push(name);
        }
        // for nl in non_leaves {
        //     orbit_map.remove(&nl);
        // }

        orbit_map
    }

    fn checksum(initial: &Rc<RefCell<Self>>) -> usize {
        let mut current = Rc::clone(initial);
        let mut count = 0;
        loop {
            let next = current.borrow().inner.as_ref().map(Rc::clone);
            match next {
                Some(n) => {
                    std::mem::replace(&mut current, n);
                    count += 1;
                }
                None => break,
            }
        }

        count
    }

    fn checksum_leaves<'a>(leaves: impl IntoIterator<Item = &'a Rc<RefCell<Self>>>) -> usize {
        leaves.into_iter()
            .map(Self::checksum)
            .sum()
    }
}

fn parse_line(line: String) -> Orbit {
    let mut parts = line.splitn(2, ')');
    Orbit {
        inner: String::from(parts.next().unwrap().trim()),
        satelite: String::from(parts.next().unwrap().trim()),
    }
}

fn parse_input(reader: impl BufRead) -> Result<Vec<Orbit>> {
    let orbits: Vec<Orbit> = reader.lines()
        .filter(|r| r.is_err() || !r.as_ref().unwrap().is_empty())
        .map(|r| r.map(parse_line).map_err(anyhow::Error::from))
        .collect::<Result<Vec<Orbit>>>()?;

    Ok(orbits)
}

fn main() -> Result<()> {
    let in_fd = get_input_reader();
    let orbits = parse_input(in_fd)?;

    let tree = OrbitTree::from_orbits(orbits);
        
    let checksum = OrbitTree::checksum_leaves(tree.values());

    println!("Orbital checksum: {}", checksum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::io;
    use super::{OrbitTree, parse_input};

    const PUZ_6_EXAMPLE: &str = "
        COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L";
    
    #[test]
    fn verify_checksum_d() {
        let orbits = parse_input(&mut io::Cursor::new(PUZ_6_EXAMPLE)).unwrap();

        let tree = OrbitTree::from_orbits(orbits);
        
        let actual = OrbitTree::checksum(tree.get("D").unwrap());
        const EXPECTED: usize = 3;

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn verify_checksum_l() {
        let orbits = parse_input(&mut io::Cursor::new(PUZ_6_EXAMPLE)).unwrap();

        let tree = OrbitTree::from_orbits(orbits);
        
        let actual = OrbitTree::checksum(tree.get("L").unwrap());
        const EXPECTED: usize = 7;

        assert_eq!(EXPECTED, actual);
    }
        
    #[test]
    fn verify_checksum_com() {
        let orbits = parse_input(&mut io::Cursor::new(PUZ_6_EXAMPLE)).unwrap();

        let tree = OrbitTree::from_orbits(orbits);
        
        let actual = OrbitTree::checksum(tree.get("COM").unwrap());
        const EXPECTED: usize = 0;

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn verify_checksum_example() {
        let orbits = parse_input(&mut io::Cursor::new(PUZ_6_EXAMPLE)).unwrap();

        let tree = OrbitTree::from_orbits(orbits);
        
        let actual = OrbitTree::checksum_leaves(tree.values());
        const EXPECTED: usize = 42;

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn verify_part_1() {
        let orbits = parse_input(&mut io::Cursor::new(include_str!("../../inputs/input-06"))).unwrap();

        let tree = OrbitTree::from_orbits(orbits);
        
        let actual = OrbitTree::checksum_leaves(tree.values());
        const EXPECTED: usize = 453028;

        assert_eq!(EXPECTED, actual);
    }
}