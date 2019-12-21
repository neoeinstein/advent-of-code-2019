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
//!
//! ## Part Two
//!
//! Strangely, the exit isn't open when you reach it. Then, you remember: the
//! ancient Plutonians were famous for building recursive spaces.
//!
//! The marked connections in the maze aren't portals: they physically connect
//! to a larger or smaller copy of the maze. Specifically, the labeled tiles
//! around the inside edge actually connect to a smaller copy of the same maze,
//! and the smaller copy's inner labeled tiles connect to yet a smaller copy,
//! and so on.
//!
//! When you enter the maze, you are at the outermost level; when at the
//! outermost level, only the outer labels AA and ZZ function (as the start and
//! end, respectively); all other outer labeled tiles are effectively walls. At
//! any other level, AA and ZZ count as walls, but the other outer labeled tiles
//! bring you one level outward.
//!
//! Your goal is to find a path through the maze that brings you back to ZZ at
//! the outermost level of the maze.
//!
//! In the first example above, the shortest path is now the loop around the
//! right side. If the starting level is 0, then taking the previously-shortest
//! path would pass through BC (to level 1), DE (to level 2), and FG (back to
//! level 1). Because this is not the outermost level, ZZ is a wall, and the
//! only option is to go back around to BC, which would only send you even
//! deeper into the recursive maze.
//!
//! In the second example above, there is no path that brings you to ZZ at the
//! outermost level.
//!
//! Here is a more interesting example:
//!
//! ```text
//!              Z L X W       C                 
//!              Z P Q B       K                 
//!   ###########.#.#.#.#######.###############  
//!   #...#.......#.#.......#.#.......#.#.#...#  
//!   ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
//!   #.#...#.#.#...#.#.#...#...#...#.#.......#  
//!   #.###.#######.###.###.#.###.###.#.#######  
//!   #...#.......#.#...#...#.............#...#  
//!   #.#########.#######.#.#######.#######.###  
//!   #...#.#    F       R I       Z    #.#.#.#  
//!   #.###.#    D       E C       H    #.#.#.#  
//!   #.#...#                           #...#.#  
//!   #.###.#                           #.###.#  
//!   #.#....OA                       WB..#.#..ZH
//!   #.###.#                           #.#.#.#  
//! CJ......#                           #.....#  
//!   #######                           #######  
//!   #.#....CK                         #......IC
//!   #.###.#                           #.###.#  
//!   #.....#                           #...#.#  
//!   ###.###                           #.#.#.#  
//! XF....#.#                         RF..#.#.#  
//!   #####.#                           #######  
//!   #......CJ                       NM..#...#  
//!   ###.#.#                           #.###.#  
//! RE....#.#                           #......RF
//!   ###.###        X   X       L      #.#.#.#  
//!   #.....#        F   Q       P      #.#.#.#  
//!   ###.###########.###.#######.#########.###  
//!   #.....#...#.....#.......#...#.....#.#...#  
//!   #####.#.###.#######.#######.###.###.#.#.#  
//!   #.......#.......#.#.#.#.#...#...#...#.#.#  
//!   #####.###.#####.#.#.#.#.###.###.#.###.###  
//!   #.......#.....#.#...#...............#...#  
//!   #############.#.#.###.###################  
//!                A O F   N                     
//!                A A D   M                     
//! ```
//!
//! One shortest path through the maze is the following:
//!
//! * Walk from AA to XF (16 steps)
//! * Recurse into level 1 through XF (1 step)
//! * Walk from XF to CK (10 steps)
//! * Recurse into level 2 through CK (1 step)
//! * Walk from CK to ZH (14 steps)
//! * Recurse into level 3 through ZH (1 step)
//! * Walk from ZH to WB (10 steps)
//! * Recurse into level 4 through WB (1 step)
//! * Walk from WB to IC (10 steps)
//! * Recurse into level 5 through IC (1 step)
//! * Walk from IC to RF (10 steps)
//! * Recurse into level 6 through RF (1 step)
//! * Walk from RF to NM (8 steps)
//! * Recurse into level 7 through NM (1 step)
//! * Walk from NM to LP (12 steps)
//! * Recurse into level 8 through LP (1 step)
//! * Walk from LP to FD (24 steps)
//! * Recurse into level 9 through FD (1 step)
//! * Walk from FD to XQ (8 steps)
//! * Recurse into level 10 through XQ (1 step)
//! * Walk from XQ to WB (4 steps)
//! * Return to level 9 through WB (1 step)
//! * Walk from WB to ZH (10 steps)
//! * Return to level 8 through ZH (1 step)
//! * Walk from ZH to CK (14 steps)
//! * Return to level 7 through CK (1 step)
//! * Walk from CK to XF (10 steps)
//! * Return to level 6 through XF (1 step)
//! * Walk from XF to OA (14 steps)
//! * Return to level 5 through OA (1 step)
//! * Walk from OA to CJ (8 steps)
//! * Return to level 4 through CJ (1 step)
//! * Walk from CJ to RE (8 steps)
//! * Return to level 3 through RE (1 step)
//! * Walk from RE to IC (4 steps)
//! * Recurse into level 4 through IC (1 step)
//! * Walk from IC to RF (10 steps)
//! * Recurse into level 5 through RF (1 step)
//! * Walk from RF to NM (8 steps)
//! * Recurse into level 6 through NM (1 step)
//! * Walk from NM to LP (12 steps)
//! * Recurse into level 7 through LP (1 step)
//! * Walk from LP to FD (24 steps)
//! * Recurse into level 8 through FD (1 step)
//! * Walk from FD to XQ (8 steps)
//! * Recurse into level 9 through XQ (1 step)
//! * Walk from XQ to WB (4 steps)
//! * Return to level 8 through WB (1 step)
//! * Walk from WB to ZH (10 steps)
//! * Return to level 7 through ZH (1 step)
//! * Walk from ZH to CK (14 steps)
//! * Return to level 6 through CK (1 step)
//! * Walk from CK to XF (10 steps)
//! * Return to level 5 through XF (1 step)
//! * Walk from XF to OA (14 steps)
//! * Return to level 4 through OA (1 step)
//! * Walk from OA to CJ (8 steps)
//! * Return to level 3 through CJ (1 step)
//! * Walk from CJ to RE (8 steps)
//! * Return to level 2 through RE (1 step)
//! * Walk from RE to XQ (14 steps)
//! * Return to level 1 through XQ (1 step)
//! * Walk from XQ to FD (8 steps)
//! * Return to level 0 through FD (1 step)
//! * Walk from FD to ZZ (18 steps)
//!
//! This path takes a total of 396 steps to move from AA at the outermost layer
//! to ZZ at the outermost layer.
//!
//! In your maze, when accounting for recursion, how many steps does it take to
//! get from the open tile marked AA to the open tile marked ZZ, both at the
//! outermost layer?

