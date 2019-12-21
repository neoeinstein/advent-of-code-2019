//! # Day 20: Donut Maze
//!
//! You notice a strange pattern on the surface of Pluto and land nearby to get
//! a closer look. Upon closer inspection, you realize you've come across one of
//! the famous space-warping mazes of the long-lost Pluto civilization!
//!
//! Because there isn't much space on Pluto, the civilization that used to live
//! here thrived by inventing a method for folding spacetime. Although the
//! technology is no longer understood, mazes like this one provide a small
//! glimpse into the daily life of an ancient Pluto citizen.
//!
//! This maze is shaped like a donut. Portals along the inner and outer edge of
//! the donut can instantly teleport you from one side to the other. For
//! example:
//!
//! ```text
//!          A
//!          A
//!   #######.#########
//!   #######.........#
//!   #######.#######.#
//!   #######.#######.#
//!   #######.#######.#
//!   #####  B    ###.#
//! BC...##  C    ###.#
//!   ##.##       ###.#
//!   ##...DE  F  ###.#
//!   #####    G  ###.#
//!   #########.#####.#
//! DE..#######...###.#
//!   #.#########.###.#
//! FG..#########.....#
//!   ###########.#####
//!              Z
//!              Z
//! ```
//!
//! This map of the maze shows solid walls (`#`) and open passages (`.`). Every
//! maze on Pluto has a start (the open tile next to `AA`) and an end (the open
//! tile next to `ZZ`). Mazes on Pluto also have portals; this maze has three
//! pairs of portals: BC, DE, and FG. When on an open tile next to one of these
//! labels, a single step can take you to the other tile with the same label.
//! (You can only walk on `.` tiles; labels and empty space are not
//! traversable.)
//!
//! One path through the maze doesn't require any portals. Starting at `AA`, you
//! could go down 1, right 8, down 12, left 4, and down 1 to reach `ZZ`, a total
//! of 26 steps.
//!
//! However, there is a shorter path: You could walk from `AA` to the inner `BC`
//! portal (4 steps), warp to the outer `BC` portal (1 step), walk to the inner
//! `DE` (6 steps), warp to the outer `DE` (1 step), walk to the outer `FG` (4
//! steps), warp to the inner `FG` (1 step), and finally walk to `ZZ` (6 steps).
//! In total, this is only 23 steps.
//!
//! Here is a larger example:
//!
//! ```text
//!                    A
//!                    A
//!   #################.#############
//!   #.#...#...................#.#.#
//!   #.#.#.###.###.###.#########.#.#
//!   #.#.#.......#...#.....#.#.#...#
//!   #.#########.###.#####.#.#.###.#
//!   #.............#.#.....#.......#
//!   ###.###########.###.#####.#.#.#
//!   #.....#        A   C    #.#.#.#
//!   #######        S   P    #####.#
//!   #.#...#                 #......VT
//!   #.#.#.#                 #.#####
//!   #...#.#               YN....#.#
//!   #.###.#                 #####.#
//! DI....#.#                 #.....#
//!   #####.#                 #.###.#
//! ZZ......#               QG....#..AS
//!   ###.###                 #######
//! JO..#.#.#                 #.....#
//!   #.#.#.#                 ###.#.#
//!   #...#..DI             BU....#..LF
//!   #####.#                 #.#####
//! YN......#               VT..#....QG
//!   #.###.#                 #.###.#
//!   #.#...#                 #.....#
//!   ###.###    J L     J    #.#.###
//!   #.....#    O F     P    #.#...#
//!   #.###.#####.#.#####.#####.###.#
//!   #...#.#.#...#.....#.....#.#...#
//!   #.#####.###.###.#.#.#########.#
//!   #...#.#.....#...#.#.#.#.....#.#
//!   #.###.#####.###.###.#.#.#######
//!   #.#.........#...#.............#
//!   #########.###.###.#############
//!            B   J   C
//!            U   P   P
//! ```
//!
//! Here, `AA` has no direct path to `ZZ`, but it does connect to `AS` and `CP`.
//! By passing through `AS`, `QG`, `BU`, and `JO`, you can reach `ZZ` in 58
//! steps.
//!
//! In your maze, how many steps does it take to get from the open tile marked
//! `AA` to the open tile marked `ZZ`?

use super::{Grid, GridPosition, Orientation};
use anyhow::{anyhow, Error, Result};
use petgraph::prelude::*;
use std::collections::HashMap;

const PUZZLE_INPUT: &str = include_str!("../inputs/input-20");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Element {
    Passable,
    Wall,
    Letter(char),
    Space,
}

impl Element {
    fn is_passable(self) -> bool {
        self == Element::Passable
    }

    fn get_letter(self) -> Option<char> {
        match self {
            Element::Letter(c) => Some(c),
            _ => None,
        }
    }
}

impl std::str::FromStr for Element {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let e = match s {
            " " => Element::Space,
            "." => Element::Passable,
            "#" => Element::Wall,
            x if x.len() == 1 => {
                let b = x.as_bytes()[0] as char;
                if b >= 'A' && b <= 'Z' {
                    Element::Letter(b)
                } else {
                    Err(anyhow!("Bad element: {}", b))?
                }
            }
            _ => Err(anyhow!("Unknown element: {}", s))?,
        };
        Ok(e)
    }
}

