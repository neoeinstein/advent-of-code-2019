//! # Day 8: Space Image Format
//!
//! The Elves' spirits are lifted when they realize you have an opportunity to
//! reboot one of their Mars rovers, and so they are curious if you would spend
//! a brief sojourn on Mars. You land your ship near the rover.
//!
//! When you reach the rover, you discover that it's already in the process of
//! rebooting! It's just waiting for someone to enter a BIOS password. The Elf
//! responsible for the rover takes a picture of the password (your puzzle
//! input) and sends it to you via the Digital Sending Network.
//!
//! Unfortunately, images sent via the Digital Sending Network aren't encoded
//! with any normal encoding; instead, they're encoded in a special Space Image
//! Format. None of the Elves seem to remember why this is the case. They send
//! you the instructions to decode it.
//!
//! Images are sent as a series of digits that each represent the color of a
//! single pixel. The digits fill each row of the image left-to-right, then move
//! downward to the next row, filling rows top-to-bottom until every pixel of
//! the image is filled.
//!
//! Each image actually consists of a series of identically-sized layers that
//! are filled in this way. So, the first digit corresponds to the top-left
//! pixel of the first layer, the second digit corresponds to the pixel to the
//! right of that on the same layer, and so on until the last digit, which
//! corresponds to the bottom-right pixel of the last layer.
//!
//! For example, given an image 3 pixels wide and 2 pixels tall, the image data
//! `123456789012` corresponds to the following image layers:
//!
//! ```text
//! Layer 1: 123
//!          456
//!
//! Layer 2: 789
//!          012
//! ```
//!
//! The image you received is 25 pixels wide and 6 pixels tall.
//!
//! To make sure the image wasn't corrupted during transmission, the Elves would
//! like you to find the layer that contains the fewest 0 digits. On that layer,
//! what is the number of 1 digits multiplied by the number of 2 digits?

use advent_of_code_2019::get_input_reader;
use anyhow::Result;
use std::{
    borrow::Cow,
    fmt,
    io::Read,
    iter::{DoubleEndedIterator, ExactSizeIterator},
};

#[derive(Debug, PartialEq, Eq)]
struct Image {
    dimensions: (usize, usize),
    data: Vec<u8>,
}

impl Image {
    const fn layer_size(&self) -> usize {
        self.dimensions.0 * self.dimensions.1
    }

    fn from_bytes(bytes: impl Into<Vec<u8>>, dimensions: (usize, usize)) -> Self {
        let data = bytes.into();
        assert_eq!(data.len() % (dimensions.0 * dimensions.1), 0);

        Self { dimensions, data }
    }

    fn layer_at(&self, start: usize) -> Layer {
        let layer = &self.data[start..start + self.layer_size()];
        debug_assert_eq!(layer.len(), self.layer_size());
        Layer {
            dimensions: self.dimensions,
            data: Cow::Borrowed(layer),
        }
    }

    fn layer_count(&self) -> usize {
        self.data.len() / self.layer_size()
    }

    fn layers(&self) -> LayerIterator {
        LayerIterator {
            lower: 0,
            upper: self.layer_count(),
            image: self,
        }
    }

    fn layer(&self, idx: usize) -> Layer {
        let start = idx * self.layer_size();
        self.layer_at(start)
    }

    fn resolve(&self) -> Layer {
        let mut layers = self.layers();
        let foreground = layers.next().unwrap();

        let image = layers.fold(foreground, |mask, mut back| {
            println!("Current:\n{}", mask);
            back.apply_mask(mask);
            // std::thread::sleep(std::time::Duration::from_millis(500));
            back
        });

        image
    }

    fn resolve_back(&self) -> Layer {
        let mut layers = self.layers().rev();
        let background = layers.next().unwrap();

        let image = layers.fold(background, |mut back, mask| {
            println!("Current:\n{}", back);
            back.apply_mask(mask);
            // std::thread::sleep(std::time::Duration::from_millis(500));
            back
        });

        image
    }
}

impl<'a> fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let image = self.resolve();

        image.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Layer<'a> {
    dimensions: (usize, usize),
    data: Cow<'a, [u8]>,
}

impl<'a> Layer<'a> {
    fn count_chars(&self) -> [u32; 3] {
        let mut counts = [0; 3];
        for b in self.data.as_ref() {
            match counts.get_mut((b - b'0') as usize) {
                Some(bucket) => *bucket += 1,
                None => (),
            }
        }
        counts
    }

    fn apply_mask(&mut self, mask: Layer<'a>) {
        assert_eq!(self.dimensions, mask.dimensions);
        for (b, m) in self.data.to_mut().iter_mut().zip(mask.data.iter()) {
            match m {
                b'2' => (),
                _ => *b = *m,
            }
        }
    }
}

