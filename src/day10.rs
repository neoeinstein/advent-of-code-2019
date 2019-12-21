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
//!
//! ## Part Two
//!
//! Once you give them the coordinates, the Elves quickly deploy an Instant
//! Monitoring Station to the location and discover the worst: there are simply
//! too many asteroids.
//!
//! The only solution is complete vaporization by giant laser.
//!
//! Fortunately, in addition to an asteroid scanner, the new monitoring station
//! also comes equipped with a giant rotating laser perfect for vaporizing
//! asteroids. The laser starts by pointing up and always rotates clockwise,
//! vaporizing any asteroid it hits.
//!
//! If multiple asteroids are exactly in line with the station, the laser only
//! has enough power to vaporize one of them before continuing its rotation. In
//! other words, the same asteroids that can be detected can be vaporized, but
//! if vaporizing one asteroid makes another one detectable, the newly-detected
//! asteroid won't be vaporized until the laser has returned to the same
//! position by rotating a full 360 degrees.
//!
//! For example, consider the following map, where the asteroid with the new
//! monitoring station (and laser) is marked X:
//!
//! ```text
//! .#....#####...#..
//! ##...##.#####..##
//! ##...#...#.#####.
//! ..#.....X...###..
//! ..#.#.....#....##
//! ```
//!
//! The first nine asteroids to get vaporized, in order, would be:
//!
//! ```text
//! .#....###24...#..
//! ##...##.13#67..9#
//! ##...#...5.8####.
//! ..#.....X...###..
//! ..#.#.....#....##
//! ```
//!
//! Note that some asteroids (the ones behind the asteroids marked 1, 5, and 7)
//! won't have a chance to be vaporized until the next full rotation. The laser
//! continues rotating; the next nine to be vaporized are:
//!
//! ```text
//! .#....###.....#..
//! ##...##...#.....#
//! ##...#......1234.
//! ..#.....X...5##..
//! ..#.9.....8....76
//! ```
//!
//! The next nine to be vaporized are then:
//!
//! ```text
//! .8....###.....#..
//! 56...9#...#.....#
//! 34...7...........
//! ..2.....X....##..
//! ..1..............
//! ```
//!
//! Finally, the laser completes its first full rotation (1 through 3), a second
//! rotation (4 through 8), and vaporizes the last asteroid (9) partway through
//! its third rotation:
//!
//! ```text
//! ......234.....6..
//! ......1...5.....7
//! .................
//! ........X....89..
//! .................
//! ```
//!
//! In the large example above (the one with the best monitoring station
//! location at 11,13):
//!
//! * The 1st asteroid to be vaporized is at 11,12.
//! * The 2nd asteroid to be vaporized is at 12,1.
//! * The 3rd asteroid to be vaporized is at 12,2.
//! * The 10th asteroid to be vaporized is at 12,8.
//! * The 20th asteroid to be vaporized is at 16,0.
//! * The 50th asteroid to be vaporized is at 16,9.
//! * The 100th asteroid to be vaporized is at 10,16.
//! * The 199th asteroid to be vaporized is at 9,6.
//! * The 200th asteroid to be vaporized is at 8,2.
//! * The 201st asteroid to be vaporized is at 10,9.
//! * The 299th and final asteroid to be vaporized is at 11,1.
//!
//! The Elves are placing bets on which will be the 200th asteroid to be
//! vaporized. Win the bet by determining which asteroid that will be; what
//! do you get if you multiply its X coordinate by 100 and then add its Y
//! coordinate? (For example, 8,2 becomes 802.)

use num_integer::Integer;
use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::{HashMap, HashSet, VecDeque},
    fmt, io,
    ops::{Add, Neg, Sub},
};

pub const PUZZLE_INPUT: &str = include_str!("../inputs/input-10");

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Slope {
    x: isize,
    y: isize,
}

impl PartialOrd for Slope {
    fn partial_cmp(&self, r: &Self) -> Option<Ordering> {
        self.theta().partial_cmp(&r.theta())
    }
}

