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
//!
//! ## Part Two
//!
//! The game didn't run because you didn't put in any quarters. Unfortunately,
//! you did not bring any quarters. Memory address 0 represents the number of
//! quarters that have been inserted; set it to 2 to play for free.
//!
//! The arcade cabinet has a joystick that can move left and right. The software
//! reads the position of the joystick with input instructions:
//!
//! * If the joystick is in the neutral position, provide 0.
//! * If the joystick is tilted to the left, provide -1.
//! * If the joystick is tilted to the right, provide 1.
//!
//! The arcade cabinet also has a segment display capable of showing a single
//! number that represents the player's current score. When three output
//! instructions specify X=-1, Y=0, the third output instruction is not a tile;
//! the value instead specifies the new score to show in the segment display.
//! For example, a sequence of output values like `-1,0,12345` would show 12345
//! as the player's current score.
//!
//! Beat the game by breaking all the blocks. What is your score after the last
//! block is broken?

use super::Position2D;
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, convert::TryFrom, fmt};
use termion::{clear, color, cursor, style};
use tokio::sync::{mpsc, watch};

const PUZZLE_INPUT: &str = include_str!("../inputs/input-13");

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Bounce {
    Bouncy,
    Air,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn bounce(self) -> Bounce {
        match self {
            Tile::Empty | Tile::Ball => Bounce::Air,
            _ => Bounce::Bouncy,
        }
    }
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
        match self {
            Tile::Empty => write!(f, " "),
            Tile::Wall => write!(
                f,
                "{}{}█{}",
                color::Fg(color::White),
                color::Bg(color::White),
                style::Reset
            ),
            Tile::Block => write!(f, "{}░{}", color::Fg(color::Blue), style::Reset),
            Tile::Paddle => write!(f, "{}={}", color::Fg(color::LightYellow), style::Reset),
            Tile::Ball => write!(
                f,
                "{}{}·{}",
                color::Fg(color::Magenta),
                style::Bold,
                style::Reset
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JoystickPosition {
    Left = -1,
    Neutral = 0,
    Right = 1,
}

impl From<JoystickPosition> for intcode::Word {
    fn from(p: JoystickPosition) -> Self {
        match p {
            JoystickPosition::Left => -1,
            JoystickPosition::Neutral => 0,
            JoystickPosition::Right => 1,
        }
    }
}

impl fmt::Display for JoystickPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JoystickPosition::Left => write!(f, "{}Left{}", color::Fg(color::Yellow), style::Reset),
            JoystickPosition::Neutral => f.write_str("Neutral"),
            JoystickPosition::Right => write!(f, "{}Right{}", color::Fg(color::Cyan), style::Reset),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BallVert {
    Up,
    Down,
}

impl BallVert {
    fn offset(self) -> intcode::Word {
        match self {
            BallVert::Up => -1,
            BallVert::Down => 1,
        }
    }
}

impl fmt::Display for BallVert {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BallVert::Up => write!(f, "{}Up{}", color::Fg(color::Green), style::Reset),
            BallVert::Down => write!(f, "{}Down{}", color::Fg(color::Red), style::Reset),
        }
    }
}

impl std::ops::Neg for BallVert {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            BallVert::Up => BallVert::Down,
            BallVert::Down => BallVert::Up,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BallDirection {
    Left,
    Right,
}

impl BallDirection {
    fn offset(self) -> intcode::Word {
        match self {
            BallDirection::Left => -1,
            BallDirection::Right => 1,
        }
    }
}

impl fmt::Display for BallDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BallDirection::Left => write!(f, "{}Left{}", color::Fg(color::Yellow), style::Reset),
            BallDirection::Right => write!(f, "{}Right{}", color::Fg(color::Cyan), style::Reset),
        }
    }
}

impl std::ops::Neg for BallDirection {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            BallDirection::Left => BallDirection::Right,
            BallDirection::Right => BallDirection::Left,
        }
    }
}

struct TargetIterator {
    field: Field,
}

impl Iterator for TargetIterator {
    type Item = intcode::Word;
    fn next(&mut self) -> Option<Self::Item> {
        self.field.step_until_ball_height_is_zero()
    }
}

#[derive(Clone, Debug)]
struct Field {
    tiles: Vec<Tile>,
    width: intcode::Word,
    score: intcode::Word,
    ball: Position2D,
    paddle: Position2D,
    ball_dir: BallDirection,
    ball_vert: BallVert,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Score: {}{}{}{}",
            color::Fg(color::LightWhite),
            style::Bold,
            self.score,
            style::Reset
        )?;
        for line in self.tiles.chunks_exact(self.width as usize + 1) {
            for tile in line {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        writeln!(
            f,
            "{}Remaining blocks: {}{}{}{}",
            clear::CurrentLine,
            color::Fg(color::LightWhite),
            style::Bold,
            self.blocks_remaining(),
            style::Reset
        )?;
        writeln!(
            f,
            "{}Ball: {} {} {}, Paddle: {}",
            clear::CurrentLine,
            self.ball_dir,
            self.ball_vert,
            self.ball,
            self.paddle
        )?;
        Ok(())
    }
}

