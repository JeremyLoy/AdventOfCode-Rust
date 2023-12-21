use crate::_2023::_12::Spring::{Damaged, Operational, Unknown};
use itertools::Itertools;

pub struct Record {
    springs: Vec<Spring>,
    contiguous_damaged: Vec<i32>,
}

impl Record {
    pub fn expand(&mut self) {
        let mut expanded = self.springs.clone();
        for _ in 0..4 {
            expanded.push(Unknown);
            expanded.append(&mut self.springs.clone());
        }
        self.springs = expanded;
        self.contiguous_damaged = self.contiguous_damaged.repeat(5);
    }
    pub fn valid_permutations(&self) -> i32 {
        let unknown_indices: Vec<_> = self
            .springs
            .iter()
            .enumerate()
            .filter(|(_i, s)| **s == Unknown)
            .map(|e| e.0)
            .collect();
        let damaged_count = self.springs.iter().filter(|s| **s == Damaged).count();
        let damaged_to_add = self.contiguous_damaged.iter().sum::<i32>() as usize - damaged_count;

        unknown_indices
            .iter()
            .combinations(damaged_to_add)
            .filter(|damaged_indices| {
                let mut springs = self.springs.clone();
                for damaged in damaged_indices {
                    springs[**damaged] = Damaged;
                }
                for spring in &mut springs {
                    if *spring == Unknown {
                        *spring = Operational;
                    }
                }
                Record::valid(&springs, &self.contiguous_damaged)
            })
            .count() as i32
    }

    pub fn valid(springs: &[Spring], contiguous_damaged: &[i32]) -> bool {
        let found_contiguous_damaged: Vec<i32> = springs
            .split(|a| *a == Operational)
            .map(|a| {
                a.iter()
                    .filter(|b| **b == Damaged)
                    .collect::<Vec<&Spring>>()
            })
            .map(|a| a.len() as i32)
            .filter(|a| *a != 0)
            .collect();

        found_contiguous_damaged == contiguous_damaged
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    pub fn from_char(c: char) -> Spring {
        match c {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => panic!("unknown Spring {c}"),
        }
    }
}

pub fn parse(input: &str) -> Vec<Record> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (springs, contiguous_damaged) = line.split_once(' ')?;
            let springs = springs.chars().map(Spring::from_char).collect();
            let contiguous_damaged = contiguous_damaged
                .split(',')
                .map(str::parse)
                .filter_map(Result::ok)
                .collect();

            Some(Record {
                springs,
                contiguous_damaged,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    const INPUT: &str = include_str!("../../input/2023/12.txt");

    #[test]
    fn test_1_sample() {
        let input = parse(SAMPLE);

        assert_eq!(
            input.iter().map(Record::valid_permutations).sum::<i32>(),
            21
        );
    }

    #[test]
    #[ignore]
    fn test_1() {
        let input = parse(INPUT);

        assert_eq!(
            input.iter().map(Record::valid_permutations).sum::<i32>(),
            6_852
        );
    }

    #[test]
    #[ignore]
    fn test_2_sample() {
        let mut input = parse(SAMPLE);
        input.iter_mut().for_each(Record::expand);

        assert_eq!(
            input.iter().map(Record::valid_permutations).sum::<i32>(),
            525_152
        );
    }

    #[test]
    #[ignore]
    fn test_2() {
        let mut input = parse(INPUT);
        input.iter_mut().for_each(Record::expand);

        assert_eq!(
            input.iter().map(Record::valid_permutations).sum::<i32>(),
            525_152
        );
    }
}
