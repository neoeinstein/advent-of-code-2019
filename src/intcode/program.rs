use std::{io, mem};
use super::Address;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    data: Vec<isize>,
}

impl Program {
    pub fn from_vec(data: Vec<isize>) -> Self {
        Self { data }
    }

    pub fn from_reader(input: &mut dyn io::Read) -> io::Result<Program> {
        let mut raw_data = String::new();
        input.read_to_string(&mut raw_data)?;
    
        let data = raw_data.split(',')
            .filter(|op| !op.is_empty())
            .map(|op| op.trim().parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e)))
            .collect::<io::Result<Vec<isize>>>()?;

        Ok(Self::from_vec(data))
    }

    pub fn data(&self) -> &[isize] {
        &self.data
    }

    pub fn max_address(&self) -> Address {
        Address::new(self.data.len() - 1)
    }

    pub fn try_read(&self, address: Address) -> Option<isize> {
        self.data.get(address.value()).copied()
    }

    pub fn try_write(&mut self, address: Address, value: isize) -> Option<isize> {
        let sloc = self.data.get_mut(address.value())?;
        Some(mem::replace(sloc, value))
    }
}
