//! # Day 3: Crossed Wires
//!
//! The gravity assist was successful, and you're well on your way to the Venus
//! refuelling station. During the rush back on Earth, the fuel management
//! system wasn't completely installed, so that's next on the priority list.
//!
//! Opening the front panel reveals a jumble of wires. Specifically, two wires
//! are connected to a central port and extend outward on a grid. You trace the
//! path each wire takes as it leaves the central port, one wire per line of
//! text (your puzzle input).
//!
//! The wires twist and turn, but the two wires occasionally cross paths. To fix
//! the circuit, you need to find the intersection point closest to the central
//! port. Because the wires are on a grid, use the Manhattan distance for this
//! measurement. While the wires do technically cross right at the central port
//! where they both start, this point does not count, nor does a wire count as
//! crossing with itself.
//!
//! For example, if the first wire's path is `R8,U5,L5,D3`, then starting from
//! the central port (o), it goes right 8, up 5, left 5, and finally down 3:
//!
//! ```text
//! ...........
//! ...........
//! ...........
//! ....+----+.
//! ....|....|.
//! ....|....|.
//! ....|....|.
//! .........|.
//! .o-------+.
//! ...........
//! ```
//!
//! Then, if the second wire's path is `U7,R6,D4,L4`, it goes up 7, right 6,
//! down 4, and left 4:
//!
//! ```text
//! ...........
//! .+-----+...
//! .|.....|...
//! .|..+--X-+.
//! .|..|..|.|.
//! .|.-X--+.|.
//! .|..|....|.
//! .|.......|.
//! .o-------+.
//! ...........
//! ```
//!
//! These wires cross at two locations (marked X), but the lower-left one is
//! closer to the central port: its distance is 3 + 3 = 6.
//!
//! Here are a few more examples:
//!
//! * distance 159
//!
//! ```text
//! R75,D30,R83,U83,L12,D49,R71,U7,L72
//! U62,R66,U55,R34,D71,R55,D58,R83
//! ```
//!
//! * distance 135
//!
//! ```text
//! R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//! U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
//! ```
//!
//! What is the Manhattan distance from the central port to the closest
//! intersection?
//!
//! ## Part Two
//!
//! It turns out that this circuit is very timing-sensitive; you actually need
//! to minimize the signal delay.
//!
//! To do this, calculate the number of steps each wire takes to reach each
//! intersection; choose the intersection where the sum of both wires' steps is
//! lowest. If a wire visits a position on the grid multiple times, use the
//! steps value from the first time it visits that position when calculating the
//! total value of a specific intersection.
//!
//! The number of steps a wire takes is the total number of grid squares the
//! wire has entered to get to that location, including the intersection being
//! considered. Again consider the example from above:
//!
//! ```text
//! ...........
//! .+-----+...
//! .|.....|...
//! .|..+--X-+.
//! .|..|..|.|.
//! .|.-X--+.|.
//! .|..|....|.
//! .|.......|.
//! .o-------+.
//! ...........
//! ```
//!
//! In the above example, the intersection closest to the central port is
//! reached after 8+5+5+2 = 20 steps by the first wire and 7+6+4+3 = 20 steps by
//! the second wire for a total of 20+20 = 40 steps.
//!
//! However, the top-right intersection is better: the first wire takes only
//! 8+5+2 = 15 and the second wire takes only 7+6+2 = 15, a total of 15+15 = 30
//! steps.
//!
//! Here are the best steps for the extra examples from above:
//!
//! * 610 steps
//!
//! ```text
//! R75,D30,R83,U83,L12,D49,R71,U7,L72
//! U62,R66,U55,R34,D71,R55,D58,R83
//! ```
//!
//! * 410 steps
//!
//! ```text
//! R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//! U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
//! ```
//!
//! What is the fewest combined steps the wires must take to reach an
//! intersection?

use std::{
    fmt,
    io::{self, BufRead},
    str::FromStr,
};

pub const PUZZLE_INPUT: &str = include_str!("../inputs/input-03");

