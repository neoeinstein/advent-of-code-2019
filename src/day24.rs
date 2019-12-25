//! # Day 24: Planet of Discord
//!
//! You land on Eris, your last stop before reaching Santa. As soon as you do,
//! your sensors start picking up strange life forms moving around: Eris is
//! infested with bugs! With an over 24-hour roundtrip for messages between you
//! and Earth, you'll have to deal with this problem on your own.
//!
//! Eris isn't a very large place; a scan of the entire area fits into a 5x5
//! grid (your puzzle input). The scan shows bugs (`#`) and empty spaces (`.`).
//!
//! Each minute, The bugs live and die based on the number of bugs in the four
//! adjacent tiles:
//!
//! * A bug dies (becoming an empty space) unless there is exactly one bug
//!   adjacent to it.
//! * An empty space becomes infested with a bug if exactly one or two bugs are
//!   adjacent to it.
//!
//! Otherwise, a bug or empty space remains the same. (Tiles on the edges of the
//! grid have fewer than four adjacent tiles; the missing tiles count as empty
//! space.) This process happens in every location simultaneously; that is,
//! within the same minute, the number of adjacent bugs is counted for every
//! tile first, and then the tiles are updated.
//!
//! Here are the first few minutes of an example scenario:
//!
//! Initial state:
//!
//! ```text
//! ....#
//! #..#.
//! #..##
//! ..#..
//! #....
//! ```
//!
//! After 1 minute:
//!
//! ```text
//! #..#.
//! ####.
//! ###.#
//! ##.##
//! .##..
//! ```
//!
//! After 2 minutes:
//!
//! ```text
//! #####
//! ....#
//! ....#
//! ...#.
//! #.###
//! ```
//!
//! After 3 minutes:
//!
//! ```text
//! #....
//! ####.
//! ...##
//! #.##.
//! .##.#
//! ```
//!
//! After 4 minutes:
//!
//! ```text
//! ####.
//! ....#
//! ##..#
//! .....
//! ##...
//! ```
//!
//! To understand the nature of the bugs, watch for the first time a layout of
//! bugs and empty spaces matches any previous layout. In the example above, the
//! first layout to appear twice is:
//!
//! ```text
//! .....
//! .....
//! .....
//! #....
//! .#...
//! ```
//!
//! To calculate the biodiversity rating for this layout, consider each tile
//! left-to-right in the top row, then left-to-right in the second row, and so
//! on. Each of these tiles is worth biodiversity points equal to increasing
//! powers of two: 1, 2, 4, 8, 16, 32, and so on. Add up the biodiversity points
//! for tiles with bugs; in this example, the 16th tile (32768 points) and 22nd
//! tile (2097152 points) have bugs, a total biodiversity rating of 2129920.
//!
//! What is the biodiversity rating for the first layout that appears twice?
//!
//! ## Part Two
//!
//! After careful analysis, one thing is certain: you have no idea where all
//! these bugs are coming from.
//!
//! Then, you remember: Eris is an old Plutonian settlement! Clearly, the bugs
//! are coming from recursively-folded space.
//!
//! This 5x5 grid is only one level in an infinite number of recursion levels.
//! The tile in the middle of the grid is actually another 5x5 grid, the grid in
//! your scan is contained as the middle tile of a larger 5x5 grid, and so on.
//! Two levels of grids look like this:
//!
//! ```text
//!      |     |         |     |
//!      |     |         |     |
//!      |     |         |     |
//! -----+-----+---------+-----+-----
//!      |     |         |     |
//!      |     |         |     |
//!      |     |         |     |
//! -----+-----+---------+-----+-----
//!      |     | | | | | |     |
//!      |     |-+-+-+-+-|     |
//!      |     | | | | | |     |
//!      |     |-+-+-+-+-|     |
//!      |     | | |?| | |     |
//!      |     |-+-+-+-+-|     |
//!      |     | | | | | |     |
//!      |     |-+-+-+-+-|     |
//!      |     | | | | | |     |
//! -----+-----+---------+-----+-----
//!      |     |         |     |
//!      |     |         |     |
//!      |     |         |     |
//! -----+-----+---------+-----+-----
//!      |     |         |     |
//!      |     |         |     |
//!      |     |         |     |
//! ```
//!
//! (To save space, some of the tiles are not drawn to scale.) Remember, this is
//! only a small part of the infinitely recursive grid; there is a 5x5 grid that
//! contains this diagram, and a 5x5 grid that contains that one, and so on.
//! Also, the `?` in the diagram contains another 5x5 grid, which itself
//! contains another 5x5 grid, and so on.
//!
//! The scan you took (your puzzle input) shows where the bugs are on a single
//! level of this structure. The middle tile of your scan is empty to
//! accommodate the recursive grids within it. Initially, no other levels
//! contain bugs.
//!
//! Tiles still count as adjacent if they are directly up, down, left, or right
//! of a given tile. Some tiles have adjacent tiles at a recursion level above
//! or below its own level. For example:
//!
//! ```text
//!      |     |         |     |
//!   1  |  2  |    3    |  4  |  5
//!      |     |         |     |
//! -----+-----+---------+-----+-----
//!      |     |         |     |
//!   6  |  7  |    8    |  9  |  10
//!      |     |         |     |
//! -----+-----+---------+-----+-----
//!      |     |A|B|C|D|E|     |
//!      |     |-+-+-+-+-|     |
//!      |     |F|G|H|I|J|     |
//!      |     |-+-+-+-+-|     |
//!  11  | 12  |K|L|?|N|O|  14 |  15
//!      |     |-+-+-+-+-|     |
//!      |     |P|Q|R|S|T|     |
//!      |     |-+-+-+-+-|     |
//!      |     |U|V|W|X|Y|     |
//! -----+-----+---------+-----+-----
//!      |     |         |     |
//!  16  | 17  |    18   |  19 |  20
//!      |     |         |     |
//! -----+-----+---------+-----+-----
//!      |     |         |     |
//!  21  | 22  |    23   |  24 |  25
//!      |     |         |     |
//! ```
//!
//! * Tile `19` has four adjacent tiles: `14`, `18`, `20`, and `24`.
//! * Tile `G` has four adjacent tiles: `B`, `F`, `H`, and `L`.
//! * Tile `D` has four adjacent tiles: `8`, `C`, `E`, and `I`.
//! * Tile `E` has four adjacent tiles: `8`, `D`, `14`, and `J`.
//! * Tile `14` has eight adjacent tiles: `9`, `E`, `J`, `O`, `T`, `Y`, `15`,
//!   and `19`.
//! * Tile `N` has eight adjacent tiles: `I`, `O`, `S`, and five tiles within
//!   the sub-grid marked `?`.
//!
//! The rules about bugs living and dying are the same as before.
//!
//! For example, consider the same initial state as above:
//!
//! ```text
//! ....#
//! #..#.
//! #.?##
//! ..#..
//! #....
//! ```
//!
//! The center tile is drawn as `?` to indicate the next recursive grid. Call
//! this level 0; the grid within this one is level 1, and the grid that
//! contains this one is level -1. Then, after ten minutes, the grid at each
//! level would look like this:
//!
//! Depth -5:
//!
//! ```text
//! ..#..
//! .#.#.
//! ..?.#
//! .#.#.
//! ..#..
//! ```
//!
//! Depth -4:
//!
//! ```text
//! ...#.
//! ...##
//! ..?..
//! ...##
//! ...#.
//! ```
//!
//! Depth -3:
//!
//! ```text
//! #.#..
//! .#...
//! ..?..
//! .#...
//! #.#..
//! ```
//!
//! Depth -2:
//!
//! ```text
//! .#.##
//! ....#
//! ..?.#
//! ...##
//! .###.
//! ```
//!
//! Depth -1:
//!
//! ```text
//! #..##
//! ...##
//! ..?..
//! ...#.
//! .####
//! ```
//!
//! Depth 0:
//!
//! ```text
//! .#...
//! .#.##
//! .#?..
//! .....
//! .....
//! ```
//!
//! Depth 1:
//!
//! ```text
//! .##..
//! #..##
//! ..?.#
//! ##.##
//! #####
//! ```
//!
//! Depth 2:
//!
//! ```text
//! ###..
//! ##.#.
//! #.?..
//! .#.##
//! #.#..
//! ```
//!
//! Depth 3:
//!
//! ```text
//! ..###
//! .....
//! #.?..
//! #....
//! #...#
//! ```
//!
//! Depth 4:
//!
//! ```text
//! .###.
//! #..#.
//! #.?..
//! ##.#.
//! .....
//! ```
//!
//! Depth 5:
//!
//! ```text
//! ####.
//! #..#.
//! #.?#.
//! ####.
//! .....
//! ```
//!
//! In this example, after 10 minutes, a total of 99 bugs are present.
//!
//! Starting with your scan, how many bugs are present after 200 minutes?

