//! # Day 19: Tractor Beam
//!
//! Unsure of the state of Santa's ship, you borrowed the tractor beam
//! technology from Triton. Time to test it out.
//!
//! When you're safely away from anything else, you activate the tractor beam,
//! but nothing happens. It's hard to tell whether it's working if there's
//! nothing to use it on. Fortunately, your ship's drone system can be
//! configured to deploy a drone to specific coordinates and then check whether
//! it's being pulled. There's even an Intcode program (your puzzle input) that
//! gives you access to the drone system.
//!
//! The program uses two input instructions to request the X and Y position to
//! which the drone should be deployed. Negative numbers are invalid and will
//! confuse the drone; all numbers should be zero or positive.
//!
//! Then, the program will output whether the drone is stationary (0) or being
//! pulled by something (1). For example, the coordinate X=0, Y=0 is directly in
//! front of the tractor beam emitter, so the drone control program will always
//! report 1 at that location.
//!
//! To better understand the tractor beam, it is important to get a good picture
//! of the beam itself. For example, suppose you scan the 10x10 grid of points
//! closest to the emitter:
//!
//! ```text
//!        X
//!   0->      9
//!  0#.........
//!  |.#........
//!  v..##......
//!   ...###....
//!   ....###...
//! Y .....####.
//!   ......####
//!   ......####
//!   .......###
//!  9........##
//! ```
//!
//! In this example, the number of points affected by the tractor beam in the
//! 10x10 area closest to the emitter is 27.
//!
//! However, you'll need to scan a larger area to understand the shape of the
//! beam. How many points are affected by the tractor beam in the 50x50 area
//! closest to the emitter? (For each of X and Y, this will be 0 through 49.)
//!
//! ## Part Two
//!
//! You aren't sure how large Santa's ship is. You aren't even sure if you'll
//! need to use this thing on Santa's ship, but it doesn't hurt to be prepared.
//! You figure Santa's ship might fit in a 100x100 square.
//!
//! The beam gets wider as it travels away from the emitter; you'll need to be a
//! minimum distance away to fit a square of that size into the beam fully.
//! (Don't rotate the square; it should be aligned to the same axes as the drone
//! grid.)
//!
//! For example, suppose you have the following tractor beam readings:
//!
//! ```text
//! #.......................................
//! .#......................................
//! ..##....................................
//! ...###..................................
//! ....###.................................
//! .....####...............................
//! ......#####.............................
//! ......######............................
//! .......#######..........................
//! ........########........................
//! .........#########......................
//! ..........#########.....................
//! ...........##########...................
//! ...........############.................
//! ............############................
//! .............#############..............
//! ..............##############............
//! ...............###############..........
//! ................###############.........
//! ................#################.......
//! .................########OOOOOOOOOO.....
//! ..................#######OOOOOOOOOO#....
//! ...................######OOOOOOOOOO###..
//! ....................#####OOOOOOOOOO#####
//! .....................####OOOOOOOOOO#####
//! .....................####OOOOOOOOOO#####
//! ......................###OOOOOOOOOO#####
//! .......................##OOOOOOOOOO#####
//! ........................#OOOOOOOOOO#####
//! .........................OOOOOOOOOO#####
//! ..........................##############
//! ..........................##############
//! ...........................#############
//! ............................############
//! .............................###########
//! ```
//!
//! In this example, the 10x10 square closest to the emitter that fits entirely
//! within the tractor beam has been marked `O`. Within it, the point closest to
//! the emitter (the only highlighted O) is at X=25, Y=20.
//!
//! Find the 100x100 square closest to the emitter that fits entirely within the
//! tractor beam; within that square, find the point closest to the emitter.
//! What value do you get if you take that point's X coordinate, multiply it by
//! 10000, then add the point's Y coordinate? (In the example above, this would
//! be 250020.)

use super::{Grid, GridPosition, Orientation};
use anyhow::{anyhow, Result};
use std::{cmp::Ordering, fmt};
use tokio::sync::mpsc::channel;

const PUZZLE_INPUT: &str = include_str!("../inputs/input-19");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BeamPosition {
    OutOfBeam,
    InBeam,
}

impl Default for BeamPosition {
    fn default() -> Self {
        Self::OutOfBeam
    }
}

impl fmt::Display for BeamPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match *self {
            BeamPosition::OutOfBeam => ".",
            BeamPosition::InBeam => "#",
        };
        f.write_str(ch)
    }
}

