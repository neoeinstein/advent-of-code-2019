//! Day 12: The N-Body Problem
//!
//! The space near Jupiter is not a very safe place; you need to be careful of a
//! big distracting red spot, extreme radiation, and a whole lot of moons
//! swirling around. You decide to start by tracking the four largest moons: Io,
//! Europa, Ganymede, and Callisto.
//!
//! After a brief scan, you calculate the position of each moon (your puzzle
//! input). You just need to simulate their motion so you can avoid them.
//!
//! Each moon has a 3-dimensional position (x, y, and z) and a 3-dimensional
//! velocity. The position of each moon is given in your scan; the x, y, and z
//! velocity of each moon starts at 0.
//!
//! Simulate the motion of the moons in time steps. Within each time step, first
//! update the velocity of every moon by applying gravity. Then, once all moons'
//! velocities have been updated, update the position of every moon by applying
//! velocity. Time progresses by one step once all of the positions are updated.
//!
//! To apply gravity, consider every pair of moons. On each axis (x, y, and z),
//! the velocity of each moon changes by exactly +1 or -1 to pull the moons
//! together. For example, if Ganymede has an x position of 3, and Callisto has
//! a x position of 5, then Ganymede's x velocity changes by +1 (because 5 > 3)
//! and Callisto's x velocity changes by -1 (because 3 < 5). However, if the
//! positions on a given axis are the same, the velocity on that axis does not
//! change for that pair of moons.
//!
//! Once all gravity has been applied, apply velocity: simply add the velocity
//! of each moon to its own position. For example, if Europa has a position of
//! x=1, y=2, z=3 and a velocity of x=-2, y=0,z=3, then its new position would
//! be x=-1, y=2, z=6. This process does not modify the velocity of any moon.
//!
//! For example, suppose your scan reveals the following positions:
//!
//! ```text
//! <x=-1, y=0, z=2>
//! <x=2, y=-10, z=-7>
//! <x=4, y=-8, z=8>
//! <x=3, y=5, z=-1>
//! ```
//!
//! Simulating the motion of these moons would produce the following:
//!
//! After 0 steps:
//!
//! ```text
//! pos=<x=-1, y=  0, z= 2>, vel=<x= 0, y= 0, z= 0>
//! pos=<x= 2, y=-10, z=-7>, vel=<x= 0, y= 0, z= 0>
//! pos=<x= 4, y= -8, z= 8>, vel=<x= 0, y= 0, z= 0>
//! pos=<x= 3, y=  5, z=-1>, vel=<x= 0, y= 0, z= 0>
//! ```
//!
//! After 1 step:
//!
//! ```text
//! pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>
//! pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>
//! pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>
//! pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>
//! ```
//!
//! After 2 steps:
//!
//! ```text
//! pos=<x= 5, y=-3, z=-1>, vel=<x= 3, y=-2, z=-2>
//! pos=<x= 1, y=-2, z= 2>, vel=<x=-2, y= 5, z= 6>
//! pos=<x= 1, y=-4, z=-1>, vel=<x= 0, y= 3, z=-6>
//! pos=<x= 1, y=-4, z= 2>, vel=<x=-1, y=-6, z= 2>
//! ```
//!
//! After 3 steps:
//!
//! ```text
//! pos=<x= 5, y=-6, z=-1>, vel=<x= 0, y=-3, z= 0>
//! pos=<x= 0, y= 0, z= 6>, vel=<x=-1, y= 2, z= 4>
//! pos=<x= 2, y= 1, z=-5>, vel=<x= 1, y= 5, z=-4>
//! pos=<x= 1, y=-8, z= 2>, vel=<x= 0, y=-4, z= 0>
//! ```
//!
//! After 4 steps:
//!
//! ```text
//! pos=<x= 2, y=-8, z= 0>, vel=<x=-3, y=-2, z= 1>
//! pos=<x= 2, y= 1, z= 7>, vel=<x= 2, y= 1, z= 1>
//! pos=<x= 2, y= 3, z=-6>, vel=<x= 0, y= 2, z=-1>
//! pos=<x= 2, y=-9, z= 1>, vel=<x= 1, y=-1, z=-1>
//! ```
//!
//! After 5 steps:
//!
//! ```text
//! pos=<x=-1, y=-9, z= 2>, vel=<x=-3, y=-1, z= 2>
//! pos=<x= 4, y= 1, z= 5>, vel=<x= 2, y= 0, z=-2>
//! pos=<x= 2, y= 2, z=-4>, vel=<x= 0, y=-1, z= 2>
//! pos=<x= 3, y=-7, z=-1>, vel=<x= 1, y= 2, z=-2>
//! ```
//!
//! After 6 steps:
//!
//! ```text
//! pos=<x=-1, y=-7, z= 3>, vel=<x= 0, y= 2, z= 1>
//! pos=<x= 3, y= 0, z= 0>, vel=<x=-1, y=-1, z=-5>
//! pos=<x= 3, y=-2, z= 1>, vel=<x= 1, y=-4, z= 5>
//! pos=<x= 3, y=-4, z=-2>, vel=<x= 0, y= 3, z=-1>
//! ```
//!
//! After 7 steps:
//!
//! ```text
//! pos=<x= 2, y=-2, z= 1>, vel=<x= 3, y= 5, z=-2>
//! pos=<x= 1, y=-4, z=-4>, vel=<x=-2, y=-4, z=-4>
//! pos=<x= 3, y=-7, z= 5>, vel=<x= 0, y=-5, z= 4>
//! pos=<x= 2, y= 0, z= 0>, vel=<x=-1, y= 4, z= 2>
//! ```
//!
//! After 8 steps:
//!
//! ```text
//! pos=<x= 5, y= 2, z=-2>, vel=<x= 3, y= 4, z=-3>
//! pos=<x= 2, y=-7, z=-5>, vel=<x= 1, y=-3, z=-1>
//! pos=<x= 0, y=-9, z= 6>, vel=<x=-3, y=-2, z= 1>
//! pos=<x= 1, y= 1, z= 3>, vel=<x=-1, y= 1, z= 3>
//! ```
//!
//! After 9 steps:
//!
//! ```text
//! pos=<x= 5, y= 3, z=-4>, vel=<x= 0, y= 1, z=-2>
//! pos=<x= 2, y=-9, z=-3>, vel=<x= 0, y=-2, z= 2>
//! pos=<x= 0, y=-8, z= 4>, vel=<x= 0, y= 1, z=-2>
//! pos=<x= 1, y= 1, z= 5>, vel=<x= 0, y= 0, z= 2>
//! ```
//!
//! After 10 steps:
//!
//! ```text
//! pos=<x= 2, y= 1, z=-3>, vel=<x=-3, y=-2, z= 1>
//! pos=<x= 1, y=-8, z= 0>, vel=<x=-1, y= 1, z= 3>
//! pos=<x= 3, y=-6, z= 1>, vel=<x= 3, y= 2, z=-3>
//! pos=<x= 2, y= 0, z= 4>, vel=<x= 1, y=-1, z=-1>
//! ```
//!
//! Then, it might help to calculate the total energy in the system. The total
//! energy for a single moon is its potential energy multiplied by its kinetic
//! energy. A moon's potential energy is the sum of the absolute values of its
//! x, y, and z position coordinates. A moon's kinetic energy is the sum of the
//! absolute values of its velocity coordinates. Below, each line shows the
//! calculations for a moon's potential energy (pot), kinetic energy (kin), and
//! total energy:
//!
//! Energy after 10 steps:
//!
//! ```text
//! pot: 2 + 1 + 3 =  6;   kin: 3 + 2 + 1 = 6;   total:  6 * 6 = 36
//! pot: 1 + 8 + 0 =  9;   kin: 1 + 1 + 3 = 5;   total:  9 * 5 = 45
//! pot: 3 + 6 + 1 = 10;   kin: 3 + 2 + 3 = 8;   total: 10 * 8 = 80
//! pot: 2 + 0 + 4 =  6;   kin: 1 + 1 + 1 = 3;   total:  6 * 3 = 18
//! Sum of total energy: 36 + 45 + 80 + 18 = 179
//! ```
//!
//! In the above example, adding together the total energy for all moons after
//! 10 steps produces the total energy in the system, 179.
//!
//! Here's a second example:
//!
//! ```text
//! <x=-8, y=-10, z=0>
//! <x=5, y=5, z=10>
//! <x=2, y=-7, z=3>
//! <x=9, y=-8, z=-3>
//! ```
//!
//! Every ten steps of simulation for 100 steps produces:
//!
//! After 0 steps:
//!
//! ```text
//! pos=<x= -8, y=-10, z=  0>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  5, y=  5, z= 10>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  2, y= -7, z=  3>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  9, y= -8, z= -3>, vel=<x=  0, y=  0, z=  0>
//! ```
//!
//! After 10 steps:
//!
//! ```text
//! pos=<x= -9, y=-10, z=  1>, vel=<x= -2, y= -2, z= -1>
//! pos=<x=  4, y= 10, z=  9>, vel=<x= -3, y=  7, z= -2>
//! pos=<x=  8, y=-10, z= -3>, vel=<x=  5, y= -1, z= -2>
//! pos=<x=  5, y=-10, z=  3>, vel=<x=  0, y= -4, z=  5>
//! ```
//!
//! After 20 steps:
//!
//! ```text
//! pos=<x=-10, y=  3, z= -4>, vel=<x= -5, y=  2, z=  0>
//! pos=<x=  5, y=-25, z=  6>, vel=<x=  1, y=  1, z= -4>
//! pos=<x= 13, y=  1, z=  1>, vel=<x=  5, y= -2, z=  2>
//! pos=<x=  0, y=  1, z=  7>, vel=<x= -1, y= -1, z=  2>
//! ```
//!
//! After 30 steps:
//!
//! ```text
//! pos=<x= 15, y= -6, z= -9>, vel=<x= -5, y=  4, z=  0>
//! pos=<x= -4, y=-11, z=  3>, vel=<x= -3, y=-10, z=  0>
//! pos=<x=  0, y= -1, z= 11>, vel=<x=  7, y=  4, z=  3>
//! pos=<x= -3, y= -2, z=  5>, vel=<x=  1, y=  2, z= -3>
//! ```
//!
//! After 40 steps:
//!
//! ```text
//! pos=<x= 14, y=-12, z= -4>, vel=<x= 11, y=  3, z=  0>
//! pos=<x= -1, y= 18, z=  8>, vel=<x= -5, y=  2, z=  3>
//! pos=<x= -5, y=-14, z=  8>, vel=<x=  1, y= -2, z=  0>
//! pos=<x=  0, y=-12, z= -2>, vel=<x= -7, y= -3, z= -3>
//! ```
//!
//! After 50 steps:
//!
//! ```text
//! pos=<x=-23, y=  4, z=  1>, vel=<x= -7, y= -1, z=  2>
//! pos=<x= 20, y=-31, z= 13>, vel=<x=  5, y=  3, z=  4>
//! pos=<x= -4, y=  6, z=  1>, vel=<x= -1, y=  1, z= -3>
//! pos=<x= 15, y=  1, z= -5>, vel=<x=  3, y= -3, z= -3>
//! ```
//!
//! After 60 steps:
//!
//! ```text
//! pos=<x= 36, y=-10, z=  6>, vel=<x=  5, y=  0, z=  3>
//! pos=<x=-18, y= 10, z=  9>, vel=<x= -3, y= -7, z=  5>
//! pos=<x=  8, y=-12, z= -3>, vel=<x= -2, y=  1, z= -7>
//! pos=<x=-18, y= -8, z= -2>, vel=<x=  0, y=  6, z= -1>
//! ```
//!
//! After 70 steps:
//!
//! ```text
//! pos=<x=-33, y= -6, z=  5>, vel=<x= -5, y= -4, z=  7>
//! pos=<x= 13, y= -9, z=  2>, vel=<x= -2, y= 11, z=  3>
//! pos=<x= 11, y= -8, z=  2>, vel=<x=  8, y= -6, z= -7>
//! pos=<x= 17, y=  3, z=  1>, vel=<x= -1, y= -1, z= -3>
//! ```
//!
//! After 80 steps:
//!
//! ```text
//! pos=<x= 30, y= -8, z=  3>, vel=<x=  3, y=  3, z=  0>
//! pos=<x= -2, y= -4, z=  0>, vel=<x=  4, y=-13, z=  2>
//! pos=<x=-18, y= -7, z= 15>, vel=<x= -8, y=  2, z= -2>
//! pos=<x= -2, y= -1, z= -8>, vel=<x=  1, y=  8, z=  0>
//! ```
//!
//! After 90 steps:
//!
//! ```text
//! pos=<x=-25, y= -1, z=  4>, vel=<x=  1, y= -3, z=  4>
//! pos=<x=  2, y= -9, z=  0>, vel=<x= -3, y= 13, z= -1>
//! pos=<x= 32, y= -8, z= 14>, vel=<x=  5, y= -4, z=  6>
//! pos=<x= -1, y= -2, z= -8>, vel=<x= -3, y= -6, z= -9>
//! ```
//!
//! After 100 steps:
//!
//! ```text
//! pos=<x=  8, y=-12, z= -9>, vel=<x= -7, y=  3, z=  0>
//! pos=<x= 13, y= 16, z= -3>, vel=<x=  3, y=-11, z= -5>
//! pos=<x=-29, y=-11, z= -1>, vel=<x= -3, y=  7, z=  4>
//! pos=<x= 16, y=-13, z= 23>, vel=<x=  7, y=  1, z=  1>
//! ```
//!
//! Energy after 100 steps:
//!
//! ```text
//! pot:  8 + 12 +  9 = 29;   kin: 7 +  3 + 0 = 10;   total: 29 * 10 = 290
//! pot: 13 + 16 +  3 = 32;   kin: 3 + 11 + 5 = 19;   total: 32 * 19 = 608
//! pot: 29 + 11 +  1 = 41;   kin: 3 +  7 + 4 = 14;   total: 41 * 14 = 574
//! pot: 16 + 13 + 23 = 52;   kin: 7 +  1 + 1 =  9;   total: 52 *  9 = 468
//! Sum of total energy: 290 + 608 + 574 + 468 = 1940
//! ```
//!
//! What is the total energy in the system after simulating the moons given in
//! your scan for 1000 steps?

