//! # Day 17: Set and Forget
//!
//! An early warning system detects an incoming solar flare and automatically
//! activates the ship's electromagnetic shield. Unfortunately, this has cut off
//! the Wi-Fi for many small robots that, unaware of the impending danger, are
//! now trapped on exterior scaffolding on the unsafe side of the shield. To
//! rescue them, you'll have to act quickly!
//!
//! The only tools at your disposal are some wired cameras and a small vacuum
//! robot currently asleep at its charging station. The video quality is poor,
//! but the vacuum robot has a needlessly bright LED that makes it easy to spot
//! no matter where it is.
//!
//! An Intcode program, the Aft Scaffolding Control and Information Interface
//! (ASCII, your puzzle input), provides access to the cameras and the vacuum
//! robot. Currently, because the vacuum robot is asleep, you can only access
//! the cameras.
//!
//! Running the ASCII program on your Intcode computer will provide the current
//! view of the scaffolds. This is output, purely coincidentally, as ASCII code:
//! 35 means #, 46 means ., 10 starts a new line of output below the current
//! one, and so on. (Within a line, characters are drawn left-to-right.)
//!
//! In the camera output, `#` represents a scaffold and `.` represents open
//! space. The vacuum robot is visible as `^`, `v`, `<`, or `>` depending on
//! whether it is facing up, down, left, or right respectively. When drawn like
//! this, the vacuum robot is always on a scaffold; if the vacuum robot ever
//! walks off of a scaffold and begins tumbling through space uncontrollably, it
//! will instead be visible as `X`.
//!
//! In general, the scaffold forms a path, but it sometimes loops back onto
//! itself. For example, suppose you can see the following view from the
//! cameras:
//!
//! ```text
//! ..#..........
//! ..#..........
//! #######...###
//! #.#...#...#.#
//! #############
//! ..#...#...#..
//! ..#####...^..
//! ```
//!
//! Here, the vacuum robot, ^ is facing up and sitting at one end of the
//! scaffold near the bottom-right of the image. The scaffold continues up,
//! loops across itself several times, and ends at the top-left of the image.
//!
//! The first step is to calibrate the cameras by getting the alignment
//! parameters of some well-defined points. Locate all scaffold intersections;
//! for each, its alignment parameter is the distance between its left edge and
//! the left edge of the view multiplied by the distance between its top edge
//! and the top edge of the view. Here, the intersections from the above image
//! are marked O:
//!
//! ```text
//! ..#..........
//! ..#..........
//! ##O####...###
//! #.#...#...#.#
//! ##O###O###O##
//! ..#...#...#..
//! ..#####...^..
//! ```
//!
//! For these intersections:
//!
//! * The top-left intersection is 2 units from the left of the image and 2
//!   units from the top of the image, so its alignment parameter is 2 * 2 = 4.
//! * The bottom-left intersection is 2 units from the left and 4 units from the
//!   top, so its alignment parameter is 2 * 4 = 8.
//! * The bottom-middle intersection is 6 from the left and 4 from the top, so
//!   its alignment parameter is 24.
//! * The bottom-right intersection's alignment parameter is 40.
//!
//! To calibrate the cameras, you need the sum of the alignment parameters. In
//! the above example, this is 76.
//!
//! Run your ASCII program. What is the sum of the alignment parameters for the
//! scaffold intersections?
//!
//! ## Part Two
//!
//! Now for the tricky part: notifying all the other robots about the solar
//! flare. The vacuum robot can do this automatically if it gets into range of a
//! robot. However, you can't see the other robots on the camera, so you need to
//! be thorough instead: you need to make the vacuum robot visit every part of
//! the scaffold at least once.
//!
//! The vacuum robot normally wanders randomly, but there isn't time for that
//! today. Instead, you can override its movement logic with new rules.
//!
//! Force the vacuum robot to wake up by changing the value in your ASCII
//! program at address 0 from 1 to 2. When you do this, you will be
//! automatically prompted for the new movement rules that the vacuum robot
//! should use. The ASCII program will use input instructions to receive them,
//! but they need to be provided as ASCII code; end each line of logic with a
//! single newline, ASCII code 10.
//!
//! First, you will be prompted for the main movement routine. The main routine
//! may only call the movement functions: A, B, or C. Supply the movement
//! functions to use as ASCII text, separating them with commas (,, ASCII code
//! 44), and ending the list with a newline (ASCII code 10). For example, to
//! call A twice, then alternate between B and C three times, provide the string
//! A,A,B,C,B,C,B,C and then a newline.
//!
//! Then, you will be prompted for each movement function. Movement functions
//! may use L to turn left, R to turn right, or a number to move forward that
//! many units. Movement functions may not call other movement functions. Again,
//! separate the actions with commas and end the list with a newline. For
//! example, to move forward 10 units, turn left, move forward 8 units, turn
//! right, and finally move forward 6 units, provide the string 10,L,8,R,6 and
//! then a newline.
//!
//! Finally, you will be asked whether you want to see a continuous video feed;
//! provide either y or n and a newline. Enabling the continuous video feed can
//! help you see what's going on, but it also requires a significant amount of
//! processing power, and may even cause your Intcode computer to overheat.
//!
//! Due to the limited amount of memory in the vacuum robot, the ASCII
//! definitions of the main routine and the movement functions may each contain
//! at most 20 characters, not counting the newline.
//!
//! For example, consider the following camera feed:
//!
//! ```text
//! #######...#####
//! #.....#...#...#
//! #.....#...#...#
//! ......#...#...#
//! ......#...###.#
//! ......#.....#.#
//! ^########...#.#
//! ......#.#...#.#
//! ......#########
//! ........#...#..
//! ....#########..
//! ....#...#......
//! ....#...#......
//! ....#...#......
//! ....#####......
//! ```
//!
//! In order for the vacuum robot to visit every part of the scaffold at least
//! once, one path it could take is:
//!
//! ```text
//! R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2
//! ```
//!
//! Without the memory limit, you could just supply this whole string to
//! function A and have the main routine call A once. However, you'll need to
//! split it into smaller parts.
//!
//! One approach is:
//!
//! * Main routine: `A,B,C,B,A,C` (ASCII input: `65, 44, 66, 44, 67, 44, 66, 44,
//!   65, 44, 67, 10`)
//! * Function A: `R,8,R,8` (ASCII input: `82, 44, 56, 44, 82, 44, 56, 10`)
//! * Function B: `R,4,R,4,R,8` (ASCII input: `82, 44, 52, 44, 82, 44, 52, 44,
//!   82, 44, 56, 10`)
//! * Function C: `L,6,L,2` (ASCII input: `76, 44, 54, 44, 76, 44, 50, 10`)
//!
//! Visually, this would break the desired path into the following parts:
//!
//! ```text
//! A,        B,            C,        B,            A,        C
//! R,8,R,8,  R,4,R,4,R,8,  L,6,L,2,  R,4,R,4,R,8,  R,8,R,8,  L,6,L,2
//!
//! CCCCCCA...BBBBB
//! C.....A...B...B
//! C.....A...B...B
//! ......A...B...B
//! ......A...CCC.B
//! ......A.....C.B
//! ^AAAAAAAA...C.B
//! ......A.A...C.B
//! ......AAAAAA#AB
//! ........A...C..
//! ....BBBB#BBBB..
//! ....B...A......
//! ....B...A......
//! ....B...A......
//! ....BBBBA......
//! ```
//!
//! Of course, the scaffolding outside your ship is much more complex.
//!
//! As the vacuum robot finds other robots and notifies them of the impending
//! solar flare, it also can't help but leave them squeaky clean, collecting any
//! space dust it finds. Once it finishes the programmed set of movements,
//! assuming it hasn't drifted off into space, the cleaning robot will return to
//! its docking station and report the amount of space dust it collected as a
//! large, non-ASCII value in a single output instruction.
//!
//! After visiting every part of the scaffold at least once, how much dust does
//! the vacuum robot report it has collected?

