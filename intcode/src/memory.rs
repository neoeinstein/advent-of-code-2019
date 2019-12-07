use super::{Address, Word};
use std::{io, mem};

/// An Intcode memory
///
/// Intcode programs are a vector of signed integers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Memory {
    data: Vec<Word>,
}

impl Memory {
    /// Initializes Intcode memory from a vector of data
    pub fn from_vec(data: Vec<Word>) -> Self {
        Self { data }
    }

    /// Initializes Intcode memory from a string
    ///
    /// Expected format is a series of signed ASCII integers separated by
    /// commas. Whitespace is allowed between numbers and commas.
    ///
    /// An example of a valid input:
    ///
    /// ```text
    /// 1,9,10,3,
    /// 2,3,11,0,
    /// 99,
    /// 30,40,50
    /// ```
    pub fn from_str(input: &str) -> io::Result<Memory> {
        let data = input
            .split(',')
            .filter_map(|op| {
                let trimmed = op.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(
                        op.trim()
                            .parse()
                            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e)),
                    )
                }
            })
            .collect::<io::Result<Vec<Word>>>()?;

        Ok(Self::from_vec(data))
    }

    /// Initializes Intcode memory from an `io::Read`er
    ///
    /// This function will read in the entire dataset before parsing.
    pub fn from_reader(input: &mut dyn io::Read) -> io::Result<Memory> {
        let mut raw_data = String::new();
        input.read_to_string(&mut raw_data)?;

        Self::from_str(&raw_data)
    }

    /// Initializes Intcode memory from an `io::BufRead`er
    ///
    /// This function will read in the entire dataset before parsing.
    pub fn from_buf_reader(input: &mut dyn io::BufRead) -> io::Result<Memory> {
        let mut data = Vec::new();
        let mut buf = Vec::with_capacity(16);
        loop {
            buf.clear();

            match input.read_until(b',', &mut buf)? {
                0 => break,
                c => {
                    debug_assert!(c == buf.len());
                    let raw = std::str::from_utf8(&buf[..c - 1])
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                    let trimmed = raw.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    let value = trimmed
                        .parse()
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
                    data.push(value);
                }
            }
        }

        Ok(Self::from_vec(data))
    }

    /// Provides immutable access to the underlying memory
    pub fn raw(&self) -> &[Word] {
        &self.data
    }

    /// Returns the maximum valid address in memory
    pub fn max_address(&self) -> Address {
        Address::new(self.data.len() - 1)
    }

    /// Attempts to read a value from a given address
    ///
    /// Returns `None` if the address is outside the bounds of legal addresses.
    pub fn try_read(&self, address: Address) -> Option<Word> {
        self.data.get(address.value()).copied()
    }

    /// Attempts to write a value to the given address
    ///
    /// Returns the prior value at that address, or `None` if the address was
    /// outside the bounds of legal addresses.
    pub fn try_write(&mut self, address: Address, value: Word) -> Option<Word> {
        let sloc = self.data.get_mut(address.value())?;
        Some(mem::replace(sloc, value))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Memory, Word};
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    const DATA: &str = "1,  32,3, -52,
    4,12,44,
    ";
    const EXPECTED: &[Word] = &[1, 32, 3, -52, 4, 12, 44];

    #[test]
    fn check_buf_reader() -> Result<()> {
        let mut reader = std::io::Cursor::new(DATA);

        assert_eq!(EXPECTED, Memory::from_buf_reader(&mut reader)?.raw());

        Ok(())
    }

    #[test]
    fn check_reader() -> Result<()> {
        let mut reader = std::io::Cursor::new(DATA);

        assert_eq!(EXPECTED, Memory::from_reader(&mut reader)?.raw());

        Ok(())
    }

    #[test]
    fn check_str() -> Result<()> {
        assert_eq!(EXPECTED, Memory::from_str(DATA)?.raw());

        Ok(())
    }
}
