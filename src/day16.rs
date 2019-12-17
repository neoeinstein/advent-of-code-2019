//! # Day 16: Flawed Frequency Transmission
//!
//! You're 3/4ths of the way through the gas giants. Not only do roundtrip
//! signals to Earth take five hours, but the signal quality is quite bad as
//! well. You can clean up the signal with the Flawed Frequency Transmission
//! algorithm, or FFT.
//!
//! As input, FFT takes a list of numbers. In the signal you received (your
//! puzzle input), each number is a single digit: data like 15243 represents the
//! sequence `1, 5, 2, 4, 3`.
//!
//! FFT operates in repeated phases. In each phase, a new list is constructed
//! with the same length as the input list. This new list is also used as the
//! input for the next phase.
//!
//! Each element in the new list is built by multiplying every value in the
//! input list by a value in a repeating pattern and then adding up the results.
//! So, if the input list were 9, 8, 7, 6, 5 and the pattern for a given element
//! were 1, 2, 3, the result would be `9*1 + 8*2 + 7*3 + 6*1 + 5*2` (with each
//! input element on the left and each value in the repeating pattern on the
//! right of each multiplication). Then, only the ones digit is kept: 38 becomes
//! 8, -17 becomes 7, and so on.
//!
//! While each element in the output array uses all of the same input array
//! elements, the actual repeating pattern to use depends on which output
//! element is being calculated. The base pattern is `0, 1, 0, -1`. Then, repeat
//! each value in the pattern a number of times equal to the position in the
//! output list being considered. Repeat once for the first element, twice for
//! the second element, three times for the third element, and so on. So, if the
//! third element of the output list is being calculated, repeating the values
//! would produce: `0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1`.
//!
//! When applying the pattern, skip the very first value exactly once. (In other
//! words, offset the whole pattern left by one.) So, for the second element of
//! the output list, the actual pattern used would be: `0, 1, 1, 0, 0, -1, -1,
//! 0, 0, 1, 1, 0, 0, -1, -1, …`
//!
//! After using this process to calculate each element of the output list, the
//! phase is complete, and the output list of this phase is used as the new
//! input list for the next phase, if any.
//!
//! Given the input signal 12345678, below are four phases of FFT. Within each
//! phase, each output digit is calculated on a single line with the result at
//! the far right; each multiplication operation shows the input digit on the
//! left and the pattern value on the right:
//!
//! Input signal: 12345678
//!
//! ```text
//! 1*1  + 2*0  + 3*-1 + 4*0  + 5*1  + 6*0  + 7*-1 + 8*0  = 4
//! 1*0  + 2*1  + 3*1  + 4*0  + 5*0  + 6*-1 + 7*-1 + 8*0  = 8
//! 1*0  + 2*0  + 3*1  + 4*1  + 5*1  + 6*0  + 7*0  + 8*0  = 2
//! 1*0  + 2*0  + 3*0  + 4*1  + 5*1  + 6*1  + 7*1  + 8*0  = 2
//! 1*0  + 2*0  + 3*0  + 4*0  + 5*1  + 6*1  + 7*1  + 8*1  = 6
//! 1*0  + 2*0  + 3*0  + 4*0  + 5*0  + 6*1  + 7*1  + 8*1  = 1
//! 1*0  + 2*0  + 3*0  + 4*0  + 5*0  + 6*0  + 7*1  + 8*1  = 5
//! 1*0  + 2*0  + 3*0  + 4*0  + 5*0  + 6*0  + 7*0  + 8*1  = 8
//! ```
//!
//! After 1 phase: 48226158
//!
//! ```text
//! 4*1  + 8*0  + 2*-1 + 2*0  + 6*1  + 1*0  + 5*-1 + 8*0  = 3
//! 4*0  + 8*1  + 2*1  + 2*0  + 6*0  + 1*-1 + 5*-1 + 8*0  = 4
//! 4*0  + 8*0  + 2*1  + 2*1  + 6*1  + 1*0  + 5*0  + 8*0  = 0
//! 4*0  + 8*0  + 2*0  + 2*1  + 6*1  + 1*1  + 5*1  + 8*0  = 4
//! 4*0  + 8*0  + 2*0  + 2*0  + 6*1  + 1*1  + 5*1  + 8*1  = 0
//! 4*0  + 8*0  + 2*0  + 2*0  + 6*0  + 1*1  + 5*1  + 8*1  = 4
//! 4*0  + 8*0  + 2*0  + 2*0  + 6*0  + 1*0  + 5*1  + 8*1  = 3
//! 4*0  + 8*0  + 2*0  + 2*0  + 6*0  + 1*0  + 5*0  + 8*1  = 8
//! ```
//!
//! After 2 phases: 34040438
//!
//! ```text
//! 3*1  + 4*0  + 0*-1 + 4*0  + 0*1  + 4*0  + 3*-1 + 8*0  = 0
//! 3*0  + 4*1  + 0*1  + 4*0  + 0*0  + 4*-1 + 3*-1 + 8*0  = 3
//! 3*0  + 4*0  + 0*1  + 4*1  + 0*1  + 4*0  + 3*0  + 8*0  = 4
//! 3*0  + 4*0  + 0*0  + 4*1  + 0*1  + 4*1  + 3*1  + 8*0  = 1
//! 3*0  + 4*0  + 0*0  + 4*0  + 0*1  + 4*1  + 3*1  + 8*1  = 5
//! 3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*1  + 3*1  + 8*1  = 5
//! 3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*0  + 3*1  + 8*1  = 1
//! 3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*0  + 3*0  + 8*1  = 8
//! ```
//!
//! After 3 phases: 03415518
//!
//! ```text
//! 0*1  + 3*0  + 4*-1 + 1*0  + 5*1  + 5*0  + 1*-1 + 8*0  = 0
//! 0*0  + 3*1  + 4*1  + 1*0  + 5*0  + 5*-1 + 1*-1 + 8*0  = 1
//! 0*0  + 3*0  + 4*1  + 1*1  + 5*1  + 5*0  + 1*0  + 8*0  = 0
//! 0*0  + 3*0  + 4*0  + 1*1  + 5*1  + 5*1  + 1*1  + 8*0  = 2
//! 0*0  + 3*0  + 4*0  + 1*0  + 5*1  + 5*1  + 1*1  + 8*1  = 9
//! 0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*1  + 1*1  + 8*1  = 4
//! 0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*0  + 1*1  + 8*1  = 9
//! 0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*0  + 1*0  + 8*1  = 8
//! ```
//!
//! After 4 phases: 01029498
//!
//! Here are the first eight digits of the final output list after 100 phases
//! for some larger inputs:
//!
//! * 80871224585914546619083218645595 becomes 24176176.
//! * 19617804207202209144916044189917 becomes 73745418.
//! * 69317163492948606335995924319873 becomes 52432133.
//!
//! After 100 phases of FFT, what are the first eight digits in the final output
//! list?
//!
//! ## Part Two
//!
//! Now that your FFT is working, you can decode the real signal.
//!
//! The real signal is your puzzle input repeated 10000 times. Treat this new
//! signal as a single input list. Patterns are still calculated as before, and
//! 100 phases of FFT are still applied.
//!
//! The first seven digits of your initial input signal also represent the
//! message offset. The message offset is the location of the eight-digit
//! message in the final output list. Specifically, the message offset indicates
//! the number of digits to skip before reading the eight-digit message. For
//! example, if the first seven digits of your initial input signal were
//! 1234567, the eight-digit message would be the eight digits after skipping
//! 1,234,567 digits of the final output list. Or, if the message offset were 7
//! and your final output list were 98765432109876543210, the eight-digit
//! message would be 21098765. (Of course, your real message offset will be a
//! seven-digit number, not a one-digit number like 7.)
//!
//! Here is the eight-digit message in the final output list after 100 phases.
//! The message offset given in each input has been highlighted. (Note that the
//! inputs given below are repeated 10000 times to find the actual starting
//! input lists.)
//!
//! * 03036732577212944063491565474664 becomes 84462026.
//! * 02935109699940807407585447034323 becomes 78725270.
//! * 03081770884921959731165446850517 becomes 53553731.
//!
//! After repeating your input signal 10000 times and running 100 phases of FFT,
//! what is the eight-digit message embedded in the final output list?

