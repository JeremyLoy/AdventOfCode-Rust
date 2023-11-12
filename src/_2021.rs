use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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

#[allow(dead_code)]
fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok)
}

#[allow(dead_code)]
fn parse_lines_as_i32(lines: impl Iterator<Item = String>) -> impl Iterator<Item = i32> {
    lines.filter_map(|line| line.trim().parse::<i32>().ok())
}

#[test]
fn test_1_1_sample() {
    let input = "
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
        "
    .lines()
    .map(|line| line.to_string());
    let numbers: Vec<i32> = parse_lines_as_i32(input).collect();

    let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 1);

    assert_eq!(count, 7);
}

#[test]
fn test_1_1() {
    let lines = read_lines("input/2021/1.txt");
    let numbers: Vec<i32> = parse_lines_as_i32(lines).collect();

    let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 1);

    assert_eq!(count, 1583);
}

#[test]
fn test_1_2_sample() {
    let input = "
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
        "
    .lines()
    .map(|line| line.to_string());
    let numbers: Vec<i32> = parse_lines_as_i32(input).collect();

    let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 3);

    assert_eq!(count, 5);
}

#[test]
fn test_1_2() {
    let lines = read_lines("input/2021/1.txt");
    let numbers: Vec<i32> = parse_lines_as_i32(lines).collect();

    let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 3);

    assert_eq!(count, 1627);
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

#[test]
fn test_2_1_sample() {
    let input = "
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
        "
    .lines()
    .map(|line| line.to_string());
    let commands = Command::parse_batch(input);

    let result = calculate_distance(commands);

    assert_eq!(result, 150);
}

#[test]
fn test_2_1() {
    let input = read_lines("input/2021/2.txt");
    let commands = Command::parse_batch(input);

    let result = calculate_distance(commands);

    assert_eq!(result, 2_150_351);
}

#[test]
fn test_2_2_sample() {
    let input = "
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
        "
    .lines()
    .map(|line| line.to_string());
    let commands = Command::parse_batch(input);

    let result = calculate_aim_and_distance(commands);

    assert_eq!(result, 900);
}

#[test]
fn test_2_2() {
    let input = read_lines("input/2021/2.txt");
    let commands = Command::parse_batch(input);

    let result = calculate_aim_and_distance(commands);

    assert_eq!(result, 1_842_742_223);
}