use super::{Grid, GridPosition, Orientation};
use anyhow::{anyhow, Error, Result};
use petgraph::prelude::*;
use std::{
    cmp,
    collections::{BinaryHeap, HashMap},
    fmt,
};

const PUZZLE_INPUT: &str = include_str!("../inputs/input-20");

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
struct WarpId([u8; 2]);

impl WarpId {
    const START: WarpId = WarpId(*b"AA");
    const FINISH: WarpId = WarpId(*b"ZZ");
}

impl fmt::Display for WarpId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let to_ascii = [self.0[0] & 0x7f, self.0[1] & 0x7f];
        let x = std::str::from_utf8(&to_ascii).unwrap();
        f.write_str(x)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
enum Warp {
    Inner(WarpId),
    Outer(WarpId),
}

impl Warp {
    fn is_inner(self) -> bool {
        match self {
            Self::Inner(_) => true,
            _ => false,
        }
    }

    fn is_outer(self) -> bool {
        match self {
            Self::Outer(_) => true,
            _ => false,
        }
    }

    fn id(self) -> WarpId {
        match self {
            Self::Inner(wid) | Self::Outer(wid) => wid,
        }
    }

    fn compliment(self) -> Self {
        match self {
            Self::Inner(wid) => Self::Outer(wid),
            Self::Outer(wid) => Self::Inner(wid),
        }
    }
}

impl fmt::Display for Warp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Inner(wid) => write!(f, "{}↣", wid),
            Self::Outer(wid) => write!(f, "↣{}", wid),
        }
    }
}

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
                    return Err(anyhow!("Bad element: {}", b));
                }
            }
            _ => return Err(anyhow!("Unknown element: {}", s)),
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