use anyhow::Result;

pub const PUZZLE_INPUT: &str = include_str!("../inputs/input-16");

pub fn parse_input(data: &str) -> Vec<u8> {
    data.trim()
        .chars()
        .map(|c| c.to_digit(10).map(|x| x as u8).expect("a digit"))
        .collect::<Vec<u8>>()
}

pub struct FftIter {
    data: Vec<u8>,
    sequence: &'static [i8],
    buffer: Vec<u8>,
    iterations: usize,
}

impl FftIter {
    pub fn new(data: Vec<u8>, sequence: &'static [i8]) -> Self {
        Self {
            buffer: vec![0; data.len()],
            data,
            sequence,
            iterations: 0,
        }
    }

    pub fn current(&self) -> &[u8] {
        &self.data
    }
}

impl Iterator for FftIter {
    type Item = ();
    fn next(&mut self) -> Option<()> {
        log::info!("Iteration {}", self.iterations);
        fft(&self.data, &self.sequence, &mut self.buffer);
        std::mem::swap(&mut self.data, &mut self.buffer);
        self.iterations += 1;
        Some(())
    }
}

pub const BASIC_SEQUENCE: &[i8] = &[0, 1, 0, -1];

fn fft(data: &[u8], sequence: &[i8], output: &mut [u8]) {
    for (step, i) in output.iter_mut().enumerate() {
        *i = fft_step(data, sequence, step);
    }
}

