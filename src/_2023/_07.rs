use crate::_2023::_07::Card::{Ace, Jack, Joker, King, Number, Queen};
use crate::_2023::_07::Hand::{
    FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair,
};
use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Card {
    Joker,
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(s: char, include_joker: bool) -> Option<Self> {
        Some(match s {
            '2'..='9' => Number(s.to_digit(10).expect("was a digit")),
            'T' => Number(10),
            'J' => {
                if include_joker {
                    Joker
                } else {
                    Jack
                }
            }
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => return None,
        })
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Hand {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPair(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
}

impl Hand {
    pub fn parse_batch(
        i: impl Iterator<Item = String>,
        include_joker: bool,
    ) -> Option<Vec<(Hand, i32)>> {
        i.map(|s| Hand::from_str(&s, include_joker).ok())
            .sorted()
            .collect()
    }

    fn from_str(s: &str, include_joker: bool) -> Result<(Self, i32), Box<dyn Error>> {
        let (cards, bid) = s.trim().split_once(' ').ok_or("expected to split once")?;

        let cards: Vec<Card> = cards
            .chars()
            .map(|c| Card::from_char(c, include_joker))
            .collect::<Option<Vec<Card>>>()
            .ok_or("could not parse cards")?;

        let bid = bid.parse::<i32>()?;

        let mut count_by_card = HashMap::new();
        for card in cards.as_slice() {
            *count_by_card.entry(card).or_default() += 1;
        }

        let joker_count = count_by_card.remove(&Joker).unwrap_or(0);

        let mut sorted_counts: Vec<i32> = count_by_card.values().copied().sorted().rev().collect();
        if let Some(first) = sorted_counts.first_mut() {
            *first += joker_count;
        } else {
            sorted_counts.insert(0, joker_count);
        }

        let hand = match sorted_counts.as_slice() {
            [5, ..] => FiveOfAKind(cards),
            [4, ..] => FourOfAKind(cards),
            [3, 2, ..] => FullHouse(cards),
            [3, ..] => ThreeOfAKind(cards),
            [2, 2, ..] => TwoPair(cards),
            [2, ..] => OnePair(cards),
            _ => HighCard(cards),
        };

        Ok((hand, bid))
    }
}

pub fn total_winnings(hands: &[(Hand, i32)]) -> u64 {
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.1 as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::to_lines;
    use crate::input_parsing::Input::{Path, Raw};

    #[test]
    fn test_1_sample() {
        let input = Raw("\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
");
        let hands = Hand::parse_batch(to_lines(input), false).unwrap();

        assert_eq!(total_winnings(&hands), 6_440);
    }

    #[test]
    fn test_1() {
        let input = Path("input/2023/07.txt");

        let hands = Hand::parse_batch(to_lines(input), false).unwrap();

        assert_eq!(total_winnings(&hands), 250_254_244);
    }

    #[test]
    fn test_2_sample() {
        let input = Raw("\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
");

        let hands = Hand::parse_batch(to_lines(input), true).unwrap();

        assert_eq!(total_winnings(&hands), 5_905);
    }

    #[test]
    fn test_2() {
        let input = Path("input/2023/07.txt");

        let hands = Hand::parse_batch(to_lines(input), true).unwrap();

        assert_eq!(total_winnings(&hands), 250_087_440);
    }
}
