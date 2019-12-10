//! # Day 4: Secure Container
//!
//! You arrive at the Venus fuel depot only to discover it's protected by a
//! password. The Elves had written the password on a sticky note, but someone
//! threw it out.
//!
//! However, they do remember a few key facts about the password:
//!
//! * It is a six-digit number.
//! * The value is within the range given in your puzzle input.
//! * Two adjacent digits are the same (like 22 in 122345).
//! * Going from left to right, the digits never decrease; they only ever
//!   increase or stay the same (like 111123 or 135679).
//!
//! Other than the range rule, the following are true:
//!
//! * 111111 meets these criteria (double 11, never decreases).
//! * 223450 does not meet these criteria (decreasing pair of digits 50).
//! * 123789 does not meet these criteria (no double).
//!
//! How many different passwords within the range given in your puzzle input
//! meet these criteria?
//!
//! ## Part Two
//!
//! An Elf just remembered one more important detail: the two adjacent matching
//! digits are not part of a larger group of matching digits.
//!
//! Given this additional criterion, but still ignoring the range rule, the
//! following are now true:
//!
//! * 112233 meets these criteria because the digits never decrease and all
//!   repeated digits are exactly two digits long.
//! * 123444 no longer meets the criteria (the repeated 44 is part of a larger
//!   group of 444).
//! * 111122 meets the criteria (even though 1 is repeated more than twice, it
//!   still contains a double 22).
//!
//! How many different passwords within the range given in your puzzle input
//! meet all of the criteria?

use std::ops;

pub const PUZZLE_INPUT: &str = include_str!("../inputs/input-04");

pub fn parse_input(input: &str) -> ops::RangeInclusive<u32> {
    let values: Vec<u32> = input
        .split('-')
        .filter(|op| !op.is_empty())
        .map(|op| op.trim().parse().expect("data must be a valid integer"))
        .collect();

    (values[0]..=values[1])
}

struct DigitIterator(u32);

impl From<u32> for DigitIterator {
    fn from(x: u32) -> Self {
        Self(x)
    }
}

impl Iterator for DigitIterator {
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

fn digits_are_in_increasing_order(x: u32) -> bool {
    let mut last = 9u8;
    for digit in DigitIterator::from(x) {
        if digit > last {
            return false;
        } else {
            last = digit;
        }
    }

    true
}

fn contains_a_pair(x: u32) -> bool {
    let mut iter = DigitIterator::from(x);
    let mut last = iter.next().expect("number to have digits");

    for digit in iter {
        if digit == last {
            return true;
        }

        last = digit;
    }

    false
}

fn contains_a_proper_pair(x: u32) -> bool {
    let mut iter = DigitIterator::from(x);
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

    is_in_pair && !too_long
}

pub fn find_valid_passwords_part_1(range: ops::RangeInclusive<u32>) -> usize {
    range
        .filter(|&x| digits_are_in_increasing_order(x))
        .filter(|&x| contains_a_pair(x))
        .count()
}

pub fn find_valid_passwords_part_2(range: ops::RangeInclusive<u32>) -> usize {
    range
        .filter(|&x| digits_are_in_increasing_order(x))
        .filter(|&x| contains_a_proper_pair(x))
        .count()
}

pub fn run() {
    let range = parse_input(PUZZLE_INPUT);

    let result = find_valid_passwords_part_1(range.clone());
    println!("Valid passwords: {}", result);

    let result = find_valid_passwords_part_2(range);
    println!("Valid passwords (part 2): {}", result);
}
