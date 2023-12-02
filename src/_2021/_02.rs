#[derive(Debug)]
pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    pub fn parse(line: &str) -> Option<Self> {
        let mut line = line.split_whitespace();
        let direction = line.next()?;
        let amount = line.next()?;
        let amount = amount.parse::<i32>().ok()?;
        match direction {
            "forward" => Some(Self::Forward(amount)),
            "down" => Some(Self::Down(amount)),
            "up" => Some(Self::Up(amount)),
            _ => None,
        }
    }

    pub fn parse_batch(lines: impl Iterator<Item = String>) -> Vec<Self> {
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
/// # use advent_of_code_rust::_2021::_02::{Command, calculate_distance};
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
/// # use advent_of_code_rust::_2021::_02::{Command, calculate_aim_and_distance};
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::to_lines;
    use crate::input_parsing::Input::{Path, Raw};

    #[test]
    fn test_1_sample() {
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
    fn test_1() {
        let input = to_lines(Path("input/2021/02.txt"));
        let commands = Command::parse_batch(input);

        let result = calculate_distance(commands);

        assert_eq!(result, 2_150_351);
    }

    #[test]
    fn test_2_sample() {
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
    fn test_2() {
        let input = to_lines(Path("input/2021/02.txt"));
        let commands = Command::parse_batch(input);

        let result = calculate_aim_and_distance(commands);

        assert_eq!(result, 1_842_742_223);
    }
}
