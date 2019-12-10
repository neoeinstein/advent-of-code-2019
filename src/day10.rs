//! # Day 10: Monitoring Station
//!
//! You fly into the asteroid belt and reach the Ceres monitoring station. The
//! Elves here have an emergency: they're having trouble tracking all of the
//! asteroids and can't be sure they're safe.
//!
//! The Elves would like to build a new monitoring station in a nearby area of
//! space; they hand you a map of all of the asteroids in that region (your
//! puzzle input).
//!
//! The map indicates whether each position is empty (.) or contains an asteroid
//! (#). The asteroids are much smaller than they appear on the map, and every
//! asteroid is exactly in the center of its marked position. The asteroids can
//! be described with X,Y coordinates where X is the distance from the left edge
//! and Y is the distance from the top edge (so the top-left corner is 0,0 and
//! the position immediately to its right is 1,0).
//!
//! Your job is to figure out which asteroid would be the best place to build a
//! new monitoring station. A monitoring station can detect any asteroid to
//! which it has direct line of sight - that is, there cannot be another
//! asteroid exactly between them. This line of sight can be at any angle, not
//! just lines aligned to the grid or diagonally. The best location is the
//! asteroid that can detect the largest number of other asteroids.
//!
//! For example, consider the following map:
//!
//! ```text
//! .#..#
//! .....
//! #####
//! ....#
//! ...##
//! ```
//!
//! The best location for a new monitoring station on this map is the
//! highlighted asteroid at 3,4 because it can detect 8 asteroids, more than any
//! other location. (The only asteroid it cannot detect is the one at 1,0; its
//! view of this asteroid is blocked by the asteroid at 2,2.) All other
//! asteroids are worse locations; they can detect 7 or fewer other asteroids.
//! Here is the number of other asteroids a monitoring station on each asteroid
//! could detect:
//!
//! ```text
//! .7..7
//! .....
//! 67775
//! ....7
//! ...87
//! ```
//!
//! Here is an asteroid (#) and some examples of the ways its line of sight
//! might be blocked. If there were another asteroid at the location of a
//! capital letter, the locations marked with the corresponding lowercase letter
//! would be blocked and could not be detected:
//!
//! ```text
//! #.........
//! ...A......
//! ...B..a...
//! .EDCG....a
//! ..F.c.b...
//! .....c....
//! ..efd.c.gb
//! .......c..
//! ....f...c.
//! ...e..d..c
//! ```
//!
//! Here are some larger examples:
//!
//! * Best is 5,8 with 33 other asteroids detected:
//!
//! ```text
//! ......#.#.
//! #..#.#....
//! ..#######.
//! .#.#.###..
//! .#..#.....
//! ..#....#.#
//! #..#....#.
//! .##.#..###
//! ##...#..#.
//! .#....####
//! ```
//!
//! * Best is 1,2 with 35 other asteroids detected:
//!
//! ```text
//! #.#...#.#.
//! .###....#.
//! .#....#...
//! ##.#.#.#.#
//! ....#.#.#.
//! .##..###.#
//! ..#...##..
//! ..##....##
//! ......#...
//! .####.###.
//! ```
//!
//! * Best is 6,3 with 41 other asteroids detected:
//!
//! ```text
//! .#..#..###
//! ####.###.#
//! ....###.#.
//! ..###.##.#
//! ##.##.#.#.
//! ....###..#
//! ..#.#..#.#
//! #..#.#.###
//! .##...##.#
//! .....#.#..
//! ```
//!
//! * Best is 11,13 with 210 other asteroids detected:
//!
//! ```text
//! .#..##.###...#######
//! ##.############..##.
//! .#.######.########.#
//! .###.#######.####.#.
//! #####.##.#.##.###.##
//! ..#####..#.#########
//! ####################
//! #.####....###.#.#.##
//! ##.#################
//! #####.##.###..####..
//! ..######..##.#######
//! ####.##.####...##..#
//! .#####..#.######.###
//! ##...#.##########...
//! #.##########.#######
//! .####.#.###.###.#.##
//! ....##.##.###..#####
//! .#.#.###########.###
//! #.#.#.#####.####.###
//! ###.##.####.##.#..##
//! ```
//!
//! Find the best location for a new monitoring station. How many other
//! asteroids can be detected from that location?

