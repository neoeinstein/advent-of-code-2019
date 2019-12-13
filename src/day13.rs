//! # Day 13: Care Package
//!
//! As you ponder the solitude of space and the ever-increasing three-hour
//! roundtrip for messages between you and Earth, you notice that the Space Mail
//! Indicator Light is blinking. To help keep you sane, the Elves have sent you
//! a care package.
//!
//! It's a new game for the ship's arcade cabinet! Unfortunately, the arcade is
//! all the way on the other end of the ship. Surely, it won't be hard to build
//! your own - the care package even comes with schematics.
//!
//! The arcade cabinet runs Intcode software like the game the Elves sent (your
//! puzzle input). It has a primitive screen capable of drawing square tiles on
//! a grid. The software draws tiles to the screen with output instructions:
//! every three output instructions specify the x position (distance from the
//! left), y position (distance from the top), and tile id. The tile id is
//! interpreted as follows:
//!
//! * 0 is an empty tile. No game object appears in this tile.
//! * 1 is a wall tile. Walls are indestructible barriers.
//! * 2 is a block tile. Blocks can be broken by the ball.
//! * 3 is a horizontal paddle tile. The paddle is indestructible.
//! * 4 is a ball tile. The ball moves diagonally and bounces off objects.
//!
//! For example, a sequence of output values like `1,2,3,6,5,4` would draw a
//! horizontal paddle tile (1 tile from the left and 2 tiles from the top) and a
//! ball tile (6 tiles from the left and 5 tiles from the top).
//!
//! Start the game. How many block tiles are on the screen when the game exits?

use itertools::Itertools;
use std::{collections::HashMap, convert::TryFrom, fmt};

const PUZZLE_INPUT: &str = include_str!("../inputs/input-13");

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<intcode::Word> for Tile {
    type Error = anyhow::Error;
    fn try_from(w: intcode::Word) -> Result<Self, Self::Error> {
        match w {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::Paddle),
            4 => Ok(Tile::Ball),
            _ => Err(anyhow::anyhow!("Unknown tile type")),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tile = match self {
            Tile::Empty => " ",
            Tile::Wall => "█",
            Tile::Block => "░",
            Tile::Paddle => "=",
            Tile::Ball => "·",
        };
        f.write_str(tile)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Position2D {
    x: usize,
    y: usize,
}

impl Position2D {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub fn run() -> anyhow::Result<()> {
    let game = intcode::Memory::from_str(PUZZLE_INPUT)?;

    let mut runtime = tokio::runtime::Runtime::new()?;

    let mut exe = intcode::AsyncExecutable::from(game);

    let result: anyhow::Result<_> = runtime.block_on(async move {
        let drain = exe.drain().to_vec();

        exe.execute().await?;
        Ok(drain.await?)
    });

    let data = result?;

    let mut blocks = HashMap::<Position2D, Tile>::new();
    blocks.extend(data.into_iter().chunks(3).into_iter().map(|mut c| {
        let position = Position2D {
            x: c.next().unwrap() as usize,
            y: c.next().unwrap() as usize,
        };
        let tile = Tile::try_from(c.next().unwrap()).unwrap();
        (position, tile)
    }));

    let count_blocks = blocks.values().filter(|&&t| t == Tile::Block).count();
    let tiles_by_type = blocks.iter().map(|(k, v)| (v, k)).into_group_map();
    let tiles_by_type: HashMap<_, _> = tiles_by_type
        .into_iter()
        .map(|(k, v)| (k, v.len()))
        .collect();

    println!("Blocks in the output: {}", count_blocks);
    println!("Blocks in the output: {:?}", tiles_by_type);

    Ok(())
}
