use std::fmt;

/// An address into the memory of an Intcode program
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Address(usize);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "address {}", self.0)
    }
}

impl Address {
    /// Constructs a new `Address` from a raw offset
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    /// Attempts to construct an `Address` from an Intcode value
    /// 
    /// If the value is negative, `None` will be returned.
    pub fn try_from_value(value: isize) -> Option<Self> {
        if value >= 0 {
            Some(Self::new(value as usize))
        } else {
            None
        }
    }

    /// Extracts the raw address offset
    pub const fn value(self) -> usize {
        self.0
    }
}

