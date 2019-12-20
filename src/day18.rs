//! # Day 18: Many-Worlds Interpretation
//!
//! As you approach Neptune, a planetary security system detects you and
//! activates a giant tractor beam on Triton! You have no choice but to land.
//!
//! A scan of the local area reveals only one interesting feature: a massive
//! underground vault. You generate a map of the tunnels (your puzzle input).
//! The tunnels are too narrow to move diagonally.
//!
//! Only one entrance (marked `@`) is present among the open passages (marked
//! `.`) and stone walls (`#`), but you also detect an assortment of keys (shown
//! as lowercase letters) and doors (shown as uppercase letters). Keys of a
//! given letter open the door of the same letter: a opens A, b opens B, and so
//! on. You aren't sure which key you need to disable the tractor beam, so
//! you'll need to collect all of them.
//!
//! For example, suppose you have the following map:
//!
//! ```text
//! #########
//! #b.A.@.a#
//! #########
//! ```
//!
//! Starting from the entrance (`@`), you can only access a large door (`A`) and
//! a key (`a`). Moving toward the door doesn't help you, but you can move 2
//! steps to collect the key, unlocking A in the process:
//!
//! ```text
//! #########
//! #b.....@#
//! #########
//! ```
//!
//! Then, you can move 6 steps to collect the only other key, `b`:
//!
//! ```text
//! #########
//! #@......#
//! #########
//! ```
//!
//! So, collecting every key took a total of 8 steps.
//!
//! Here is a larger example:
//!
//! ```text
//! ########################
//! #f.D.E.e.C.b.A.@.a.B.c.#
//! ######################.#
//! #d.....................#
//! ########################
//! ```
//!
//! The only reasonable move is to take key `a` and unlock door `A`:
//!
//! ```text
//! ########################
//! #f.D.E.e.C.b.....@.B.c.#
//! ######################.#
//! #d.....................#
//! ########################
//! ```
//!
//! Then, do the same with key `b`:
//!
//! ```text
//! ########################
//! #f.D.E.e.C.@.........c.#
//! ######################.#
//! #d.....................#
//! ########################
//! ```
//!
//! ...and the same with key `c`:
//!
//! ```text
//! ########################
//! #f.D.E.e.............@.#
//! ######################.#
//! #d.....................#
//! ########################
//! ```
//!
//! Now, you have a choice between keys `d` and `e`. While key `e` is closer,
//! collecting it now would be slower in the long run than collecting key `d`
//! first, so that's the best choice:
//!
//! ```text
//! ########################
//! #f...E.e...............#
//! ######################.#
//! #@.....................#
//! ########################
//! ```
//!
//! Finally, collect key `e` to unlock door `E`, then collect key `f`, taking a
//! grand total of 86 steps.
//!
//! Here are a few more examples:
//!
//! * Shortest path is 132 steps: `b, a, c, d, f, e, g`
//!
//! ```text
//! ########################
//! #...............b.C.D.f#
//! #.######################
//! #.....@.a.B.c.d.A.e.F.g#
//! ########################
//! ```
//!
//! * Shortest paths are 136 steps; one is: `a, f, b, j, g, n, h, d, l, o, e, p,
//!   c, i, k, m`
//!
//! ```text
//! #################
//! #i.G..c...e..H.p#
//! ########.########
//! #j.A..b...f..D.o#
//! ########@########
//! #k.E..a...g..B.n#
//! ########.########
//! #l.F..d...h..C.m#
//! #################
//! ```
//!
//! * Shortest paths are 81 steps; one is: `a, c, f, i, d, g, b, e, h`
//!
//! ```text
//! ########################
//! #@..............ac.GI.b#
//! ###d#e#f################
//! ###A#B#C################
//! ###g#h#i################
//! ########################
//! ```
//!
//! How many steps is the shortest path that collects all of the keys?
//!
//! ## Part Two
//!
//! You arrive at the vault only to discover that there is not one vault, but
//! four - each with its own entrance.
//!
//! On your map, find the area in the middle that looks like this:
//!
//! ```text
//! ...
//! .@.
//! ...
//! ```
//!
//! Update your map to instead use the correct data:
//!
//! ```text
//! @#@
//! ###
//! @#@
//! ```
//!
//! This change will split your map into four separate sections, each with its
//! own entrance:
//!
//! ```text
//! #######       #######
//! #a.#Cd#       #a.#Cd#
//! ##...##       ##@#@##
//! ##.@.##  -->  #######
//! ##...##       ##@#@##
//! #cB#Ab#       #cB#Ab#
//! #######       #######
//! ```
//!
//! Because some of the keys are for doors in other vaults, it would take much
//! too long to collect all of the keys by yourself. Instead, you deploy four
//! remote-controlled robots. Each starts at one of the entrances (@).
//!
//! Your goal is still to collect all of the keys in the fewest steps, but now,
//! each robot has its own position and can move independently. You can only
//! remotely control a single robot at a time. Collecting a key instantly
//! unlocks any corresponding doors, regardless of the vault in which the key or
//! door is found.
//!
//! For example, in the map above, the top-left robot first collects key a,
//! unlocking door A in the bottom-right vault:
//!
//! ```text
//! #######
//! #@.#Cd#
//! ##.#@##
//! #######
//! ##@#@##
//! #cB#.b#
//! #######
//! ```
//!
//! Then, the bottom-right robot collects key b, unlocking door B in the
//! bottom-left vault:
//!
//! ```text
//! #######
//! #@.#Cd#
//! ##.#@##
//! #######
//! ##@#.##
//! #c.#.@#
//! #######
//! ```
//!
//! Then, the bottom-left robot collects key c:
//!
//! ```text
//! #######
//! #@.#.d#
//! ##.#@##
//! #######
//! ##.#.##
//! #@.#.@#
//! #######
//! ```
//!
//! Finally, the top-right robot collects key d:
//!
//! ```text
//! #######
//! #@.#.@#
//! ##.#.##
//! #######
//! ##.#.##
//! #@.#.@#
//! #######
//! ```
//!
//! In this example, it only took 8 steps to collect all of the keys.
//!
//! Sometimes, multiple robots might have keys available, or a robot might have
//! to wait for multiple keys to be collected:
//!
//! ```text
//! ###############
//! #d.ABC.#.....a#
//! ######@#@######
//! ###############
//! ######@#@######
//! #b.....#.....c#
//! ###############
//! ```
//!
//! First, the top-right, bottom-left, and bottom-right robots take turns
//! collecting keys a, b, and c, a total of 6 + 6 + 6 = 18 steps. Then, the
//! top-left robot can access key d, spending another 6 steps; collecting all of
//! the keys here takes a minimum of 24 steps.
//!
//! Here's a more complex example:
//!
//! ```text
//! #############
//! #DcBa.#.GhKl#
//! #.###@#@#I###
//! #e#d#####j#k#
//! ###C#@#@###J#
//! #fEbA.#.FgHi#
//! #############
//! ```
//!
//! * Top-left robot collects key a.
//! * Bottom-left robot collects key b.
//! * Top-left robot collects key c.
//! * Bottom-left robot collects key d.
//! * Top-left robot collects key e.
//! * Bottom-left robot collects key f.
//! * Bottom-right robot collects key g.
//! * Top-right robot collects key h.
//! * Bottom-right robot collects key i.
//! * Top-right robot collects key j.
//! * Bottom-right robot collects key k.
//! * Top-right robot collects key l.
//!
//! In the above example, the fewest steps to collect all of the keys is 32.
//!
//! Here's an example with more choices:
//!
//! ```text
//! #############
//! #g#f.D#..h#l#
//! #F###e#E###.#
//! #dCba@#@BcIJ#
//! #############
//! #nK.L@#@G...#
//! #M###N#H###.#
//! #o#m..#i#jk.#
//! #############
//! ```
//!
//! One solution with the fewest steps is:
//!
//! * Top-left robot collects key e.
//! * Top-right robot collects key h.
//! * Bottom-right robot collects key i.
//! * Top-left robot collects key a.
//! * Top-left robot collects key b.
//! * Top-right robot collects key c.
//! * Top-left robot collects key d.
//! * Top-left robot collects key f.
//! * Top-left robot collects key g.
//! * Bottom-right robot collects key k.
//! * Bottom-right robot collects key j.
//! * Top-right robot collects key l.
//! * Bottom-left robot collects key n.
//! * Bottom-left robot collects key m.
//! * Bottom-left robot collects key o.
//!
//! This example requires at least 72 steps to collect all keys.
//!
//! After updating your map and using the remote-controlled robots, what is the
//! fewest steps necessary to collect all of the keys?

