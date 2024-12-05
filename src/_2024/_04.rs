use anyhow::Result;
use std::collections::HashMap;
use std::ops::{Add, Mul};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Coordinate(i32, i32);

impl Mul<i32> for Coordinate {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Coordinate) -> Self::Output {
        Coordinate(self.0 + other.0, self.1 + other.1)
    }
}

pub const NORTH: Coordinate = Coordinate(0, 1);
pub const SOUTH: Coordinate = Coordinate(0, -1);
pub const EAST: Coordinate = Coordinate(1, 0);
pub const WEST: Coordinate = Coordinate(-1, 0);
pub const NORTHEAST: Coordinate = Coordinate(1, 1);
pub const NORTHWEST: Coordinate = Coordinate(-1, 1);
pub const SOUTHEAST: Coordinate = Coordinate(1, -1);
pub const SOUTHWEST: Coordinate = Coordinate(-1, -1);

// Static array of direction coordinates for iteration
pub static DIRECTIONS: [Coordinate; 8] = [
    NORTH, SOUTH, EAST, WEST, NORTHEAST, NORTHWEST, SOUTHEAST, SOUTHWEST,
];

#[derive(Debug)]
pub struct WordSearch(HashMap<Coordinate, char>);

impl FromStr for WordSearch {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut word_search = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                word_search.insert(Coordinate(x as i32, y as i32), c);
            }
        }

        Ok(WordSearch(word_search))
    }
}

impl WordSearch {
    pub fn count_x_mas(self) -> usize {
        self.0
            .iter()
            .filter(|(_, c)| **c == 'X')
            .map(|(coord, _)| {
                let mut count = 0;
                for direction in &DIRECTIONS {
                    let m = match self.0.get(&(*coord + *direction)) {
                        Some(&'M') => true,
                        _ => continue,
                    };

                    let a = match self.0.get(&(*coord + (*direction * 2))) {
                        Some(&'A') => true,
                        _ => continue,
                    };

                    let s = match self.0.get(&(*coord + (*direction * 3))) {
                        Some(&'S') => true,
                        _ => continue,
                    };

                    if m && a && s {
                        count += 1;
                    }
                }
                count
            })
            .sum()
    }
    pub fn count_mas_x(self) -> usize {
        self.0
            .iter()
            .filter_map(|(coord, c)| {
                let a = *c == 'A';
                if !a {
                    return None;
                }
                let nw = *self.0.get(&(*coord + NORTHWEST))?;
                let sw = *self.0.get(&(*coord + SOUTHWEST))?;
                let ne = *self.0.get(&(*coord + NORTHEAST))?;
                let se = *self.0.get(&(*coord + SOUTHEAST))?;

                // a mas-x has exactly 2 m and 2 s, and the matching letters will never be diagnonal
                // from each other. This simple boolean logic handles checking for all 4 permutations
                // in one pass
                let corners = [nw, sw, ne, se];
                let s = corners.iter().filter(|c| **c == 'S').count();
                let m = corners.iter().filter(|c| **c == 'M').count();
                if (nw != se) && s == 2 && m == 2 {
                    Some(())
                } else {
                    None
                }
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    const INPUT: &str = include_str!("../../input/2024/04.txt");

    #[test]
    fn test_1_sample() {
        let word_search: WordSearch = SAMPLE.parse().unwrap();

        assert_eq!(word_search.count_x_mas(), 18);
    }

    #[test]
    fn test_1() {
        let word_search: WordSearch = INPUT.parse().unwrap();

        assert_eq!(word_search.count_x_mas(), 2_551);
    }

    #[test]
    fn test_2_sample() {
        let word_search: WordSearch = SAMPLE.parse().unwrap();

        assert_eq!(word_search.count_mas_x(), 9);
    }

    #[test]
    fn test_2() {
        let word_search: WordSearch = INPUT.parse().unwrap();

        assert_eq!(word_search.count_mas_x(), 1_985);
    }
}