use super::{Grid, GridPosition, Orientation};
use anyhow::{anyhow, Error, Result};
use std::collections::BTreeSet;

const PUZZLE_INPUT: &str = include_str!("../inputs/input-24");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

impl Cell {
    fn is_alive(self) -> bool {
        self == Cell::Alive
    }
}

impl std::str::FromStr for Cell {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." | "?" => Ok(Self::Dead),
            "#" => Ok(Self::Alive),
            _ => Err(anyhow!("Unrecognized cell type: {}", s)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Field {
    grid: Grid<Cell>,
}

impl Field {
    fn new(grid: Grid<Cell>) -> Self {
        Self { grid }
    }

    fn biodiversity_value(&self) -> u32 {
        let mut val = 0;
        for (i, _) in self
            .grid
            .enumerate()
            .map(|(_, c)| c)
            .enumerate()
            .filter(|x| *x.1 == Cell::Alive)
        {
            val += 1 << i;
        }
        val
    }

    fn step(&self) -> Self {
        let iter = self.grid.enumerate().map(|(pos, cell)| {
            let live_neighbors = self
                .grid
                .neighbors(pos)
                .filter(|(_, c)| c.is_alive())
                .count();
            log::trace!(
                "Position {}: cell = {:?}, live neighbors = {}",
                pos,
                cell,
                live_neighbors
            );
            match cell {
                Cell::Alive if live_neighbors == 1 => Cell::Alive,
                Cell::Dead if live_neighbors == 1 || live_neighbors == 2 => Cell::Alive,
                _ => Cell::Dead,
            }
        });
        Self {
            grid: Grid::from_elements(self.grid.rows(), self.grid.columns(), iter),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RecursiveField {
    grids: Vec<Grid<Cell>>,
}

impl RecursiveField {
    fn new(grid: Grid<Cell>) -> Self {
        Self { grids: vec![grid] }
    }

    fn neighbors(position: GridPosition) -> impl IntoIterator<Item = (isize, GridPosition)> {
        let left_outer = if position.col == 0 {
            Some((1, GridPosition { row: 2, col: 1 }))
        } else {
            None
        };
        let top_outer = if position.row == 0 {
            Some((1, GridPosition { row: 1, col: 2 }))
        } else {
            None
        };
        let right_outer = if position.col == 4 {
            Some((1, GridPosition { row: 2, col: 3 }))
        } else {
            None
        };
        let bottom_outer = if position.row == 4 {
            Some((1, GridPosition { row: 3, col: 2 }))
        } else {
            None
        };
        let outer = left_outer
            .into_iter()
            .chain(top_outer)
            .chain(right_outer)
            .chain(bottom_outer);
        let left_inner = if position.col == 1 && position.row == 2 {
            vec![
                (-1, GridPosition { row: 0, col: 0 }),
                (-1, GridPosition { row: 1, col: 0 }),
                (-1, GridPosition { row: 2, col: 0 }),
                (-1, GridPosition { row: 3, col: 0 }),
                (-1, GridPosition { row: 4, col: 0 }),
            ]
        } else {
            Vec::default()
        };
        let top_inner = if position.row == 1 && position.col == 2 {
            vec![
                (-1, GridPosition { row: 0, col: 0 }),
                (-1, GridPosition { row: 0, col: 1 }),
                (-1, GridPosition { row: 0, col: 2 }),
                (-1, GridPosition { row: 0, col: 3 }),
                (-1, GridPosition { row: 0, col: 4 }),
            ]
        } else {
            Vec::default()
        };
        let right_inner = if position.col == 3 && position.row == 2 {
            vec![
                (-1, GridPosition { row: 0, col: 4 }),
                (-1, GridPosition { row: 1, col: 4 }),
                (-1, GridPosition { row: 2, col: 4 }),
                (-1, GridPosition { row: 3, col: 4 }),
                (-1, GridPosition { row: 4, col: 4 }),
            ]
        } else {
            Vec::default()
        };
        let bottom_inner = if position.row == 3 && position.col == 2 {
            vec![
                (-1, GridPosition { row: 4, col: 0 }),
                (-1, GridPosition { row: 4, col: 1 }),
                (-1, GridPosition { row: 4, col: 2 }),
                (-1, GridPosition { row: 4, col: 3 }),
                (-1, GridPosition { row: 4, col: 4 }),
            ]
        } else {
            Vec::default()
        };
        let inner = left_inner
            .into_iter()
            .chain(top_inner)
            .chain(right_inner)
            .chain(bottom_inner);
        let same = position
            .neighbor(Orientation::North)
            .into_iter()
            .chain(position.neighbor(Orientation::West))
            .chain(position.neighbor(Orientation::East))
            .chain(position.neighbor(Orientation::South))
            .filter(|p| !(p.row == 2 && p.col == 2))
            .map(|p| (0, p));
        outer.chain(inner).chain(same)
    }

    fn grid_step(&self, level: isize) -> Grid<Cell> {
        let empty_grid = Grid::new(Cell::Dead, 5, 5);
        let grid = if level >= 0 {
            self.grids
                .get(level as usize)
                .cloned()
                .unwrap_or(empty_grid)
        } else {
            empty_grid
        };
        if grid == Grid::new(Cell::Dead, 5, 5) {
            log::debug!("Processing empty grid");
        }
        let iter = grid.enumerate().map(|(pos, cell)| {
            if pos == GridPosition::new(2, 2) {
                return Cell::Dead;
            }
            let live_neighbors = Self::neighbors(pos)
                .into_iter()
                .filter_map(|(offset, p)| {
                    let actual_lvl = level + offset;
                    if actual_lvl < 0 || actual_lvl >= self.grids.len() as isize {
                        None
                    } else {
                        self.grids[actual_lvl as usize].get(p)
                    }
                })
                .filter(|c| c.is_alive())
                .count();
            let result = match cell {
                Cell::Alive if live_neighbors == 1 => Cell::Alive,
                Cell::Dead if live_neighbors == 1 || live_neighbors == 2 => Cell::Alive,
                _ => Cell::Dead,
            };
            log::trace!(
                "Position {}: cell = {:?}, live neighbors = {}, result = {:?}",
                pos,
                cell,
                live_neighbors,
                result
            );
            result
        });

        Grid::from_elements(5, 5, iter)
    }

    fn step(&self) -> Self {
        let mut new_grids: Vec<_> = (-1..=self.grids.len() as isize)
            .map(|lvl| {
                log::debug!("Processing level {}", lvl);
                self.grid_step(lvl)
            })
            .collect();

        let start = if new_grids[0].enumerate().any(|(_, c)| c.is_alive()) {
            0
        } else {
            1
        };

        let end = if new_grids[new_grids.len() - 1]
            .enumerate()
            .any(|(_, c)| c.is_alive())
        {
            new_grids.len()
        } else {
            new_grids.len() - 1
        };

        Self {
            grids: new_grids.drain(start..end).collect(),
        }
    }

    fn bugs(&self) -> usize {
        self.grids
            .iter()
            .map(|g| g.enumerate().filter(|(_, c)| c.is_alive()).count())
            .sum()
    }
}

fn part1(mut field: Field) -> u32 {
    let mut history = BTreeSet::new();
    let mut i = 0;
    while history.insert(field.biodiversity_value()) {
        log::debug!("Step {} biodeversity: {}", i, field.biodiversity_value());
        field = field.step();
        i += 1;
    }
    log::info!(
        "Final step {} biodeversity: {}",
        i,
        field.biodiversity_value()
    );
    field.biodiversity_value()
}

fn part2(mut field: RecursiveField, steps: usize) -> RecursiveField {
    for i in 0..steps {
        log::debug!("Step {} bug_count: {}", i, field.bugs());
        field = field.step();
    }
    log::info!("Final step bug_count: {}", field.bugs());
    field
}

pub fn run() -> Result<()> {
    let grid: Grid<Cell> = PUZZLE_INPUT.parse()?;

    let field = Field::new(grid.clone());
    let result = part1(field);
    println!("First repeated biodeversity value: {}", result);

    let field2 = RecursiveField::new(grid);
    let result = part2(field2, 200);
    println!(
        "Bugs in recursive field after 200 minutes: {}",
        result.bugs()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Field, RecursiveField};
    use pretty_assertions::assert_eq;

    const EXAMPLE_STEP_0: &str = "\
                                  ....#\n\
                                  #..#.\n\
                                  #..##\n\
                                  ..#..\n\
                                  #....";

    const EXAMPLE_STEP_1: &str = "\
                                  #..#.\n\
                                  ####.\n\
                                  ###.#\n\
                                  ##.##\n\
                                  .##..";

    const EXAMPLE_STEP_2: &str = "\
                                  #####\n\
                                  ....#\n\
                                  ....#\n\
                                  ...#.\n\
                                  #.###";

    const EXAMPLE_STEP_3: &str = "\
                                  #....\n\
                                  ####.\n\
                                  ...##\n\
                                  #.##.\n\
                                  .##.#";

    const EXAMPLE_STEP_4: &str = "\
                                  ####.\n\
                                  ....#\n\
                                  ##..#\n\
                                  .....\n\
                                  ##...";

    #[test]
    fn validate_step_1() {
        crate::init_logging();
        let init = Field::new(EXAMPLE_STEP_0.parse().unwrap());
        let expected = Field::new(EXAMPLE_STEP_1.parse().unwrap());

        assert_eq!(init.step(), expected);
    }

    #[test]
    fn validate_step_2() {
        crate::init_logging();
        let init = Field::new(EXAMPLE_STEP_1.parse().unwrap());
        let expected = Field::new(EXAMPLE_STEP_2.parse().unwrap());

        assert_eq!(init.step(), expected);
    }

    #[test]
    fn validate_step_3() {
        crate::init_logging();
        let init = Field::new(EXAMPLE_STEP_2.parse().unwrap());
        let expected = Field::new(EXAMPLE_STEP_3.parse().unwrap());

        assert_eq!(init.step(), expected);
    }

    #[test]
    fn validate_step_4() {
        crate::init_logging();
        let init = Field::new(EXAMPLE_STEP_3.parse().unwrap());
        let expected = Field::new(EXAMPLE_STEP_4.parse().unwrap());

        assert_eq!(init.step(), expected);
    }

    #[test]
    fn part1_example() {
        crate::init_logging();
        let init = Field::new(EXAMPLE_STEP_0.parse().unwrap());
        let expected = 2_129_920;

        assert_eq!(super::part1(init), expected);
    }

    const PART_2_INITIAL: &str = "\
                                  ....#\n\
                                  #..#.\n\
                                  #.?##\n\
                                  ..#..\n\
                                  #....";

    const PART_2_AFTER_1_MINUTE: [&str; 3] = [
        ".....\n\
         ..#..\n\
         ..?#.\n\
         ..#..\n\
         .....",
        "#..#.\n\
         ####.\n\
         ##?.#\n\
         ##.##\n\
         .##..",
        "....#\n\
         ....#\n\
         ..?.#\n\
         ....#\n\
         #####",
    ];

    const PART_2_AFTER_10_MINUTES: [&str; 11] = [
        "..#..\n\
         .#.#.\n\
         ..?.#\n\
         .#.#.\n\
         ..#..",
        "...#.\n\
         ...##\n\
         ..?..\n\
         ...##\n\
         ...#.",
        "#.#..\n\
         .#...\n\
         ..?..\n\
         .#...\n\
         #.#..",
        ".#.##\n\
         ....#\n\
         ..?.#\n\
         ...##\n\
         .###.",
        "#..##\n\
         ...##\n\
         ..?..\n\
         ...#.\n\
         .####",
        ".#...\n\
         .#.##\n\
         .#?..\n\
         .....\n\
         .....",
        ".##..\n\
         #..##\n\
         ..?.#\n\
         ##.##\n\
         #####",
        "###..\n\
         ##.#.\n\
         #.?..\n\
         .#.##\n\
         #.#..",
        "..###\n\
         .....\n\
         #.?..\n\
         #....\n\
         #...#",
        ".###.\n\
         #..#.\n\
         #.?..\n\
         ##.#.\n\
         .....",
        "####.\n\
         #..#.\n\
         #.?#.\n\
         ####.\n\
         .....",
    ];

    #[test]
    fn part_2_example_10_mins() {
        crate::init_logging();
        let init = RecursiveField::new(PART_2_INITIAL.parse().unwrap());
        let expected = RecursiveField {
            grids: PART_2_AFTER_10_MINUTES[..]
                .iter()
                .rev()
                .map(|s| s.parse().unwrap())
                .collect(),
        };

        assert_eq!(super::part2(init, 10), expected);
    }

    #[test]
    fn part_2_example_1_min() {
        crate::init_logging();
        let init = RecursiveField::new(PART_2_INITIAL.parse().unwrap());
        let expected = RecursiveField {
            grids: PART_2_AFTER_1_MINUTE[..]
                .iter()
                .rev()
                .map(|s| s.parse().unwrap())
                .collect(),
        };

        assert_eq!(super::part2(init, 1), expected);
    }
}
