use crate::_2023::_13::Reflection::Horizontal;
use std::str::FromStr;
use Reflection::Vertical;

pub struct Valley {
    horiz: Vec<String>,
    vert: Vec<String>,
}

#[derive(Eq, PartialEq)]
pub enum Reflection {
    Horizontal(i32),
    Vertical(i32),
}

impl Valley {
    fn is_valid_line_of_reflection(start: usize, pattern: &[String]) -> bool {
        let mut i = start as isize;
        let mut j = i + 1;
        while i >= 0 && j < pattern.len() as isize {
            if pattern[i as usize] != pattern[j as usize] {
                return false;
            }
            i -= 1;
            j += 1;
        }
        true
    }
    pub fn line_of_reflection(&self) -> Reflection {
        for i in 0..self.horiz.len() - 1 {
            if Self::is_valid_line_of_reflection(i, &self.horiz) {
                return Horizontal(i as i32 + 1);
            }
        }
        for i in 0..self.vert.len() - 1 {
            if Self::is_valid_line_of_reflection(i, &self.vert) {
                return Vertical(i as i32 + 1);
            }
        }

        Vertical(0)
    }
    fn invert_at(s: &str, index: usize) -> String {
        let mut chars: Vec<_> = s.chars().collect();
        if chars[index] == '.' {
            chars[index] = '#';
        } else if chars[index] == '#' {
            chars[index] = '.';
        }
        chars.into_iter().collect()
    }
    pub fn smudged_line_of_reflection(&mut self) -> Reflection {
        let original_line_of_reflection = self.line_of_reflection();
        for y in 0..self.horiz.len() {
            for x in 0..self.vert.len() {
                self.horiz[y] = Self::invert_at(&self.horiz[y], x);
                self.vert[x] = Self::invert_at(&self.vert[x], y);
                for i in 0..self.horiz.len() - 1 {
                    if Self::is_valid_line_of_reflection(i, &self.horiz)
                        && original_line_of_reflection != Horizontal(i as i32 + 1)
                    {
                        return Horizontal(i as i32 + 1);
                    }
                }
                for i in 0..self.vert.len() - 1 {
                    if Self::is_valid_line_of_reflection(i, &self.vert)
                        && original_line_of_reflection != Vertical(i as i32 + 1)
                    {
                        return Vertical(i as i32 + 1);
                    }
                }
                self.horiz[y] = Self::invert_at(&self.horiz[y], x);
                self.vert[x] = Self::invert_at(&self.vert[x], y);
            }
        }

        Vertical(0)
    }

    pub fn score(&self) -> i32 {
        match self.line_of_reflection() {
            Vertical(i) => i,
            Horizontal(i) => i * 100,
        }
    }

    pub fn smudged_score(&mut self) -> i32 {
        match self.smudged_line_of_reflection() {
            Vertical(i) => i,
            Horizontal(i) => i * 100,
        }
    }
}

impl FromStr for Valley {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let horiz: Vec<String> = s.lines().map(str::to_owned).collect();
        let vert = (0..horiz[0].len())
            .map(|i| {
                horiz
                    .iter()
                    .map(|row| row.get(i..=i).expect("string should be long enough"))
                    .collect()
            })
            .collect();
        Ok(Self { horiz, vert })
    }
}

pub fn parse(input: &str) -> Vec<Valley> {
    input.split("\n\n").flat_map(str::parse).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    const INPUT: &str = include_str!("../../input/2023/13.txt");

    #[test]
    fn test_1_sample() {
        let input = parse(SAMPLE);

        assert_eq!(input.iter().map(Valley::score).sum::<i32>(), 405);
    }

    #[test]
    fn test_1() {
        let input = parse(INPUT);

        assert_eq!(input.iter().map(Valley::score).sum::<i32>(), 28_895);
    }

    #[test]
    fn test_2_sample() {
        let mut input = parse(SAMPLE);

        assert_eq!(
            input.iter_mut().map(Valley::smudged_score).sum::<i32>(),
            400
        );
    }

    #[test]
    fn test_2() {
        let mut input = parse(INPUT);

        assert_eq!(
            input.iter_mut().map(Valley::smudged_score).sum::<i32>(),
            31_603
        );
    }
}