use std::{
    collections::{HashMap, HashSet},
    fmt, io,
    ops::{Neg, Sub},
};

pub const PUZZLE_INPUT: &str = include_str!("../inputs/input-10");

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Slope {
    x: isize,
    y: isize,
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    a = a.abs();
    b = b.abs();
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

impl Slope {
    fn simplified(mut self) -> Self {
        let div = gcd(self.x, self.y);
        self.x /= div;
        self.y /= div;
        self
    }
}

impl Neg for Slope {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;
        self
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct AsteroidPosition {
    x: usize,
    y: usize,
}

impl Sub for AsteroidPosition {
    type Output = Slope;
    fn sub(self, r: Self) -> Slope {
        Slope {
            x: self.x as isize - r.x as isize,
            y: self.y as isize - r.y as isize,
        }
        .simplified()
    }
}

impl fmt::Display for AsteroidPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn parse_input(input: &str) -> io::Result<Vec<AsteroidPosition>> {
    use io::{BufRead, Cursor};

    let lines = Cursor::new(input)
        .lines()
        .collect::<io::Result<Vec<String>>>()?;

    let asteroids = lines
        .into_iter()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
        .flat_map(|(y, l)| {
            l.trim()
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| AsteroidPosition { x, y })
                .collect::<Vec<AsteroidPosition>>()
        })
        .collect();

    Ok(asteroids)
}

fn calculate_visiblity(
    asteroids: Vec<AsteroidPosition>,
) -> HashMap<AsteroidPosition, HashSet<Slope>> {
    let mut map: HashMap<_, HashSet<_>> = HashMap::new();

    for (i, a) in asteroids.iter().copied().enumerate() {
        for b in asteroids.iter().skip(i + 1).copied() {
            let slope = b - a;
            map.entry(a).or_default().insert(slope);
            map.entry(b).or_default().insert(-slope);
        }
    }

    map
}

pub fn find_best_place_for_monitoring(
    asteroids: Vec<AsteroidPosition>,
) -> (AsteroidPosition, usize) {
    let visibility = calculate_visiblity(asteroids);
    visibility
        .into_iter()
        .map(|(p, s)| (p, s.len()))
        .max_by(|(_, c), (_, d)| c.cmp(d))
        .unwrap()
}

pub fn run() -> anyhow::Result<()> {
    let asteroids = parse_input(PUZZLE_INPUT)?;
    let (best_pos, vis_count) = find_best_place_for_monitoring(asteroids);
    println!(
        "From position {}, {} other asteroids are visible",
        best_pos, vis_count
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Slope;

    #[test]
    fn gcf_15_12() {
        assert_eq!(super::gcd(15, 12), 3);
    }

    #[test]
    fn gcf_80_120() {
        assert_eq!(super::gcd(80, 120), 40);
    }

    #[test]
    fn gcf_neg80_120() {
        assert_eq!(super::gcd(-80, 120), 40);
    }

    #[test]
    fn gcf_neg80_neg120() {
        assert_eq!(super::gcd(-80, -120), 40);
    }

    #[test]
    fn gcf_80_neg120() {
        assert_eq!(super::gcd(80, -120), 40);
    }

    #[test]
    fn gcf_80_0() {
        assert_eq!(super::gcd(80, 0), 80);
    }

    #[test]
    fn gcf_0_80() {
        assert_eq!(super::gcd(0, 80), 80);
    }

    #[test]
    fn slope_reduce_64_32() {
        let slope = Slope { x: 64, y: 32 };

        const EXPECTED: Slope = Slope { x: 2, y: 1 };

        assert_eq!(slope.simplified(), EXPECTED);
    }

    #[test]
    fn slope_reduce_0_32() {
        let slope = Slope { x: 0, y: 32 };

        const EXPECTED: Slope = Slope { x: 0, y: 1 };

        assert_eq!(slope.simplified(), EXPECTED);
    }

    #[test]
    fn slope_reduce_64_0() {
        let slope = Slope { x: 64, y: 0 };

        const EXPECTED: Slope = Slope { x: 1, y: 0 };

        assert_eq!(slope.simplified(), EXPECTED);
    }

    const PART_1_EXAMPLE_1: &str = "
        .#..#
        .....
        #####
        ....#
        ...##";

    const PART_1_EXAMPLE_2: &str = "
        ......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####";

    const PART_1_EXAMPLE_3: &str = "
        #.#...#.#.
        .###....#.
        .#....#...
        ##.#.#.#.#
        ....#.#.#.
        .##..###.#
        ..#...##..
        ..##....##
        ......#...
        .####.###.";

    const PART_1_EXAMPLE_4: &str = "
        .#..#..###
        ####.###.#
        ....###.#.
        ..###.##.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#..";

    const PART_1_EXAMPLE_5: &str = "
        .#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##";

    #[test]
    fn verify_parse_1() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_1)?;

        assert_eq!(asteroids.len(), 10);

        Ok(())
    }

    #[test]
    fn verify_parse_2() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_2)?;

        assert_eq!(asteroids.len(), 40);

        Ok(())
    }

    #[test]
    fn verify_parse_3() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_3)?;

        assert_eq!(asteroids.len(), 40);

        Ok(())
    }

    #[test]
    fn verify_parse_4() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_4)?;

        assert_eq!(asteroids.len(), 50);

        Ok(())
    }

    #[test]
    fn verify_parse_5() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_5)?;

        assert_eq!(asteroids.len(), 300);

        Ok(())
    }

    #[test]
    fn calc_visibility_1() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_1)?;
        let visibility = super::calculate_visiblity(asteroids);
        let best = visibility
            .get(&super::AsteroidPosition { x: 3, y: 4 })
            .unwrap();

        assert_eq!(best.len(), 8);

        Ok(())
    }

    #[test]
    fn calc_visibility_2() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_2)?;
        let visibility = super::calculate_visiblity(asteroids);
        let best = visibility
            .get(&super::AsteroidPosition { x: 5, y: 8 })
            .unwrap();

        assert_eq!(best.len(), 33);

        Ok(())
    }

    #[test]
    fn calc_visibility_3() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_3)?;
        let visibility = super::calculate_visiblity(asteroids);
        let best = visibility
            .get(&super::AsteroidPosition { x: 1, y: 2 })
            .unwrap();

        assert_eq!(best.len(), 35);

        Ok(())
    }

    #[test]
    fn calc_visibility_4() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_4)?;
        let visibility = super::calculate_visiblity(asteroids);
        let best = visibility
            .get(&super::AsteroidPosition { x: 6, y: 3 })
            .unwrap();

        assert_eq!(best.len(), 41);

        Ok(())
    }

    #[test]
    fn calc_visibility_5() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_5)?;
        let visibility = super::calculate_visiblity(asteroids);
        let best = visibility
            .get(&super::AsteroidPosition { x: 11, y: 13 })
            .unwrap();

        assert_eq!(best.len(), 210);

        Ok(())
    }

    #[test]
    fn best_monitoring_location_1() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_1)?;
        let best = super::find_best_place_for_monitoring(asteroids);
        const EXPECTED: super::AsteroidPosition = super::AsteroidPosition { x: 3, y: 4 };

        assert_eq!(best.0, EXPECTED);

        Ok(())
    }

    #[test]
    fn best_monitoring_location_2() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_2)?;
        let best = super::find_best_place_for_monitoring(asteroids);
        const EXPECTED: super::AsteroidPosition = super::AsteroidPosition { x: 5, y: 8 };

        assert_eq!(best.0, EXPECTED);

        Ok(())
    }

    #[test]
    fn best_monitoring_location_3() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_3)?;
        let best = super::find_best_place_for_monitoring(asteroids);
        const EXPECTED: super::AsteroidPosition = super::AsteroidPosition { x: 1, y: 2 };

        assert_eq!(best.0, EXPECTED);

        Ok(())
    }

    #[test]
    fn best_monitoring_location_4() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_4)?;
        let best = super::find_best_place_for_monitoring(asteroids);
        const EXPECTED: super::AsteroidPosition = super::AsteroidPosition { x: 6, y: 3 };

        assert_eq!(best.0, EXPECTED);

        Ok(())
    }

    #[test]
    fn best_monitoring_location_5() -> std::io::Result<()> {
        let asteroids = super::parse_input(PART_1_EXAMPLE_5)?;
        let best = super::find_best_place_for_monitoring(asteroids);
        const EXPECTED: super::AsteroidPosition = super::AsteroidPosition { x: 11, y: 13 };

        assert_eq!(best.0, EXPECTED);

        Ok(())
    }
}