pub fn parse_input(input: &str) -> (Wire, Wire) {
    let mut lines = io::Cursor::new(input)
        .lines()
        .map(|l| l.expect("error reading line"))
        .filter(|l| !l.is_empty())
        .map(|wire_str| wire_str.parse().expect("data must be a valid integer"));
    (lines.next().unwrap(), lines.next().unwrap())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_vertical(self) -> bool {
        match self {
            Direction::Up | Direction::Down => true,
            Direction::Left | Direction::Right => false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InvalidDirection;

impl FromStr for Direction {
    type Err = InvalidDirection;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(InvalidDirection);
        }

        match s.chars().next().unwrap() {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(InvalidDirection),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Direction::Up => "U",
            Direction::Down => "D",
            Direction::Left => "L",
            Direction::Right => "R",
        };

        f.write_str(c)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Directive {
    dir: Direction,
    distance: i32,
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.dir, self.distance)
    }
}

#[derive(Debug)]
enum InvalidDirective {
    InvalidDirection,
    BadDistance(std::num::ParseIntError),
}

impl From<InvalidDirection> for InvalidDirective {
    fn from(_: InvalidDirection) -> Self {
        InvalidDirective::InvalidDirection
    }
}

impl From<std::num::ParseIntError> for InvalidDirective {
    fn from(x: std::num::ParseIntError) -> Self {
        InvalidDirective::BadDistance(x)
    }
}

impl FromStr for Directive {
    type Err = InvalidDirective;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(InvalidDirective::InvalidDirection);
        }

        let (d, dist) = s.split_at(1);
        Ok(Directive {
            dir: d.parse()?,
            distance: dist.parse()?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CandidatePoint {
    point: (i32, i32),
    total_wire_delay: usize,
}

impl CandidatePoint {
    pub fn manhattan_distance_from_origin(self) -> usize {
        self.point.0.abs() as usize + self.point.1.abs() as usize
    }

    pub fn wire_delay_to_point(self) -> usize {
        self.total_wire_delay
    }

    fn from_segments(seg1: &Segment, seg2: &Segment) -> Option<CandidatePoint> {
        if seg1.directive.dir.is_vertical() == seg2.directive.dir.is_vertical() {
            return None;
        }

        let (ver_seg, hor_seg) = if seg1.directive.dir.is_vertical() {
            (seg1, seg2)
        } else {
            (seg2, seg1)
        };

        let candidate = (ver_seg.point.0, hor_seg.point.1);
        let ver_seg_starts_below_candidate = ver_seg.point.1 < candidate.1;
        let ver_seg_ends_above_candidate = ver_seg.next_point().1 > candidate.1;
        let hor_seg_starts_left_of_candidate = hor_seg.point.0 < candidate.0;
        let hor_seg_ends_right_of_candidate = hor_seg.next_point().0 > candidate.0;

        if ver_seg_starts_below_candidate == ver_seg_ends_above_candidate
            && hor_seg_starts_left_of_candidate == hor_seg_ends_right_of_candidate
        {
            let added_ver_dist = (candidate.1 - ver_seg.point.1).abs() as usize;
            let added_hor_dist = (candidate.0 - hor_seg.point.0).abs() as usize;

            let candidate = CandidatePoint {
                point: candidate,
                total_wire_delay: ver_seg.wire_delay
                    + hor_seg.wire_delay
                    + added_ver_dist
                    + added_hor_dist,
            };

            log::debug!(
                "{} and {} intersect at candidate {}",
                ver_seg,
                hor_seg,
                candidate
            );
            Some(candidate)
        } else {
            None
        }
    }
}

impl fmt::Display for CandidatePoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} [wire delay: {}; dist: {}]",
            self.point,
            self.total_wire_delay,
            self.manhattan_distance_from_origin()
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Segment {
    point: (i32, i32),
    directive: Directive,
    wire_delay: usize,
}

impl Segment {
    fn next_point(&self) -> (i32, i32) {
        match self.directive.dir {
            Direction::Left => (self.point.0 - self.directive.distance, self.point.1),
            Direction::Right => (self.point.0 + self.directive.distance, self.point.1),
            Direction::Down => (self.point.0, self.point.1 - self.directive.distance),
            Direction::Up => (self.point.0, self.point.1 + self.directive.distance),
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} [wire delay: {}]",
            self.point, self.directive, self.wire_delay
        )
    }
}

#[derive(Clone, Debug)]
pub struct Wire {
    segments: Vec<Segment>,
}

impl Wire {
    pub fn find_intersections(
        &self,
        seg: &Segment,
        weight_function: fn(CandidatePoint) -> usize,
    ) -> Option<usize> {
        self.segments
            .iter()
            .filter_map(|self_seg| CandidatePoint::from_segments(self_seg, seg))
            .map(weight_function)
            .min()
    }
}

impl FromStr for Wire {
    type Err = InvalidDirection;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, _, segments) = s
            .split(',')
            .filter(|op| !op.is_empty())
            .map(|dir| dir.parse().expect("invalid segment"))
            .fold(
                ((0, 0), 0, Vec::new()),
                |(point, wire_delay, mut segs), directive| {
                    let seg = Segment {
                        point,
                        directive,
                        wire_delay,
                    };
                    let next = seg.next_point();
                    let next_dist = wire_delay + seg.directive.distance as usize;
                    log::trace!("{:?} => {:?}", seg, next);
                    segs.push(seg);
                    (next, next_dist, segs)
                },
            );
        Ok(Wire { segments })
    }
}

