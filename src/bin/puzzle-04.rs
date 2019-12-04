use std::{
    convert::TryFrom,
    io::Read,
    ops,
};
use advent_of_code_2019::get_input_reader;

struct DigitIterator {
    x: u32,
}

impl From<u32> for DigitIterator {
    fn from(x: u32) -> Self {
        Self { x }
    }
}

impl Iterator for DigitIterator
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == 0 {
            return None;
        }
        let (div, rem) = (self.x / 10, self.x % 10);
        self.x = div;
        Some(rem as u8)
    }
}

fn digits_are_in_increasing_order(x: &u32) -> bool {
    let mut last = 9u8;
    for digit in DigitIterator::from(*x) {
        if digit > last {
            return false;
        } else {
            last = digit;
        }
    }
    
    return true;
}

fn contains_a_pair(x: &u32) -> bool {
    let mut iter = DigitIterator::from(*x);
    let mut last = iter.next().expect("number to have digits");
    
    for digit in iter {
        if digit == last {
            return true;
        }
        last = digit;
    }

    return false;
}

fn parse_input() -> ops::RangeInclusive<u32> {
    let mut in_fd = get_input_reader();
    let mut data = String::new();
    in_fd.read_to_string(&mut data).expect("error reading data");

    let values: Vec<u32> = data.split('-')
        .filter(|op| !op.is_empty())
        .map(|op| op.trim().parse().expect("data must be a valid integer"))
        .collect();

    (values[0]..=values[1])
}

fn main() {
    let range = parse_input();

    let result = range.filter(digits_are_in_increasing_order).filter(contains_a_pair).count();

    println!("Count: {}", result);
}
