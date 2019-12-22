//! # Day 22: Slam Shuffle
//!
//! There isn't much to do while you wait for the droids to repair your ship. At
//! least you're drifting in the right direction. You decide to practice a new
//! card shuffle you've been working on.
//!
//! Digging through the ship's storage, you find a deck of space cards! Just
//! like any deck of space cards, there are 10007 cards in the deck numbered `0`
//! through `10006`. The deck must be new - they're still in factory order, with
//! `0` on the top, then `1`, then `2`, and so on, all the way through to
//! `10006` on the bottom.
//!
//! You've been practicing three different techniques that you use while
//! shuffling. Suppose you have a deck of only 10 cards (numbered `0` through
//! `9`):
//!
//! To `deal into new stack`, create a new stack of cards by dealing the top
//! card of the deck onto the top of the new stack repeatedly until you run out
//! of cards:
//!
//! ```text
//! Top          Bottom
//! 0 1 2 3 4 5 6 7 8 9   Your deck
//!                       New stack
//!
//!   1 2 3 4 5 6 7 8 9   Your deck
//!                   0   New stack
//!
//!     2 3 4 5 6 7 8 9   Your deck
//!                 1 0   New stack
//!
//!       3 4 5 6 7 8 9   Your deck
//!               2 1 0   New stack
//!
//! Several steps later...
//!
//!                   9   Your deck
//!   8 7 6 5 4 3 2 1 0   New stack
//!
//!                       Your deck
//! 9 8 7 6 5 4 3 2 1 0   New stack
//! ```
//!
//! Finally, pick up the new stack you've just created and use it as the deck
//! for the next technique.
//!
//! To `cut N` cards, take the top `N` cards off the top of the deck and move
//! them as a single unit to the bottom of the deck, retaining their order. For
//! example, to `cut 3`:
//!
//! ```text
//! Top          Bottom
//! 0 1 2 3 4 5 6 7 8 9   Your deck
//!
//!       3 4 5 6 7 8 9   Your deck
//! 0 1 2                 Cut cards
//!
//! 3 4 5 6 7 8 9         Your deck
//!               0 1 2   Cut cards
//!
//! 3 4 5 6 7 8 9 0 1 2   Your deck
//! ```
//!
//! You've also been getting pretty good at a version of this technique where
//! `N` is negative! In that case, cut (the absolute value of) `N` cards from
//! the bottom of the deck onto the top. For example, to `cut -4`:
//!
//! ```text
//! Top          Bottom
//! 0 1 2 3 4 5 6 7 8 9   Your deck
//!
//! 0 1 2 3 4 5           Your deck
//!             6 7 8 9   Cut cards
//!
//!         0 1 2 3 4 5   Your deck
//! 6 7 8 9               Cut cards
//!
//! 6 7 8 9 0 1 2 3 4 5   Your deck
//! ```
//!
//! To `deal with increment N`, start by clearing enough space on your table to
//! lay out all of the cards individually in a long line. Deal the top card into
//! the leftmost position. Then, move `N` positions to the right and deal the
//! next card there. If you would move into a position past the end of the space
//! on your table, wrap around and keep counting from the leftmost card again.
//! Continue this process until you run out of cards.
//!
//! For example, to `deal with increment 3`:
//!
//! ```text
//! 
//! 0 1 2 3 4 5 6 7 8 9   Your deck
//! . . . . . . . . . .   Space on table
//! ^                     Current position
//!
//! Deal the top card to the current position:
//!
//!   1 2 3 4 5 6 7 8 9   Your deck
//! 0 . . . . . . . . .   Space on table
//! ^                     Current position
//!
//! Move the current position right 3:
//!
//!   1 2 3 4 5 6 7 8 9   Your deck
//! 0 . . . . . . . . .   Space on table
//!       ^               Current position
//!
//! Deal the top card:
//!
//!     2 3 4 5 6 7 8 9   Your deck
//! 0 . . 1 . . . . . .   Space on table
//!       ^               Current position
//!
//! Move right 3 and deal:
//!
//!       3 4 5 6 7 8 9   Your deck
//! 0 . . 1 . . 2 . . .   Space on table
//!             ^         Current position
//!
//! Move right 3 and deal:
//!
//!         4 5 6 7 8 9   Your deck
//! 0 . . 1 . . 2 . . 3   Space on table
//!                   ^   Current position
//!
//! Move right 3, wrapping around, and deal:
//!
//!           5 6 7 8 9   Your deck
//! 0 . 4 1 . . 2 . . 3   Space on table
//!     ^                 Current position
//!
//! And so on:
//!
//! 0 7 4 1 8 5 2 9 6 3   Space on table
//! ```
//!
//! Positions on the table which already contain cards are still counted;
//! they're not skipped. Of course, this technique is carefully designed so it
//! will never put two cards in the same position or leave a position empty.
//!
//! Finally, collect the cards on the table so that the leftmost card ends up at
//! the top of your deck, the card to its right ends up just below the top card,
//! and so on, until the rightmost card ends up at the bottom of the deck.
//!
//! The complete shuffle process (your puzzle input) consists of applying many
//! of these techniques. Here are some examples that combine techniques; they
//! all start with a factory order deck of 10 cards:
//!
//! ```text
//! deal with increment 7
//! deal into new stack
//! deal into new stack
//! Result: 0 3 6 9 2 5 8 1 4 7
//! ```
//!
//! ```text
//! cut 6
//! deal with increment 7
//! deal into new stack
//! Result: 3 0 7 4 1 8 5 2 9 6
//! ```
//!
//! ```text
//! deal with increment 7
//! deal with increment 9
//! cut -2
//! Result: 6 3 0 7 4 1 8 5 2 9
//! ```
//!
//! ```text
//! deal into new stack
//! cut -2
//! deal with increment 7
//! cut 8
//! cut -4
//! deal with increment 7
//! cut 3
//! deal with increment 9
//! deal with increment 3
//! cut -1
//! Result: 9 2 5 8 1 4 7 0 3 6
//! ```
//!
//! Positions within the deck count from `0` at the top, then `1` for the card
//! immediately below the top card, and so on to the bottom. (That is, cards
//! start in the position matching their number.)
//!
//! After shuffling your factory order deck of 10007 cards, what is the position
//! of card `2019`?

