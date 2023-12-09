pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(str::split_whitespace)
        .map(|line| line.map(str::parse).filter_map(Result::ok).collect())
        .collect()
}

pub fn extrapolate(history: &[i32]) -> i32 {
    let sequences = sequences(history);

    let mut additional_column = vec![*sequences.first().unwrap().last().unwrap()];
    for w in sequences.windows(2) {
        additional_column.push(w[1].last().unwrap() + additional_column.last().unwrap());
    }

    *additional_column.last().unwrap()
}

pub fn extrapolate_backwards(history: &[i32]) -> i32 {
    let mut history = history.to_vec();
    history.reverse();
    extrapolate(&history)
}

fn sequences(history: &[i32]) -> Vec<Vec<i32>> {
    let mut sequences: Vec<Vec<i32>> = vec![history.to_vec()];
    while !sequences.last().unwrap().iter().all(|i| *i == 0) {
        sequences.push(
            sequences
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect(),
        );
    }
    sequences.reverse();
    sequences
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    const INPUT: &str = include_str!("../../input/2023/09.txt");

    #[test]
    fn test_1_sample() {
        let histories = parse(SAMPLE);
        let sum: i32 = histories.iter().map(|h| extrapolate(h)).sum();
        assert_eq!(sum, 114);
    }

    #[test]
    fn test_1() {
        let histories = parse(INPUT);
        let sum: i32 = histories.iter().map(|h| extrapolate(h)).sum();
        assert_eq!(sum, 1_581_679_977);
    }

    #[test]
    fn test_2_sample() {
        let histories = parse(SAMPLE);
        let sum: i32 = histories.iter().map(|h| extrapolate_backwards(h)).sum();
        assert_eq!(sum, 2);
    }

    #[test]
    fn test_2() {
        let histories = parse(INPUT);
        let sum: i32 = histories.iter().map(|h| extrapolate_backwards(h)).sum();
        assert_eq!(sum, 889);
    }
}
