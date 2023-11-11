use std::fs;
use std::io::{self, BufRead};
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
fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = io::Result<String>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines())
}

#[allow(dead_code)]
fn parse_lines_as_i32(
    lines: impl Iterator<Item = io::Result<String>>,
) -> impl Iterator<Item = i32> {
    lines
        .filter_map(Result::ok)
        .filter_map(|line| line.trim().parse::<i32>().ok())
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
    .map(|line| Ok(line.to_string()));
    let numbers: Vec<i32> = parse_lines_as_i32(input).collect();
    assert_eq!(count_of_increasing_pairs_in_windowed_sums(&numbers, 1), 7);
}

#[test]
fn test_1_1() {
    let lines = read_lines("input/2021/1.txt").unwrap();
    let numbers: Vec<i32> = parse_lines_as_i32(lines).collect();
    assert_eq!(
        count_of_increasing_pairs_in_windowed_sums(&numbers, 1),
        1583
    );
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
    .map(|line| Ok(line.to_string()));

    let numbers: Vec<i32> = parse_lines_as_i32(input).collect();
    assert_eq!(count_of_increasing_pairs_in_windowed_sums(&numbers, 3), 5);
}

#[test]
fn test_1_2() {
    let lines = read_lines("input/2021/1.txt").unwrap();
    let numbers: Vec<i32> = parse_lines_as_i32(lines).collect();
    assert_eq!(
        count_of_increasing_pairs_in_windowed_sums(&numbers, 3),
        1627
    );
}
