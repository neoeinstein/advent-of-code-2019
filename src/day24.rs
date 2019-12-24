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

use super::Grid;
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
            "." => Ok(Self::Dead),
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

pub fn run() -> Result<()> {
    let grid = PUZZLE_INPUT.parse()?;
    let field = Field::new(grid);

    let result = part1(field);
    println!("First repeated biodeversity value: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Field;
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
}
