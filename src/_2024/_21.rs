use itertools::Itertools;
use std::collections::HashMap;
use std::iter::once;

// a hardcoded list of all 141 possible permutations of input
// see bin/generate_2024_21_static.rs for the solution
//
// 11^2 = 121 for the digit pad (
// 5^2 = 25 for the arrow key pad
// -5 for AA (which exists in both lists) and <>, ^v, ><, v^ that would never be a useful move
fn get_lookup_table() -> HashMap<(char, char), Vec<char>> {
    let input = include_str!("_21.txt");
    let mut lookup_table = HashMap::new();
    for line in input.lines() {
        let (first, second, chars) = line.split(',').collect_tuple().unwrap();
        lookup_table.insert(
            (
                first.chars().next().unwrap(),
                second.chars().next().unwrap(),
            ),
            chars.chars().collect(),
        );
    }
    lookup_table
}

// chunks ^A^AAA^A into ^A,^A,A,A,^A etc
fn make_chunks(sequence: &[char]) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut chunk = String::new();
    for &c in sequence {
        chunk.push(c);
        if c == 'A' {
            chunks.push(chunk.clone());
            chunk.clear();
        }
    }
    chunks
}

// recursively solve chunks with memoization
// because each robot starts and ends on A, no Dijkstra or anything like that is required
pub fn expand_code(
    code: &str,
    num_robots: usize,
    lookup_table: &HashMap<(char, char), Vec<char>>,
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    fn solve_chunk(
        chunk: &str,
        depth: usize,
        lookup_table: &HashMap<(char, char), Vec<char>>,
        cache: &mut HashMap<(String, usize), usize>,
    ) -> usize {
        if let Some(&cached) = cache.get(&(chunk.to_string(), depth)) {
            return cached;
        }
        if depth == 0 {
            return chunk.len();
        }
        let mut expanded_sequence = Vec::new();
        for (from, to) in once('A').chain(chunk.chars()).tuple_windows() {
            expanded_sequence.extend(lookup_table.get(&(from, to)).unwrap().clone());
        }
        let sum = make_chunks(&expanded_sequence)
            .into_iter()
            .map(|chunk| solve_chunk(&chunk, depth - 1, lookup_table, cache))
            .sum();
        cache.insert((chunk.to_string(), depth), sum);
        sum
    }

    once(code)
        // num_robots + 1 here as the lookup map handles both the robot and the human seamlessly
        .map(|chunk| solve_chunk(chunk, num_robots + 1, lookup_table, cache))
        .sum()
}

pub fn complexity(code: &str, sequence: usize) -> usize {
    code.strip_suffix('A').unwrap().parse::<usize>().unwrap() * sequence
}

pub fn sum_complexity(input: &str, num_robots: usize) -> usize {
    let lookup_table = get_lookup_table();
    let mut cache = HashMap::new();
    input
        .lines()
        .map(|code| {
            complexity(
                code,
                expand_code(code, num_robots, &lookup_table, &mut cache),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
029A
980A
179A
456A
379A
";
    const INPUT: &str = include_str!("../../input/2024/21.txt");

    #[test]
    fn test_1_sample() {
        let complexity = sum_complexity(SAMPLE, 2);

        assert_eq!(complexity, 126_384);
    }

    #[test]
    fn test_1() {
        let complexity = sum_complexity(INPUT, 2);

        assert_eq!(complexity, 136_780);
    }

    #[test]
    fn test_2() {
        let complexity = sum_complexity(INPUT, 25);

        assert_eq!(complexity, 167_538_833_832_712);
    }
}
