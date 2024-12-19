use anyhow::{anyhow, Result};
use itertools::{repeat_n, Itertools};

pub fn parse(input: &str) -> Result<Vec<(u64, Vec<u64>)>> {
    input
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            let (left, right) = line
                .split(": ")
                .collect_tuple()
                .ok_or(anyhow!("could not split line {line_number} on :"))?;
            let left = left.parse()?;
            let right = right
                .split_whitespace()
                .map(|s| {
                    s.parse()
                        .map_err(|e| anyhow!("failed to parse line {line_number}: {}", e))
                })
                .collect::<Result<Vec<_>>>()?;
            Ok((left, right))
        })
        .collect()
}

pub fn solve(input: &[(u64, Vec<u64>)], ops: &[fn(u64, u64) -> u64]) -> u64 {
    input
        .iter()
        .filter(|(left, right)| {
            repeat_n(ops, right.len() - 1)
                .multi_cartesian_product()
                .any(|p| {
                    let result = p
                        .iter()
                        .zip(right[1..].iter())
                        .fold(right[0], |res, (op, next)| op(res, *next));
                    result == *left
                })
        })
        .map(|(left, _)| left)
        .sum()
}

pub fn concat(a: u64, b: u64) -> u64 {
    a.to_string()
        .chars()
        .chain(b.to_string().chars())
        .collect::<String>()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::{Add, Mul};

    const SAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    const INPUT: &str = include_str!("../../input/2024/07.txt");

    #[test]
    fn test_1_sample() {
        let input = parse(SAMPLE).unwrap();
        let sum = solve(&input, &[Add::add, Mul::mul]);

        assert_eq!(sum, 3_749);
    }

    #[test]
    fn test_1() {
        let input = parse(INPUT).unwrap();
        let sum = solve(&input, &[Add::add, Mul::mul]);

        assert_eq!(sum, 1_298_300_076_754);
    }

    #[test]
    fn test_2_sample() {
        let input = parse(SAMPLE).unwrap();
        let sum = solve(&input, &[Add::add, Mul::mul, concat]);

        assert_eq!(sum, 11_387);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT).unwrap();
        let sum = solve(&input, &[Add::add, Mul::mul, concat]);

        assert_eq!(sum, 248_427_118_972_289);
    }
}