impl Field {
    fn blocks_remaining(&self) -> usize {
        self.tiles.iter().filter(|&&t| t == Tile::Block).count()
    }

    #[inline]
    fn idx(&self, pos: Position2D) -> usize {
        let idx = (pos.x + (self.width + 1) * pos.y) as usize;
        assert!(idx < self.tiles.len());
        idx
    }

    fn move_paddle_to(&mut self, x: intcode::Word) {
        let idx = self.idx(self.paddle);
        self.tiles[idx] = Tile::Empty;
        self.paddle = Position2D::new(x, self.paddle.y);
        let new_idx = self.idx(self.paddle);
        self.tiles[new_idx] = Tile::Paddle;
    }

    fn move_ball_to(&mut self, pos: Position2D) {
        let idx = self.idx(self.ball);
        self.tiles[idx] = Tile::Empty;
        self.ball = pos;
        let new_idx = self.idx(self.ball);
        self.tiles[new_idx] = Tile::Ball;
    }

    fn step(&mut self) {
        log::trace!(
            "Ball at {:?}, moving {:?} {:?}",
            self.ball,
            self.ball_vert,
            self.ball_dir
        );

        let next_pos = Position2D::new(
            self.ball.x + self.ball_dir.offset(),
            self.ball.y + self.ball_vert.offset(),
        );
        let above_pos = Position2D::new(self.ball.x, self.ball.y + self.ball_vert.offset());
        let side_pos = Position2D::new(self.ball.x + self.ball_dir.offset(), self.ball.y);

        let next_tile = self.get_tile(next_pos);
        let above_tile = self.get_tile(above_pos);
        let side_tile = self.get_tile(side_pos);

        log::trace!("Tiles: {:?} {:?} {:?}", next_tile, above_tile, side_tile);

        match (next_tile.bounce(), above_tile.bounce(), side_tile.bounce()) {
            (Bounce::Bouncy, Bounce::Air, Bounce::Air) => {
                log::trace!("Corner bounce");
                if next_tile == Tile::Block {
                    self.set_tile(next_pos, Tile::Empty);
                }
                self.ball_vert = -self.ball_vert;
                self.ball_dir = -self.ball_dir;
            }
            (_, _, Bounce::Bouncy) => {
                log::trace!("Side bounce");
                if side_tile == Tile::Block {
                    self.set_tile(side_pos, Tile::Empty);
                }
                self.ball_dir = -self.ball_dir;
            }
            (_, Bounce::Bouncy, _) => {
                log::trace!("Vertical bounce");
                if above_tile == Tile::Block {
                    self.set_tile(above_pos, Tile::Empty);
                }
                self.ball_vert = -self.ball_vert;
            }
            _ => {
                log::trace!("No bounce");
                self.move_ball_to(next_pos);
            }
        }
        assert!(self.ball.x > 0 && self.ball.x < self.width);
        assert!(self.paddle.x > 0 && self.paddle.x < self.width);
    }

    fn paddle_target_iterator(self) -> TargetIterator {
        TargetIterator { field: self }
    }

    fn step_until_ball_height_is_zero(&mut self) -> Option<intcode::Word> {
        let mut stopable = false;
        loop {
            self.step();
            if self.blocks_remaining() == 0 {
                return None;
            }
            if self.ball_height() == 0 {
                self.move_paddle_to(self.ball.x);
                if stopable {
                    return Some(self.ball.x);
                }
            } else {
                stopable = true;
            }
            //println!("{}{}", ""/*termion::clear::All*/, copy);
        }
    }

    fn ball_height(&self) -> intcode::Word {
        self.paddle.y - self.ball.y - 1
    }

    fn get_tile(&self, pos: Position2D) -> Tile {
        let idx = pos.x + (self.width + 1) * pos.y;
        self.tiles[idx as usize]
    }

    fn set_tile(&mut self, pos: Position2D, tile: Tile) {
        match tile {
            Tile::Ball => {
                self.ball_dir = match self.ball.x.cmp(&pos.x) {
                    Ordering::Less => BallDirection::Right,
                    Ordering::Equal => self.ball_dir,
                    Ordering::Greater => BallDirection::Left,
                };

                self.ball_vert = match self.ball.y.cmp(&pos.y) {
                    Ordering::Less => BallVert::Down,
                    Ordering::Equal => self.ball_vert,
                    Ordering::Greater => BallVert::Up,
                };
                self.ball = pos;
            }
            Tile::Paddle => self.paddle = pos,
            _ => {}
        }

        let idx = pos.x + (self.width + 1) * pos.y;
        self.tiles[idx as usize] = tile;
    }
}

async fn receive_next(
    display: &mut mpsc::Receiver<intcode::Word>,
) -> Option<(intcode::Word, intcode::Word, intcode::Word)> {
    let x = if let Some(x) = display.recv().await {
        x
    } else {
        return None;
    };

    let y = if let Some(y) = display.recv().await {
        y
    } else {
        return None;
    };

    let t = if let Some(t) = display.recv().await {
        t
    } else {
        return None;
    };

    Some((x, y, t))
}

