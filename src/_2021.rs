use itertools::Itertools;
use std::collections::HashMap;
use std::convert::identity;

/// Counts the number of increasing pairs in windowed sums of given data.
///
/// # Arguments
///
/// * `data` - A vector of integers.
/// * `window_size` - The size of the window used to calculate sums.
///
/// # Examples
///
/// ```
/// # use advent_of_code_rust::_2021::count_of_increasing_pairs_in_windowed_sums;
///
/// let data = vec![1, 2, 3, 4, 5];
/// let window_size = 3;
/// let count = count_of_increasing_pairs_in_windowed_sums(&data, window_size);
/// // 1 + 2 + 3 = 6
/// // 2 + 3 + 4 = 9
/// // 3 + 4 + 5 = 12
///
/// // 12 > 9
/// // 9 > 6
///
/// assert_eq!(count, 2);
/// ```
pub fn count_of_increasing_pairs_in_windowed_sums(data: &[i32], window_size: usize) -> i32 {
    let windowed_sums: Vec<i32> = data
        .windows(window_size)
        .map(|window| window.iter().sum::<i32>())
        .collect();

    let count_increasing: i32 = windowed_sums
        .windows(2)
        .filter(|window_pair| window_pair[0] < window_pair[1])
        .count() as i32;

    count_increasing
}

#[derive(Debug)]
pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    pub fn parse(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (direction, amount) = (parts.get(0)?, parts.get(1)?);
        let amount = amount.parse::<i32>().ok()?;
        match *direction {
            "forward" => Some(Self::Forward(amount)),
            "down" => Some(Self::Down(amount)),
            "up" => Some(Self::Up(amount)),
            _ => None,
        }
    }

    pub fn parse_batch<I: IntoIterator<Item = String>>(lines: I) -> Vec<Self> {
        lines
            .into_iter()
            .filter_map(|line| Self::parse(&line))
            .collect()
    }
}

/// Calculates the submarine's distance from origin based on a series of commands.
///
/// # Arguments
///
/// * `commands` - A vector of `Command` objects representing the actions to be performed.
///
/// # Returns
///
/// The horizontal position and depth multiplied together
///
/// # Example
///
/// ```
/// # use advent_of_code_rust::_2021::{Command, calculate_distance};
///
/// let commands = vec![
///     Command::Forward(10),
///     Command::Down(5),
///     Command::Up(3),
/// ];
///
/// let distance = calculate_distance(commands);
/// assert_eq!(distance, 20);
/// ```
pub fn calculate_distance(commands: Vec<Command>) -> i32 {
    let mut horizontal_position = 0;
    let mut vertical_depth = 0;
    for command in commands {
        match command {
            Command::Forward(amount) => horizontal_position += amount,
            Command::Down(amount) => vertical_depth += amount,
            Command::Up(amount) => vertical_depth -= amount,
        }
    }
    horizontal_position * vertical_depth
}

/// Calculates the aim and distance of the submarine based on the given commands.
///
/// # Arguments
///
/// * `commands` - A vector of `Command` representing the commands to be executed.
///
/// # Returns
///
/// The horizontal position and depth multiplied together
///
/// # Examples
///
/// ```
/// # use advent_of_code_rust::_2021::{Command, calculate_aim_and_distance};
///
/// let commands = vec![
///     Command::Down(5),
///     Command::Up(2),
///     Command::Forward(10),
/// ];
/// let result = calculate_aim_and_distance(commands);
/// assert_eq!(result, 300);
/// ```
pub fn calculate_aim_and_distance(commands: Vec<Command>) -> i32 {
    let mut horizontal_position = 0;
    let mut vertical_depth = 0;
    let mut aim = 0;
    for command in commands {
        match command {
            Command::Forward(amount) => {
                horizontal_position += amount;
                vertical_depth += aim * amount;
            }
            Command::Down(amount) => aim += amount,
            Command::Up(amount) => aim -= amount,
        }
    }
    horizontal_position * vertical_depth
}

pub fn find_all_most_common_bits(binary_report: &Vec<String>) -> String {
    let mut freq_of_ones = HashMap::new();

    for s in binary_report {
        for (i, c) in s.char_indices() {
            match c {
                '1' => {
                    let count = freq_of_ones.entry(i).or_insert(0);
                    *count += 1;
                }
                _ => (),
            }
        }
    }
    let mut ret = String::new();

    for i in 0..freq_of_ones.len() {
        match freq_of_ones.get(&i) {
            Some(i) => {
                if *i > (binary_report.len() / 2) {
                    ret.push('1')
                } else {
                    ret.push('0')
                }
            }
            _ => panic!("index {} wasn't found in freq map", i),
        }
    }

    ret
}

#[derive(Debug)]
pub enum BitCriteria {
    Oxygen,
    CO2,
}

#[derive(Debug)]
pub struct BingoBoard {
    board: [[BingoCell; 5]; 5],
}