use anyhow::{anyhow, Error, Result};
use std::collections::VecDeque;

const PUZZLE_INPUT: &str = include_str!("../inputs/input-22");

enum Technique {
    NewStack,
    Increment(usize),
    Cut(isize),
}

impl std::str::FromStr for Technique {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed == "deal into new stack" {
            Ok(Self::NewStack)
        } else if trimmed.starts_with("cut") {
            let qty = trimmed
                .rsplit(' ')
                .next()
                .ok_or_else(|| anyhow!("bad cut instruction: {}", trimmed))?;
            Ok(Self::Cut(qty.parse()?))
        } else if trimmed.starts_with("deal with increment") {
            let qty = trimmed
                .rsplit(' ')
                .next()
                .ok_or_else(|| anyhow!("bad increment instruction: {}", trimmed))?;
            Ok(Self::Increment(qty.parse()?))
        } else {
            Err(anyhow!("Invalid instruction: {}", trimmed))
        }
    }
}

fn parse_input(s: &str) -> Result<Vec<Technique>> {
    use std::io::{BufRead, Cursor};

    Cursor::new(s)
        .lines()
        .filter_map(|r| match r {
            Ok(l) => {
                let trimmed = l.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.parse().map_err(Error::from))
                }
            }
            Err(e) => Some(Err(Error::from(e))),
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Deck {
    cards: VecDeque<Card>,
}

impl Deck {
    fn new(size: usize) -> Self {
        std::iter::successors(Some(0u16), |c| Some(c + 1))
            .take(size)
            .collect()
    }

    fn cards(&self) -> impl Iterator<Item = Card> + '_ {
        self.cards.iter().copied()
    }

    fn shuffle(&mut self, t: impl IntoIterator<Item = Technique>) {
        t.into_iter().for_each(|t| self.apply_technique(t));
    }

    fn apply_technique(&mut self, t: Technique) {
        match t {
            Technique::NewStack => self.new_stack(),
            Technique::Cut(c) => self.cut(c),
            Technique::Increment(i) => self.deal_incr(i),
        }
    }

    fn new_stack(&mut self) {
        self.cards = self.cards.iter().copied().rev().collect();
    }

    fn cut(&mut self, point: isize) {
        match point.cmp(&0) {
            std::cmp::Ordering::Less => self.cards.rotate_right(-point as usize),
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => self.cards.rotate_left(point as usize),
        }
    }

    fn deal_incr(&mut self, incr: usize) {
        use num_integer::Integer;
        // Increment must be relatively prime
        debug_assert_eq!(self.cards.len().gcd(&incr), 1);

        let new_cards = self.cards.clone();
        let cards = std::mem::replace(&mut self.cards, new_cards);

        let mut i = 0;
        for c in cards {
            self.cards[i] = c;
            i = (i + incr) % self.cards.len();
        }
    }
}

