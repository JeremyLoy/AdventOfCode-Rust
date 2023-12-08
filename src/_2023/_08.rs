use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashMap;

pub fn parse(i: &[String]) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let mut i = i.iter();
    let instructions = i
        .next()
        .expect("instructions should exist")
        .chars()
        .collect();

    let graph = i
        .map(|line| {
            let (key, values) = line.split_once('=').expect("line to be split on =");

            let (left, right) = values.split_once(',').expect("line has values");
            let (left, right) = (
                left.trim().trim_start_matches('('),
                right.trim().trim_end_matches(')'),
            );

            (key.trim(), (left, right))
        })
        .collect();

    (instructions, graph)
}

pub fn is_zzz(current: &str) -> bool {
    current == "ZZZ"
}

pub fn ends_with_z(current: &str) -> bool {
    current.ends_with('Z')
}

pub fn steps_to_done<F>(
    instructions: &[char],
    graph: &HashMap<&str, (&str, &str)>,
    start: &str,
    is_done: F,
) -> u64
where
    F: Fn(&str) -> bool,
{
    instructions
        .iter()
        .cycle()
        .fold_while((start, 0), |(mut current, steps), instruction| {
            let (left, right) = graph.get(current).expect("all nodes should have an entry");
            if *instruction == 'L' {
                current = left;
            } else {
                current = right;
            }
            if is_done(current) {
                Done((current, steps + 1))
            } else {
                Continue((current, steps + 1))
            }
        })
        .into_inner()
        .1
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn lcm_of_list(numbers: &[u64]) -> u64 {
    numbers.iter().fold(1, |acc, &num| lcm(acc, num))
}

pub fn ghost_steps_to_z(instructions: &[char], graph: &HashMap<&str, (&str, &str)>) -> u64 {
    let ghosts: Vec<&str> = graph
        .keys()
        .filter(|key| key.ends_with('A'))
        .copied()
        .collect();
    let steps: Vec<u64> = ghosts
        .iter()
        .map(|ghost| steps_to_done(instructions, graph, ghost, ends_with_z))
        .collect();
    lcm_of_list(&steps)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::to_lines;
    use crate::input_parsing::Input::{Path, Raw};

    #[test]
    fn test_1_sample() {
        let input1 = Raw("\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
");

        let input2 = Raw("\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
");
        let input1 = to_lines(input1).collect::<Vec<_>>();
        let input2 = to_lines(input2).collect::<Vec<_>>();

        let first = parse(&input1);
        let second = parse(&input2);

        assert_eq!(steps_to_done(&first.0, &first.1, "AAA", is_zzz), 2);
        assert_eq!(steps_to_done(&second.0, &second.1, "AAA", is_zzz), 6);
    }

    #[test]
    fn test_1() {
        let input = Path("input/2023/08.txt");

        let input = to_lines(input).collect::<Vec<_>>();
        let graph = parse(&input);

        assert_eq!(steps_to_done(&graph.0, &graph.1, "AAA", is_zzz), 19_241);
    }

    #[test]
    fn test_2_sample() {
        let input = Raw("\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
");

        let input = to_lines(input).collect::<Vec<_>>();
        let graph = parse(&input);

        assert_eq!(ghost_steps_to_z(&graph.0, &graph.1), 6);
    }

    #[test]
    fn test_2() {
        let input = Path("input/2023/08.txt");

        let input = to_lines(input).collect::<Vec<_>>();
        let graph = parse(&input);

        assert_eq!(ghost_steps_to_z(&graph.0, &graph.1), 9_606_140_307_013);
    }
}
