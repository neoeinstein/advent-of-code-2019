//! # Day 11: Space Police
//!
//! On the way to Jupiter, you're pulled over by the Space Police.
//!
//! "Attention, unmarked spacecraft! You are in violation of Space Law! All
//! spacecraft must have a clearly visible registration identifier! You have 24
//! hours to comply or be sent to Space Jail!"
//!
//! Not wanting to be sent to Space Jail, you radio back to the Elves on Earth
//! for help. Although it takes almost three hours for their reply signal to
//! reach you, they send instructions for how to power up the emergency hull
//! painting robot and even provide a small Intcode program (your puzzle input)
//! that will cause it to paint your ship appropriately.
//!
//! There's just one problem: you don't have an emergency hull painting robot.
//!
//! You'll need to build a new emergency hull painting robot. The robot needs to
//! be able to move around on the grid of square panels on the side of your
//! ship, detect the color of its current panel, and paint its current panel
//! black or white. (All of the panels are currently black.)
//!
//! The Intcode program will serve as the brain of the robot. The program uses
//! input instructions to access the robot's camera: provide 0 if the robot is
//! over a black panel or 1 if the robot is over a white panel. Then, the
//! program will output two values:
//!
//! * First, it will output a value indicating the color to paint the panel the
//!   robot is over: 0 means to paint the panel black, and 1 means to paint the
//!   panel white.
//! * Second, it will output a value indicating the direction the robot should
//!   turn: 0 means it should turn left 90 degrees, and 1 means it should turn
//!   right 90 degrees.
//!
//! After the robot turns, it should always move forward exactly one panel. The
//! robot starts facing up.
//!
//! The robot will continue running for a while like this and halt when it is
//! finished drawing. Do not restart the Intcode computer inside the robot
//! during this process.
//!
//! For example, suppose the robot is about to start running. Drawing black
//! panels as ., white panels as #, and the robot pointing the direction it is
//! facing (< ^ > v), the initial state and region near the robot looks like
//! this:
//!
//! ```text
//! .....
//! .....
//! ..^..
//! .....
//! .....
//! ```
//!
//! The panel under the robot (not visible here because a ^ is shown instead) is
//! also black, and so any input instructions at this point should be provided
//! 0. Suppose the robot eventually outputs 1 (paint white) and then 0 (turn
//! left). After taking these actions and moving forward one panel, the region
//! now looks like this:
//!
//! ```text
//! .....
//! .....
//! .<#..
//! .....
//! .....
//! ```
//!
//! Input instructions should still be provided 0. Next, the robot might output
//! 0 (paint black) and then 0 (turn left):
//!
//! ```text
//! .....
//! .....
//! ..#..
//! .v...
//! .....
//! ```
//!
//! After more outputs (1,0, 1,0):
//!
//! ```text
//! .....
//! .....
//! ..^..
//! .##..
//! .....
//! ```
//!
//! The robot is now back where it started, but because it is now on a white
//! panel, input instructions should be provided 1. After several more outputs
//! (0,1, 1,0, 1,0), the area looks like this:
//!
//! ```text
//! .....
//! ..<#.
//! ...#.
//! .##..
//! .....
//! ```
//!
//! Before you deploy the robot, you should probably have an estimate of the
//! area it will cover: specifically, you need to know the number of panels it
//! paints at least once, regardless of color. In the example above, the robot
//! painted 6 panels at least once. (It painted its starting panel twice, but
//! that panel is still only counted once; it also never painted the panel it
//! ended on.)
//!
//! Build a new emergency hull painting robot and run the Intcode program on it.
//! How many panels does it paint at least once?
//!
//! ## Part Two
//!
//! You're not sure what it's trying to paint, but it's definitely not a
//! registration identifier. The Space Police are getting impatient.
//!
//! Checking your external ship cameras again, you notice a white panel marked
//! "emergency hull painting robot starting panel". The rest of the panels are
//! still black, but it looks like the robot was expecting to start on a white
//! panel, not a black one.
//!
//! Based on the Space Law Space Brochure that the Space Police attached to one
//! of your windows, a valid registration identifier is always eight capital
//! letters. After starting the robot on a single white panel instead, what
//! registration identifier does it paint on your hull?

use std::{collections::HashMap, convert::TryFrom, fmt, ops};
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub const PUZZLE_INPUT: &str = include_str!("../inputs/input-11");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PanelColor {
    Black,
    White,
}

impl Default for PanelColor {
    fn default() -> Self {
        Self::Black
    }
}

impl fmt::Display for PanelColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            PanelColor::Black => "·",
            PanelColor::White => "█",
        };

        f.write_str(value)
    }
}

impl From<PanelColor> for intcode::Word {
    fn from(c: PanelColor) -> intcode::Word {
        match c {
            PanelColor::Black => 0,
            PanelColor::White => 1,
        }
    }
}

