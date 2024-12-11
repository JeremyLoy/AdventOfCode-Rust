use anyhow::{anyhow, Result};
use std::collections::HashMap;

#[allow(clippy::missing_errors_doc)]
pub fn parse(input: &str) -> Result<Vec<u64>> {
    input
        .split_whitespace()
        .map(|c| c.parse().map_err(|e| anyhow!("Invalid input: {c} : {e}")))
        .collect()
}

pub fn blink(stones: &[u64], times: usize) -> usize {
    fn blink_once(stone: u64, blinks: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
        if blinks == 0 {
            1
        } else if let Some(derived_stones) = cache.get(&(stone, blinks)) {
            *derived_stones
        } else {
            let blinks_remaining = blinks - 1;
            let stone_string = stone.to_string();
            let derived_stones = if stone == 0 {
                blink_once(1, blinks_remaining, cache)
            } else if stone_string.chars().count() % 2 == 0 {
                let (left, right) = stone_string.split_at(stone_string.len() / 2);
                let (left, right) = (left.parse().unwrap(), right.parse().unwrap());
                blink_once(left, blinks_remaining, cache)
                    + blink_once(right, blinks_remaining, cache)
            } else {
                blink_once(stone * 2_024, blinks_remaining, cache)
            };
            cache.insert((stone, blinks), derived_stones);
            derived_stones
        }
    }
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|stone| blink_once(*stone, times, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::iter::IntoParallelIterator;
    use rayon::iter::ParallelIterator;

    const SAMPLE: &str = "125 17";
    const INPUT: &str = include_str!("../../input/2024/11.txt");

    #[test]
    fn test_1_sample() {
        let stones = parse(SAMPLE).unwrap();
        let stones = blink(&stones, 25);

        assert_eq!(stones, 55_312);
    }

    #[test]
    fn test_1() {
        let stones = parse(INPUT).unwrap();
        let stones = blink(&stones, 25);

        assert_eq!(stones, 200_446);
    }

    #[test]
    fn test_2_sample() {
        let stones = parse(SAMPLE).unwrap();
        let stones = blink(&stones, 75);

        assert_eq!(stones, 65_601_038_650_482);
    }

    #[test]
    fn test_2() {
        let stones = parse(INPUT).unwrap();
        let sum: usize = stones
            .into_par_iter()
            .map(|stone| blink(&[stone], 75))
            .sum();

        assert_eq!(sum, 238_317_474_993_392);
    }
}
