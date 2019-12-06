use std::{cell::RefCell, collections::HashMap, io::BufRead, rc::Rc};
use anyhow::Result;
use advent_of_code_2019::get_input_reader;

#[derive(Clone, Debug)]
struct Orbit {
    satelite: String,
    planet: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OrbitInner {
    node: Rc<OrbitNode>,
    name: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct OrbitNode {
    inner: RefCell<Option<OrbitInner>>,
}

struct OrbitIterator(Rc<OrbitNode>);

impl Iterator for OrbitIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let (next, name) =
            self.0.inner.borrow().as_ref()
                .map(|i| (Rc::clone(&i.node), i.name.clone()))?;
        
        self.0 = next;
        Some(name)
    }
}

#[derive(Clone, Debug)]
struct OrbitTree {
    orbit_map: HashMap<String, Rc<OrbitNode>>,
}

impl OrbitTree {
    fn from_orbits(orbits: impl IntoIterator<Item = Orbit>) -> Self {
        let mut orbit_map = HashMap::<String, Rc<OrbitNode>>::new();
        for o in orbits {
            let planet_name = o.planet.clone();
            let planet_node = orbit_map.entry(o.planet).or_default();
            let inner = OrbitInner {
                node: Rc::clone(planet_node),
                name: planet_name,
            };

            let satelite = orbit_map.entry(o.satelite).or_default();
            debug_assert_eq!(&None, &*satelite.inner.borrow());
            
            *satelite.inner.borrow_mut() = Some(inner);
        }

        Self { orbit_map }
    }

    #[cfg(test)]
    fn checksum_node(&self, name: &str) -> Option<usize> {
        self.orbit_map.get(name).cloned().map(checksum_node)
    }

    fn checksum(&self) -> Option<usize> {
        if self.orbit_map.is_empty() {
            return None;
        }

        Some(checksum_iter(self.orbit_map.values().cloned()))
    }

    #[cfg(test)]
    fn find_most_recent_common_ancestor(&self, name1: &str, name2: &str) -> Option<String> {
        let left = self.orbit_map.get(name1)?;
        let mut left_chain = vec![String::from(name1)];
        left_chain.extend(OrbitIterator(Rc::clone(left)));

        let right = self.orbit_map.get(name2)?;
        let mut right_chain = vec![String::from(name2)];
        right_chain.extend(OrbitIterator(Rc::clone(right)));

        let mut last = None;
        for (l, r) in left_chain.into_iter().rev().zip(right_chain.into_iter().rev()) {
            if l != r {
                return last;
            } else {
                last = Some(l);
            }
        }

        last
    }

    fn find_minimal_orbital_transfers(&self, name1: &str, name2: &str) -> Option<usize> {
        let left = self.orbit_map.get(name1)?;
        let mut left_chain = vec![String::from(name1)];
        left_chain.extend(OrbitIterator(Rc::clone(left)));

        let right = self.orbit_map.get(name2)?;
        let mut right_chain = vec![String::from(name2)];
        right_chain.extend(OrbitIterator(Rc::clone(right)));

        let mut left_iter = left_chain.into_iter().rev();
        let mut right_iter = right_chain.into_iter().rev();

        let mut l = left_iter.next();
        let mut r = right_iter.next();

        if l != r || (l.is_none() && r.is_none()) {
            return None;
        }

        while l == r {
            l = left_iter.next();
            r = right_iter.next();
        }

        Some(left_iter.count() + right_iter.count())
    }
}

fn checksum_node(initial: Rc<OrbitNode>) -> usize {
    let mut current = initial;
    let mut count = 0;
    loop {
        let next = current.inner.borrow().as_ref().map(|n| Rc::clone(&n.node));
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

fn checksum_iter<'a>(nodes: impl IntoIterator<Item = Rc<OrbitNode>>) -> usize {
    nodes.into_iter()
        .map(checksum_node)
        .sum()
}

fn parse_line(line: String) -> Orbit {
    let mut parts = line.splitn(2, ')');
    Orbit {
        planet: String::from(parts.next().unwrap().trim()),
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

    let checksum = tree.checksum()
        .expect("orbit tree to have a checksum");

    println!("Orbital checksum: {}", checksum);

    let minimal_transfers = tree.find_minimal_orbital_transfers("YOU", "SAN");

    println!("Transfers to move between YOU and SAN: {:?}", minimal_transfers);

    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::io;
    use super::{OrbitTree, parse_input};

    const PUZ_6_PART_1_EXAMPLE: &str = "
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

    fn get_part_1_tree() -> OrbitTree {
        let orbits = parse_input(&mut io::Cursor::new(PUZ_6_PART_1_EXAMPLE)).unwrap();

        OrbitTree::from_orbits(orbits)
    }
    
    #[test]
    fn verify_checksum_d() {
        let actual = get_part_1_tree().checksum_node("D").unwrap();
        const EXPECTED: usize = 3;

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn verify_checksum_l() {
        let actual = get_part_1_tree().checksum_node("L").unwrap();
        const EXPECTED: usize = 7;

        assert_eq!(EXPECTED, actual);
    }
        
    #[test]
    fn verify_checksum_com() {
        let actual = get_part_1_tree().checksum_node("COM").unwrap();
        const EXPECTED: usize = 0;

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn verify_checksum_example() {
        let actual = get_part_1_tree().checksum().unwrap();
        const EXPECTED: usize = 42;

        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn verify_part_1() {
        let orbits = parse_input(&mut io::Cursor::new(include_str!("../../inputs/input-06"))).unwrap();

        let actual = OrbitTree::from_orbits(orbits).checksum().unwrap();
        const EXPECTED: usize = 453028;

        assert_eq!(EXPECTED, actual);
    }

    const PUZ_6_PART_2_EXAMPLE: &str = "
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
        K)L
        K)YOU
        I)SAN";

    fn get_part_2_tree() -> OrbitTree {
        let orbits = parse_input(&mut io::Cursor::new(PUZ_6_PART_2_EXAMPLE)).unwrap();

        OrbitTree::from_orbits(orbits)
    }

    #[test]
    fn find_most_recent_common_ancestor() {
        let actual = get_part_2_tree().find_most_recent_common_ancestor("YOU", "SAN");
        let expected = Some(String::from("D"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn find_minimal_orbital_transfers() {
        let actual = get_part_2_tree().find_minimal_orbital_transfers("YOU", "SAN");
        let expected = Some(4);

        assert_eq!(expected, actual);
    }

    #[test]
    fn find_minimal_orbital_transfers_part_2() {
        let orbits = parse_input(&mut io::Cursor::new(include_str!("../../inputs/input-06"))).unwrap();

        let actual = OrbitTree::from_orbits(orbits)
            .find_minimal_orbital_transfers("YOU", "SAN");
        let expected = Some(562);

        assert_eq!(expected, actual);
    }
}