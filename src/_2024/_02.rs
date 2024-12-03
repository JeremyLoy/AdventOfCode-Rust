use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug)]
pub struct Report(Vec<i32>);

impl Report {
    /// Determines if any permutation of the sequence of numbers in the `Report` struct is "safe".
    /// A permutation in this context
    ///   is defined as the sequence obtained by removing a single item from the original sequence.
    ///
    /// A sequence is considered "safe" if it either follows a pattern where all differences
    /// between consecutive numbers are within 1 to 3 (inclusive),
    /// maintaining a consistent order (increasing or decreasing), or if any permutation
    /// of the sequence can satisfy the same conditions.
    ///
    /// # Returns
    ///
    /// - `true` if the sequence itself or any permutation (with one item removed) adheres to the "safe" criteria.
    /// - `false` otherwise.
    pub fn is_safe_permute(&self) -> bool {
        self.is_safe()
            || self.0.iter().enumerate().any(|(index, _)| {
                let (left, right) = self.0.split_at(index);
                let iter = left.iter().chain(&right[1..]);

                Self::iter_is_safe(iter)
            })
    }

    /// Checks if the sequence of numbers in the `Report` struct is "safe".
    ///
    /// A sequence is considered "safe" if it follows a pattern where all differences
    ///   between consecutive numbers are within 1 to 3 (inclusive),
    ///   maintain a consistent order (increasing or decreasing).
    ///
    /// # Returns
    ///
    /// - `true` if the sequence adheres to the "safe" criteria.
    /// - `false` otherwise.
    pub fn is_safe(&self) -> bool {
        Self::iter_is_safe(self.0.iter())
    }
    fn iter_is_safe<'a, I>(iter: I) -> bool
    where
        I: Iterator<Item = &'a i32>,
    {
        let mut cmp = None;
        iter.tuple_windows().all(|(left, right)| {
            cmp.get_or_insert_with(|| if right < left { i32::gt } else { i32::lt });

            (1..=3).contains(&(left - right).abs()) && cmp.unwrap()(left, right)
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
