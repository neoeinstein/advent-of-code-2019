//! # Day 12: The N-Body Problem
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
//! ## Part Two
//!
//! All this drifting around in space makes you wonder about the nature of the
//! universe. Does history really repeat itself? You're curious whether the
//! moons will ever return to a previous state.
//!
//! Determine the number of steps that must occur before all of the moons'
//! positions and velocities exactly match a previous point in time.
//!
//! For example, the first example above takes 2772 steps before they exactly
//! match a previous point in time; it eventually returns to the initial state:
//!
//! After 0 steps:
//!
//! ```text
//! pos=<x= -1, y=  0, z=  2>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  2, y=-10, z= -7>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  4, y= -8, z=  8>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  3, y=  5, z= -1>, vel=<x=  0, y=  0, z=  0>
//! ```
//!
//! After 2770 steps:
//!
//! ```text
//! pos=<x=  2, y= -1, z=  1>, vel=<x= -3, y=  2, z=  2>
//! pos=<x=  3, y= -7, z= -4>, vel=<x=  2, y= -5, z= -6>
//! pos=<x=  1, y= -7, z=  5>, vel=<x=  0, y= -3, z=  6>
//! pos=<x=  2, y=  2, z=  0>, vel=<x=  1, y=  6, z= -2>
//! ```
//!
//! After 2771 steps:
//!
//! ```text
//! pos=<x= -1, y=  0, z=  2>, vel=<x= -3, y=  1, z=  1>
//! pos=<x=  2, y=-10, z= -7>, vel=<x= -1, y= -3, z= -3>
//! pos=<x=  4, y= -8, z=  8>, vel=<x=  3, y= -1, z=  3>
//! pos=<x=  3, y=  5, z= -1>, vel=<x=  1, y=  3, z= -1>
//! ```
//!
//! After 2772 steps:
//!
//! ```text
//! pos=<x= -1, y=  0, z=  2>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  2, y=-10, z= -7>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  4, y= -8, z=  8>, vel=<x=  0, y=  0, z=  0>
//! pos=<x=  3, y=  5, z= -1>, vel=<x=  0, y=  0, z=  0>
//! ```
//!
//! Of course, the universe might last for a very long time before repeating.
//! Here's a copy of the second example from above:
//!
//! ```text
//! <x=-8, y=-10, z=0>
//! <x=5, y=5, z=10>
//! <x=2, y=-7, z=3>
//! <x=9, y=-8, z=-3>
//! ```
//!
//! This set of initial positions takes 4686774924 steps before it repeats a
//! previous state! Clearly, you might need to find a more efficient way to
//! simulate the universe.
//!
//! How many steps does it take to reach the first state that exactly matches a
//! previous state?

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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Axis {
    position: i16,
    velocity: i16,
}

impl Axis {
    fn new(position: i16) -> Self {
        Self {
            position,
            velocity: 0,
        }
    }

