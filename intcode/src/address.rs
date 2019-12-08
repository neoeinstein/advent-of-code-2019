use super::{error, Word};
use std::convert::TryFrom;
use std::fmt;

/// An address into the memory of an Intcode program
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Address(usize);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Address {
    /// Constructs a new `Address` from a raw offset
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    /// Extracts the raw address offset
    pub const fn value(self) -> usize {
        self.0
    }
}

impl TryFrom<Word> for Address {
    type Error = error::InvalidAddress;
    fn try_from(w: Word) -> Result<Self, error::InvalidAddress> {
        if w >= 0 {
            Ok(Self::new(w as usize))
        } else {
            Err(error::InvalidAddress::new(w))
        }
    }
}