pub fn find_nearest_intersection(
    left: &Wire,
    right: &Wire,
    weight_function: fn(CandidatePoint) -> usize,
) -> Option<usize> {
    left.segments
        .iter()
        .filter_map(|seg| right.find_intersections(seg, weight_function))
        .min()
}

pub fn run() {
    let wires = parse_input(PUZZLE_INPUT);

    let manhattan = find_nearest_intersection(
        &wires.0,
        &wires.1,
        CandidatePoint::manhattan_distance_from_origin,
    );
    println!(
        "Nearest intersection by manhattan distance: {:?}",
        manhattan
    );

    let wire_delay =
        find_nearest_intersection(&wires.0, &wires.1, CandidatePoint::wire_delay_to_point);
    println!("Nearest intersection by wire delay: {:?}", wire_delay);
}


#[cfg(test)]
mod tests {
    use super::{CandidatePoint, Wire};
    use pretty_assertions::assert_eq;

    const WIRE_A1: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
    const WIRE_A2: &str = "U62,R66,U55,R34,D71,R55,D58,R83";
    const WIRE_B1: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
    const WIRE_B2: &str = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn least_manhattan_distance_a() {
        let wire1: Wire = WIRE_A1.parse().unwrap();
        let wire2: Wire = WIRE_A2.parse().unwrap();

        const EXPECTED: usize = 159;

        least_distance_to_closest_intersection(
            &wire1,
            &wire2,
            CandidatePoint::manhattan_distance_from_origin,
            EXPECTED,
        );
    }

    #[test]
    fn least_manhattan_distance_b() {
        let wire1: Wire = WIRE_B1.parse().unwrap();
        let wire2: Wire = WIRE_B2.parse().unwrap();

        const EXPECTED: usize = 135;

        least_distance_to_closest_intersection(
            &wire1,
            &wire2,
            CandidatePoint::manhattan_distance_from_origin,
            EXPECTED,
        );
    }

    #[test]
    fn least_wire_delay_a() {
        let wire1: Wire = WIRE_A1.parse().unwrap();
        let wire2: Wire = WIRE_A2.parse().unwrap();

        const EXPECTED: usize = 610;

        least_distance_to_closest_intersection(
            &wire1,
            &wire2,
            CandidatePoint::wire_delay_to_point,
            EXPECTED,
        );
    }

    #[test]
    fn least_wire_delay_b() {
        let wire1: Wire = WIRE_B1.parse().unwrap();
        let wire2: Wire = WIRE_B2.parse().unwrap();

        const EXPECTED: usize = 410;

        least_distance_to_closest_intersection(
            &wire1,
            &wire2,
            CandidatePoint::wire_delay_to_point,
            EXPECTED,
        );
    }

    fn least_distance_to_closest_intersection(
        wire1: &Wire,
        wire2: &Wire,
        distance_calc: fn(CandidatePoint) -> usize,
        expected: usize,
    ) {
        let actual = super::find_nearest_intersection(wire1, wire2, distance_calc).unwrap();

        assert_eq!(actual, expected);
    }
}
