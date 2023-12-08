use crate::_2023::_07::Card::{Ace, Jack, Joker, King, Number, Queen};
use crate::_2023::_07::HandType::{
    FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair,
};
use itertools::Itertools;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: i32,
}

impl Hand {
    pub fn parse_batch(i: impl Iterator<Item = String>, include_joker: bool) -> Option<Vec<Hand>> {
        i.map(|s| Hand::from_str(&s, include_joker).ok())
            .sorted()
            .collect()
    }

    fn from_str(s: &str, include_joker: bool) -> Result<Self, Box<dyn Error>> {
        let (cards, bid) = s.trim().split_once(' ').ok_or("expected to split once")?;
        let cards: Vec<Card> = cards
            .chars()
            .map(|c| Card::from_char(c, include_joker))
            .collect::<Option<Vec<Card>>>()
            .ok_or("could not parse cards")?;
        let bid = bid.parse::<i32>()?;
        Ok(Hand {
            hand_type: HandType::from_cards(&cards),
            cards,
            bid,
        })
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_ordering = self.hand_type.cmp(&other.hand_type);
        if hand_type_ordering == Equal {
            return self.cards.cmp(&other.cards);
        }
        hand_type_ordering
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    pub fn from_cards(cards: &[Card]) -> HandType {
        let mut count_by_card = HashMap::new();
        for card in cards {
            *count_by_card.entry(card).or_insert(0) += 1;
        }

        let joker_count = count_by_card.remove(&Joker).unwrap_or(0);

        let mut sorted_counts: Vec<i32> = count_by_card.values().copied().sorted().rev().collect();
        if let Some(first) = sorted_counts.first_mut() {
            *first += joker_count;
        } else {
            sorted_counts.insert(0, joker_count);
        }

        match sorted_counts.as_slice() {
            [5, ..] => FiveOfAKind,
            [4, ..] => FourOfAKind,
            [3, 2, ..] => FullHouse,
            [3, ..] => ThreeOfAKind,
            [2, 2, ..] => TwoPair,
            [2, ..] => OnePair,
            _ => HighCard,
        }
    }
}

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

pub fn total_winnings(hands: &[Hand]) -> u64 {
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.bid as u64)
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