fn fft_step(data: &[u8], sequence: &[i8], step: usize) -> u8 {
    let val: i32 = data
        .iter()
        .copied()
        .zip(
            sequence
                .iter()
                .copied()
                .flat_map(move |i| std::iter::repeat(i).take(step + 1))
                .cycle()
                .skip(1),
        )
        .map(|(d, s)| d as i32 * s as i32)
        .sum();
    (val.abs() % 10) as u8
}

pub fn calc_offset(data: &[u8]) -> usize {
    data[..7]
        .iter()
        .copied()
        .fold(0, |s, c| s * 10 + c as usize) as usize
}

pub fn short_cut_high_offset(data: &[u8], repeats: usize, offset: usize, rounds: usize) -> [u8; 8] {
    let cycle_len = data.len() * repeats;
    assert!(offset < cycle_len);
    assert!(offset >= cycle_len / 2);

    let mut v: Vec<_> = data
        .iter()
        .cycle()
        .take(repeats * data.len())
        .skip(offset)
        .copied()
        .collect();

    (0..rounds).for_each(|_| {
        let mut v_sum: u32 = v.iter().map(|&x| x as u32).sum();
        v.iter_mut().for_each(|x| {
            let tmp = v_sum;
            v_sum -= *x as u32;
            *x = (tmp % 10) as u8;
        })
    });
    let mut result = [0; 8];
    result[..].copy_from_slice(&v[..8]);
    result
}