async fn read_position(program: &intcode::Memory, pos: GridPosition) -> Result<BeamPosition> {
    let mut camera = channel(1);
    let mut command = channel(2);
    let mut exe = intcode::AsyncExecutable::from(program.clone());
    exe.pipe_outputs_to(camera.0);
    exe.pipe_inputs_from(command.1);
    tokio::spawn(exe.execute());

    if command.0.send(pos.col as intcode::Word).await.is_err() {
        return Err(anyhow!("Unexpected end on col"));
    }
    if command.0.send(pos.row as intcode::Word).await.is_err() {
        return Err(anyhow!("Unexpected end on row"));
    }

    if let Some(result) = camera.1.recv().await {
        match result {
            0 => Ok(BeamPosition::OutOfBeam),
            1 => Ok(BeamPosition::InBeam),
            _ => Err(anyhow!("Unknown beam response: {}", result)),
        }
    } else {
        Err(anyhow!("Unexpected end on read"))
    }
}

async fn test_orientation(
    program: &intcode::Memory,
    hint: GridPosition,
    goal: usize,
    orientation: Orientation,
) -> Result<Ordering> {
    if read_position(program, hint.relative(orientation, goal - 1).unwrap()).await?
        == BeamPosition::OutOfBeam
    {
        Ok(Ordering::Less)
    } else if read_position(program, hint.relative(orientation, goal).unwrap()).await?
        == BeamPosition::OutOfBeam
    {
        Ok(Ordering::Equal)
    } else {
        Ok(Ordering::Greater)
    }
}

async fn find_for_size(
    program: &intcode::Memory,
    mut hint: GridPosition,
    goal: usize,
) -> Result<GridPosition> {
    let mut last_move;
    let mut correction = Orientation::South;
    loop {
        let mut attempts = 0;
        while read_position(program, hint).await? == BeamPosition::OutOfBeam {
            log::warn!(
                "hint {} outside of beam, trying to correct {:?}",
                hint,
                correction
            );
            hint = hint.neighbor(correction).unwrap();
            attempts += 1;
            if attempts > 5 {
                hint.move_relative(correction.reverse(), attempts);
                correction.turn_left();
                attempts = 0;
            }
            // return Err(anyhow!("hint {} outside of beam", hint));
        }

        let height_cmp = test_orientation(program, hint, goal, Orientation::South).await?;
        let width_cmp = test_orientation(program, hint, goal, Orientation::East).await?;

        log::debug!(
            "testing {}: height {:?}, width {:?}, goal: {}",
            hint,
            height_cmp,
            width_cmp,
            goal
        );

        if height_cmp == Ordering::Equal && width_cmp == Ordering::Equal {
            return Ok(hint);
        }

        match height_cmp {
            Ordering::Less => {
                last_move = Orientation::East;
                correction = Orientation::South;
            }
            Ordering::Equal => match width_cmp {
                Ordering::Less => {
                    last_move = Orientation::South;
                    correction = Orientation::East;
                }
                Ordering::Equal => return Ok(hint),
                Ordering::Greater => {
                    last_move = Orientation::North;
                    correction = Orientation::West;
                }
            },
            Ordering::Greater => {
                last_move = Orientation::West;
                correction = Orientation::North;
            }
        }
        hint = hint.neighbor(last_move).unwrap();
        log::debug!("moving {} to {}", last_move, hint);
    }
}

// async fn find_for_size(program: &intcode::Memory, mut hint: GridPosition,
// goal: usize) -> Result<GridPosition> {     loop {
//         let mut col_test = GridPosition {
//             col: hint.col,
//             row: hint.row + goal,
//         };
//         let mut row_test = GridPosition {
//             col: hint.col + goal,
//             row: hint.row,
//         };
//         let mut out_of_beam = false;
//         loop {
//             loop {
//                 if read_position(program, hint).await? ==
// BeamPosition::OutOfBeam {                     if out_of_beam {
//                         return Err(anyhow!("Fell out of beam at {}", hint));
//                     }
//                     hint.col += 1;
//                 } else {
//                     out_of_beam = false;
//                     break;
//                 }
//             }

