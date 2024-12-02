use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug)]
pub struct Report(Vec<i32>);

impl Report {
    pub fn is_safe_permute(&self) -> bool {
        self.is_safe()
            || self.0.iter().enumerate().any(|(index, _)| {
                let (left, right) = self.0.split_at(index);
                let iter = left.iter().chain(&right[1..]);

                Self::iter_is_safe(iter)
            })
    }
    pub fn is_safe(&self) -> bool {
        Self::iter_is_safe(self.0.iter())
    }
    fn iter_is_safe<'a, I>(iter: I) -> bool
    where
        I: Iterator<Item = &'a i32>,
    {
        let mut first = true;
        let mut cmp: fn(&i32, &i32) -> bool = i32::lt;
        iter.tuple_windows().all(|(left, right)| {
            if first {
                first = false;
                if right < left {
                    cmp = i32::gt;
                }
            }
            (1..=3).contains(&(left - right).abs()) && cmp(left, right)
        })
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn parse(input: &str) -> Result<Vec<Report>> {
    input
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            line.split_whitespace()
                .map(str::parse::<i32>)
                .process_results(|res| Report(res.collect()))
                .map_err(|e| anyhow!("failed to parse line {line_number}: {}", e))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    const INPUT: &str = include_str!("../../input/2024/02.txt");

    #[test]
    fn test_1_sample() {
        let input = parse(SAMPLE).unwrap();
        let number_safe = input.iter().filter(|r| r.is_safe()).count();

        assert_eq!(number_safe, 2);
    }

    #[test]
    fn test_1() {
        let input = parse(INPUT).unwrap();
        let number_safe = input.iter().filter(|r| r.is_safe()).count();

        assert_eq!(number_safe, 585);
    }

    #[test]
    fn test_2_sample() {
        let input = parse(SAMPLE).unwrap();
        let number_safe = input.iter().filter(|r| r.is_safe_permute()).count();

        assert_eq!(number_safe, 4);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT).unwrap();
        let number_safe = input.iter().filter(|r| r.is_safe_permute()).count();

        assert_eq!(number_safe, 626);
    }
}