impl<T> std::iter::FromIterator<T> for Deck
where
    Card: From<T>,
{
    fn from_iter<I: IntoIterator<Item = T>>(i: I) -> Self {
        Self {
            cards: i.into_iter().map(Card::from).collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Card(u16);

impl From<u16> for Card {
    fn from(c: u16) -> Self {
        Self(c)
    }
}

pub fn run() -> Result<()> {
    let techniques = parse_input(PUZZLE_INPUT)?;
    let mut deck = Deck::new(10007);

    deck.shuffle(techniques);
    let pos = deck.cards().position(|c| c == Card(2019)).unwrap();
    println!("Position of card 2019 after shuffling: {}", pos);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_input, Deck};
    use pretty_assertions::assert_eq;

    fn test_deck() -> Deck {
        Deck::new(10)
    }

    #[test]
    fn new_stack() {
        let mut deck = test_deck();

        deck.new_stack();
        let expected: Deck = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0].into_iter().collect();

        assert_eq!(deck, expected);
    }

    #[test]
    fn cut_3() {
        let mut deck = test_deck();

        deck.cut(3);
        let expected: Deck = vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2].into_iter().collect();

        assert_eq!(deck, expected);
    }

    #[test]
    fn cut_neg4() {
        let mut deck = test_deck();

        deck.cut(-4);
        let expected: Deck = vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5].into_iter().collect();

        assert_eq!(deck, expected);
    }

    #[test]
    fn deal_incr_3() {
        let mut deck = test_deck();

        deck.deal_incr(3);
        let expected: Deck = vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3].into_iter().collect();

        assert_eq!(deck, expected);
    }

    #[test]
    fn deal_incr_1() {
        let mut deck = test_deck();

        deck.deal_incr(1);
        let expected: Deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();

        assert_eq!(deck, expected);
    }

    #[test]
    fn deal_incr_9() {
        let mut deck = test_deck();

        deck.deal_incr(9);
        let expected: Deck = vec![0, 9, 8, 7, 6, 5, 4, 3, 2, 1].into_iter().collect();

        assert_eq!(deck, expected);
    }

    #[test]
    fn deal_incr_7() {
        let mut deck = test_deck();

        deck.deal_incr(7);
        let expected: Deck = vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7].into_iter().collect();

        assert_eq!(deck, expected);
    }

    #[test]
    fn example_1() {
        const EXAMPLE: &str = "\
                               deal with increment 7\n\
                               deal into new stack\n\
                               deal into new stack";

        let techniques = parse_input(EXAMPLE).unwrap();

        let mut deck = test_deck();
        deck.shuffle(techniques);

        let expected: Deck = vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7].into_iter().collect();

        assert_eq!(deck, expected);
    }

    #[test]
    fn example_2() {
        const EXAMPLE: &str = "\
                               cut 6\n\
                               deal with increment 7\n\
                               deal into new stack";

        let techniques = parse_input(EXAMPLE).unwrap();

        let mut deck = test_deck();
        deck.shuffle(techniques);

        let expected: Deck = vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6].into_iter().collect();

        assert_eq!(deck, expected);
    }

    #[test]
    fn example_3() {
        const EXAMPLE: &str = "\
                               deal with increment 7\n\
                               deal with increment 9\n\
                               cut -2";

        let techniques = parse_input(EXAMPLE).unwrap();

        let mut deck = test_deck();
        deck.shuffle(techniques);

        let expected: Deck = vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9].into_iter().collect();

        assert_eq!(deck, expected);
    }

    #[test]
    fn example_4() {
        const EXAMPLE: &str = "\
                               deal into new stack\n\
                               cut -2\n\
                               deal with increment 7\n\
                               cut 8\n\
                               cut -4\n\
                               deal with increment 7\n\
                               cut 3\n\
                               deal with increment 9\n\
                               deal with increment 3\n\
                               cut -1";

        let techniques = parse_input(EXAMPLE).unwrap();

        let mut deck = test_deck();
        deck.shuffle(techniques);

        let expected: Deck = vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6].into_iter().collect();

        assert_eq!(deck, expected);
    }
}