impl TryFrom<intcode::Word> for PanelColor {
    type Error = anyhow::Error;
    fn try_from(w: intcode::Word) -> Result<Self, Self::Error> {
        match w {
            0 => Ok(PanelColor::Black),
            1 => Ok(PanelColor::White),
            _ => Err(anyhow::anyhow!("Invalid paint command")),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Orientation {
    North,
    East,
    South,
    West,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Orientation::North => "^",
            Orientation::East => ">",
            Orientation::South => "v",
            Orientation::West => "<",
        };
        f.write_str(value)
    }
}

#[derive(Clone, Copy, Debug)]
enum Turn {
    Left,
    Right,
}

impl ops::Add<Turn> for Orientation {
    type Output = Orientation;
    fn add(self, r: Turn) -> Self::Output {
        match (self, r) {
            (Orientation::North, Turn::Left) | (Orientation::South, Turn::Right) => {
                Orientation::West
            }
            (Orientation::North, Turn::Right) | (Orientation::South, Turn::Left) => {
                Orientation::East
            }
            (Orientation::East, Turn::Left) | (Orientation::West, Turn::Right) => {
                Orientation::North
            }
            (Orientation::East, Turn::Right) | (Orientation::West, Turn::Left) => {
                Orientation::South
            }
        }
    }
}

impl ops::AddAssign<Turn> for Orientation {
    fn add_assign(&mut self, r: Turn) {
        *self = *self + r;
    }
}

impl TryFrom<intcode::Word> for Turn {
    type Error = anyhow::Error;
    fn try_from(w: intcode::Word) -> Result<Self, Self::Error> {
        match w {
            0 => Ok(Turn::Left),
            1 => Ok(Turn::Right),
            _ => Err(anyhow::anyhow!("Invalid turn command")),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    const ORIGIN: Self = Position { x: 0, y: 0 };
}

impl ops::Add<Orientation> for Position {
    type Output = Self;
    fn add(self, o: Orientation) -> Self::Output {
        let mut new = self;
        match o {
            Orientation::North => new.y += 1,
            Orientation::South => new.y -= 1,
            Orientation::East => new.x += 1,
            Orientation::West => new.x -= 1,
        }
        new
    }
}

impl ops::AddAssign<Orientation> for Position {
    fn add_assign(&mut self, o: Orientation) {
        match o {
            Orientation::North => self.y += 1,
            Orientation::South => self.y -= 1,
            Orientation::East => self.x += 1,
            Orientation::West => self.x -= 1,
        }
    }
}

#[derive(Debug)]
struct EmergencyHullPaintingRobot {
    current: Position,
    orientation: Orientation,
    visited: HashMap<Position, PanelColor>,
    strokes: usize,
}

impl Default for EmergencyHullPaintingRobot {
    fn default() -> Self {
        Self {
            current: Position::ORIGIN,
            orientation: Orientation::North,
            visited: HashMap::new(),
            strokes: 0,
        }
    }
}

impl EmergencyHullPaintingRobot {
    async fn execute(
        &mut self,
        background: PanelColor,
        mut camera: Sender<intcode::Word>,
        mut commands: Receiver<intcode::Word>,
    ) -> anyhow::Result<()> {
        loop {
            let current = self.visited.entry(self.current).or_insert(background);

            if camera.send(intcode::Word::from(*current)).await.is_err() {
                log::info!("camera no longer listening to input; halting");
                break;
            }

            let paint = if let Some(p) = commands.recv().await {
                PanelColor::try_from(p)?
            } else {
                log::warn!("connection closed before receiving paint command; halting");
                break;
            };

            let turn = if let Some(t) = commands.recv().await {
                Turn::try_from(t)?
            } else {
                log::warn!("connection closed before receiving paint command; halting");
                break;
            };

            *current = paint;
            self.orientation += turn;
            self.current += self.orientation;
            self.strokes += 1;
        }

        Ok(())
    }

    pub async fn run_painter(
        &mut self,
        painter: intcode::Memory,
        background: PanelColor,
    ) -> anyhow::Result<()> {
        let mut exe = intcode::AsyncExecutable::from(painter);
        let camera = channel(1);
        let commands = channel(2);

        exe.pipe_inputs_from(camera.1);
        exe.pipe_outputs_to(commands.0);

        let join = tokio::spawn(exe.execute());

        self.execute(background, camera.0, commands.1).await?;

        join.await??;

        Ok(())
    }
}

fn convert_painted_panels_to_image(
    painted: &HashMap<Position, PanelColor>,
) -> Vec<Vec<PanelColor>> {
    let (min_x, max_x, min_y, max_y) = painted.keys().fold(
        (
            isize::max_value(),
            isize::min_value(),
            isize::max_value(),
            isize::min_value(),
        ),
        |(min_x, max_x, min_y, max_y), p| {
            (
                min_x.min(p.x),
                max_x.max(p.x),
                min_y.min(p.y),
                max_y.max(p.y),
            )
        },
    );

    println!("x: [{}, {}]; y: [{}, {}]", min_x, max_x, min_y, max_y);
    let row_count = (max_y - min_y + 1).abs() as usize;
    let col_count = (max_x - min_x + 1).abs() as usize;
    let mut image = Vec::with_capacity(row_count);
    image.resize(row_count, vec![PanelColor::Black; col_count]);

    for (k, v) in painted.iter().filter(|(_, &v)| v != PanelColor::Black) {
        image[(k.y - max_y).abs() as usize][(k.x - min_x) as usize] = *v;
    }

    image
}

fn print_image(robot: &EmergencyHullPaintingRobot) {
    let image = convert_painted_panels_to_image(&robot.visited);

    println!(
        "Robot painted {} panels after {} strokes",
        robot.visited.len(),
        robot.strokes
    );
    println!("Image:");
    for row in image {
        for p in row {
            print!("{}", p);
        }
        println!();
    }
}

pub fn run() -> anyhow::Result<()> {
    let painter: intcode::Memory = PUZZLE_INPUT.parse()?;

    let mut runtime = tokio::runtime::Runtime::new()?;

    let mut robot = EmergencyHullPaintingRobot::default();
    runtime.block_on(robot.run_painter(painter.clone(), PanelColor::Black))?;
    print_image(&robot);

    let mut robot = EmergencyHullPaintingRobot::default();
    runtime.block_on(robot.run_painter(painter, PanelColor::White))?;
    print_image(&robot);

    Ok(())
}
