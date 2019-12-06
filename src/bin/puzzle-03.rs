use std::{
    fmt,
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
struct CandidatePoint {
    point: (i32, i32),
    total_wire_delay: usize,
}

impl CandidatePoint {
    fn manhattan_distance_from_origin(self) -> usize {
        self.point.0.abs() as usize + self.point.1.abs() as usize
    }

    fn wire_delay_to_point(self) -> usize {
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
        
        if ver_seg_starts_below_candidate == ver_seg_ends_above_candidate && hor_seg_starts_left_of_candidate == hor_seg_ends_right_of_candidate {
            let added_ver_dist = (candidate.1 - ver_seg.point.1).abs() as usize;
            let added_hor_dist = (candidate.0 - hor_seg.point.0).abs() as usize;

            let candidate = CandidatePoint {
                point: candidate,
                total_wire_delay: ver_seg.wire_delay + hor_seg.wire_delay + added_ver_dist + added_hor_dist,
            };
    
            println!("{} and {} intersect at candidate {}", ver_seg, hor_seg, candidate);
            Some(candidate)
        } else {
            None
        }
    }
}

impl fmt::Display for CandidatePoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} [wire delay: {}; dist: {}]", self.point, self.total_wire_delay, self.manhattan_distance_from_origin())
    }
}

#[derive(Clone, Copy, Debug)]
struct Segment {
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
        write!(f, "{:?} {} [wire delay: {}]", self.point, self.directive, self.wire_delay)
    }
}

#[derive(Clone, Debug)]
struct Wire {
    segments: Vec<Segment>
}

impl Wire {
    fn find_intersections(&self, seg: &Segment, weight_function: fn(CandidatePoint) -> usize) -> Option<usize> {
        self.segments.iter()
            .filter_map(|self_seg| CandidatePoint::from_segments(self_seg, seg))
            .map(weight_function)
            .min()
    }
}

impl FromStr for Wire {
    type Err = InvalidDirection;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, _, segments) = s.split(',')
            .filter(|op| !op.is_empty())
            .map(|dir| dir.parse().expect("invalid segment"))
            .fold(((0, 0), 0, Vec::new()), |(point, wire_delay, mut segs), directive| {
                let seg = Segment {
                    point,
                    directive,
                    wire_delay,
                };
                let next = seg.next_point();
                let next_dist = wire_delay + seg.directive.distance as usize;
                //println!("{:?} => {:?}", seg, next);
                segs.push(seg);
                (next, next_dist, segs)
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

    let weight_function = 
        if cfg!(feature = "part-1") {
            CandidatePoint::wire_delay_to_point
        } else {
            CandidatePoint::manhattan_distance_from_origin
        };
    
    let dist = wires[0].segments.iter()
        .filter_map(|seg| wires[1].find_intersections(seg, weight_function))
        .min();

    println!("{:?}", dist);
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

        least_distance_to_closest_intersection(wire1, wire2, CandidatePoint::manhattan_distance_from_origin, EXPECTED);
    }

    #[test]
    fn least_manhattan_distance_b() {
        let wire1: Wire = WIRE_B1.parse().unwrap();
        let wire2: Wire = WIRE_B2.parse().unwrap();
        
        const EXPECTED: usize = 135;

        least_distance_to_closest_intersection(wire1, wire2, CandidatePoint::manhattan_distance_from_origin, EXPECTED);
    }

    #[test]
    fn least_wire_delay_a() {
        let wire1: Wire = WIRE_A1.parse().unwrap();
        let wire2: Wire = WIRE_A2.parse().unwrap();
        
        const EXPECTED: usize = 610;

        least_distance_to_closest_intersection(wire1, wire2, CandidatePoint::wire_delay_to_point, EXPECTED);
    }

    #[test]
    fn least_wire_delay_b() {
        let wire1: Wire = WIRE_B1.parse().unwrap();
        let wire2: Wire = WIRE_B2.parse().unwrap();
        
        const EXPECTED: usize = 410;

        least_distance_to_closest_intersection(wire1, wire2, CandidatePoint::wire_delay_to_point, EXPECTED);
    }

    fn least_distance_to_closest_intersection(wire1: Wire, wire2: Wire, distance_calc: fn(CandidatePoint) -> usize, expected: usize) {
        let actual = wire1.segments.iter()
            .filter_map(|seg| wire2.find_intersections(seg, distance_calc))
            .min()
            .unwrap();

        assert_eq!(expected, actual);
    }
}