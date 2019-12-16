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
