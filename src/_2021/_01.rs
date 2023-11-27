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
/// # use advent_of_code_rust::_2021::_01::count_of_increasing_pairs_in_windowed_sums;
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::to_vec;
    use crate::input_parsing::Input::{Path, Raw};
    use crate::input_parsing::Separator::Newline;

    #[test]
    fn test_1_sample() {
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
        ";
        let numbers = to_vec(Raw(input), Newline);

        let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 1);

        assert_eq!(count, 7);
    }

    #[test]
    fn test_1() {
        let numbers = to_vec(Path("input/2021/1.txt"), Newline);

        let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 1);

        assert_eq!(count, 1583);
    }

    #[test]
    fn test_2_sample() {
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
        ";
        let numbers = to_vec(Raw(input), Newline);

        let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 3);

        assert_eq!(count, 5);
    }

    #[test]
    fn test_2() {
        let numbers: Vec<i32> = to_vec(Path("input/2021/1.txt"), Newline);

        let count = count_of_increasing_pairs_in_windowed_sums(&numbers, 3);

        assert_eq!(count, 1627);
    }
}
