use super::{error, Word};
use std::{convert::TryFrom, fmt, ops};

/// An address into the memory of an Intcode program
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Address(usize);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Address {
    /// Constructs a new `Address` from a raw value
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    /// Extracts the raw address
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

/// A relative offset from a base address into the memory of an Intcode program
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Relative(Word);

impl fmt::Display for Relative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:+}", self.0)
    }
}

impl Relative {
    /// Constructs a new `Relative` offset from a raw value
    pub const fn new(offset: Word) -> Self {
        Self(offset)
    }

    /// Extracts the raw offset
    pub const fn value(self) -> Word {
        self.0
    }
}

impl From<Word> for Relative {
    fn from(w: Word) -> Self {
        Self::new(w)
    }
}

impl ops::Add<Relative> for Address {
    type Output = Result<Self, error::InvalidAddress>;
    fn add(self, offset: Relative) -> Self::Output {
        let addr = self.0 as Word + offset.0;
        Address::try_from(addr)
    }
}