impl<'a> fmt::Display for Layer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, b) in self.data.iter().enumerate() {
            if i > 0 && (i % self.dimensions.0) == 0 {
                writeln!(f, "")?;
            }

            let c = match b {
                b'0' => "█",
                b'1' => "·",
                b'2' => "░",
                _ => unreachable!(),
            };
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct LayerIterator<'a> {
    lower: usize,
    upper: usize,
    image: &'a Image,
}

impl<'a> Iterator for LayerIterator<'a> {
    type Item = Layer<'a>;
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.image.layer_count();
        (size, Some(size))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.lower == self.upper {
            return None;
        }
        let layer = self.image.layer(self.lower);
        self.lower += 1;
        Some(layer)
    }
}

impl<'a> DoubleEndedIterator for LayerIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.lower == self.upper {
            return None;
        }
        let layer = self.image.layer(self.upper - 1);
        self.upper -= 1;
        Some(layer)
    }
}

impl<'a> ExactSizeIterator for LayerIterator<'a> {}

fn main() -> Result<()> {
    let mut input = String::new();
    get_input_reader().read_to_string(&mut input)?;

    const LAYER_DIMENSIONS: (usize, usize) = (25, 6);

    let image = Image::from_bytes(input.trim(), LAYER_DIMENSIONS);

    let mut best_layer = [u32::max_value(); 3];
    let mut best_layer_num = 0;
    for layer in image.layers().enumerate() {
        let current_count = layer.1.count_chars();
        if current_count[0] < best_layer[0] {
            best_layer = current_count;
            best_layer_num = layer.0;
        }
    }

    println!(
        "Best layer was layer {}: {:?} => {}",
        best_layer_num,
        best_layer,
        best_layer[1] * best_layer[2]
    );

    for (i, layer) in image.layers().enumerate() {
        println!("Layer {}:\n{}", i, layer);
    }

    println!("Resolving… [from background forward]");
    let resolved = image.resolve_back();
    println!("Resolved:\n{}", resolved);

    println!("Resolving… [from foreground backward]");
    let resolved = image.resolve();
    println!("Resolved:\n{}", resolved);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Image;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    const TEST_IMAGE_1: &[u8] = b"123456789012";
    const LAYER_DIMENSIONS_1: (usize, usize) = (3, 2);

    #[test]
    fn part_1_example_layer_count() -> Result<()> {
        const EXPECTED: usize = 2;
        let image = Image::from_bytes(TEST_IMAGE_1, LAYER_DIMENSIONS_1);

        let actual = image.layer_count();

        assert_eq!(actual, EXPECTED);

        Ok(())
    }

    #[test]
    fn part_1_example_layer_0_char_count() -> Result<()> {
        const EXPECTED: [u32; 3] = [0, 1, 1];
        let image = Image::from_bytes(TEST_IMAGE_1, LAYER_DIMENSIONS_1);

        let actual = image.layer(0).count_chars();

        assert_eq!(actual, EXPECTED);

        Ok(())
    }

    #[test]
    fn part_1_example_layer_1_char_count() -> Result<()> {
        const EXPECTED: [u32; 3] = [1, 1, 1];
        let image = Image::from_bytes(TEST_IMAGE_1, LAYER_DIMENSIONS_1);

        let actual = image.layer(1).count_chars();

        assert_eq!(actual, EXPECTED);

        Ok(())
    }

    const TEST_IMAGE_2: &[u8] = b"0222112222120000";
    const LAYER_DIMENSIONS_2: (usize, usize) = (2, 2);

    #[test]
    fn part_2_example_layer_count() -> Result<()> {
        const EXPECTED: usize = 4;
        let image = Image::from_bytes(TEST_IMAGE_2, LAYER_DIMENSIONS_2);

        let actual = image.layer_count();

        assert_eq!(actual, EXPECTED);

        Ok(())
    }

    #[test]
    fn part_2_example_resolve() -> Result<()> {
        const EXPECTED: &[u8] = b"0110";
        const EXPECTED_LAYER: super::Layer<'static> = super::Layer {
            dimensions: LAYER_DIMENSIONS_2,
            data: std::borrow::Cow::Borrowed(EXPECTED),
        };
        let image = Image::from_bytes(TEST_IMAGE_2, LAYER_DIMENSIONS_2);

        for (i, layer) in image.layers().enumerate() {
            println!("Layer {}:\n{}", i, layer);
        }

        let actual = image.resolve();

        println!("Expected:\n{}", EXPECTED_LAYER);
        println!("Result:\n{}", actual);

        assert_eq!(actual, EXPECTED_LAYER);

        Ok(())
    }

    #[test]
    fn part_2_example_resolve_cmp() -> Result<()> {
        let image = Image::from_bytes(TEST_IMAGE_2, LAYER_DIMENSIONS_2);

        let left = image.resolve();
        let right = image.resolve_back();

        println!("resolve():\n{}", left);
        println!("resolve_back():\n{}", right);

        assert_eq!(left, right);

        Ok(())
    }
}
