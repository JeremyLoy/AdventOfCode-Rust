use itertools::Itertools;
use std::collections::HashSet;
use std::iter::successors;

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(secret: i64) -> i64 {
    secret % 16_777_216
}

fn next_secret(mut secret: i64) -> i64 {
    secret = prune(mix(secret, secret * 64));
    secret = prune(mix(secret, secret / 32));
    secret = prune(mix(secret, secret * 2_048));
    secret
}

type Window = (i8, i8, i8, i8);

fn calculate_index(window: Window) -> usize {
    // Map the numbers to range [0, 18] and calculate the unique index
    let offset: i8 = 9; // Offset to map range [-9, 9] to [0, 18]
    let base: usize = 19;

    ((window.0 + offset) as usize) * base.pow(3)
        + ((window.1 + offset) as usize) * base.pow(2)
        + ((window.2 + offset) as usize) * base.pow(1)
        + ((window.3 + offset) as usize)
}

// simultaneously solve p1 and p2 so that a large amount of vectors
// or hashmaps do not need to be used
// the values -9->9 are the only possible offsets, so a single Vec of 19^4 is used
// to store counters.
pub fn solve(input: &str) -> (i64, u16) {
    let mut final_price_sum = 0;
    let mut buckets = vec![0; 19_usize.pow(4)];
    let mut seen = HashSet::new();
    input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .for_each(|secret| {
            let mut final_price = 0;

            // successors here is really cool - infinite iterator where the next value is derived from the previous
            // instead of returning None with a counter, I'm using take(2_000)
            successors(Some(secret), |&s| {
                let next = next_secret(s);
                final_price = next;
                Some(next)
            })
            .take(2_000)
            .tuple_windows()
            // get the last most base 10 digit, and the diff between previous
            // converting to i8 as these numbers will always be between -9 and +9
            // u16 is used for the banana total to keep the vec as small as possible.
            // if the puzzle required a larger total number of bananas, than this is what would be increased
            .map(|(a, b)| ((b % 10) as u16, (b % 10 - a % 10) as i8))
            .tuple_windows()
            .for_each(|(a, b, c, d)| {
                // a vec of size 19^4 is used for buckets of all possible windows and their banana count, so the index
                // is found with a function
                let index = calculate_index((a.1, b.1, c.1, d.1));

                // !!! important. only insert the first seen value
                if !seen.contains(&index) {
                    buckets[index] += d.0;
                    seen.insert(index);
                }
            });
            // add this secret's final value to the p1 total
            final_price_sum += final_price;
            seen.clear();
        });
    // the largest value
    (final_price_sum, *buckets.iter().max().unwrap())
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
        let input = solve(SAMPLE);

        assert_eq!(input.0, 37_327_623);
    }

    #[test]
    fn test_1() {
        let input = solve(INPUT);

        assert_eq!(input.0, 14_392_541_715);
    }

    #[test]
    fn test_2_sample() {
        let input = solve(SAMPLE_2);

        assert_eq!(input.1, 23);
    }

    #[test]
    fn test_2() {
        let input = solve(INPUT);

        assert_eq!(input.1, 1_628);
    }
}