#[derive(Debug, Copy, Clone)]
pub enum BingoCell {
    Marked(i32),
    Unmarked(i32),
}
impl BingoBoard {
    // Extracting cell parsing logic to a separate function
    fn parse_cell(number_str: &str) -> Option<BingoCell> {
        let number = number_str.parse::<i32>().ok()?;
        Some(BingoCell::Unmarked(number))
    }
    pub fn parse(input: &[String]) -> Option<Self> {
        let mut board = [[BingoCell::Unmarked(0); 5]; 5];
        for (i, line) in input.iter().enumerate() {
            for (j, number_str) in line.split_whitespace().enumerate() {
                board[i][j] = Self::parse_cell(number_str)?;
            }
        }
        Some(BingoBoard { board })
    }

    pub fn parse_batch<I: Iterator<Item = String>>(lines: I) -> Vec<Self> {
        lines
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .chunks(5)
            .into_iter()
            .map(|chunk| BingoBoard::parse(chunk.collect_vec().as_mut_slice()))
            .filter_map(identity)
            .collect()
    }

    pub fn calculate_score(&self, last_call: i32) -> i32 {
        let mut score = 0;
        for row in self.board.iter() {
            for cell in row.iter() {
                if let BingoCell::Unmarked(value) = cell {
                    score += value;
                }
            }
        }
        score * last_call
    }

