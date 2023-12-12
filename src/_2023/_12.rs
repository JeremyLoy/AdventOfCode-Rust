pub fn parse(input: &str) -> i32 {
    input.parse().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const SAMPLE: &str = "\
";
    const INPUT: &str = include_str!("../../input/2023/12.txt");

    #[test]
    fn test_1_sample() {
        let input = parse(SAMPLE);

        assert_eq!(input, 1 + 1);
    }

    #[test]
    fn test_1() {
        let input = parse(INPUT);

        assert_eq!(input, 1 + 1);
    }

    #[test]
    fn test_2_sample() {
        let input = parse(SAMPLE);

        assert_eq!(input, 1 + 1);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT);

        assert_eq!(input, 1 + 1);
    }
}