    fn step_velocity(&mut self, other: &mut Self) {
        use std::cmp::Ordering;

        let dv = match self.position.cmp(&other.position) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        };
        self.velocity -= dv;
        other.velocity += dv;
    }

    fn step_position(&mut self) {
        self.position += self.velocity;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Axis3D {
    x: Axis,
    y: Axis,
    z: Axis,
}

impl From<Position3D> for Axis3D {
    fn from(p: Position3D) -> Self {
        Self {
            x: Axis::new(p.x),
            y: Axis::new(p.y),
            z: Axis::new(p.z),
        }
    }
}

impl Axis3D {
    fn step_velocity(&mut self, other: &mut Self) {
        self.x.step_velocity(&mut other.x);
        self.y.step_velocity(&mut other.y);
        self.z.step_velocity(&mut other.z);
    }

    fn step_position(&mut self) {
        self.x.step_position();
        self.y.step_position();
        self.z.step_position();
    }
}

impl From<Axis3D> for Position3D {
    fn from(a: Axis3D) -> Self {
        Self {
            x: a.x.position,
            y: a.y.position,
            z: a.z.position,
        }
    }
}

impl From<Axis3D> for Velocity3D {
    fn from(a: Axis3D) -> Self {
        Self {
            x: a.x.velocity,
            y: a.y.velocity,
            z: a.z.velocity,
        }
    }
}

impl Energetic for Axis3D {
    fn energy(&self) -> usize {
        let potential = self.x.position.abs() as usize + self.y.position.abs() as usize + self.z.position.abs() as usize;
        let kinetic = self.x.velocity.abs() as usize + self.y.velocity.abs() as usize + self.z.velocity.abs() as usize;
        potential * kinetic
    }
}

fn step_until_loop(mut axis: [Axis; 4]) -> usize {
    let mut steps = 0;
    let mut states = std::collections::HashSet::new();
    while states.insert(axis) {
        for i in 0..3 {
            for j in (i + 1)..4 {
                let (l, r) = axis[..].split_at_mut(j);
                l[i].step_velocity(&mut r[0]);
            }
        }
        for i in 0..4 {
            axis[i].step_position();
        }
        steps += 1;
    }

    steps
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MoonField {
    moons: Vec<Axis3D>,
}

impl MoonField {
    pub fn new(initial_pos: Vec<Position3D>) -> Self {
        Self {
            moons: initial_pos.into_iter().map(Axis3D::from).collect(),
        }
    }

    fn x(&self) -> Vec<Axis> {
        self.moons.iter().cloned().map(|m| m.x).collect()
    }

    fn y(&self) -> Vec<Axis> {
        self.moons.iter().cloned().map(|m| m.y).collect()
    }

    fn z(&self) -> Vec<Axis> {
        self.moons.iter().cloned().map(|m| m.z).collect()
    }

    fn step_velocity(&mut self) {
        for i in 0..(self.moons.len() - 1) {
            for j in (i + 1)..self.moons.len() {
                let (l, r) = self.moons.split_at_mut(j);
                l[i].step_velocity(&mut r[0]);
            }
        }
    }

    fn step_positions(&mut self) {
        for m in self.moons.iter_mut() {
            m.step_position();
        }
    }

    pub fn step(&mut self) {
        self.step_velocity();
        self.step_positions();
    }

    fn cycle_time(&self) -> usize {
        let x = self.x();
        let xs = [x[0], x[1], x[2], x[3]];
        let x_steps = step_until_loop(xs);
        log::info!("x steps: {}", x_steps);

        let y = self.y();
        let ys = [y[0], y[1], y[2], y[3]];
        let y_steps = step_until_loop(ys);
        log::info!("y steps: {}", y_steps);

        let z = self.z();
        let zs = [z[0], z[1], z[2], z[3]];
        let z_steps = step_until_loop(zs);
        log::info!("z steps: {}", z_steps);

        lcm(lcm(x_steps, y_steps), z_steps)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    let mut d = 0;
    while a % 2 == 0 && b % 2 == 0 {
        a /= 2;
        b /= 2;
        d += 1;
    }
    while a != b {
        if a % 2 == 0 {
            a /= 2;
        } else if b % 2 == 0 {
            b /= 2;
        } else if a > b {
            a = (a - b) / 2;
        } else {
            b = (b - a) / 2;
        }
    }
    a << d
}

impl Energetic for MoonField {
    fn energy(&self) -> usize {
        self.moons
            .iter()
            .map(Energetic::energy)
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

    println!(
        "Steps to repeat initial condition: {}",
        field.cycle_time(),
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

        assert_eq!(field.moons.iter().copied().map(Velocity3D::from).collect::<Vec<_>>(), expected_velocities);

        field.step_positions();

        let expected_positions = vec![
            Position3D { x: 2, y: -1, z: 1 },
            Position3D { x: 3, y: -7, z: -4 },
            Position3D { x: 1, y: -7, z: 5 },
            Position3D { x: 2, y: 2, z: 0 },
        ];

        assert_eq!(field.moons.iter().copied().map(Position3D::from).collect::<Vec<_>>(), expected_positions);

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

    #[test]
    fn find_cycle_time_1() -> Result<()> {
        let _ = env_logger::Builder::new().is_test(true).try_init();
        let positions = parse_input(EXAMPLE_INPUT_1)?;
        let field = MoonField::new(positions);

        const EXPECTED: usize = 2772;

        assert_eq!(field.cycle_time(), EXPECTED);

        Ok(())
    }

    #[test]
    fn find_cycle_time_2() -> Result<()> {
        let _ = env_logger::Builder::new().is_test(true).try_init();
        let positions = parse_input(EXAMPLE_INPUT_2)?;
        let field = MoonField::new(positions);

        const EXPECTED: usize = 4686774924;

        assert_eq!(field.cycle_time(), EXPECTED);

        Ok(())
    }}
