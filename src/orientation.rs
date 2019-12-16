use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    North,
    South,
    West,
    East,
}

impl Orientation {
    pub fn left(self) -> Self {
        match self {
            Orientation::North => Orientation::West,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
            Orientation::East => Orientation::North,
        }
    }

    pub fn turn_left(&mut self) {
        *self = self.left()
    }

    pub fn right(self) -> Self {
        match self {
            Orientation::North => Orientation::East,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
            Orientation::East => Orientation::South,
        }
    }

    pub fn turn_right(&mut self) {
        *self = self.right()
    }

    pub fn reverse(self) -> Self {
        match self {
            Orientation::North => Orientation::South,
            Orientation::South => Orientation::North,
            Orientation::West => Orientation::East,
            Orientation::East => Orientation::West,
        }
    }

    pub fn turn_around(&mut self) {
        *self = self.reverse()
    }
}

impl std::ops::Neg for Orientation {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.reverse()
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir = match self {
            Orientation::North => "North",
            Orientation::South => "South",
            Orientation::West => "West",
            Orientation::East => "East",
        };

        f.write_str(dir)
    }
}
