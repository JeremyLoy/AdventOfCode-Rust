use crate::_2023::_07::Card::{
    Ace, Eight, Five, Four, Jack, Joker, King, Nine, Queen, Seven, Six, Ten, Three, Two,
};
use crate::_2023::_07::HandType::{
    FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair,
};
use itertools::Itertools;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::error::Error;

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

#[derive(Ord, PartialOrd, Eq, PartialEq)]
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

        let non_joker_cards: Vec<i32> = count_by_card.values().copied().collect();

        let max_non_joker_count = *non_joker_cards.iter().max().unwrap_or(&0);

        let three_of_a_kind = non_joker_cards.contains(&3);
        let two_pair = non_joker_cards.iter().filter(|i| **i == 2).count() == 2;
        let one_pair = non_joker_cards.contains(&2);

        if joker_count + max_non_joker_count == 5 {
            return FiveOfAKind;
        }
        if joker_count + max_non_joker_count == 4 {
            return FourOfAKind;
        }
        if (three_of_a_kind && one_pair) || (two_pair && joker_count == 1) {
            return FullHouse;
        }
        if max_non_joker_count + joker_count == 3 {
            return ThreeOfAKind;
        }
        if two_pair || (one_pair && joker_count > 0) {
            return TwoPair;
        }
        if one_pair || joker_count > 0 {
            return OnePair;
        }
        HighCard
    }
}

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(s: char, include_joker: bool) -> Option<Self> {
        Some(match s {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
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
