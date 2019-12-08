use super::{Address, Word};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
#[error("{value} is an invalid address")]
pub struct InvalidAddress {
    value: Word,
}

impl InvalidAddress {
    pub(crate) const fn new(value: Word) -> Self {
        Self { value }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
#[error("attempted to access out of bounds memory address {address}")]
pub struct OutOfBoundsAccess {
    address: Address,
}

impl OutOfBoundsAccess {
    pub(crate) const fn new(address: Address) -> Self {
        Self { address }
    }
}
