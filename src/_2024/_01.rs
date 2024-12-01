use anyhow::{anyhow, Result};
use itertools::Itertools;
/// Parses the input string, which is expected to contain lines of two integers
/// separated by whitespace. Each integer on a line represents a number in one
/// of two columns. The function returns a result containing a tuple of vectors
/// of integers, representing the numbers in the left and right columns respectively,
/// or an error if parsing fails.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input data to be parsed.
///
/// # Returns
///
/// * `Ok((Vec<i32>, Vec<i32>))` - A tuple containing two vectors of integers.
///   The first vector contains the numbers from the left column and the second
///   vector contains the numbers from the right column, sorted in ascending order.
///
/// * `Err(anyhow::Error)` - An error if any parsing operation fails.
///
/// # Errors
///
/// This function will return an error if any of the following conditions occur:
/// - A line does not contain exactly two integers.
/// - Any of the numbers cannot be parsed into an `i32`.
///
/// # Examples
///
/// ```
/// use anyhow::Result;
/// use advent_of_code_rust::_2024::_01::parse;
/// let input = "1 2\n3 4\n5 6";
/// let result = parse(input).unwrap();
/// assert_eq!(result, (vec![1, 3, 5], vec![2, 4, 6]));
/// ```
pub fn parse(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let (mut left_column, mut right_column): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<i32>)
                .collect_tuple()
                .ok_or(anyhow!("row did not contain exactly two items"))
                .and_then(|(left, right)| Ok((left?, right?)))
        })
        .try_collect::<_, Vec<(_, _)>, _>()?
        .into_iter()
        .unzip();

    left_column.sort_unstable();
    right_column.sort_unstable();

    Ok((left_column, right_column))
}

/// Computes the sum of absolute differences between corresponding elements of two vectors.
///
/// This function takes a tuple with two vectors of integers, representing the left and
/// right columns. It calculates the absolute difference for each pair of elements from
/// these vectors and returns the sum of these differences.
///
/// # Arguments
///
/// * `(left, right)` - A tuple where each element is a reference to a vector of integers.
///   'left' and 'right' represent the numbers from parsed input columns.
///
/// # Returns
///
/// * `i32` - The sum of absolute differences between the elements of the left and right vectors.
///
/// # Panics
///
/// This function will panic if the lengths of the `left` and `right` vectors are not the same,
/// as it assumes each element in `left` has a corresponding element in `right`.
///
/// # Examples
///
/// ```
/// use advent_of_code_rust::_2024::_01::sum_of_distances;
/// let left = vec![1, 3, 5];
/// let right = vec![2, 4, 6];
/// let result = sum_of_distances(&(left, right));
/// assert_eq!(result, 3);
/// ```
pub fn sum_of_distances((left, right): &(Vec<i32>, Vec<i32>)) -> i32 {
    left.iter()
        .copied()
        .zip(right.iter().copied())
        .map(|(left_number, right_number)| (left_number - right_number).abs())
        .sum()
}

/// Calculates a similarity score between two vectors of integers.
///
/// The function computes the similarity score by first creating a frequency map
/// of the numbers in the `right` vector. It then iterates over the `left` vector
/// and calculates a score based on the frequency of each number in the `right` vector.
///
/// # Arguments
///
/// * `(left, right)` - A tuple where each element is a reference to a vector of integers.
///   'left' and 'right' represent the numbers from parsed input columns.
///
/// # Returns
///
/// * `i32` - The similarity score calculated based on the frequency of numbers
///   in the `right` vector relative to `left`.
///
/// # Examples
///
/// ```
/// use advent_of_code_rust::_2024::_01::similarity_score;
/// let left = vec![3, 4, 2];
/// let right = vec![4, 3, 5, 3, 9, 3];
/// let result = similarity_score(&(left, right));
/// assert_eq!(result, 13);
/// ```
pub fn similarity_score((left, right): &(Vec<i32>, Vec<i32>)) -> i32 {
    let frequency_map =
        right
            .iter()
            .copied()
            .fold(std::collections::HashMap::new(), |mut freq_map, number| {
                *freq_map.entry(number).or_default() += 1;
                freq_map
            });

    left.iter().copied().fold(0, |acc, left_number| {
        let freq: i32 = frequency_map.get(&left_number).copied().unwrap_or_default();
        let similarity = left_number * freq;
        acc + similarity
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
    const INPUT: &str = include_str!("../../input/2024/01.txt");

    #[test]
    fn test_1_sample() {
        let lists = parse(SAMPLE).unwrap();
        let sum = sum_of_distances(&lists);

        assert_eq!(sum, 11);
    }

    #[test]
    fn test_1() {
        let lists = parse(INPUT).unwrap();
        let sum = sum_of_distances(&lists);

        assert_eq!(sum, 2_113_135);
    }

    #[test]
    fn test_2_sample() {
        let lists = parse(SAMPLE).unwrap();
        let score = similarity_score(&lists);

        assert_eq!(score, 31);
    }

    #[test]
    fn test_2() {
        let lists = parse(INPUT).unwrap();
        let score = similarity_score(&lists);

        assert_eq!(score, 19_097_157);
    }
}