    pub fn mark(&mut self, number: i32) {
        for row in self.board.iter_mut() {
            for cell in row.iter_mut() {
                if let BingoCell::Unmarked(value) = cell {
                    if *value == number {
                        *cell = BingoCell::Marked(number);
                    }
                }
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        for row in self.board.iter() {
            if row.iter().all(|&cell| matches!(cell, BingoCell::Marked(_))) {
                return true;
            }
        }
        for col in 0..5 {
            if self
                .board
                .iter()
                .all(|row| matches!(row[col], BingoCell::Marked(_)))
            {
                return true;
            }
        }
        false
    }
}

pub fn find_component_rating(mut binary_report: Vec<String>, bit_criteria: BitCriteria) -> String {
    let mut freq0 = 0;
    let mut freq1 = 0;
    let mut position = 0;

    while binary_report.len() != 1 {
        for s in &binary_report {
            match s.chars().nth(position) {
                Some('0') => freq0 += 1,
                Some('1') => freq1 += 1,
                Some(e) => panic!("unhandled char {}", e),
                None => panic!("no char at pos {}", position),
            }
        }
        let bit_to_keep = match bit_criteria {
            BitCriteria::Oxygen => {
                if freq1 >= freq0 {
                    '1'
                } else {
                    '0'
                }
            }
            BitCriteria::CO2 => {
                if freq0 > freq1 {
                    '1'
                } else {
                    '0'
                }
            }
        };
        binary_report.retain(|s| s.chars().nth(position).eq(&Some(bit_to_keep)));
        position += 1;
        freq0 = 0;
        freq1 = 0;
    }

    binary_report.pop().unwrap()
}

pub fn flip_binary_str_bits(binary: &str) -> String {
    binary
        .chars()
        .map(|bit| match bit {
            '0' => '1',
            '1' => '0',
            _ => panic!("Invalid bit: {}", bit),
        })
        .collect()
}

pub fn binary_str_to_decimal(binary: &str) -> i32 {
    i32::from_str_radix(binary, 2).expect("Failed to convert binary string to decimal")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::_2021::test::Input::{Path, Raw};
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    enum Input<'a> {
        Path(&'a str),
        Raw(&'a str),
    }

    fn to_lines(input: Input) -> Box<dyn Iterator<Item = String> + '_> {
        match input {
            Path(path) => {
                let file = File::open(path).expect("Failed to open file");
                let reader = BufReader::new(file);
                Box::new(
                    reader
                        .lines()
                        .filter_map(Result::ok)
                        .map(|s| s.trim().to_owned())
                        .filter(|s| !s.is_empty()),
                )
            }
            Raw(s) => Box::new(
                s.lines()
                    .map(|s| s.trim().to_owned())
                    .filter(|s| !s.is_empty()),
            ),
        }
    }

    fn parse_lines_as_i32(lines: impl Iterator<Item = String>) -> impl Iterator<Item = i32> {
        lines.filter_map(|line| line.trim().parse::<i32>().ok())
    }

    #[test]
    fn test_1_1_sample() {
        let input = to_lines(Raw("
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
        "));
        let numbers: Vec<i32> = parse_lines_as_i32(input).collect();

        let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 1);

        assert_eq!(count, 7);
    }

    #[test]
    fn test_1_1() {
        let lines = to_lines(Path("input/2021/1.txt"));
        let numbers: Vec<i32> = parse_lines_as_i32(lines).collect();

        let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 1);

        assert_eq!(count, 1583);
    }

    #[test]
    fn test_1_2_sample() {
        let input = to_lines(Raw("
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
        "));
        let numbers: Vec<i32> = parse_lines_as_i32(input).collect();

        let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 3);

        assert_eq!(count, 5);
    }

    #[test]
    fn test_1_2() {
        let lines = to_lines(Path("input/2021/1.txt"));
        let numbers: Vec<i32> = parse_lines_as_i32(lines).collect();

        let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 3);

        assert_eq!(count, 1627);
    }

    #[test]
    fn test_2_1_sample() {
        let input = to_lines(Raw("
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
        "));
        let commands = Command::parse_batch(input);

        let result = calculate_distance(commands);

        assert_eq!(result, 150);
    }

    #[test]
    fn test_2_1() {
        let input = to_lines(Path("input/2021/2.txt"));
        let commands = Command::parse_batch(input);

        let result = calculate_distance(commands);

        assert_eq!(result, 2_150_351);
    }

    #[test]
    fn test_2_2_sample() {
        let input = to_lines(Raw("
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
        "));
        let commands = Command::parse_batch(input);

        let result = calculate_aim_and_distance(commands);

        assert_eq!(result, 900);
    }

    #[test]
    fn test_2_2() {
        let input = to_lines(Path("input/2021/2.txt"));
        let commands = Command::parse_batch(input);

        let result = calculate_aim_and_distance(commands);

        assert_eq!(result, 1_842_742_223);
    }

    #[test]
    fn test_3_1_sample() {
        let input: Vec<String> = to_lines(Raw("
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
        "))
        .collect();

        let gamma_rate = find_all_most_common_bits(&input);
        let epsilon_rate = flip_binary_str_bits(&gamma_rate);

        let power_consumption =
            binary_str_to_decimal(&gamma_rate) * binary_str_to_decimal(&epsilon_rate);

        assert_eq!(power_consumption, 198)
    }

    #[test]
    fn test_3_1() {
        let input: Vec<String> = to_lines(Path("input/2021/3.txt")).collect();

        let gamma_rate = find_all_most_common_bits(&input);
        let epsilon_rate = flip_binary_str_bits(&gamma_rate);

        let power_consumption =
            binary_str_to_decimal(&gamma_rate) * binary_str_to_decimal(&epsilon_rate);

        assert_eq!(power_consumption, 3_633_500)
    }

    #[test]
    fn test_3_2_sample() {
        let input: Vec<String> = to_lines(Raw("
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
        "))
        .collect();

        let oxygen_generator_rating = find_component_rating(input.clone(), BitCriteria::Oxygen);
        let co2_scrubber_rating = find_component_rating(input, BitCriteria::CO2);
        let life_support_rating = binary_str_to_decimal(&oxygen_generator_rating)
            * binary_str_to_decimal(&co2_scrubber_rating);

        assert_eq!(life_support_rating, 230)
    }

    #[test]
    fn test_3_2() {
        let input: Vec<String> = to_lines(Path("input/2021/3.txt")).collect();

        let oxygen_generator_rating = find_component_rating(input.clone(), BitCriteria::Oxygen);
        let co2_scrubber_rating = find_component_rating(input, BitCriteria::CO2);
        let life_support_rating = binary_str_to_decimal(&oxygen_generator_rating)
            * binary_str_to_decimal(&co2_scrubber_rating);

        assert_eq!(life_support_rating, 4_550_283)
    }

    #[test]
    fn test_4_1_sample() {
        let mut input = to_lines(Raw("
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
        "));

        let calls: Vec<i32> = input
            .next()
            .unwrap()
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        let mut boards = BingoBoard::parse_batch(input);
        for call in calls {
            for board in boards.iter_mut() {
                board.mark(call);
                if board.is_winner() {
                    assert_eq!(board.calculate_score(call), 4_512);
                    return;
                }
            }
        }
        panic!("no winner was found");
    }

    #[test]
    fn test_4_1() {
        let mut input = to_lines(Path("input/2021/4.txt"));

        let calls: Vec<i32> = input
            .next()
            .unwrap()
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        let mut boards = BingoBoard::parse_batch(input);
        for call in calls {
            for board in boards.iter_mut() {
                board.mark(call);
                if board.is_winner() {
                    assert_eq!(board.calculate_score(call), 8_136);
                    return;
                }
            }
        }
        panic!("no winner was found");
    }

    #[test]
    fn test_4_2_sample() {
        let mut input = to_lines(Raw("
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
        "));

        let calls: Vec<i32> = input
            .next()
            .unwrap()
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        let mut boards = BingoBoard::parse_batch(input);
        let mut last_winning_score = 0;
        let mut past_winners = HashSet::new();

        for call in calls {
            for i in 0..boards.len() {
                let board = boards.get_mut(i).unwrap();
                board.mark(call);
                if board.is_winner() && !past_winners.contains(&i) {
                    last_winning_score = board.calculate_score(call);
                    past_winners.insert(i);
                }
            }
        }

        assert_eq!(last_winning_score, 1_924);
    }

    #[test]
    fn test_4_2() {
        let mut input = to_lines(Path("input/2021/4.txt"));

        let calls: Vec<i32> = input
            .next()
            .unwrap()
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        let mut boards = BingoBoard::parse_batch(input);
        let mut last_winning_score = 0;
        let mut past_winners = HashSet::new();

        for call in calls {
            for i in 0..boards.len() {
                let board = boards.get_mut(i).unwrap();
                board.mark(call);
                if board.is_winner() && !past_winners.contains(&i) {
                    last_winning_score = board.calculate_score(call);
                    past_winners.insert(i);
                }
            }
        }

        assert_eq!(last_winning_score, 12_738);
    }
}
