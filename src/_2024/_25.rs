use itertools::Itertools;

type KeyOrLock = [u8; 5];

pub fn parse(input: &str) -> (Vec<KeyOrLock>, Vec<KeyOrLock>) {
    let (locks, keys): (Vec<&str>, Vec<&str>) = input
        .split("\n\n")
        .partition(|x: &&str| x.starts_with("#####"));

    let keys = keys
        .iter()
        .map(|key| {
            let mut arr = [0; 5];
            for line in key.lines().take(6) {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        arr[i] += 1;
                    }
                }
            }
            arr
        })
        .collect_vec();

    let locks = locks
        .iter()
        .map(|lock| {
            let mut arr = [0; 5];
            for line in lock.lines().skip(1) {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        arr[i] += 1;
                    }
                }
            }
            arr
        })
        .collect_vec();

    (keys, locks)
}

pub fn possible_keys((keys, locks): &(Vec<KeyOrLock>, Vec<KeyOrLock>)) -> usize {
    keys.iter()
        .cartesian_product(locks)
        .filter(|(key, lock)| key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";
    const INPUT: &str = include_str!("../../input/2024/25.txt");

    #[test]
    fn test_1_sample() {
        let keys_and_locks = parse(SAMPLE);

        assert_eq!(possible_keys(&keys_and_locks), 3);
    }

    #[test]
    fn test_1() {
        let keys_and_locks = parse(INPUT);

        assert_eq!(possible_keys(&keys_and_locks), 2_900);
    }
}
