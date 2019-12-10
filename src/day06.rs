//! # Day 6: Universal Orbit Map
//!
//! You've landed at the Universal Orbit Map facility on Mercury. Because
//! navigation in space often involves transferring between orbits, the orbit
//! maps here are useful for finding efficient routes between, for example, you
//! and Santa. You download a map of the local orbits (your puzzle input).
//!
//! Except for the universal Center of Mass (COM), every object in space is in
//! orbit around exactly one other object. An orbit looks roughly like this:
//!
//! ```text
//!                   \
//!                    \
//!                     |
//!                     |
//! AAA--> o            o <--BBB
//!                     |
//!                     |
//!                    /
//!                   /
//! ```
//!
//! In this diagram, the object BBB is in orbit around AAA. The path that BBB
//! takes around AAA (drawn with lines) is only partly shown. In the map data,
//! this orbital relationship is written AAA)BBB, which means "BBB is in orbit
//! around AAA".
//!
//! Before you use your map data to plot a course, you need to make sure it
//! wasn't corrupted during the download. To verify maps, the Universal Orbit
//! Map facility uses orbit count checksums - the total number of direct orbits
//! (like the one shown above) and indirect orbits.
//!
//! Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain
//! can be any number of objects long: if A orbits B, B orbits C, and C orbits
//! D, then A indirectly orbits D.
//!
//! For example, suppose you have the following map:
//!
//! ```text
//! COM)B
//! B)C
//! C)D
//! D)E
//! E)F
//! B)G
//! G)H
//! D)I
//! E)J
//! J)K
//! K)L
//! ```
//!
//! Visually, the above map of orbits looks like this:
//!
//! ```text
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I
//! ```
//!
//! In this visual representation, when two objects are connected by a line, the
//! one on the right directly orbits the one on the left.
//!
//! Here, we can count the total number of orbits as follows:
//!
//! * D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.
//! * L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a total
//!   of 7 orbits.
//! * COM orbits nothing.
//!
//! The total number of direct and indirect orbits in this example is 42.
//!
//! What is the total number of direct and indirect orbits in your map data?
//!
//! ## Part Two
//!
//! Now, you just need to figure out how many orbital transfers you (YOU) need
//! to take to get to Santa (SAN).
//!
//! You start at the object YOU are orbiting; your destination is the object SAN
//! is orbiting. An orbital transfer lets you move from any object to an object
//! orbiting or orbited by that object.
//!
//! For example, suppose you have the following map:
//!
//! ```text
//! COM)B
//! B)C
//! C)D
//! D)E
//! E)F
//! B)G
//! G)H
//! D)I
//! E)J
//! J)K
//! K)L
//! K)YOU
//! I)SAN
//! ```
//!
//! Visually, the above map of orbits looks like this:
//!
//! ```text
//!                           YOU
//!                          /
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I - SAN
//! ```
//!
//! In this example, YOU are in orbit around K, and SAN is in orbit around I. To
//! move from K to I, a minimum of 4 orbital transfers are required:
//!
//! * K to J
//! * J to E
//! * E to D
//! * D to I
//!
//! Afterward, the map of orbits looks like this:
//!
//! ```text
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I - SAN
//!                  \
//!                   YOU
//! ```
//!
//! What is the minimum number of orbital transfers required to move from the
//! object YOU are orbiting to the object SAN is orbiting? (Between the objects
//! they are orbiting - not between YOU and SAN.)

pub const PUZZLE_INPUT: &str = include_str!("../inputs/input-06");

use anyhow::Result;
use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, BufRead},
    rc::Rc,
};

#[derive(Clone, Debug)]
pub struct Orbit {
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
        let (next, name) = self
            .0
            .inner
            .borrow()
            .as_ref()
            .map(|i| (Rc::clone(&i.node), i.name.clone()))?;

        self.0 = next;
        Some(name)
    }
}

#[derive(Clone, Debug)]
pub struct OrbitTree {
    orbit_map: HashMap<String, Rc<OrbitNode>>,
}

impl OrbitTree {
    pub fn from_orbits(orbits: impl IntoIterator<Item = Orbit>) -> Self {
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

    pub fn checksum(&self) -> Option<usize> {
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
        for (l, r) in left_chain
            .into_iter()
            .rev()
            .zip(right_chain.into_iter().rev())
        {
            if l != r {
                return last;
            } else {
                last = Some(l);
            }
        }

        last
    }

    pub fn find_minimal_orbital_transfers(&self, name1: &str, name2: &str) -> Option<usize> {
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

fn checksum_iter(nodes: impl IntoIterator<Item = Rc<OrbitNode>>) -> usize {
    nodes.into_iter().map(checksum_node).sum()
}

fn parse_line(line: String) -> Orbit {
    let mut parts = line.splitn(2, ')');
    Orbit {
        planet: String::from(parts.next().unwrap().trim()),
        satelite: String::from(parts.next().unwrap().trim()),
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Orbit>> {
    let orbits: Vec<Orbit> = io::Cursor::new(input)
        .lines()
        .filter(|r| r.is_err() || !r.as_ref().unwrap().is_empty())
        .map(|r| r.map(parse_line).map_err(anyhow::Error::from))
        .collect::<Result<Vec<Orbit>>>()?;

    Ok(orbits)
}

#[cfg(test)]
mod tests {
    use super::{parse_input, OrbitTree};
    use pretty_assertions::assert_eq;

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
        let orbits = parse_input(PUZ_6_PART_1_EXAMPLE).unwrap();

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
        let orbits = parse_input(super::PUZZLE_INPUT).unwrap();

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
        let orbits = parse_input(PUZ_6_PART_2_EXAMPLE).unwrap();

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
        let orbits = parse_input(super::PUZZLE_INPUT).unwrap();

        let actual = OrbitTree::from_orbits(orbits).find_minimal_orbital_transfers("YOU", "SAN");
        let expected = Some(562);

        assert_eq!(expected, actual);
    }
}