use super::{Grid, GridPosition, Orientation};
use anyhow::{anyhow, Error, Result};
use petgraph::prelude::*;
use smallvec::SmallVec;
use std::{cmp, collections::HashMap, fmt};

pub const PUZZLE_INPUT: &str = include_str!("../inputs/input-18");

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyId(u8);

impl fmt::Display for KeyId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.sign_plus() {
            write!(f, "{}", (self.0 + b'A') as char)
        } else {
            write!(f, "{}", (self.0 + b'a') as char)
        }
    }
}

impl fmt::Debug for KeyId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = self.0 + if f.sign_plus() { b'A' } else { b'a' };

        f.debug_tuple("KeyId").field(&(x as char)).finish()
    }
}

impl PartialEq<char> for KeyId {
    fn eq(&self, o: &char) -> bool {
        (self.0 + b'a') as char == *o
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Element {
    Wall,
    Empty,
    Door(KeyId),
    Key(KeyId),
    Origin,
}

impl Element {
    fn is_origin(self) -> bool {
        match self {
            Element::Origin => true,
            _ => false,
        }
    }

    fn is_wall(self) -> bool {
        match self {
            Element::Wall => true,
            _ => false,
        }
    }

    fn is_key(self) -> bool {
        match self {
            Element::Key(_) => true,
            _ => false,
        }
    }

    fn is_door(self) -> bool {
        match self {
            Element::Door(_) => true,
            _ => false,
        }
    }

    fn key_id(self) -> Option<KeyId> {
        match self {
            Element::Key(i) => Some(i),
            _ => None,
        }
    }

    fn door_id(self) -> Option<KeyId> {
        match self {
            Element::Door(i) => Some(i),
            _ => None,
        }
    }
}

impl std::str::FromStr for Element {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(anyhow!("invalid input for element"));
        }

        let elem = match s {
            "#" => Self::Wall,
            " " | "." => Self::Empty,
            "@" => Self::Origin,
            x => {
                let b = x.bytes().nth(0).unwrap();
                if b >= b'a' && b <= b'z' {
                    Self::Key(KeyId(b - b'a'))
                } else if b >= b'A' && b <= b'Z' {
                    Self::Door(KeyId(b - b'A'))
                } else {
                    return Err(anyhow!("unrecognized element: {}", x));
                }
            }
        };
        Ok(elem)
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Wall => f.write_str("#"),
            Self::Empty => f.write_str(" "),
            Self::Origin => f.write_str("@"),
            Self::Key(x) => write!(f, "{}", x),
            Self::Door(x) => write!(f, "{:+}", x),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct KeyPath {
    keys_required: KeySet,
    steps: usize,
}

impl cmp::Ord for KeyPath {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl cmp::PartialOrd for KeyPath {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
pub struct Maze {
    grid: Grid<Element>,
    graph: UnGraphMap<GridPosition, usize>,
    origins: Vec<GridPosition>,
    origin_paths: Vec<Vec<(KeyId, KeyPath)>>,
    key_paths: HashMap<KeyId, Vec<(KeyId, KeyPath)>>,
    keys_to_find: KeySet,
}

impl Maze {
    // fn doors(&self) -> impl Iterator<Item = (u8, GridPosition)> + '_ {
    //     self.grid.enumerate()
    //         .filter_map(|(pos, &elem)| elem.door_id().map(|id| (id, pos)))
    // }

    pub fn greedy<T>(&self) -> Option<T>
    where
        T: Ord + Clone + Default + std::ops::AddAssign<KeyWithScore>,
    {
        type GreedyIntermediate<'a> = (KeyId, usize, KeySet, usize, &'a [(KeyId, KeyPath)]);
        let mut remaining = self.keys_to_find;
        let mut greedy_bots: Vec<_> = self.origin_paths.iter().map(|x| x.as_slice()).collect();
        let mut greedy = Some(T::default());
        while !remaining.is_empty() {
            let mut greedy_move: Option<GreedyIntermediate> = None;
            for (bot_num, bot_paths) in greedy_bots.iter().enumerate() {
                for &(k, v) in bot_paths.iter() {
                    if !v.keys_required.is_disjoint(remaining) {
                        continue;
                    }
                    if v.steps == 0 {
                        continue;
                    }
                    if !remaining.has_key(k) {
                        continue;
                    }

                    if let Some(m) = greedy_move {
                        if m.1 > v.steps {
                            let mut next_remaining = remaining;
                            next_remaining.remove_key(k);
                            greedy_move =
                                Some((k, v.steps, next_remaining, bot_num, &self.key_paths[&k]));
                        }
                    } else {
                        let mut next_remaining = remaining;
                        next_remaining.remove_key(k);
                        greedy_move =
                            Some((k, v.steps, next_remaining, bot_num, &self.key_paths[&k]));
                    }
                }
            }
            if greedy_move.is_none() {
                println!("bots: {:?}", greedy_bots);
                println!("remaining {}: {:?}", remaining.count(), remaining);
                //println!("best: {:?}", greedy);
            }

            let (kid, s, rem, b, m) = greedy_move.unwrap();
            if let Some(g) = &mut greedy {
                *g += KeyWithScore(kid, s);
            } else {
                let mut g = T::default();
                g += KeyWithScore(kid, s);
                greedy = Some(g);
            }
            remaining = rem;
            greedy_bots[b] = m;
        }

        //println!("Greedy path: {:?}", greedy);
        greedy
    }

    fn memoized_dfs<T>(
        &self,
        cache: &mut HashMap<CacheParams, Option<T>>,
        bots: Vec<Option<KeyId>>,
        keys_remaining: KeySet,
        score_limit: usize,
    ) -> Option<T>
    where
        T: Ord + Clone + Default + std::ops::AddAssign<KeyWithScore>,
    {
        if let Some(e) = cache.get(&CacheParams {
            bots: bots.clone(),
            keys_remaining,
        }) {
            log::trace!("Found state in cache");
            return e.clone();
        }
        if keys_remaining.is_empty() {
            cache.insert(
                CacheParams {
                    bots,
                    keys_remaining,
                },
                Some(T::default()),
            );
            return Some(T::default());
        }
        if score_limit == 0 {
            cache.insert(
                CacheParams {
                    bots,
                    keys_remaining,
                },
                None,
            );
            return None;
        }
        let mut best: Option<T> = None;
        for (bid, &b) in bots.iter().enumerate() {
            let map = if let Some(kid) = b {
                &self.key_paths[&kid]
            } else {
                &self.origin_paths[bid]
            };
            for &(k, v) in map.iter() {
                if !v.keys_required.is_disjoint(keys_remaining) {
                    continue;
                }
                if !keys_remaining.has_key(k) {
                    continue;
                }

                let mut bots = bots.clone();
                bots[bid] = Some(k);

                let mut next_keys = keys_remaining;
                next_keys.remove_key(k);

                if let Some(mut score) =
                    self.memoized_dfs(cache, bots, next_keys, score_limit.saturating_sub(v.steps))
                {
                    score += KeyWithScore(k, v.steps);

                    if best == None || *best.as_ref().unwrap() > score {
                        best = Some(score);
                    }
                }
            }
        }
        cache.insert(
            CacheParams {
                bots,
                keys_remaining,
            },
            best.clone(),
        );
        best
    }

    pub fn dfs_with_path(&self) -> Option<ScoreWithPath> {
        let greedy: ScoreWithPath = self.greedy()?;
        let mut result: ScoreWithPath = self.memoized_dfs(
            &mut HashMap::new(),
            std::iter::repeat(None)
                .take(self.origin_paths.len())
                .collect(),
            self.keys_to_find,
            greedy.score,
        )?;
        result.path.reverse();
        Some(result)
    }

    pub fn dfs(&self) -> Option<usize> {
        let greedy_score: usize = self.greedy()?;
        let result: usize = self.memoized_dfs(
            &mut HashMap::new(),
            std::iter::repeat(None)
                .take(self.origin_paths.len())
                .collect(),
            self.keys_to_find,
            greedy_score,
        )?;
        Some(result)
    }

    pub fn bfs(&self) -> Option<(KeyList, usize)> {
        let greedy_steps: usize = self.greedy()?;

        use std::{cmp::Reverse, collections::BinaryHeap};

        let mut queue = BinaryHeap::new();
        let mut visits = 0;

        queue.push(Reverse(Queue {
            bots: self.origin_paths.iter().map(|x| x.as_slice()).collect(),
            keys_remaining: self.keys_to_find,
            total_steps: 0,
            key_order: KeyList::new(),
        }));

        while let Some(Reverse(next)) = queue.pop() {
            visits += 1;
            if visits % 1_000_000 == 0 {
                log::debug!(
                    "Visited {} nodes with {} keys remaining: current: {:?}",
                    visits,
                    next.keys_remaining.count(),
                    next
                );
            }
            // Are we there?
            if next.keys_remaining.is_empty() {
                return Some((next.key_order, next.total_steps));
            }

            // Nope, add some steps
            for (bot_num, bot_paths) in next.bots.iter().enumerate() {
                for &(k, v) in bot_paths.iter() {
                    // If the keys required includes any of the keys we haven't gotten, then we
                    // cannot visit this key
                    if !v.keys_required.is_disjoint(next.keys_remaining) {
                        continue;
                    }
                    if v.steps == 0 || v.steps + next.total_steps > greedy_steps {
                        continue;
                    }
                    if !next.keys_remaining.has_key(k) {
                        continue;
                    }
                    let total_steps = next.total_steps + v.steps;
                    let mut key_order = next.key_order.clone();
                    key_order.push(k);
                    let mut bots = next.bots.clone();
                    bots[bot_num] = &self.key_paths[&k];
                    // if bots[bot_num].values().any(|v| v.steps + total_steps)
                    let mut keys_remaining = next.keys_remaining;
                    keys_remaining.remove_key(k);
                    if keys_remaining.is_empty() {
                        return Some((key_order, next.total_steps + v.steps));
                    }
                    queue.push(Reverse(Queue {
                        total_steps,
                        bots,
                        keys_remaining,
                        key_order,
                    }))
                }
            }
        }

        None
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ScoreWithPath {
    score: usize,
    path: KeyList,
}

impl cmp::Ord for ScoreWithPath {
    fn cmp(&self, o: &Self) -> cmp::Ordering {
        self.score.cmp(&o.score)
    }
}

impl cmp::PartialOrd for ScoreWithPath {
    fn partial_cmp(&self, o: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(o))
    }
}

impl std::ops::AddAssign<KeyWithScore> for ScoreWithPath {
    fn add_assign(&mut self, o: KeyWithScore) {
        self.path.push(o.0);
        self.score += o.1;
    }
}

impl std::ops::AddAssign<KeyWithScore> for usize {
    fn add_assign(&mut self, o: KeyWithScore) {
        *self += o.1;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyWithScore(KeyId, usize);

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct CacheParams {
    bots: Vec<Option<KeyId>>,
    keys_remaining: KeySet,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Queue<'a> {
    bots: Vec<&'a [(KeyId, KeyPath)]>,
    keys_remaining: KeySet,
    total_steps: usize,
    key_order: KeyList,
}

impl<'a> cmp::Ord for Queue<'a> {
    fn cmp(&self, o: &Self) -> cmp::Ordering {
        let c = self.total_steps.cmp(&o.total_steps);
        if c == cmp::Ordering::Equal {
            self.keys_remaining.count().cmp(&o.keys_remaining.count())
        } else {
            c
        }
    }
}

impl<'a> cmp::PartialOrd for Queue<'a> {
    fn partial_cmp(&self, o: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(o))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Path {
    steps: usize,
    path: Vec<KeyId>,
}

impl cmp::Ord for Path {
    fn cmp(&self, o: &Self) -> cmp::Ordering {
        self.steps.cmp(&o.steps)
    }
}

impl cmp::PartialOrd for Path {
    fn partial_cmp(&self, o: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(o))
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct KeySet(u32);

impl KeySet {
    fn is_empty(self) -> bool {
        self.0 == 0
    }

    fn count(self) -> u32 {
        self.0.count_ones()
    }

    fn add_key(&mut self, kid: KeyId) {
        self.0 |= 1 << kid.0;
    }

    fn remove_key(&mut self, kid: KeyId) {
        self.0 ^= (1 << kid.0) & self.0;
    }

    fn is_disjoint(self, other: Self) -> bool {
        self.0 & other.0 == 0
    }

    fn has_key(self, kid: KeyId) -> bool {
        let mask = 1 << kid.0;
        self.0 & mask == mask
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tagged_graph: UnGraphMap<_, _> = self
            .graph
            .all_edges()
            .map(|(s, t, &w)| {
                (
                    TaggedNode {
                        position: s,
                        element: *self.grid.get(s).unwrap(),
                    },
                    TaggedNode {
                        position: t,
                        element: *self.grid.get(t).unwrap(),
                    },
                    w,
                )
            })
            .collect();
        writeln!(f, "Grid:\n{}", self.grid)?;
        writeln!(f, "Graph:\n{}", petgraph::dot::Dot::new(&tagged_graph))?;
        writeln!(f, "Origin Paths:\n{:?}", self.origin_paths)?;
        writeln!(f, "Key Paths:\n{:?}", self.key_paths)
    }
}

#[derive(Clone, Copy)]
struct TaggedNode {
    position: GridPosition,
    element: Element,
}

impl fmt::Display for TaggedNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.element, self.position)
    }
}

impl std::hash::Hash for TaggedNode {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        std::hash::Hash::hash(&self.position, h);
    }
}

impl PartialEq for TaggedNode {
    fn eq(&self, o: &Self) -> bool {
        self.position.eq(&o.position)
    }
}

impl Eq for TaggedNode {}

impl cmp::Ord for TaggedNode {
    fn cmp(&self, o: &Self) -> cmp::Ordering {
        self.position.cmp(&o.position)
    }
}

impl cmp::PartialOrd for TaggedNode {
    fn partial_cmp(&self, o: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(o))
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
            if !elem.is_wall() {
                graph.add_edge(pos, n_pos, 1);
            }
        }
    }
}

impl From<Grid<Element>> for Maze {
    fn from(grid: Grid<Element>) -> Self {
        let mut keys = HashMap::new();
        let mut keys_to_find = KeySet::default();
        let mut origins = Vec::new();
        for (pos, &elem) in grid.enumerate() {
            if elem.is_origin() {
                origins.push(pos);
            } else if let Some(kid) = elem.key_id() {
                keys_to_find.add_key(kid);
                keys.insert(kid, pos);
            }
        }

        let mut graph = UnGraphMap::new();

        for (pos, &elem) in grid.enumerate() {
            graph.add_node(pos);
            if !elem.is_wall() {
                add_edge_if_neighbor_is_not_wall(&grid, &mut graph, pos, Orientation::East);
                add_edge_if_neighbor_is_not_wall(&grid, &mut graph, pos, Orientation::South);
            }
        }

        log::debug!("Graph uncompressed:\n{}", petgraph::dot::Dot::new(&graph));

        compress_ungraph(&grid, &mut graph);

        log::debug!("Graph compressed:\n{}", petgraph::dot::Dot::new(&graph));

        let key_paths: HashMap<_, _> = keys
            .iter()
            .map(|(&id, &pos)| (id, key_finder_bfs(&grid, &graph, &keys, pos)))
            .collect();

        let origin_paths: Vec<_> = origins
            .iter()
            .map(|&pos| key_finder_bfs(&grid, &graph, &keys, pos))
            .collect();

        Self {
            grid,
            graph,
            origins,
            origin_paths,
            key_paths,
            keys_to_find,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MinScored<K, T>(pub K, pub T);

impl<K: Ord, T> PartialEq for MinScored<K, T> {
    #[inline]
    fn eq(&self, other: &MinScored<K, T>) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}

impl<K: Ord, T> Eq for MinScored<K, T> {}

impl<K: Ord, T> PartialOrd for MinScored<K, T> {
    #[inline]
    fn partial_cmp(&self, other: &MinScored<K, T>) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, T> Ord for MinScored<K, T> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

fn key_finder_dijkstra(
    grid: &Grid<Element>,
    graph: &UnGraphMap<GridPosition, usize>,
    start: GridPosition,
) -> HashMap<GridPosition, KeyPath> {
    use petgraph::visit::{VisitMap, Visitable};
    use std::collections::{hash_map::Entry, BinaryHeap};

    let mut visited = graph.visit_map();
    let mut scores = HashMap::new();
    let mut visit_next = BinaryHeap::new();
    scores.insert(start, KeyPath::default());
    visit_next.push(MinScored(KeyPath::default(), start));
    while let Some(MinScored(node_score, node)) = visit_next.pop() {
        if visited.is_visited(&node) {
            continue;
        }
        for edge in graph.edges(node) {
            let next = edge.target();
            if visited.is_visited(&next) {
                continue;
            }

            let weight = *edge.weight();
            let elem = *grid.get(edge.target()).expect("in grid");

            let mut next_score = node_score;
            next_score.steps += weight;
            if let Some(did) = elem.door_id() {
                next_score.keys_required.add_key(did);
            }

            match scores.entry(next) {
                Entry::Occupied(ent) if next_score < *ent.get() => {
                    *ent.into_mut() = next_score;
                }
                Entry::Occupied(ent) => {
                    next_score = *ent.get();
                }
                Entry::Vacant(ent) => {
                    ent.insert(next_score.clone());
                }
            }
            visit_next.push(MinScored(next_score, next));
        }
        visited.visit(node);
    }
    scores.remove(&start);
    scores
}

fn key_finder_bfs(
    grid: &Grid<Element>,
    graph: &UnGraphMap<GridPosition, usize>,
    keys: &HashMap<KeyId, GridPosition>,
    origin: GridPosition,
) -> Vec<(KeyId, KeyPath)> {
    // let mut bfs = petgraph::visit::Bfs::new(graph, origin);
    // let mut paths = HashMap::new();
    // paths.insert(origin, KeyPath { keys_required: HashSet::new(), steps: 0 });
    // while let Some(n) = bfs.next(&graph) {
    //     let weight = graph.edge_weight(current, n).expect("edge in graph");
    //     let elem = *grid.get(n).expect("in grid");
    //     let mut n_kpath = paths.get(&current).expect("predecessor").clone();
    //     n_kpath.steps += weight;
    //     if let Some(did) = elem.door_id() {
    //         n_kpath.keys_required.insert(did);
    //     }
    //     paths.insert(n, n_kpath);
    // }

    let mut paths = key_finder_dijkstra(grid, graph, origin);

    let mut imm: Vec<_> = keys
        .iter()
        .filter_map(|(&kid, kpos)| {
            let kpath = paths.remove(kpos)?;
            Some((kid, kpath))
        })
        .collect();
    imm.sort_by_key(|(_, kp)| kp.steps);
    imm
}

fn compress_ungraph(grid: &Grid<Element>, graph: &mut UnGraphMap<GridPosition, usize>) {
    let mut repeat = true;
    while repeat {
        repeat = false;
        let candidates: Vec<_> = graph.nodes().collect();
        for candidate in candidates {
            let elem = grid.get(candidate).copied().expect("to exist");
            if elem.is_key() || elem.is_door() || elem.is_origin() {
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KeyList(SmallVec<[KeyId; 32]>);

impl KeyList {
    pub fn new() -> Self {
        KeyList(SmallVec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, kid: KeyId) {
        self.0.push(kid);
    }

    pub fn reverse(&mut self) {
        self.0.reverse()
    }

    pub fn into_inner(self) -> SmallVec<[KeyId; 32]> {
        self.0
    }
}

impl From<Vec<KeyId>> for KeyList {
    fn from(v: Vec<KeyId>) -> Self {
        Self(SmallVec::from(v))
    }
}

impl std::iter::FromIterator<KeyId> for KeyList {
    fn from_iter<I: IntoIterator<Item = KeyId>>(iter: I) -> Self {
        Self(SmallVec::from_iter(iter))
    }
}

impl fmt::Display for KeyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, &kid) in self.0.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", kid)?;
            } else {
                write!(f, ", {}", kid)?;
            }
        }
        write!(f, "]")
    }
}

impl PartialEq<[char]> for KeyList {
    fn eq(&self, o: &[char]) -> bool {
        &self.0[..] == o
    }
}

pub fn run() -> Result<()> {
    let mut input: Grid<Element> = PUZZLE_INPUT.parse()?;
    let maze = Maze::from(input.clone());
    println!("Maze: {}", maze);

    if let Some(score) = maze.dfs_with_path() {
        println!("Best path in {} steps: {}", score.score, score.path);
    } else {
        println!("Could not find a path to all keys");
    }

    let origin = maze.origins[0];
    input.set(origin, Element::Wall);
    input.set(origin + Orientation::North, Element::Wall);
    input.set(origin + Orientation::South, Element::Wall);
    input.set(origin + Orientation::West, Element::Wall);
    input.set(origin + Orientation::East, Element::Wall);
    input.set(
        origin + Orientation::North + Orientation::East,
        Element::Origin,
    );
    input.set(
        origin + Orientation::South + Orientation::West,
        Element::Origin,
    );
    input.set(
        origin + Orientation::West + Orientation::North,
        Element::Origin,
    );
    input.set(
        origin + Orientation::East + Orientation::South,
        Element::Origin,
    );
    let maze2 = Maze::from(input);
    println!("Maze: {}", maze2);

    if let Some(score) = maze2.dfs_with_path() {
        println!("Best path in {} steps: {}", score.score, score.path);
    } else {
        println!("Could not find a path to all keys");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Element, Grid, Maze};
    use pretty_assertions::assert_eq;

    #[test]
    fn example_1() {
        crate::init_logging();
        const EXAMPLE: &str = "
            #########
            #b.A.@.a#
            #########";

        let input: Grid<Element> = EXAMPLE.parse().unwrap();
        let maze = Maze::from(input);

        let best = maze.dfs_with_path().expect("was no path to all keys");
        assert_eq!(best.score, 8);
        assert_eq!(&best.path, &['a', 'b'][..]);
    }

    #[test]
    fn example_2() {
        crate::init_logging();
        const EXAMPLE: &str = "
            ########################
            #f.D.E.e.C.b.A.@.a.B.c.#
            ######################.#
            #d.....................#
            ########################";

        let input: Grid<Element> = EXAMPLE.parse().unwrap();
        let maze = Maze::from(input);

        println!("graph: {}", petgraph::dot::Dot::new(&maze.graph));

        let best = maze.dfs_with_path().expect("was no path to all keys");
        assert_eq!(best.score, 86);
        assert_eq!(&best.path, &['a', 'b', 'c', 'd', 'e', 'f'][..]);
    }

    #[test]
    fn example_3() {
        crate::init_logging();
        const EXAMPLE: &str = "
            ########################
            #...............b.C.D.f#
            #.######################
            #.....@.a.B.c.d.A.e.F.g#
            ########################";

        let input: Grid<Element> = EXAMPLE.parse().unwrap();
        let maze = Maze::from(input);

        println!("graph: {}", petgraph::dot::Dot::new(&maze.graph));

        let best = maze.dfs_with_path().expect("was no path to all keys");
        assert_eq!(best.score, 132);
        assert_eq!(&best.path, &['b', 'a', 'c', 'd', 'f', 'e', 'g'][..]);
    }

    #[test]
    fn example_4() {
        crate::init_logging();
        const EXAMPLE: &str = "
            #################
            #i.G..c...e..H.p#
            ########.########
            #j.A..b...f..D.o#
            ########@########
            #k.E..a...g..B.n#
            ########.########
            #l.F..d...h..C.m#
            #################";

        let input: Grid<Element> = EXAMPLE.parse().unwrap();
        let maze = Maze::from(input);

        println!("graph: {}", petgraph::dot::Dot::new(&maze.graph));

        let steps = maze.dfs().expect("was no path to all keys");
        //println!("Found path: {:?}", path);
        assert_eq!(steps, 136);
    }

    #[test]
    fn example_5() {
        crate::init_logging();
        const EXAMPLE: &str = "
            ########################
            #@..............ac.GI.b#
            ###d#e#f################
            ###A#B#C################
            ###g#h#i################
            ########################";

        let input: Grid<Element> = EXAMPLE.parse().unwrap();
        let maze = Maze::from(input);

        println!("graph: {}", petgraph::dot::Dot::new(&maze.graph));

        let steps = maze.dfs().expect("was no path to all keys");
        //println!("Found path: {:?}", path);
        assert_eq!(steps, 81);
    }

    #[test]
    fn example_6() {
        crate::init_logging();
        const EXAMPLE: &str = "
            #######
            #a.#Cd#
            ##@#@##
            #######
            ##@#@##
            #cB#Ab#
            #######";

        let input: Grid<Element> = EXAMPLE.parse().unwrap();
        let maze = Maze::from(input);

        println!("graph: {}", petgraph::dot::Dot::new(&maze.graph));

        let best = maze.dfs_with_path().expect("was no path to all keys");
        assert_eq!(best.score, 8);
        assert_eq!(&best.path, &['a', 'b', 'c', 'd'][..]);
    }

    #[test]
    fn example_7() {
        crate::init_logging();
        const EXAMPLE: &str = "
            ###############
            #d.ABC.#.....a#
            ######@#@######
            ###############
            ######@#@######
            #b.....#.....c#
            ###############";

        let input: Grid<Element> = EXAMPLE.parse().unwrap();
        let maze = Maze::from(input);

        println!("graph: {}", petgraph::dot::Dot::new(&maze.graph));

        let steps = maze.dfs().expect("was no path to all keys");
        // println!("Found path: {:?}", path);
        assert_eq!(steps, 24);
    }

    #[test]
    fn example_8() {
        crate::init_logging();
        const EXAMPLE: &str = "
            #############
            #DcBa.#.GhKl#
            #.###@#@#I###
            #e#d#####j#k#
            ###C#@#@###J#
            #fEbA.#.FgHi#
            #############";

        let input: Grid<Element> = EXAMPLE.parse().unwrap();
        let maze = Maze::from(input);

        println!("graph: {}", petgraph::dot::Dot::new(&maze.graph));

        let best = maze.dfs_with_path().expect("was no path to all keys");
        assert_eq!(best.score, 32);
        assert_eq!(
            &best.path,
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'][..]
        );
    }

    #[test]
    fn example_9() {
        crate::init_logging();
        const EXAMPLE: &str = "
            #############
            #g#f.D#..h#l#
            #F###e#E###.#
            #dCba@#@BcIJ#
            #############
            #nK.L@#@G...#
            #M###N#H###.#
            #o#m..#i#jk.#
            #############";

        let input: Grid<Element> = EXAMPLE.parse().unwrap();
        let maze = Maze::from(input);

        println!("graph: {}", petgraph::dot::Dot::new(&maze.graph));

        let steps = maze.dfs().expect("was no path to all keys");
        // println!("Found path: {:?}", path);
        assert_eq!(steps, 72);
    }
}
