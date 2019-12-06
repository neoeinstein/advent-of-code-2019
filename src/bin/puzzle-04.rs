use std::{
    io::Read,
    ops,
};
use advent_of_code_2019::get_input_reader;

struct DigitIterator(u32);

impl From<u32> for DigitIterator {
    fn from(x: u32) -> Self {
        Self(x)
    }
}

impl Iterator for DigitIterator
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let (div, rem) = (self.0 / 10, self.0 % 10);
        self.0 = div;
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

fn contains_a_proper_pair(x: &u32) -> bool {
    let mut iter = DigitIterator::from(*x);
    let mut last = iter.next().expect("number to have digits");
    let mut is_in_pair = false;
    let mut too_long = false;
    
    for digit in iter {
        if is_in_pair && digit != last && !too_long {
            return true;
        }

        if digit == last {
            if is_in_pair {
                too_long = true;
            }
            is_in_pair = true;
        } else {
            too_long = false;
            is_in_pair = false;
        }
        
        last = digit;
    }

    return is_in_pair && !too_long;
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

    let result = range
        .filter(digits_are_in_increasing_order)
        .filter(contains_a_proper_pair)
        .count();

    println!("Count: {}", result);
}
