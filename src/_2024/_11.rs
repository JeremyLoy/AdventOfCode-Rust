use anyhow::{anyhow, Result};
use std::collections::HashMap;

#[allow(clippy::missing_errors_doc)]
pub fn parse(input: &str) -> Result<Vec<u64>> {
    input
        .split_whitespace()
        .map(|c| c.parse().map_err(|e| anyhow!("Invalid input: {c} : {e}")))
        .collect()
}

trait Stone
where
    Self: Sized,
{
    fn is_even(&self) -> bool;
    fn num_digits(&self) -> Self;
    fn split(&self) -> (Self, Self);
}

impl Stone for u64 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }

    // log10(1234) + 1 = 3 + 1 = 4
    fn num_digits(&self) -> u64 {
        u64::from(self.ilog10() + 1)
    }

    fn split(&self) -> (u64, u64) {
        // pow = 2 = loq10(1234) / 2 = 3 + 1 / 2
        // (left, right) = (1234/100) (1234 % 100)
        let pow = 10u64.pow((self.num_digits() / 2) as u32);
        (self / pow, self % pow)
    }
}

pub fn blink(stones: &[u64], times: usize) -> usize {
    fn blink_once(stone: u64, blinks: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
        if blinks == 0 {
            1
        } else if let Some(derived_stones) = cache.get(&(stone, blinks)) {
            *derived_stones
        } else {
            let blinks_remaining = blinks - 1;
            let derived_stones = if stone == 0 {
                blink_once(1, blinks_remaining, cache)
            } else if stone.num_digits().is_even() {
                let (left, right) = stone.split();
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
        let stones = blink(&stones, 75);

        assert_eq!(stones, 238_317_474_993_392);
    }
}