use super::{Grid, GridPosition, Orientation, Turn};
use anyhow::{anyhow, Result};
use num_traits::ToPrimitive;
use tokio::sync::mpsc::{channel, Receiver};

const PUZZLE_INPUT: &str = include_str!("../inputs/input-17");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Element {
    Empty,
    Scaffold,
    Robot(Orientation),
    DeadRobot,
}

impl Element {
    fn is_robot(self) -> bool {
        match self {
            Self::Robot(_) => true,
            Self::DeadRobot => true,
            _ => false,
        }
    }

    fn is_scaffold(self) -> bool {
        match self {
            Self::Robot(_) => true,
            Self::Scaffold => true,
            _ => false,
        }
    }
}

impl Default for Element {
    fn default() -> Self {
        Element::Empty
    }
}

impl std::str::FromStr for Element {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "#" => Ok(Self::Scaffold),
            "^" => Ok(Self::Robot(Orientation::North)),
            ">" => Ok(Self::Robot(Orientation::East)),
            "<" => Ok(Self::Robot(Orientation::West)),
            "v" => Ok(Self::Robot(Orientation::South)),
            "X" => Ok(Self::DeadRobot),
            _ => Err(anyhow!("invalid grid element: {}", s)),
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let rep = match self {
            Self::Empty => ".",
            Self::Scaffold => "#",
            Self::Robot(Orientation::North) => "^",
            Self::Robot(Orientation::East) => ">",
            Self::Robot(Orientation::West) => "<",
            Self::Robot(Orientation::South) => "v",
            Self::DeadRobot => "X",
        };
        f.write_str(rep)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Directive {
    Forward(usize),
    Turn(Turn),
}

impl Directive {
    // fn memory_weight(self) -> usize {
    //     match self {
    //         Self::Forward(steps) if steps > 10 => 3,
    //         _ => 2,
    //     }
    // }
}

impl std::fmt::Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Forward(steps) => std::fmt::Display::fmt(&steps, f),
            Self::Turn(Turn::Left) => f.write_str("L"),
            Self::Turn(Turn::Right) => f.write_str("R"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Robot {
    position: GridPosition,
    orientation: Option<Orientation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Field {
    grid: Grid<Element>,
    robot: Robot,
}

impl Field {
    fn checksum(&self) -> usize {
        self.grid
            .enumerate()
            .map(|(pos, &e)| (pos, e))
            .filter(|&(pos, e)| {
                e.is_scaffold()
                    && self
                        .grid
                        .get_neighbor(pos, Orientation::East)
                        .copied()
                        .map(Element::is_scaffold)
                        .unwrap_or_default()
                    && self
                        .grid
                        .get_neighbor(pos, Orientation::West)
                        .copied()
                        .map(Element::is_scaffold)
                        .unwrap_or_default()
                    && self
                        .grid
                        .get_neighbor(pos, Orientation::North)
                        .copied()
                        .map(Element::is_scaffold)
                        .unwrap_or_default()
                    && self
                        .grid
                        .get_neighbor(pos, Orientation::South)
                        .copied()
                        .map(Element::is_scaffold)
                        .unwrap_or_default()
            })
            .map(|(pos, _)| pos.row * pos.col)
            .sum()
    }

    fn find_path(&self) -> Vec<Directive> {
        let mut path = Vec::new();
        let mut position = self.robot.position;
        let mut orientation = self.robot.orientation.unwrap();
        let mut directive = Directive::Forward(0);
        loop {
            match self.grid.get_neighbor(position, orientation) {
                Some(Element::Scaffold) => {
                    match directive {
                        Directive::Forward(x) => directive = Directive::Forward(x + 1),
                        _ => {
                            if directive != Directive::Forward(0) {
                                path.push(directive);
                            }
                            directive = Directive::Forward(0);
                        }
                    }
                    position += orientation;
                }
                _ => match directive {
                    Directive::Forward(x) => {
                        if directive != Directive::Forward(0) {
                            path.push(Directive::Forward(x + 1));
                        }
                        directive = Directive::Turn(Turn::Left);
                        orientation.turn_left();
                    }
                    Directive::Turn(Turn::Left) => {
                        directive = Directive::Turn(Turn::Right);
                        orientation.turn_around();
                    }
                    Directive::Turn(Turn::Right) => {
                        break;
                    }
                },
            }
        }

        println!("path: {:?}", path);

        path
    }
}

impl std::str::FromStr for Field {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid<Element> = s.parse()?;
        let (pos, o) = grid
            .enumerate()
            .filter(|&(_, e)| e.is_robot())
            .map(|(pos, &e)| match e {
                Element::Robot(o) => (pos, Some(o)),
                _ => (pos, None),
            })
            .next()
            .ok_or_else(|| anyhow!("Robot missing from camera"))?;

        let robot = Robot {
            position: pos,
            orientation: o,
        };

        Ok(Self { grid, robot })
    }
}

#[derive(Debug)]
struct VaccumRobot {
    program: intcode::Memory,
}

impl VaccumRobot {
    fn new(program: intcode::Memory) -> Self {
        Self { program }
    }

    async fn read_field(camera: &mut Receiver<intcode::Word>) -> Result<Field> {
        let mut data = String::new();
        let mut nl = false;
        while let Some(w) = camera.recv().await {
            let ch = w
                .to_u32()
                .and_then(std::char::from_u32)
                .ok_or_else(|| anyhow!("invalid ASCII value: {}", w))?;
            if ch == '\n' {
                if nl {
                    break;
                } else {
                    nl = true
                }
            } else {
                nl = false
            }
            data.push(ch);
        }

        let field: Field = data.parse()?;

        println!("Data received:\n{}", field.grid);

        Ok(field)
    }

    async fn calibrate_cameras(&self) -> Result<Field> {
        let mut camera = channel(1);
        let mut exe = intcode::AsyncExecutable::from(self.program.clone());
        exe.pipe_outputs_to(camera.0);
        let join = tokio::spawn(exe.execute());
        let field = Self::read_field(&mut camera.1).await?;

        join.await??;

        Ok(field)
    }

    async fn clean(&self) -> Result<()> {
        // A,C,A,C,B,B,C,A,C,B

        // A: L,8,R,12,R,12,R,10
        // B: L,10,R,10,L,6
        // C: R,10,R,12,R,10

        let mut command = channel(20);
        let terminal = intcode::TerminalOut::new();
        let mut prog = self.program.clone();
        prog.write_arbitrary(intcode::Address::new(0), 2);
        let mut exe = intcode::AsyncExecutable::from(prog);
        exe.pipe_outputs_to(terminal.tx().clone());
        exe.pipe_inputs_from(command.1);
        let join = tokio::spawn(exe.execute());
        let term_join = tokio::spawn(terminal.write_ascii_output_to_writer(tokio::io::stdout()));
        const INPUTS: [&str; 4] = [
            "A,C,A,C,B,B,C,A,C,B",
            "L,8,R,12,R,12,R,10",
            "L,10,R,10,L,6",
            "R,10,R,12,R,10",
        ];

        for line in &INPUTS[..] {
            log::debug!("sending function: {}", line);
            for b in line.bytes() {
                command.0.send(intcode::Word::from(b)).await?;
            }
            command.0.send(intcode::Word::from(b'\n')).await?;
        }
        command.0.send(intcode::Word::from(b'n')).await?;
        command.0.send(intcode::Word::from(b'\n')).await?;

        log::debug!("waiting for vaccum robot to halt");
        join.await??;
        term_join.await??;
        log::debug!("vaccum robot halted");

        Ok(())
    }
}

pub fn run() -> Result<()> {
    let program: intcode::Memory = PUZZLE_INPUT.parse()?;

    let mut runtime = tokio::runtime::Runtime::new()?;

    let robot = VaccumRobot::new(program);
    let field = runtime.block_on(robot.calibrate_cameras())?;

    println!("Intersection checksum:\n{}", field.checksum());

    field.find_path();

    runtime.block_on(robot.clean())?;

    Ok(())
}