#[derive(Clone, Debug)]
struct DonutMaze {
    graph: UnGraphMap<GridPosition, usize>,
    start: GridPosition,
    finish: GridPosition,
}

impl DonutMaze {
    fn shortest_path(&self) -> Option<usize> {
        let (shortest, _) = petgraph::algo::astar(
            &self.graph,
            self.start,
            |q| q == self.finish,
            |e| *e.weight(),
            |_| 0,
        )?;
        Some(shortest as usize)
    }
}

impl From<Grid<Element>> for DonutMaze {
    fn from(grid: Grid<Element>) -> Self {
        let mut graph = UnGraphMap::new();
        let mut warps: HashMap<WarpId, Vec<GridPosition>> = HashMap::new();
        let mut start = None;
        let mut finish = None;

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
                        if warp_id == WarpId::START {
                            start = Some(pos);
                        } else if warp_id == WarpId::FINISH {
                            finish = Some(pos);
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
        let finish = finish.expect("end missing");

        compress_graph(&mut graph, |c| {
            if let Some(elem) = grid.get(c).copied() {
                !elem.is_passable()
            } else {
                false
            }
        });

        log::debug!("Graph compressed:\n{}", petgraph::dot::Dot::new(&graph));

        Self {
            graph,
            start,
            finish,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct GridWithWarp {
    position: GridPosition,
    warp: Option<Warp>,
}

impl GridWithWarp {
    fn new(position: GridPosition) -> Self {
        Self {
            position,
            warp: None,
        }
    }

    fn new_with_warp(position: GridPosition, warp: Warp) -> Self {
        Self {
            position,
            warp: Some(warp),
        }
    }
}

impl std::hash::Hash for GridWithWarp {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        if let Some(w) = self.warp {
            std::hash::Hash::hash(&w, h);
        } else {
            std::hash::Hash::hash(&self.position, h);
        }
    }
}

impl PartialEq for GridWithWarp {
    fn eq(&self, o: &Self) -> bool {
        self.cmp(o) == cmp::Ordering::Equal
    }
}

impl Eq for GridWithWarp {}

impl Ord for GridWithWarp {
    fn cmp(&self, o: &Self) -> cmp::Ordering {
        match (self.warp, o.warp) {
            (Some(s), Some(w)) => s.cmp(&w),
            (Some(_), None) => cmp::Ordering::Less,
            (None, Some(_)) => cmp::Ordering::Greater,
            _ => self.position.cmp(&o.position),
        }
    }
}

impl PartialOrd for GridWithWarp {
    fn partial_cmp(&self, o: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(o))
    }
}

impl fmt::Display for GridWithWarp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(w) = self.warp {
            w.fmt(f)
        } else {
            self.position.fmt(f)
        }
    }
}

