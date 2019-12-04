use std::{
    io::BufRead,
    str::FromStr,
};
use advent_of_code_2019::get_input_reader;

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
struct InvalidDirection;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Directive {
    dir: Direction,
    distance: i32,
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

fn calculate_manhattan_distance_from_origin(p: (i32, i32)) -> usize {
    p.0.abs() as usize + p.1.abs() as usize
}

fn find_intesection(seg1: &Segment, seg2: &Segment) -> Option<(i32, i32)> {
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
    
    if ver_seg_starts_below_candidate == ver_seg_ends_above_candidate && hor_seg_starts_left_of_candidate == hor_seg_ends_right_of_candidate {
        println!("*** {:?} and {:?} intersect at {:?}", ver_seg, hor_seg, candidate);
        Some(candidate)
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug)]
struct Segment {
    point: (i32, i32),
    directive: Directive,
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

#[derive(Clone, Debug)]
struct Wire {
    segments: Vec<Segment>
}

impl Wire {
    fn find_intersections(&self, seg: &Segment) -> Option<usize> {
        self.segments.iter()
            .filter_map(|self_seg| find_intesection(self_seg, seg))
            .map(calculate_manhattan_distance_from_origin)
            .min()
    }
}

impl FromStr for Wire {
    type Err = InvalidDirection;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, segments) = s.split(',')
            .filter(|op| !op.is_empty())
            .map(|dir| dir.parse().expect("invalid segment"))
            .fold(((0, 0), Vec::new()), |(point, mut segs), directive| {
                let seg = Segment {
                    point,
                    directive,
                };
                let next = seg.next_point();
                println!("{:?} => {:?}", seg, next);
                segs.push(seg);
                (next, segs)
            });
        Ok(Wire { segments })
    }
}

fn parse_input() -> Vec<Wire> {
    let in_fd = get_input_reader();
    in_fd.lines()
        .map(|l| l.expect("error reading line"))
        .filter(|l| !l.is_empty())
        .map(|wire_str| wire_str.parse().expect("data must be a valid integer"))
        .collect()
}

fn main() {
    let wires = parse_input();
    //println!("{:?}", wires);

    
    let dist = wires[0].segments.iter()
        .filter_map(|seg| wires[1].find_intersections(seg))
        .min();

    println!("{:?}", dist);
}