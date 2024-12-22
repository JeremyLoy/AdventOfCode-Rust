use itertools::Itertools;
use std::collections::HashMap;
use std::iter::once;
use std::ops::AddAssign;

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(secret: i64) -> i64 {
    secret % 16_777_216
}

fn get_all_prices(mut secret: i64) -> Vec<i64> {
    let mut prices = Vec::with_capacity(2_000);
    for _ in 0..2_000 {
        secret = prune(mix(secret, secret * 64));
        secret = prune(mix(secret, secret / 32));
        secret = prune(mix(secret, secret * 2_048));
        prices.push(secret);
    }
    prices
}

pub fn get_price_change_map(secret: i64) -> HashMap<(i64, i64, i64, i64), i64> {
    let start = secret;
    let prices = get_all_prices(secret);

    let price_changes: Vec<(i64, i64)> = once(start)
        .chain(prices)
        .tuple_windows()
        .map(|(a, b)| (b % 10, b % 10 - a % 10))
        .collect_vec();
    let mut lookup_map = HashMap::new();
    for (a, b, c, d) in price_changes.iter().tuple_windows() {
        // !!! important. only insert the first seen value
        lookup_map.entry((a.1, b.1, c.1, d.1)).or_insert(d.0);
    }
    lookup_map
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .map(|s| *get_all_prices(s).last().unwrap())
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let price_changes = input
        .lines()
        .map(|l| get_price_change_map(l.parse().unwrap()))
        .collect_vec();

    let mut merged_map = HashMap::new();
    for map in price_changes {
        for (k, v) in map {
            merged_map.entry(k).or_insert(0).add_assign(v);
        }
    }

    *merged_map.values().max().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
1
10
100
2024
";

    const SAMPLE_2: &str = "\
1
2
3
2024
";
    const INPUT: &str = include_str!("../../input/2024/22.txt");

    #[test]
    fn test_1_sample() {
        let input = part1(SAMPLE);

        assert_eq!(input, 37_327_623);
    }

    #[test]
    fn test_1() {
        let input = part1(INPUT);

        assert_eq!(input, 14_392_541_715);
    }

    #[test]
    fn test_2_sample() {
        let input = part2(SAMPLE_2);

        assert_eq!(input, 23);
    }

    #[test]
    fn test_2() {
        let input = part2(INPUT);

        assert_eq!(input, 1_628);
    }
}
