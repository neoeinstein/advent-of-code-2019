use std::fmt;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Address(usize);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "address {}", self.0)
    }
}

impl Address {
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    pub fn try_from_value(value: isize) -> Option<Self> {
        if value >= 0 {
            Some(Self::new(value as usize))
        } else {
            None
        }
    }

    pub const fn value(self) -> usize {
        self.0
    }
}