fn add_edge_if_neighbor_is_not_wall(
    grid: &Grid<Element>,
    graph: &mut UnGraphMap<GridPosition, usize>,
    pos: GridPosition,
    orientation: Orientation,
) {
    if let Some(n_pos) = pos.neighbor(orientation) {
        if let Some(elem) = grid.get(n_pos).copied() {
            if elem.is_passable() {
                graph.add_edge(pos, n_pos, 1);
            }
        }
    }
}

struct Maze {
    graph: UnGraphMap<GridPosition, usize>,
    start: GridPosition,
    end: GridPosition,
}

fn try_build_warp(
    grid: &Grid<Element>,
    wa: char,
    pos: GridPosition,
    orientation: Orientation,
) -> Option<String> {
    if grid
        .get_neighbor(pos, orientation)
        .map(|&x| x.is_passable())
        .unwrap_or(false)
    {
        if let Some(rp) = grid
            .get_neighbor(pos, orientation.reverse())
            .and_then(|&n| n.get_letter())
        {
            if orientation == Orientation::East || orientation == Orientation::South {
                return Some(format!("{}{}", wa, rp));
            } else {
                return Some(format!("{}{}", rp, wa));
            }
        }
    }
    None
}

impl From<Grid<Element>> for Maze {
    fn from(grid: Grid<Element>) -> Self {
        let mut graph = UnGraphMap::new();
        let mut warps: HashMap<String, Vec<GridPosition>> = HashMap::new();
        let mut start = None;
        let mut end = None;

        for (pos, &elem) in grid.enumerate() {
            if elem.is_passable() {
                graph.add_node(pos);
                add_edge_if_neighbor_is_not_wall(&grid, &mut graph, pos, Orientation::East);
                add_edge_if_neighbor_is_not_wall(&grid, &mut graph, pos, Orientation::South);
            } else if let Some(ch) = elem.get_letter() {
                for &o in &[
                    Orientation::East,
                    Orientation::South,
                    Orientation::West,
                    Orientation::North,
                ][..]
                {
                    if let Some(warp_id) = try_build_warp(&grid, ch, pos, o) {
                        if warp_id == "AA" {
                            start = Some(pos);
                        } else if warp_id == "ZZ" {
                            end = Some(pos);
                        }
                        let known_warps = warps.entry(warp_id).or_default();
                        known_warps.push(pos);
                        graph.add_node(pos);
                        graph.add_edge(pos, pos.neighbor(o).unwrap(), 0);
                        break;
                    }
                }
            }
        }

        for (_, v) in warps {
            for &i in &v {
                for &o in &v {
                    if o != i {
                        graph.add_edge(i, o, 1);
                    }
                }
            }
        }

        log::debug!("Graph uncompressed:\n{}", petgraph::dot::Dot::new(&graph));

        let start = start.expect("start missing");
        let end = end.expect("end missing");

        compress_ungraph(&grid, &mut graph, &[start, end]);

        log::debug!("Graph compressed:\n{}", petgraph::dot::Dot::new(&graph));

        Self { graph, start, end }
    }
}

fn compress_ungraph(
    grid: &Grid<Element>,
    graph: &mut UnGraphMap<GridPosition, usize>,
    protected: &[GridPosition],
) {
    let mut repeat = true;
    while repeat {
        repeat = false;
        let candidates: Vec<_> = graph.nodes().collect();
        for candidate in candidates {
            let elem = grid.get(candidate).copied().expect("to exist");
            // if protected.contains(&candidate) {
            //     continue;
            // }
            if !elem.is_passable() {
                continue;
            }

            let neighbors: Vec<_> = graph.neighbors(candidate).collect();
            if neighbors.is_empty() {
                graph.remove_node(candidate);
                repeat = true;
                continue;
            }
            if neighbors.len() == 1 {
                graph.remove_edge(candidate, neighbors[0]);
                graph.remove_node(candidate);
                repeat = true;
                continue;
            }
            if neighbors.len() == 2 {
                let weight_sum = graph.remove_edge(neighbors[0], candidate).unwrap()
                    + graph.remove_edge(candidate, neighbors[1]).unwrap();
                graph.remove_node(candidate);

                graph.add_edge(neighbors[0], neighbors[1], weight_sum);
                repeat = true;
                continue;
            }
        }
    }
}

pub fn run() -> Result<()> {
    let grid: Grid<Element> = PUZZLE_INPUT.parse()?;
    let m = Maze::from(grid);

    let (shortest, _) =
        petgraph::algo::astar(&m.graph, m.start, |q| q == m.end, |e| *e.weight(), |_| 0)
            .expect("path");
    println!("Shortest path: {}", shortest);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Element, Grid, Maze};
    use anyhow::Result;
    use petgraph::prelude::*;

    const EXAMPLE_1: &str = "
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";

    #[test]
    fn example_1() -> Result<()> {
        crate::init_logging();
        let grid: Grid<Element> = EXAMPLE_1.parse()?;
        let m = Maze::from(grid);

        let (shortest, _) =
            petgraph::algo::astar(&m.graph, m.start, |q| q == m.end, |e| *e.weight(), |_| 0)
                .unwrap();
        assert_eq!(shortest, 23);
        Ok(())
    }

    const EXAMPLE_2: &str = "
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";

    #[test]
    fn example_2() -> Result<()> {
        crate::init_logging();
        let grid: Grid<Element> = EXAMPLE_2.parse()?;
        let m = Maze::from(grid);

        let (shortest, _) =
            petgraph::algo::astar(&m.graph, m.start, |q| q == m.end, |e| *e.weight(), |_| 0)
                .unwrap();
        assert_eq!(shortest, 58);
        Ok(())
    }
}
