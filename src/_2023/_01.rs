pub enum ParseMode {
    Literals,
    LettersAndLiterals,
}

pub fn parse_letter_and_literal_calibration_value(str: &str) -> i32 {
    let mut digits: Vec<i32> = Vec::new();
    for i in 0..str.len() {
        let substring = &str[i..];
        if substring.starts_with("1") || substring.starts_with("one") {
            digits.push(1)
        } else if substring.starts_with("2") || substring.starts_with("two") {
            digits.push(2)
        } else if substring.starts_with("3") || substring.starts_with("three") {
            digits.push(3)
        } else if substring.starts_with("4") || substring.starts_with("four") {
            digits.push(4)
        } else if substring.starts_with("5") || substring.starts_with("five") {
            digits.push(5)
        } else if substring.starts_with("6") || substring.starts_with("six") {
            digits.push(6)
        } else if substring.starts_with("7") || substring.starts_with("seven") {
            digits.push(7)
        } else if substring.starts_with("8") || substring.starts_with("eight") {
            digits.push(8)
        } else if substring.starts_with("9") || substring.starts_with("nine") {
            digits.push(9)
        }
    }
    (digits.first().unwrap() * 10) + digits.last().unwrap()
}

pub fn parse_literal_calibration_value(str: &str) -> i32 {
    let digits: Vec<i32> = str
        .chars()
        .filter_map(|c| c.to_string().parse::<i32>().ok())
        .collect();
    (digits.first().unwrap() * 10) + digits.last().unwrap()
}

pub fn parse_batch_calibration_values(
    lines: impl Iterator<Item = String>,
    parse_mode: ParseMode,
) -> Vec<i32> {
    match parse_mode {
        ParseMode::Literals => lines.map(|l| parse_literal_calibration_value(&l)).collect(),
        ParseMode::LettersAndLiterals => lines
            .map(|l| parse_letter_and_literal_calibration_value(&l))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::to_lines;
    use crate::input_parsing::Input::{Path, Raw};
    use crate::_2023::_01::ParseMode::{LettersAndLiterals, Literals};
    use crate::_2023::_01::*;

    #[test]
    fn test_1_sample() {
        let input = Raw("\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        let calibration_values = parse_batch_calibration_values(to_lines(input), Literals);
        assert_eq!(calibration_values.iter().sum::<i32>(), 142)
    }

    #[test]
    fn test_1() {
        let input = Path("input/2023/1.txt");
        let calibration_values = parse_batch_calibration_values(to_lines(input), Literals);
        assert_eq!(calibration_values.iter().sum::<i32>(), 54_601)
    }

    #[test]
    fn test_2_sample() {
        let input = Raw("\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        let calibration_values =
            parse_batch_calibration_values(to_lines(input), LettersAndLiterals);
        assert_eq!(calibration_values.iter().sum::<i32>(), 281)
    }

    #[test]
    fn test_2() {
        let input = Path("input/2023/1.txt");
        let calibration_values =
            parse_batch_calibration_values(to_lines(input), LettersAndLiterals);
        assert_eq!(calibration_values.iter().sum::<i32>(), 54_078)
    }
}