fn add_edge2_if_neighbor_is_not_wall(
    grid: &Grid<Element>,
    graph: &mut UnGraphMap<GridWithWarp, usize>,
    pos: GridPosition,
    orientation: Orientation,
) {
    if let Some(n_pos) = pos.neighbor(orientation) {
        if let Some(elem) = grid.get(n_pos).copied() {
            if elem.is_passable() {
                graph.add_edge(GridWithWarp::new(pos), GridWithWarp::new(n_pos), 1);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct RecursiveMaze {
    lzero: UnGraphMap<GridWithWarp, usize>,
    lplus: UnGraphMap<GridWithWarp, usize>,
    start: GridPosition,
    finish: GridPosition,
}

#[derive(Clone, Debug)]
struct Queue {
    total_steps: usize,
    level: usize,
    node: GridWithWarp,
    visited: Vec<(usize, GridWithWarp)>,
}

impl PartialEq for Queue {
    fn eq(&self, o: &Self) -> bool {
        self.cmp(o) == cmp::Ordering::Equal
    }
}

impl Eq for Queue {}

impl Ord for Queue {
    fn cmp(&self, o: &Self) -> cmp::Ordering {
        self.total_steps.cmp(&o.total_steps).reverse()
    }
}

impl PartialOrd for Queue {
    fn partial_cmp(&self, o: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(o))
    }
}

impl RecursiveMaze {
    fn level(&self, level: usize) -> &UnGraphMap<GridWithWarp, usize> {
        if level == 0 {
            &self.lzero
        } else {
            &self.lplus
        }
    }

    fn shortest_path(&self) -> Option<usize> {
        let mut queue = BinaryHeap::new();
        let mut visits = 0;

        queue.push(Queue {
            total_steps: 0,
            level: 0,
            node: GridWithWarp::new_with_warp(self.start, Warp::Outer(WarpId::START)),
            visited: Vec::new(),
        });

        'pop: while let Some(next) = queue.pop() {
            visits += 1;
            if visits % 100_000 == 0 {
                log::debug!(
                    "Visited {} of {}; current: steps = {}, level = {}, node = {}, visits = {}",
                    visits,
                    queue.len(),
                    next.total_steps,
                    next.level,
                    next.node,
                    next.visited.len(),
                );
            }
            // Are we there?
            if next.level == 0
                && next
                    .node
                    .warp
                    .map(|w| w.id() == WarpId::FINISH)
                    .unwrap_or(false)
            {
                return Some(next.total_steps);
            }

            let mut next_visited = next.visited.clone();
            next_visited.push((next.level, next.node));
            for i in 1..next_visited.len() / 2 {
                let latest = &next_visited[(next_visited.len() - i)..next_visited.len()];
                let prior = &next_visited[(next_visited.len() - i * 2)..(next_visited.len() - i)];
                let liter = latest.iter().map(|(_, n)| n);
                let piter = prior.iter().map(|(_, n)| n);

                if latest[0].0 != prior[0].0 {
                    continue;
                }

                debug_assert_eq!(latest.len(), prior.len());
                debug_assert!(!latest.is_empty());

                let mut equal = true;
                for (l, p) in liter.zip(piter) {
                    if l != p {
                        equal = false;
                    }
                }

                if equal {
                    let pstr = prior
                        .iter()
                        .map(|(_, n)| n.to_string())
                        .collect::<Vec<_>>()
                        .join(",");
                    let lstr = latest
                        .iter()
                        .map(|(_, n)| n.to_string())
                        .collect::<Vec<_>>()
                        .join(",");
                    log::debug!("Found repeat cycle of len {} ([{}] x [{}])", i, pstr, lstr);
                    continue 'pop;
                }
            }

            let current_level = self.level(next.level);

            for neighbor in current_level.neighbors(next.node) {
                if next.visited.contains(&(next.level, neighbor)) {
                    continue;
                }

                let edge_weight = *current_level.edge_weight(next.node, neighbor).unwrap();
                queue.push(Queue {
                    total_steps: next.total_steps + edge_weight,
                    level: next.level,
                    node: neighbor,
                    visited: next_visited.clone(),
                })
            }

            if let Some(warp) = next.node.warp {
                let next_level = if warp.is_outer() {
                    next.level.checked_sub(1)
                } else {
                    Some(next.level + 1)
                };
                if let Some(lvl) = next_level {
                    let target =
                        GridWithWarp::new_with_warp(GridPosition::ORIGIN, warp.compliment());
                    if !next.visited.contains(&(lvl, target)) {
                        queue.push(Queue {
                            total_steps: next.total_steps + 1,
                            level: lvl,
                            node: target,
                            visited: next_visited,
                        });
                    }
                }
            }
        }

        None
    }
}

impl From<Grid<Element>> for RecursiveMaze {
    fn from(grid: Grid<Element>) -> Self {
        let mut graph = UnGraphMap::new();
        let mut start = None;
        let mut finish = None;

        for (pos, &elem) in grid.enumerate() {
            if elem.is_passable() {
                graph.add_node(GridWithWarp::new(pos));
                add_edge2_if_neighbor_is_not_wall(&grid, &mut graph, pos, Orientation::East);
                add_edge2_if_neighbor_is_not_wall(&grid, &mut graph, pos, Orientation::South);
            } else if let Some(ch) = elem.get_letter() {
                for &o in &[
                    Orientation::East,
                    Orientation::South,
                    Orientation::West,
                    Orientation::North,
                ][..]
                {
                    if let Some(warp_id) = try_build_warp(&grid, ch, pos, o) {
                        if warp_id == WarpId::START {
                            start = Some(pos);
                        } else if warp_id == WarpId::FINISH {
                            finish = Some(pos);
                        }
                        let warp = if is_inner(&grid, pos) {
                            Warp::Inner(warp_id)
                        } else {
                            Warp::Outer(warp_id)
                        };
                        let n = GridWithWarp::new_with_warp(pos, warp);
                        graph.add_node(n);
                        graph.add_edge(n, GridWithWarp::new(pos.neighbor(o).unwrap()), 0);
                        break;
                    }
                }
            }
        }

        log::debug!("Graph uncompressed:\n{}", petgraph::dot::Dot::new(&graph));

        let start = start.expect("start missing");
        let finish = finish.expect("end missing");

        let mut lzero = graph.clone();
        let mut lplus = graph;

        compress_graph(&mut lzero, |c| {
            c.warp
                .map(|w| w.is_inner() || w.id() == WarpId::START || w.id() == WarpId::FINISH)
                .unwrap_or(false)
        });
        compress_graph(&mut lplus, |c| {
            c.warp
                .map(|w| w.id() != WarpId::START && w.id() != WarpId::FINISH)
                .unwrap_or(false)
        });

        log::debug!("L0 compressed:\n{}", petgraph::dot::Dot::new(&lzero));
        log::debug!("L+ compressed:\n{}", petgraph::dot::Dot::new(&lplus));

        Self {
            lzero,
            lplus,
            start,
            finish,
        }
    }
}

fn is_inner(grid: &Grid<Element>, pos: GridPosition) -> bool {
    pos.row > 1 && pos.row < grid.rows() - 2 && pos.col > 1 && pos.col < grid.columns() - 2
}

fn try_build_warp(
    grid: &Grid<Element>,
    wa: char,
    pos: GridPosition,
    orientation: Orientation,
) -> Option<WarpId> {
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
                return Some(WarpId([rp as u8, wa as u8]));
            } else {
                return Some(WarpId([wa as u8, rp as u8]));
            }
        }
    }
    None
}

