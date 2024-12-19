use itertools::Itertools;
use std::collections::HashMap;

pub fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(',').map(str::trim).collect_vec();
    let designs = designs.lines().collect_vec();

    (towels, designs)
}

pub fn match_count<'a>(
    design: &'a str,
    all_towels: &[&'a str],
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&count) = memo.get(design) {
        return count;
    }
    let count = all_towels
        .iter()
        .filter(|&&t| design.starts_with(t))
        .map(|&towel| {
            if towel == design {
                1
            } else {
                match_count(&design[towel.len()..], all_towels, memo)
            }
        })
        .sum();
    memo.insert(design, count);
    count
}

pub fn part_1(input: &(Vec<&str>, Vec<&str>)) -> usize {
    let mut memo = HashMap::new();
    let (towels, designs) = input;
    designs
        .iter()
        .filter(|d| match_count(d, towels, &mut memo) > 0)
        .count()
}

pub fn part_2(input: &(Vec<&str>, Vec<&str>)) -> usize {
    let mut memo = HashMap::new();
    let (towels, designs) = input;
    designs
        .iter()
        .map(|d| match_count(d, towels, &mut memo))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
    const INPUT: &str = include_str!("../../input/2024/19.txt");

    #[test]
    fn test_1_sample() {
        let input = parse(SAMPLE);

        assert_eq!(part_1(&input), 6);
    }

    #[test]
    fn test_1() {
        let input = parse(INPUT);

        assert_eq!(part_1(&input), 369);
    }

    #[test]
    fn test_2_sample() {
        let input = parse(SAMPLE);

        assert_eq!(part_2(&input), 16);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT);

        assert_eq!(part_2(&input), 761_826_581_538_190);
    }
}
