//! # Day 15: Oxygen System
//!
//! Out here in deep space, many things can go wrong. Fortunately, many of those
//! things have indicator lights. Unfortunately, one of those lights is lit: the
//! oxygen system for part of the ship has failed!
//!
//! According to the readouts, the oxygen system must have failed days ago after
//! a rupture in oxygen tank two; that section of the ship was automatically
//! sealed once oxygen levels went dangerously low. A single remotely-operated
//! repair droid is your only option for fixing the oxygen system.
//!
//! The Elves' care package included an Intcode program (your puzzle input) that
//! you can use to remotely control the repair droid. By running that program,
//! you can direct the repair droid to the oxygen system and fix the problem.
//!
//! The remote control program executes the following steps in a loop forever:
//!
//! * Accept a movement command via an input instruction.
//! * Send the movement command to the repair droid.
//! * Wait for the repair droid to finish the movement operation.
//! * Report on the status of the repair droid via an output instruction.
//!
//! Only four movement commands are understood: north (1), south (2), west (3),
//! and east (4). Any other command is invalid. The movements differ in
//! Orientation, but not in distance: in a long enough east-west hallway, a
//! series of commands like 4,4,4,4,3,3,3,3 would leave the repair droid back
//! where it started.
//!
//! The repair droid can reply with any of the following status codes:
//!
//! * 0: The repair droid hit a wall. Its position has not changed.
//! * 1: The repair droid has moved one step in the requested Orientation.
//! * 2: The repair droid has moved one step in the requested Orientation; its
//!   new position is the location of the oxygen system.
//!
//! You don't know anything about the area around the repair droid, but you can
//! figure it out by watching the status codes.
//!
//! For example, we can draw the area using D for the droid, # for walls, . for
//! locations the droid can traverse, and empty space for unexplored locations.
//! Then, the initial state looks like this:
//!
//! ```text
//! 
//!
//!    D
//!
//!
//! ```
//!
//! To make the droid go north, send it 1. If it replies with 0, you know that
//! location is a wall and that the droid didn't move:
//!
//! ```text
//! 
//!    #
//!    D
//!
//!
//! ```
//!
//! To move east, send 4; a reply of 1 means the movement was successful:
//!
//! ```text
//! 
//!    #
//!    .D
//!
//!
//! ```
//!
//! Then, perhaps attempts to move north (1), south (2), and east (4) are all
//! met with replies of 0:
//!
//! ```text
//! 
//!    ##
//!    .D#
//!     #
//!
//! ```
//!
//! Now, you know the repair droid is in a dead end. Backtrack with 3 (which you
//! already know will get a reply of 1 because you already know that location is
//! open):
//!
//! ```text
//! 
//!    ##
//!    D.#
//!     #
//! ```
//!
//! Then, perhaps west (3) gets a reply of 0, south (2) gets a reply of 1, south
//! again (2) gets a reply of 0, and then west (3) gets a reply of 2:
//!
//! ```text
//! 
//!    ##
//!   #..#
//!   D.#
//!    #
//! ```
//!
//! Now, because of the reply of 2, you know you've found the oxygen system! In
//! this example, it was only 2 moves away from the repair droid's starting
//! position.
//!
//! What is the fewest number of movement commands required to move the repair
//! droid from its starting position to the location of the oxygen system?

use anyhow::Result;
use petgraph::prelude::*;
use std::{convert::TryFrom, fmt};
use termion::color;
use tokio::sync::mpsc::{channel, Receiver, Sender};

const PUZZLE_INPUT: &str = include_str!("../inputs/input-15");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Orientation {
    North,
    South,
    West,
    East,
}

impl Orientation {
    fn left(self) -> Self {
        match self {
            Orientation::North => Orientation::West,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
            Orientation::East => Orientation::North,
        }
    }

    fn turn_left(&mut self) {
        *self = self.left()
    }

    fn right(self) -> Self {
        match self {
            Orientation::North => Orientation::East,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
            Orientation::East => Orientation::South,
        }
    }

    fn turn_right(&mut self) {
        *self = self.right()
    }

    fn reverse(self) -> Self {
        match self {
            Orientation::North => Orientation::South,
            Orientation::South => Orientation::North,
            Orientation::West => Orientation::East,
            Orientation::East => Orientation::West,
        }
    }

    fn turn_around(&mut self) {
        *self = self.reverse()
    }
}

impl std::ops::Neg for Orientation {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.reverse()
    }
}