impl Ord for Slope {
    fn cmp(&self, r: &Self) -> Ordering {
        self.partial_cmp(r).unwrap()
    }
}

impl Slope {
    const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn simplified(mut self) -> Self {
        let div = self.x.gcd(&self.y);
        if div == 0 {
            return Self {
                x: self.x.signum(),
                y: self.y.signum(),
            };
        }
        self.x /= div;
        self.y /= div;
        self
    }

    #[inline]
    fn theta(self) -> f64 {
        -(self.x as f64).atan2(self.y as f64)
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

impl AsteroidPosition {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Add<Slope> for AsteroidPosition {
    type Output = Option<Self>;
    fn add(self, r: Slope) -> Self::Output {
        let next_x = self.x as isize + r.x;
        if next_x < 0 {
            return None;
        }
        let next_y = self.y as isize + r.y;
        if next_y < 0 {
            return None;
        }

        Some(Self {
            x: next_x as usize,
            y: next_y as usize,
        })
    }
}

impl Sub for AsteroidPosition {
    type Output = Slope;
    fn sub(self, r: Self) -> Self::Output {
        Slope::new(
            self.x as isize - r.x as isize,
            self.y as isize - r.y as isize,
        )
        .simplified()
    }
}

impl fmt::Display for AsteroidPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct AsteroidField {
    dimensions: (usize, usize),
    asteroids: Vec<AsteroidPosition>,
}

impl AsteroidField {
    fn calculate_all_visibility_angles_from(&self, location: AsteroidPosition) -> HashSet<Slope> {
        self.asteroids
            .iter()
            .copied()
            .filter(|&l| l != location)
            .map(|l| l - location)
            .collect()
    }

    fn calculate_all_visibilities(&self) -> HashMap<AsteroidPosition, HashSet<Slope>> {
        let mut map: HashMap<_, HashSet<_>> = HashMap::new();

        for (i, a) in self.asteroids.iter().copied().enumerate() {
            for b in self.asteroids.iter().skip(i + 1).copied() {
                let slope = b - a;
                map.entry(a).or_default().insert(slope);
                map.entry(b).or_default().insert(-slope);
            }
        }

        map
    }

    pub fn find_best_place_for_monitoring(&self) -> (AsteroidPosition, usize) {
        let visibility = self.calculate_all_visibilities();
        visibility
            .into_iter()
            .map(|(p, s)| {
                log::debug!("Asteroid {:?} can see {:?}", p, s);
                (p, s.len())
            })
            .max_by(|(_, c), (_, d)| c.cmp(d))
            .unwrap()
    }

    fn vaporize_from(&self, station: AsteroidPosition) -> Vaporizor {
        let visible: Vec<_> = self
            .calculate_all_visibility_angles_from(station)
            .into_iter()
            .collect();
        log::debug!("{} unique visible asteroids", visible.len());
        Vaporizor::new(
            station,
            self.asteroids.iter().copied().collect(),
            visible,
            self.dimensions,
        )
    }
}

pub fn parse_input(input: &str) -> io::Result<AsteroidField> {
    use io::{BufRead, Cursor};

    let lines = Cursor::new(input)
        .lines()
        .collect::<io::Result<Vec<String>>>()?;

    let mut max_y = 0;
    let mut max_x = 0;

    let asteroids = lines
        .into_iter()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
        .flat_map(|(y, l)| {
            let trimmed = l.trim();

            max_x = max_x.max(trimmed[..].len() - 1);
            max_y = y;

            trimmed
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| AsteroidPosition { x, y })
                .collect::<Vec<AsteroidPosition>>()
        })
        .collect();

    Ok(AsteroidField {
        dimensions: (max_x, max_y),
        asteroids,
    })
}

enum Position {
    Nothing,
    Station,
    Asteroid,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Position::Nothing => f.write_str(" "),
            Position::Station => f.write_str("X"),
            Position::Asteroid => f.write_str("#"),
        }
    }
}

