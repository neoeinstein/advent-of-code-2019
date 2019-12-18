use super::Orientation;
use std::fmt;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position2D {
    pub x: i64,
    pub y: i64,
}

impl Position2D {
    pub const ORIGIN: Self = Position2D { x: 0, y: 0 };

    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Position2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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

impl std::ops::Sub<Orientation> for Position2D {
    type Output = Self;
    fn sub(self, o: Orientation) -> Self::Output {
        self + -o
    }
}

impl std::ops::SubAssign<Orientation> for Position2D {
    fn sub_assign(&mut self, o: Orientation) {
        *self += -o
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GridPosition {
    pub row: usize,
    pub col: usize,
}

impl GridPosition {
    pub const ORIGIN: Self = GridPosition { row: 0, col: 0 };

    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub const fn from_idx(idx: usize, cols: usize) -> Self {
        Self {
            row: idx / cols,
            col: idx % cols,
        }
    }

    pub const fn idx(self, cols: usize) -> usize {
        self.row * cols + self.col
    }

    pub fn neighbor(self, direction: Orientation) -> Option<Self> {
        let new_positon = match direction {
            Orientation::North => {
                if self.row == 0 {
                    return None;
                }
                GridPosition::new(self.row - 1, self.col)
            }
            Orientation::South => GridPosition::new(self.row + 1, self.col),
            Orientation::West => {
                if self.col == 0 {
                    return None;
                }
                GridPosition::new(self.row, self.col - 1)
            }
            Orientation::East => GridPosition::new(self.row, self.col + 1),
        };
        Some(new_positon)
    }

    pub fn limit(self, max: GridPosition) -> Option<Self> {
        if self.col > max.col || self.row > max.row {
            None
        } else {
            Some(self)
        }
    }
}

impl fmt::Display for GridPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl std::ops::Add<Orientation> for GridPosition {
    type Output = Self;
    fn add(self, o: Orientation) -> Self::Output {
        self.neighbor(o)
            .expect("neighbor is in an invalid grid position")
    }
}

impl std::ops::AddAssign<Orientation> for GridPosition {
    fn add_assign(&mut self, o: Orientation) {
        *self = *self + o
    }
}

impl std::ops::Sub<Orientation> for GridPosition {
    type Output = Self;
    fn sub(self, o: Orientation) -> Self::Output {
        self + -o
    }
}

impl std::ops::SubAssign<Orientation> for GridPosition {
    fn sub_assign(&mut self, o: Orientation) {
        *self += -o
    }
}