impl From<Orientation> for intcode::Word {
    fn from(p: Orientation) -> Self {
        match p {
            Orientation::North => 1,
            Orientation::South => 2,
            Orientation::West => 3,
            Orientation::East => 4,
        }
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Orientation::North => write!(
                f,
                "{}North{}",
                color::Fg(color::Red),
                color::Fg(color::Reset)
            ),
            Orientation::South => write!(
                f,
                "{}South{}",
                color::Fg(color::Green),
                color::Fg(color::Reset)
            ),
            Orientation::West => write!(
                f,
                "{}West{}",
                color::Fg(color::Yellow),
                color::Fg(color::Reset)
            ),
            Orientation::East => write!(
                f,
                "{}East{}",
                color::Fg(color::Cyan),
                color::Fg(color::Reset)
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NodeType {
    Wall,
    Empty,
    Oxygen,
}

impl NodeType {
    fn movement(self) -> Movement {
        match self {
            NodeType::Wall => Movement::Blocked,
            NodeType::Empty => Movement::Open,
            NodeType::Oxygen => Movement::Open,
        }
    }
}

impl TryFrom<intcode::Word> for NodeType {
    type Error = anyhow::Error;
    fn try_from(p: intcode::Word) -> Result<Self> {
        match p {
            0 => Ok(NodeType::Wall),
            1 => Ok(NodeType::Empty),
            2 => Ok(NodeType::Oxygen),
            _ => Err(anyhow::anyhow!("Unknown node type: {}", p)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Movement {
    Blocked,
    Open,
}

impl From<NodeType> for Movement {
    fn from(nt: NodeType) -> Self {
        match nt {
            NodeType::Wall => Movement::Blocked,
            NodeType::Empty => Movement::Open,
            NodeType::Oxygen => Movement::Open,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Position2D {
    x: isize,
    y: isize,
}

impl Position2D {
    const ORIGIN: Self = Position2D { x: 0, y: 0 };
}

impl std::ops::Add<Orientation> for Position2D {
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

impl std::ops::AddAssign<Orientation> for Position2D {
    fn add_assign(&mut self, o: Orientation) {
        match o {
            Orientation::North => self.y += 1,
            Orientation::South => self.y -= 1,
            Orientation::East => self.x += 1,
            Orientation::West => self.x -= 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct NodePosition {
    position: Position2D,
    node_type: NodeType,
}

impl std::hash::Hash for NodePosition {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        std::hash::Hash::hash(&self.position, h)
    }
}

impl PartialEq for NodePosition {
    fn eq(&self, other: &Self) -> bool {
        self.position.eq(&other.position)
    }
}

impl Eq for NodePosition {}

impl Ord for NodePosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let xord = self.position.x.cmp(&other.position.x);
        if xord == std::cmp::Ordering::Equal {
            self.position.y.cmp(&other.position.y)
        } else {
            xord
        }
    }
}

impl PartialOrd for NodePosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Default)]
struct RepairDroid {
    map: UnGraphMap<NodePosition, Orientation>,
}

impl RepairDroid {
    fn min_steps_to(&self, goal: NodeType, start: Position2D) -> Option<usize> {
        let start = NodePosition {
            position: start,
            node_type: NodeType::Empty,
        };
        petgraph::algo::astar(&self.map, start, move |f| f.node_type == goal, |_| 1, |_| 0)
            .map(|(k, _path)| k)
    }

    async fn execute(
        &mut self,
        mut commands: Sender<intcode::Word>,
        mut camera: Receiver<intcode::Word>,
    ) -> anyhow::Result<Option<Position2D>> {
        let mut visited = std::collections::HashSet::new();
        let mut position = Position2D::ORIGIN;
        let mut current_node = self.map.add_node(NodePosition {
            position,
            node_type: NodeType::Empty,
        });
        let mut oriented = Orientation::North;
        let mut movement_stack: Vec<(Orientation, NodePosition)> = Vec::new();
        let mut oxygen = None;
        let mut backtrack = false;
        visited.insert(position);
        loop {
            if backtrack {
                if let Some((orient, prev)) = movement_stack.pop() {
                    log::debug!("Backtracking");
                    if commands.send(intcode::Word::from(-orient)).await.is_err() {
                        log::info!("droid stopped responding; halting");
                        break;
                    }

                    let move_result = if let Some(p) = camera.recv().await {
                        NodeType::try_from(p)?
                    } else {
                        log::warn!("connection closed before receiving movement result; halting");
                        break;
                    };

                    assert_eq!(move_result.movement(), Movement::Open);

                    backtrack = false;
                    oriented = orient;
                    current_node = prev;
                    position = prev.position;
                } else {
                    log::info!("Finished search; halting");
                    break;
                }
            } else {
                if commands.send(intcode::Word::from(oriented)).await.is_err() {
                    log::info!("droid stopped responding; halting");
                    break;
                }

                let move_result = if let Some(p) = camera.recv().await {
                    NodeType::try_from(p)?
                } else {
                    log::warn!("connection closed before receiving movement result; halting");
                    break;
                };

                let next_pos = position + oriented;
                let node_data = NodePosition {
                    position: next_pos,
                    node_type: move_result,
                };
                let node = self.map.add_node(node_data);
                self.map.add_edge(current_node, node, oriented);
                visited.insert(next_pos);

                if move_result == NodeType::Oxygen {
                    oxygen = Some(next_pos);
                }

                if move_result.movement() == Movement::Open {
                    movement_stack.push((oriented, current_node));
                    position += oriented;
                    oriented = Orientation::North;
                    current_node = node;
                }
            }

            while visited.contains(&(position + oriented)) {
                if oriented == Orientation::North.left() {
                    backtrack = true;
                    break;
                } else {
                    oriented.turn_right();
                }
            }
        }

        Ok(oxygen)
    }

    pub async fn run_droid(
        &mut self,
        droid: intcode::Memory,
    ) -> anyhow::Result<Option<Position2D>> {
        let mut exe = intcode::AsyncExecutable::from(droid);
        let commands = channel(1);
        let camera = channel(1);

        exe.pipe_inputs_from(commands.1);
        exe.pipe_outputs_to(camera.0);

        tokio::spawn(exe.execute());

        let oxygen = self.execute(commands.0, camera.1).await?;

        Ok(oxygen)
    }
}

pub fn run() -> Result<()> {
    let droid: intcode::Memory = PUZZLE_INPUT.parse()?;

    let mut runtime = tokio::runtime::Runtime::new()?;

    let mut robot = RepairDroid::default();
    runtime
        .block_on(robot.run_droid(droid))?
        .expect("oxygen to be found");
    let steps_to_oxygen = robot
        .min_steps_to(NodeType::Oxygen, Position2D::ORIGIN)
        .expect("path between origin and oxygen");
    println!("Minimum steps to the oxygen: {}", steps_to_oxygen);

    Ok(())
}