#[derive(Clone, Debug)]
struct Vaporizor {
    station: AsteroidPosition,
    remaining_roids: HashSet<AsteroidPosition>,
    to_sweep: VecDeque<(AsteroidPosition, Slope)>,
    dims: (usize, usize),
}

impl fmt::Display for Vaporizor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..=self.dims.1 {
            for x in 0..=self.dims.0 {
                let position = AsteroidPosition::new(x, y);
                if self.station == position {
                    Position::Station.fmt(f)?;
                } else if self.remaining_roids.contains(&position) {
                    Position::Asteroid.fmt(f)?;
                } else {
                    Position::Nothing.fmt(f)?;
                }
            }
            "\n".fmt(f)?;
        }
        Ok(())
    }
}

impl Vaporizor {
    pub fn new(
        station: AsteroidPosition,
        asteroids: HashSet<AsteroidPosition>,
        known_slopes: Vec<Slope>,
        dims: (usize, usize),
    ) -> Self {
        let mut slopes = known_slopes;
        slopes.sort_unstable();
        let mut asteroids = asteroids;
        asteroids.remove(&station);
        Self {
            station,
            remaining_roids: asteroids,
            to_sweep: slopes.into_iter().map(|s| (station, s)).collect(),
            dims,
        }
    }
}

impl Iterator for Vaporizor {
    type Item = AsteroidPosition;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.remaining_roids.len();
        (len, Some(len))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_roids.is_empty() {
            return None;
        }

        while let Some(next_slope) = self.to_sweep.pop_front() {
            log::trace!(
                "next: {:?}, left: {}, to_sweep: {}",
                next_slope,
                self.remaining_roids.len(),
                self.to_sweep.len()
            );
            log::debug!("Current state:\n{}", self);
            if let Some(candidate) = next_slope.0 + next_slope.1 {
                if candidate.x <= self.dims.0 && candidate.y <= self.dims.1 {
                    if self.remaining_roids.remove(&candidate) {
                        log::trace!("found astroid! vaporizing!");
                        self.to_sweep.push_back((candidate, next_slope.1));
                        return Some(candidate);
                    } else {
                        log::trace!("no astroid there, looking wider");
                        self.to_sweep.push_front((candidate, next_slope.1));
                        continue;
                    }
                }
            }

            log::trace!("outside of bounds");
        }

        assert_eq!(self.remaining_roids.len(), 0);

        None
    }
}

