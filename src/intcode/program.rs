use super::Address;
use std::{io, mem};

/// An Intcode program
///
/// Intcode programs are a vector of signed integers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    data: Vec<isize>,
}

impl Program {
    /// Constructs a program from a vector of data
    pub fn from_vec(data: Vec<isize>) -> Self {
        Self { data }
    }

    /// Reads and parses a program from an `io::Read`er
    ///
    /// Expected format is a series of signed ASCII integers separated by
    /// commas. Whitespace is allowed between numbers and commas.
    ///
    /// An example of a valid program:
    ///
    /// ```text
    /// 1,9,10,3,
    /// 2,3,11,0,
    /// 99,
    /// 30,40,50
    /// ```
    pub fn from_reader(input: &mut dyn io::Read) -> io::Result<Program> {
        let mut raw_data = String::new();
        input.read_to_string(&mut raw_data)?;

        let data = raw_data
            .split(',')
            .filter(|op| !op.is_empty())
            .map(|op| {
                op.trim()
                    .parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
            })
            .collect::<io::Result<Vec<isize>>>()?;

        Ok(Self::from_vec(data))
    }

    /// Provides immutable access to the underlying program data
    pub fn data(&self) -> &[isize] {
        &self.data
    }

    /// Returns the lowest address for which attempts to access memory will
    /// result in an out-of-bounds access
    pub fn max_address(&self) -> Address {
        Address::new(self.data.len() - 1)
    }

    /// Attempts to read a value from a given address
    ///
    /// Returns `None` if the address is outside the bounds of program memory.
    pub fn try_read(&self, address: Address) -> Option<isize> {
        self.data.get(address.value()).copied()
    }

    /// Attempts to write a value to the given address
    ///
    /// Returns the prior value at that address, or `None` if the address was
    /// outside the bounds of program memory.
    pub fn try_write(&mut self, address: Address, value: isize) -> Option<isize> {
        let sloc = self.data.get_mut(address.value())?;
        Some(mem::replace(sloc, value))
    }
}