//             col_test = GridPosition {
//                 col: hint.col,
//                 row: hint.row + goal,
//             };
//             log::debug!("testing {}", col_test);
//             if read_position(program, col_test).await? ==
// BeamPosition::OutOfBeam {                 col_test.row -= 1;
//             } else {
//                 // break;
//             }
//             if col_test.row < hint.row {
//                 col_test.row = hint.row
//             }
//             let height = hint.row - row_test.row;
//             match height.cmp(&goal) {
//                 std::cmp::Ordering::Less => hint.col += 1,
//                 std::cmp::Ordering::Equal => break,
//                 std::cmp::Ordering::Greater => hint.col -= 1,
//             }
//         }
//         //assert!(col_test.row >= hint.row);

//         loop {
//             col_test = GridPosition {
//                 col: hint.col,
//                 row: hint.row + goal,
//             };
//             log::trace!("testing {}", row_test);
//             if read_position(program, row_test).await? ==
// BeamPosition::OutOfBeam {                 row_test.col -= 1;
//             } else {
//                 // break;
//             }
//             if row_test.col < col_test.col {
//                 row_test.col = col_test.col
//             }

//             let width = row_test.col - col_test.col;

//             match width.cmp(&goal) {
//                 std::cmp::Ordering::Less => hint.row += 1,
//                 std::cmp::Ordering::Equal => break,
//                 std::cmp::Ordering::Greater => hint.row -= 1,
//             }
//         }
//         //assert!(col_test.row >= hint.row);

//         let height = col_test.row - row_test.row;
//         let width = row_test.col - col_test.col;

//         log::debug!("tested {}: height = {}, width = {}", hint, height,
// width);

//         if height == width && height == goal {
//             return Ok(hint);
//         }
//     }
// }

async fn define_beam(program: intcode::Memory) -> Result<Grid<intcode::Word>> {
    const SIZE: usize = 50;
    let mut grid = Grid::new(intcode::Word::default(), SIZE, SIZE);

    'outer: for col in 0..SIZE {
        for row in 0..SIZE {
            let mut camera = channel(1);
            let mut command = channel(2);
            let mut exe = intcode::AsyncExecutable::from(program.clone());
            exe.pipe_outputs_to(camera.0);
            exe.pipe_inputs_from(command.1);
            tokio::spawn(exe.execute());

            if command.0.send(col as intcode::Word).await.is_err() {
                break 'outer;
            }
            if command.0.send(row as intcode::Word).await.is_err() {
                break 'outer;
            }
            if let Some(result) = camera.1.recv().await {
                grid.set(
                    GridPosition {
                        row: row as usize,
                        col: col as usize,
                    },
                    result,
                );
            } else {
                break 'outer;
            }
        }
    }

    Ok(grid)
}

async fn find_sleigh(program: &intcode::Memory) -> Result<GridPosition> {
    let mut hint = GridPosition { row: 4, col: 5 };
    for goal in 2..=100 {
        log::info!("Looking for size {} starting from {}", goal, hint);
        hint = find_for_size(program, hint, goal).await?;
    }
    Ok(hint)
}

pub fn run() -> Result<()> {
    let program: intcode::Memory = PUZZLE_INPUT.parse()?;

    let mut runtime = tokio::runtime::Runtime::new()?;

    let grid = runtime.block_on(define_beam(program.clone()))?;

    let ones = grid.enumerate().filter(|&(_, &x)| x != 0).count();
    println!("1s: {}", ones);
    println!("Grid:\n{}", grid);

    let mut min_row = 49;
    let mut max_row = 0;
    for row in 0..50 {
        if *grid.get(GridPosition { row, col: 49 }).unwrap() == 1 {
            min_row = row.min(min_row);
            max_row = row.max(max_row);
        }
    }

    let mut min_col = 49;
    let mut max_col = 0;
    for col in 0..50 {
        if *grid.get(GridPosition { row: 49, col }).unwrap() == 1 {
            min_col = col.min(min_col);
            max_col = col.max(max_col);
        }
    }

    let upper = GridPosition {
        row: if min_row > max_row { 49 } else { min_row },
        col: if min_col > max_col { 49 } else { max_col },
    };

    let lower = GridPosition {
        row: if min_row > max_row { 49 } else { max_row },
        col: if min_col > max_col { 49 } else { min_col },
    };

    println!("Upper: {}, lower: {}", upper, lower);

    let sleigh = runtime.block_on(find_sleigh(&program))?;
    //let sleigh = runtime.block_on(shrink(&program, sleigh, 100))?;

    println!("Found sleigh at {}", sleigh);

    Ok(())
}