pub fn run() -> anyhow::Result<()> {
    let field = parse_input(PUZZLE_INPUT)?;
    let (best_pos, vis_count) = field.find_best_place_for_monitoring();
    println!(
        "From position {}, {} other asteroids are visible",
        best_pos, vis_count
    );

    let mut vaporizor = field.vaporize_from(best_pos);

    // for (i, a) in vaporizor.enumerate() {
    //     println!("Vaporized {:?} at {}", a, i + 1);
    // }

    println!(
        "200th vaporized asteroid: {:?}",
        vaporizor.nth(199).unwrap()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_input, AsteroidPosition, Slope};
    use pretty_assertions::assert_eq;

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
        let field = parse_input(PART_1_EXAMPLE_1)?;

        assert_eq!(field.asteroids.len(), 10);

        Ok(())
    }

    #[test]
    fn verify_parse_2() -> std::io::Result<()> {
        let field = parse_input(PART_1_EXAMPLE_2)?;

        assert_eq!(field.asteroids.len(), 40);

        Ok(())
    }

    #[test]
    fn verify_parse_3() -> std::io::Result<()> {
        let field = parse_input(PART_1_EXAMPLE_3)?;

        assert_eq!(field.asteroids.len(), 40);

        Ok(())
    }

    #[test]
    fn verify_parse_4() -> std::io::Result<()> {
        let field = parse_input(PART_1_EXAMPLE_4)?;

        assert_eq!(field.asteroids.len(), 50);

        Ok(())
    }

    #[test]
    fn verify_parse_5() -> std::io::Result<()> {
        let field = parse_input(PART_1_EXAMPLE_5)?;

        assert_eq!(field.asteroids.len(), 300);

        Ok(())
    }

    #[test]
    fn calc_visibility_1() -> std::io::Result<()> {
        crate::init_logging();
        let field = parse_input(PART_1_EXAMPLE_1)?;
        let visibility = field.calculate_all_visibilities();
        let best = visibility.get(&AsteroidPosition::new(3, 4)).unwrap();

        assert_eq!(best.len(), 8);

        Ok(())
    }

    #[test]
    fn calc_visibility_2() -> std::io::Result<()> {
        crate::init_logging();
        let field = parse_input(PART_1_EXAMPLE_2)?;
        let visibility = field.calculate_all_visibilities();
        let best = visibility.get(&AsteroidPosition::new(5, 8)).unwrap();

        assert_eq!(best.len(), 33);

        Ok(())
    }

    #[test]
    fn calc_visibility_3() -> std::io::Result<()> {
        crate::init_logging();
        let field = parse_input(PART_1_EXAMPLE_3)?;
        let visibility = field.calculate_all_visibilities();
        let best = visibility.get(&AsteroidPosition::new(1, 2)).unwrap();

        assert_eq!(best.len(), 35);

        Ok(())
    }

    #[test]
    fn calc_visibility_4() -> std::io::Result<()> {
        crate::init_logging();
        let field = parse_input(PART_1_EXAMPLE_4)?;
        let visibility = field.calculate_all_visibilities();
        let best = visibility.get(&AsteroidPosition::new(6, 3)).unwrap();

        assert_eq!(best.len(), 41);

        Ok(())
    }

    #[test]
    fn calc_visibility_5() -> std::io::Result<()> {
        crate::init_logging();
        let field = parse_input(PART_1_EXAMPLE_5)?;
        let visibility = field.calculate_all_visibilities();
        let best = visibility.get(&AsteroidPosition::new(11, 13)).unwrap();

        assert_eq!(best.len(), 210);

        Ok(())
    }

    #[test]
    fn best_monitoring_location_1() -> std::io::Result<()> {
        crate::init_logging();
        let field = parse_input(PART_1_EXAMPLE_1)?;
        let best = field.find_best_place_for_monitoring();
        const EXPECTED: AsteroidPosition = AsteroidPosition::new(3, 4);

        assert_eq!(best.0, EXPECTED);

        Ok(())
    }

    #[test]
    fn best_monitoring_location_2() -> std::io::Result<()> {
        crate::init_logging();
        let field = parse_input(PART_1_EXAMPLE_2)?;
        let best = field.find_best_place_for_monitoring();
        const EXPECTED: AsteroidPosition = AsteroidPosition::new(5, 8);

        assert_eq!(best.0, EXPECTED);

        Ok(())
    }

    #[test]
    fn best_monitoring_location_3() -> std::io::Result<()> {
        crate::init_logging();
        let field = parse_input(PART_1_EXAMPLE_3)?;
        let best = field.find_best_place_for_monitoring();
        const EXPECTED: AsteroidPosition = AsteroidPosition::new(1, 2);

        assert_eq!(best.0, EXPECTED);

        Ok(())
    }

    #[test]
    fn best_monitoring_location_4() -> std::io::Result<()> {
        crate::init_logging();
        let field = parse_input(PART_1_EXAMPLE_4)?;
        let best = field.find_best_place_for_monitoring();
        const EXPECTED: AsteroidPosition = AsteroidPosition::new(6, 3);

        assert_eq!(best.0, EXPECTED);

        Ok(())
    }

    #[test]
    fn best_monitoring_location_5() -> std::io::Result<()> {
        crate::init_logging();
        let field = parse_input(PART_1_EXAMPLE_5)?;
        let best = field.find_best_place_for_monitoring();
        const EXPECTED: AsteroidPosition = AsteroidPosition::new(11, 13);

        assert_eq!(best.0, EXPECTED);

        Ok(())
    }

    // _2_4_
    //     6
    //  3  _
    // 157 8
    // x9  _

    const ORDERING: &[Slope] = &[
        Slope::new(0, -1),
        Slope::new(1, -4),
        Slope::new(1, -3),
        Slope::new(1, -2),
        Slope::new(2, -3),
        Slope::new(3, -4),
        Slope::new(1, -1),
        Slope::new(4, -3),
        Slope::new(3, -2),
        Slope::new(2, -1),
        Slope::new(3, -1),
        Slope::new(4, -1),
        Slope::new(1, 0),
        Slope::new(4, 1),
        Slope::new(3, 1),
        Slope::new(2, 1),
        Slope::new(3, 2),
        Slope::new(4, 3),
        Slope::new(1, 1),
        Slope::new(3, 4),
        Slope::new(2, 3),
        Slope::new(1, 2),
        Slope::new(1, 3),
        Slope::new(1, 4),
        Slope::new(0, 1),
        Slope::new(-1, 4),
        Slope::new(-1, 3),
        Slope::new(-1, 2),
        Slope::new(-2, 3),
        Slope::new(-3, 4),
        Slope::new(-1, 1),
        Slope::new(-4, 3),
        Slope::new(-3, 2),
        Slope::new(-2, 1),
        Slope::new(-3, 1),
        Slope::new(-4, 1),
        Slope::new(-1, 0),
        Slope::new(-4, -1),
        Slope::new(-3, -1),
        Slope::new(-2, -1),
        Slope::new(-3, -2),
        Slope::new(-4, -3),
        Slope::new(-1, -1),
        Slope::new(-3, -4),
        Slope::new(-2, -3),
        Slope::new(-1, -2),
        Slope::new(-1, -3),
        Slope::new(-1, -4),
    ];

    #[test]
    fn min_value() {
        const MIN: Slope = Slope::new(0, -1);
        let mut success = true;
        for l in &ORDERING[..] {
            if *l != MIN {
                let cmp = l.cmp(&MIN);
                println!("{:?} {:?} {:?}", l, cmp, MIN);
                if cmp != std::cmp::Ordering::Greater {
                    success = false;
                }
            }
        }

        assert!(success);
    }

    #[test]
    fn max_value() {
        const MAX: Slope = Slope::new(-1, isize::min_value() / 3);
        let mut success = true;
        for l in &ORDERING[..] {
            if *l != MAX {
                let cmp = l.cmp(&MAX);
                println!("{:?} {:?} {:?}", l, cmp, MAX);
                if cmp != std::cmp::Ordering::Less {
                    success = false;
                }
            }
        }

        assert!(success);
    }

    #[test]
    fn ordering() {
        let mut success = true;
        for (l, r) in ORDERING[..].iter().zip(ORDERING[1..].iter()) {
            let ord = l.cmp(r);
            println!("{:?} ({}) {:?} {:?} ({})", l, l.theta(), ord, r, r.theta());
            if ord != std::cmp::Ordering::Less {
                success = false
            }
        }
        assert!(success)
    }

    #[test]
    fn ordering_equal() {
        let mut success = true;
        for (l, r) in ORDERING[..].iter().zip(ORDERING[..].iter()) {
            let ord = l.cmp(r);
            println!("{:?} {:?} {:?}", l, ord, r);
            if ord != std::cmp::Ordering::Equal {
                success = false
            }
        }
        assert!(success)
    }

    #[test]
    fn ordering_rev() {
        let mut success = true;
        for (l, r) in ORDERING[..]
            .iter()
            .rev()
            .zip(ORDERING[..].iter().rev().skip(1))
        {
            let ord = l.cmp(r);
            println!("{:?} {:?} {:?}", l, ord, r);
            if ord != std::cmp::Ordering::Greater {
                success = false
            }
        }
        assert!(success)
    }

    const PART_2_EXAMPLE: &str = "