fn compress_graph<N: petgraph::graphmap::NodeTrait>(
    graph: &mut UnGraphMap<N, usize>,
    mut should_keep: impl FnMut(N) -> bool,
) {
    let mut repeat = true;
    while repeat {
        repeat = false;
        let candidates: Vec<_> = graph.nodes().collect();
        for candidate in candidates {
            let neighbors: Vec<_> = graph.neighbors(candidate).collect();
            if neighbors.is_empty() {
                graph.remove_node(candidate);
                repeat = true;
                continue;
            }

            if should_keep(candidate) {
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
    let m = DonutMaze::from(grid.clone());

    println!("Shortest donut path: {:?}", m.shortest_path());

    let m = RecursiveMaze::from(grid);

    println!("Shortest recursive path: {:?}", m.shortest_path());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{DonutMaze, Element, Grid, RecursiveMaze};
    use anyhow::Result;
    use pretty_assertions::assert_eq;

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
    fn example_1_donut() -> Result<()> {
        crate::init_logging();
        let grid: Grid<Element> = EXAMPLE_1.parse()?;
        let m = DonutMaze::from(grid);

        assert_eq!(m.shortest_path(), Some(23));
        Ok(())
    }

    #[test]
    fn example_1_recursive() -> Result<()> {
        crate::init_logging();
        let grid: Grid<Element> = EXAMPLE_1.parse()?;
        let m = RecursiveMaze::from(grid);

        assert_eq!(m.shortest_path(), Some(26));
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
    fn example_2_donut() -> Result<()> {
        crate::init_logging();
        let grid: Grid<Element> = EXAMPLE_2.parse()?;
        let m = DonutMaze::from(grid);

        assert_eq!(m.shortest_path(), Some(58));
        Ok(())
    }

    #[test]
    fn example_2_recursive() -> Result<()> {
        crate::init_logging();
        let grid: Grid<Element> = EXAMPLE_2.parse()?;
        let m = RecursiveMaze::from(grid);

        assert_eq!(m.shortest_path(), None);
        Ok(())
    }

    const RECUSIVE_EXAMPLE: &str = "
             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";

    #[test]
    fn example_3_donut() -> Result<()> {
        crate::init_logging();
        let grid: Grid<Element> = RECUSIVE_EXAMPLE.parse()?;
        let m = DonutMaze::from(grid);

        assert_eq!(m.shortest_path(), Some(77));
        Ok(())
    }

    #[test]
    fn example_3_recursive() -> Result<()> {
        crate::init_logging();
        let grid: Grid<Element> = RECUSIVE_EXAMPLE.parse()?;
        let m = RecursiveMaze::from(grid);

        assert_eq!(m.shortest_path(), Some(396));
        Ok(())
    }
}
