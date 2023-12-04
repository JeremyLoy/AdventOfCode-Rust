use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

pub struct LottoCard {
    id: i32,
    winning_numbers: HashSet<i32>,
    owned_numbers: HashSet<i32>,
}

impl LottoCard {
    pub fn parse_batch(iterator: impl Iterator<Item = String>) -> Option<Vec<LottoCard>> {
        iterator.map(|s| s.parse::<LottoCard>().ok()).collect()
    }
    fn matching_cards(&self) -> i32 {
        self.winning_numbers
            .intersection(&self.owned_numbers)
            .count() as i32
    }
    pub fn score(&self) -> i32 {
        let winning_count = self.matching_cards() as u32;
        if winning_count == 0 {
            0
        } else {
            2i32.pow(winning_count - 1)
        }
    }
}

pub fn total_cards(cards: Vec<LottoCard>) -> i32 {
    // adding 1 here, and subtracting 1 from the sum to avoid having to adjust card ID and index access since arrays are 0 indexed
    let mut extra_multiplier: Vec<i32> = vec![1; cards.len() + 1];

    for card in &cards {
        let matching_count = card.matching_cards();
        for _ in 0..extra_multiplier[card.id as usize] {
            let range = card.id + 1..=(card.id + matching_count);
            for id in range {
                extra_multiplier[id as usize] += 1;
            }
        }
    }

    extra_multiplier.iter().sum::<i32>() - 1
}

impl FromStr for LottoCard {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = s.split_once(':').ok_or("no semicolon found")?;
        let id = id.split_whitespace().last().ok_or("no id found")?.parse()?;

        let (winning_numbers, owned_numbers) = rest.split_once('|').ok_or("no pipe")?;

        let winning_numbers = winning_numbers
            .split_whitespace()
            .flat_map(|i| i.trim().parse())
            .collect();
        let owned_numbers = owned_numbers
            .split_whitespace()
            .flat_map(|i| i.trim().parse())
            .collect();

        Ok(LottoCard {
            id,
            winning_numbers,
            owned_numbers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::to_lines;
    use crate::input_parsing::Input::{Path, Raw};

    #[test]
    fn test_1_sample() {
        let input = Raw("\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
");

        let cards = LottoCard::parse_batch(to_lines(input)).unwrap();

        assert_eq!(cards.iter().map(LottoCard::score).sum::<i32>(), 13);
    }

    #[test]
    fn test_1() {
        let input = Path("input/2023/04.txt");

        let cards = LottoCard::parse_batch(to_lines(input)).unwrap();

        assert_eq!(cards.iter().map(LottoCard::score).sum::<i32>(), 23_678);
    }

    #[test]
    fn test_2_sample() {
        let input = Raw("\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
");

        let cards = LottoCard::parse_batch(to_lines(input)).unwrap();

        assert_eq!(total_cards(cards), 30);
    }

    #[test]
    fn test_2() {
        let input = Path("input/2023/04.txt");

        let cards = LottoCard::parse_batch(to_lines(input)).unwrap();

        assert_eq!(total_cards(cards), 15_455_663);
    }
}