.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....X...###..
..#.#.....#....##";

    #[test]
    fn vaporize_small() -> std::io::Result<()> {
        crate::init_logging();
        let field = parse_input(PART_2_EXAMPLE)?;
        let station = AsteroidPosition::new(8, 3);
        let vaporized_in_order: Vec<_> = field.vaporize_from(station).collect();

        for (i, a) in vaporized_in_order.iter().enumerate() {
            println!("Vaporized {:?} at {}", a, i + 1);
        }

        const VAPORIZE_ORDER: &[AsteroidPosition] = &[
            AsteroidPosition::new(8, 1),
            AsteroidPosition::new(9, 0),
            AsteroidPosition::new(9, 1),
            AsteroidPosition::new(10, 0),
            AsteroidPosition::new(9, 2),
            AsteroidPosition::new(11, 1),
            AsteroidPosition::new(12, 1),
            AsteroidPosition::new(11, 2),
            AsteroidPosition::new(15, 1),
            AsteroidPosition::new(12, 2),
            AsteroidPosition::new(13, 2),
            AsteroidPosition::new(14, 2),
            AsteroidPosition::new(15, 2),
            AsteroidPosition::new(12, 3),
            AsteroidPosition::new(16, 4),
            AsteroidPosition::new(15, 4),
            AsteroidPosition::new(10, 4),
            AsteroidPosition::new(4, 4),
            AsteroidPosition::new(2, 4),
            AsteroidPosition::new(2, 3),
            AsteroidPosition::new(0, 2),
            AsteroidPosition::new(1, 2),
            AsteroidPosition::new(0, 1),
            AsteroidPosition::new(1, 1),
            AsteroidPosition::new(5, 2),
            AsteroidPosition::new(1, 0),
            AsteroidPosition::new(5, 1),
            AsteroidPosition::new(6, 1),
            AsteroidPosition::new(6, 0),
            AsteroidPosition::new(7, 0),
            AsteroidPosition::new(8, 0),
            AsteroidPosition::new(10, 1),
            AsteroidPosition::new(14, 0),
            AsteroidPosition::new(16, 1),
            AsteroidPosition::new(13, 3),
            AsteroidPosition::new(14, 3),
        ];

        assert_eq!(vaporized_in_order, VAPORIZE_ORDER);

        Ok(())
    }

    #[test]
    fn vaporize_large() -> std::io::Result<()> {
        crate::init_logging();
        let field = parse_input(PART_1_EXAMPLE_5)?;
        let station = AsteroidPosition::new(11, 13);
        let vaporized_in_order: Vec<_> = field.vaporize_from(station).collect();

        for (i, a) in vaporized_in_order.iter().enumerate() {
            println!("Vaporized {:?} at {}", a, i + 1);
        }

        assert_eq!(vaporized_in_order.len(), 299);
        assert_eq!(vaporized_in_order[0], AsteroidPosition::new(11, 12));
        assert_eq!(vaporized_in_order[1], AsteroidPosition::new(12, 1));
        assert_eq!(vaporized_in_order[2], AsteroidPosition::new(12, 2));
        assert_eq!(vaporized_in_order[9], AsteroidPosition::new(12, 8));
        assert_eq!(vaporized_in_order[19], AsteroidPosition::new(16, 0));
        assert_eq!(vaporized_in_order[49], AsteroidPosition::new(16, 9));
        assert_eq!(vaporized_in_order[99], AsteroidPosition::new(10, 16));
        assert_eq!(vaporized_in_order[198], AsteroidPosition::new(9, 6));
        assert_eq!(vaporized_in_order[199], AsteroidPosition::new(8, 2));
        assert_eq!(vaporized_in_order[200], AsteroidPosition::new(10, 9));
        assert_eq!(vaporized_in_order[298], AsteroidPosition::new(11, 1));

        Ok(())
    }
}