async fn construct_field(display: &mut mpsc::Receiver<intcode::Word>) -> anyhow::Result<Field> {
    let mut tiles = Vec::new();
    let mut max_x = 0;
    let mut ball = None;
    let mut paddle = None;
    while let Some((x, y, t)) = receive_next(&mut *display).await {
        if x == -1 {
            log::info!("Game initialized");
            return Ok(Field {
                tiles,
                width: max_x,
                score: t,
                ball: ball.expect("found ball"),
                paddle: paddle.expect("found paddle"),
                ball_dir: BallDirection::Right,
                ball_vert: BallVert::Down,
            });
        } else {
            max_x = max_x.max(x);
            let tile = Tile::try_from(t)?;

            let pos = Position2D::new(x, y);

            match tile {
                Tile::Ball => ball = Some(pos),
                Tile::Paddle => paddle = Some(pos),
                _ => {}
            }

            tiles.push(tile);
        }
    }

    Err(anyhow::anyhow!("Missing tile data"))
}

struct TargetDisplay(Option<intcode::Word>);

impl fmt::Display for TargetDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(t) = self.0 {
            write!(
                f,
                "{}{}{}{}^{}{}",
                cursor::Save,
                cursor::Up(4),
                cursor::Right(t as u16),
                color::Fg(color::Red),
                style::Reset,
                cursor::Restore
            )?;
        }
        Ok(())
    }
}

async fn run_game(mut game: intcode::Memory) -> anyhow::Result<intcode::Word> {
    // Insert quarter
    game.write_arbitrary(intcode::Address::new(0), 2);

    // Cheats!
    // Location for points in memory
    // game.write_arbitrary(intcode::Address::new(386), 1000000);
    // Location for blocks remaining in memory
    // game.write_arbitrary(intcode::Address::new(205), 0);

    let exe = intcode::AsyncExecutable::from(game);
    let joystick = watch::channel(intcode::Word::from(JoystickPosition::Neutral));
    let mut display = mpsc::channel(1);

    let mut exe = exe.watch_inputs_from(joystick.1);
    exe.pipe_outputs_to(display.0);

    let join = tokio::spawn(exe.execute());

    let mut field = construct_field(&mut display.1).await?;
    let mut targets = field.clone().paddle_target_iterator();
    let mut target = targets.next();
    let mut last = Tile::Ball;

    log::info!("First target {:?}", target);
    println!("{}{}{}", clear::All, cursor::Goto(1, 1), field);

    loop {
        let joystick_pos = if let Some(t) = target {
            match t.cmp(&field.paddle.x) {
                Ordering::Less => JoystickPosition::Left,
                Ordering::Equal => JoystickPosition::Neutral,
                Ordering::Greater => JoystickPosition::Right,
            }
        } else {
            JoystickPosition::Neutral
        };

        if joystick_pos != JoystickPosition::Neutral {
            log::debug!("Moving {:?} toward target {:?}", joystick_pos, target);
        }

        if joystick.0.broadcast(joystick_pos.into()).is_err() {
            break;
        }

        let value = format!(
            "{}\n{}{}Joystick lean: {}",
            field,
            TargetDisplay(target),
            clear::CurrentLine,
            joystick_pos
        );
        println!("{}{}", cursor::Goto(1, 1), value);
        if last == Tile::Ball {
            tokio::time::delay_for(std::time::Duration::from_millis(33)).await;
        }

        let (x, y, t) = if let Some((x, y, t)) = receive_next(&mut display.1).await {
            (x, y, t)
        } else {
            break;
        };

        if x == -1 {
            field.score = t;
            log::info!("Score updated: {}", field.score);
        } else {
            let pos = Position2D::new(x, y);

            let tile = Tile::try_from(t)?;
            last = tile;

            log::trace!("Tile: {:?} at {:?}", tile, pos);
            field.set_tile(pos, tile);

            if tile == Tile::Ball && field.ball_height() == 0 {
                target = targets.next();
                log::info!("Bounce! Next target: {:?}", target);
            }
        }
    }

    join.await??;

    Ok(field.score)
}

pub fn run() -> anyhow::Result<()> {
    let game: intcode::Memory = PUZZLE_INPUT.parse()?;

    let mut runtime = tokio::runtime::Runtime::new()?;

    let mut exe = intcode::AsyncExecutable::from(game.clone());

    let result: anyhow::Result<_> = runtime.block_on(async move {
        let drain = exe.drain().into_vec();

        exe.execute().await?;
        Ok(drain.await?)
    });

    let data = result?;

    let mut blocks = HashMap::<Position2D, Tile>::new();
    blocks.extend(data.into_iter().chunks(3).into_iter().map(|mut c| {
        let position = Position2D::new(c.next().unwrap(), c.next().unwrap());
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

    let score = runtime.block_on(run_game(game))?;

    println!("Final score: {}", score);

    Ok(())
}
