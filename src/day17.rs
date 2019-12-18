//! Day 17: Set and Forget ---
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

use super::{GridPosition, Orientation, Turn};
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
    fn memory_weight(self) -> usize {
        match self {
            Self::Forward(steps) if steps > 10 => 3,
            _ => 2,
        }
    }
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
struct Grid<T> {
    elements: Vec<T>,
    columns: usize,
    rows: usize,
}

impl<T> Grid<T> {
    fn max(&self) -> GridPosition {
        GridPosition::new(self.rows - 1, self.columns - 1)
    }

    fn idx(&self, position: GridPosition) -> Option<usize> {
        let p = position.limit(self.max())?;
        Some(p.idx(self.columns))
    }

    fn get(&self, position: GridPosition) -> Option<&T> {
        let idx = self.idx(position)?;
        Some(&self.elements[idx])
    }

    fn get_neighbor(&self, position: GridPosition, direction: Orientation) -> Option<&T> {
        let neighbor = position.neighbor(direction)?;
        let idx = self.idx(neighbor)?;
        Some(&self.elements[idx])
    }

    fn set(&mut self, position: GridPosition, element: T) -> Option<T> {
        let idx = self.idx(position)?;
        Some(std::mem::replace(&mut self.elements[idx], element))
    }

    fn enumerate(&self) -> GridIterator<T> {
        GridIterator::new(self)
    }
}

impl<T> std::str::FromStr for Grid<T>
where
    T: std::str::FromStr,
{
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = Vec::new();
        let mut rows = 0;
        let mut columns = 0;
        for l in s.lines() {
            let mut cols = 0;
            for ch in l.chars() {
                let mut buf = [0_u8; 4];
                let e = ch.encode_utf8(&mut buf).parse()?;
                elements.push(e);
                cols += 1;
            }
            if columns == 0 {
                columns = cols;
            } else {
                debug_assert_eq!(cols, columns);
            }
            rows += 1;
        }

        debug_assert_eq!(elements.len(), rows * columns);

        Ok(Grid {
            elements,
            columns,
            rows,
        })
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in self.elements.chunks_exact(self.columns as usize) {
            for element in line {
                write!(f, "{}", element)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    pos: GridPosition,
    idx: usize,
}

impl<'a, T> GridIterator<'a, T> {
    const fn new(grid: &'a Grid<T>) -> Self {
        Self {
            grid,
            pos: GridPosition::ORIGIN,
            idx: 0,
        }
    }
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = (GridPosition, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.grid.elements.get(self.idx)?;
        let pos = self.pos;
        self.idx += 1;
        self.pos.col += 1;
        if self.pos.col >= self.grid.columns {
            self.pos.col = 0;
            self.pos.row += 1;
        }
        debug_assert_eq!(self.pos.idx(self.grid.columns), self.idx);
        Some((pos, item))
    }
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

    let dust_cleaned = runtime.block_on(robot.clean())?;

    println!("Dust cleaned:\n{:?}", dust_cleaned);

    Ok(())
}
