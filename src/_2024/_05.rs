use anyhow::{anyhow, Result};
use itertools::Itertools;

#[allow(clippy::missing_errors_doc)]
pub fn parse(input: &str) -> Result<i32> {
    input
        .lines()
        .map(|l| {
            l.parse::<i32>()
                .map_err(|e| anyhow!("failed to parse input: {}", e))
        })
        .process_results(|res| res.sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const SAMPLE: &str = "\
";
    const INPUT: &str = include_str!("../../input/2024/05.txt");

    #[test]
    fn test_1_sample() {
        let input = parse(SAMPLE).unwrap();

        assert_eq!(input, 1 + 1);
    }

    #[test]
    fn test_1() {
        let input = parse(INPUT).unwrap();

        assert_eq!(input, 1 + 1);
    }

    #[test]
    fn test_2_sample() {
        let input = parse(SAMPLE).unwrap();

        assert_eq!(input, 1 + 1);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT).unwrap();

        assert_eq!(input, 1 + 1);
    }
}