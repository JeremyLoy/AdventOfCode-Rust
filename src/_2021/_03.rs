use std::collections::HashMap;

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
mod tests {
    use super::*;
    use crate::input_parsing::{to_lines, Input::*};

    #[test]
    fn test_1_sample() {
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
    fn test_1() {
        let input: Vec<String> = to_lines(Path("input/2021/03.txt")).collect();

        let gamma_rate = find_all_most_common_bits(&input);
        let epsilon_rate = flip_binary_str_bits(&gamma_rate);

        let power_consumption =
            binary_str_to_decimal(&gamma_rate) * binary_str_to_decimal(&epsilon_rate);

        assert_eq!(power_consumption, 3_633_500)
    }

    #[test]
    fn test_2_sample() {
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
    fn test_2() {
        let input: Vec<String> = to_lines(Path("input/2021/03.txt")).collect();

        let oxygen_generator_rating = find_component_rating(input.clone(), BitCriteria::Oxygen);
        let co2_scrubber_rating = find_component_rating(input, BitCriteria::CO2);
        let life_support_rating = binary_str_to_decimal(&oxygen_generator_rating)
            * binary_str_to_decimal(&co2_scrubber_rating);

        assert_eq!(life_support_rating, 4_550_283)
    }
}