use lazy_static::lazy_static;
use regex::Regex;

pub const PUZZLE_INPUT: &str = include_str!("../inputs/input-12");

trait Energetic {
    fn energy(&self) -> usize;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position3D {
    x: i16,
    y: i16,
    z: i16,
}

impl std::str::FromStr for Position3D {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r#"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>"#).unwrap();
        };
        if let Some(c) = REGEX.captures(s) {
            Ok(Position3D {
                x: c.get(1).unwrap().as_str().parse()?,
                y: c.get(2).unwrap().as_str().parse()?,
                z: c.get(3).unwrap().as_str().parse()?,
            })
        } else {
            Err(anyhow::anyhow!("Invalid input string"))
        }
    }
}

impl std::ops::Add<Velocity3D> for Position3D {
    type Output = Self;
    fn add(self, r: Velocity3D) -> Self::Output {
        Position3D {
            x: self.x + r.x,
            y: self.y + r.y,
            z: self.z + r.z,
        }
    }
}

impl std::ops::AddAssign<Velocity3D> for Position3D {
    fn add_assign(&mut self, v: Velocity3D) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl Energetic for Position3D {
    fn energy(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize + self.z.abs() as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Velocity3D {
    x: i16,
    y: i16,
    z: i16,
}

impl Velocity3D {
    const ZERO: Velocity3D = Velocity3D { x: 0, y: 0, z: 0 };
}

impl Default for Velocity3D {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl std::ops::Add for Velocity3D {
    type Output = Self;
    fn add(self, r: Self) -> Self::Output {
        Velocity3D {
            x: self.x + r.x,
            y: self.y + r.y,
            z: self.z + r.z,
        }
    }
}

impl std::ops::AddAssign for Velocity3D {
    fn add_assign(&mut self, r: Self) {
        self.x += r.x;
        self.y += r.y;
        self.z += r.z;
    }
}

impl std::ops::Sub for Velocity3D {
    type Output = Self;
    fn sub(self, r: Self) -> Self::Output {
        Velocity3D {
            x: self.x - r.x,
            y: self.y - r.y,
            z: self.z - r.z,
        }
    }
}

impl std::ops::SubAssign for Velocity3D {
    fn sub_assign(&mut self, r: Self) {
        self.x -= r.x;
        self.y -= r.y;
        self.z -= r.z;
    }
}

impl Energetic for Velocity3D {
    fn energy(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize + self.z.abs() as usize
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Position3D>> {
    use std::io::{BufRead, Cursor};

    let positions = Cursor::new(input)
        .lines()
        .filter_map(|line_result| match line_result {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.parse().map_err(anyhow::Error::from))
                }
            }
            Err(err) => Some(Err(anyhow::Error::from(err))),
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(positions)
}

#[derive(Debug)]
struct MoonField {
    positions: Vec<Position3D>,
    velocities: Vec<Velocity3D>,
}

impl MoonField {
    pub fn new(initial_pos: Vec<Position3D>) -> Self {
        let mut velocities = Vec::with_capacity(initial_pos.len());
        velocities.resize(initial_pos.len(), Velocity3D::default());
        Self {
            positions: initial_pos,
            velocities,
        }
    }

    fn step_velocity(&mut self) {
        use std::cmp::Ordering;

        for (i, p) in self.positions.iter().copied().enumerate() {
            for (j, q) in self.positions.iter().skip(i + 1).copied().enumerate() {
                macro_rules! compare {
                    ($w:ident) => {{
                        log::trace!(
                            "p.{}: {} {:?} q.{}: {}",
                            stringify!($w),
                            p.$w,
                            p.$w.cmp(&q.$w),
                            stringify!($w),
                            q.$w
                        );
                        match p.$w.cmp(&q.$w) {
                            Ordering::Less => -1,
                            Ordering::Greater => 1,
                            Ordering::Equal => 0,
                        }
                    }};
                }

                log::trace!(
                    "Before: {:?} {:?}",
                    self.velocities[i],
                    self.velocities[j + i + 1]
                );
                let dx = compare!(x);
                let dy = compare!(y);
                let dz = compare!(z);
                let dv = Velocity3D {
                    x: dx,
                    y: dy,
                    z: dz,
                };
                log::trace!("Delta: {:?}", dv);
                self.velocities[i] -= dv;
                self.velocities[j + i + 1] += dv;
                log::trace!(
                    "After: {:?} {:?}",
                    self.velocities[i],
                    self.velocities[j + i + 1]
                );
            }
        }
    }

    fn step_positions(&mut self) {
        for (p, v) in self
            .positions
            .iter_mut()
            .zip(self.velocities.iter().copied())
        {
            *p += v;
        }
    }

    pub fn step(&mut self) {
        self.step_velocity();
        self.step_positions();
    }
}

impl Energetic for MoonField {
    fn energy(&self) -> usize {
        self.positions
            .iter()
            .zip(self.velocities.iter())
            .map(|(p, v)| p.energy() * v.energy())
            .sum()
    }
}

impl Iterator for MoonField {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.step();
        let energy = self.energy();
        log::debug!("Current energy: {}", energy);
        if energy == 0 {
            None
        } else {
            Some(energy)
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    let positions = parse_input(PUZZLE_INPUT)?;
    let mut field = MoonField::new(positions);

    println!(
        "Energy at step 1000: {}",
        field.nth(999).unwrap_or_default()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_input, MoonField, Position3D, Velocity3D};
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    const EXAMPLE_INPUT_1: &str = "<x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";

    #[test]
    fn verify_step_1() -> Result<()> {
        let positions = parse_input(EXAMPLE_INPUT_1)?;
        let mut field = MoonField::new(positions);

        println!("{:#?}", field);

        field.step_velocity();

        let expected_velocities = vec![
            Velocity3D { x: 3, y: -1, z: -1 },
            Velocity3D { x: 1, y: 3, z: 3 },
            Velocity3D { x: -3, y: 1, z: -3 },
            Velocity3D { x: -1, y: -3, z: 1 },
        ];

        assert_eq!(field.velocities, expected_velocities);

        field.step_positions();

        let expected_positions = vec![
            Position3D { x: 2, y: -1, z: 1 },
            Position3D { x: 3, y: -7, z: -4 },
            Position3D { x: 1, y: -7, z: 5 },
            Position3D { x: 2, y: 2, z: 0 },
        ];

        assert_eq!(field.positions, expected_positions);

        Ok(())
    }

    #[test]
    fn verify_energy_after_step_10() -> Result<()> {
        let positions = parse_input(EXAMPLE_INPUT_1)?;
        let mut field = MoonField::new(positions);

        println!("{:#?}", field);

        const EXPECTED: usize = 179;

        let energy = field.nth(9).unwrap_or_default();

        println!("{:#?}", field);

        assert_eq!(energy, EXPECTED);

        Ok(())
    }

    const EXAMPLE_INPUT_2: &str = "<x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>";

    #[test]
    fn verify_energy_after_step_100() -> Result<()> {
        let positions = parse_input(EXAMPLE_INPUT_2)?;
        let mut field = MoonField::new(positions);

        println!("{:#?}", field);

        const EXPECTED: usize = 1940;

        let energy = field.nth(99).unwrap_or_default();

        println!("{:#?}", field);

        assert_eq!(energy, EXPECTED);

        Ok(())
    }
}