pub fn run() -> Result<()> {
    let data = parse_input(PUZZLE_INPUT);

    let mut iter = FftIter::new(data.clone(), BASIC_SEQUENCE);
    iter.nth(99).unwrap();
    let fft_100 = iter.current();

    println!("Fast-fourier transform at 100: {:?}", &fft_100[..8]);

    let offset = calc_offset(&data);
    let result = short_cut_high_offset(&data, 10_000, offset, 100);

    println!(
        "Fast-fourier transform at 10000 with offset {}: {:?}",
        offset,
        &result[..],
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    const EXAMPLE_1: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];

    #[test]
    fn fft_step_1_1() {
        let expected = 4;
        let actual = super::fft_step(EXAMPLE_1, super::BASIC_SEQUENCE, 0);

        assert_eq!(actual, expected);
    }
    #[test]
    fn fft_step_1() {
        let expected = &[4, 8, 2, 2, 6, 1, 5, 8];
        let mut actual = [0; 8];
        super::fft(EXAMPLE_1, super::BASIC_SEQUENCE, &mut actual[..]);

        assert_eq!(&actual, expected);
    }

    #[test]
    fn fft_iter_1() {
        let expected = &[4, 8, 2, 2, 6, 1, 5, 8];
        let mut iter = super::FftIter::new(Vec::from(EXAMPLE_1), super::BASIC_SEQUENCE);
        iter.next().unwrap();

        assert_eq!(&iter.data[..], expected);
    }

    #[test]
    fn fft_iter_2() {
        let expected = &[3, 4, 0, 4, 0, 4, 3, 8];
        let mut iter = super::FftIter::new(Vec::from(EXAMPLE_1), super::BASIC_SEQUENCE);
        iter.nth(1).unwrap();

        assert_eq!(&iter.data[..], expected);
    }

    #[test]
    fn fft_iter_3() {
        let expected = &[0, 3, 4, 1, 5, 5, 1, 8];
        let mut iter = super::FftIter::new(Vec::from(EXAMPLE_1), super::BASIC_SEQUENCE);
        iter.nth(2).unwrap();

        assert_eq!(&iter.data[..], expected);
    }

    #[test]
    fn fft_iter_4() {
        let expected = &[0, 1, 0, 2, 9, 4, 9, 8];
        let mut iter = super::FftIter::new(Vec::from(EXAMPLE_1), super::BASIC_SEQUENCE);
        iter.nth(3).unwrap();

        assert_eq!(&iter.data[..], expected);
    }

    #[test]
    fn fft_example_2_iter_100() {
        const EXAMPLE: &str = "80871224585914546619083218645595";
        let data = super::parse_input(EXAMPLE);
        let expected = &[2, 4, 1, 7, 6, 1, 7, 6];
        let mut iter = super::FftIter::new(data, super::BASIC_SEQUENCE);
        iter.nth(99).unwrap();

        assert_eq!(&iter.current()[..8], expected);
    }

    #[test]
    fn fft_example_3_iter_100() {
        const EXAMPLE: &str = "19617804207202209144916044189917";
        let data = super::parse_input(EXAMPLE);
        let expected = &[7, 3, 7, 4, 5, 4, 1, 8];
        let mut iter = super::FftIter::new(data, super::BASIC_SEQUENCE);
        iter.nth(99).unwrap();

        assert_eq!(&iter.current()[..8], expected);
    }

    #[test]
    fn fft_example_4_iter_100() {
        const EXAMPLE: &str = "69317163492948606335995924319873";
        let data = super::parse_input(EXAMPLE);
        let expected = &[5, 2, 4, 3, 2, 1, 3, 3];
        let mut iter = super::FftIter::new(data, super::BASIC_SEQUENCE);
        iter.nth(99).unwrap();

        assert_eq!(&iter.current()[..8], expected);
    }

    #[test]
    fn fft_fast_example_2_iter_100() {
        const EXAMPLE: &str = "03036732577212944063491565474664";
        let data = super::parse_input(EXAMPLE);
        let offset = super::calc_offset(&data);
        let expected = &[8, 4, 4, 6, 2, 0, 2, 6];

        let result = super::short_cut_high_offset(&data, 10_000, offset, 100);

        assert_eq!(&result[..], expected);
    }

    #[test]
    fn fft_fast_example_3_iter_100() {
        const EXAMPLE: &str = "02935109699940807407585447034323";
        let data = super::parse_input(EXAMPLE);
        let offset = super::calc_offset(&data);
        let expected = &[7, 8, 7, 2, 5, 2, 7, 0];

        let result = super::short_cut_high_offset(&data, 10_000, offset, 100);

        assert_eq!(&result[..], expected);
    }

    #[test]
    fn fft_fast_example_4_iter_100() {
        const EXAMPLE: &str = "03081770884921959731165446850517";
        let data = super::parse_input(EXAMPLE);
        let offset = super::calc_offset(&data);
        let expected = &[5, 3, 5, 5, 3, 7, 3, 1];

        let result = super::short_cut_high_offset(&data, 10_000, offset, 100);

        assert_eq!(&result[..], expected);
    }
}
