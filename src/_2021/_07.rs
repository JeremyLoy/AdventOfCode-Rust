/// Calculates the Nth triangle number.
///
/// A triangle number is the sum of all positive integers up to and including N.
/// The formula used to calculate the Nth triangle number is (N * (N + 1)) / 2.
///
/// # Arguments
///
/// * `n` - The Nth number for which the triangle number needs to be calculated.
///
/// # Returns
///
/// The Nth triangle number.
///
/// # Example
///
/// ```rust
/// # use advent_of_code_rust::_2021::_07::triangle_number;
/// assert_eq!(triangle_number(5), 5 + 4 + 3 + 2 + 1);
/// ```
pub fn triangle_number(n: i32) -> i32 {
    (n * (n + 1)) / 2
}
pub fn find_cheapest_horizontal_position(crabs: Vec<i32>, fuel_calculator: fn(i32) -> i32) -> i32 {
    let max_crab_pos = *crabs.iter().max().unwrap();
    (0..max_crab_pos)
        .map(|horiz_pos| {
            crabs
                .iter()
                .map(|&crab_pos| fuel_calculator((horiz_pos - crab_pos).abs()))
                .sum()
        })
        .min()
        .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::{to_vec, Input::*, Separator::*};
    use std::convert::identity;
    #[test]
    fn test_1_sample() {
        let crabs = to_vec(Raw("16,1,2,0,4,2,7,1,2,14"), Comma);

        assert_eq!(find_cheapest_horizontal_position(crabs, identity), 37);
    }

    #[test]
    fn test_1() {
        let crabs = to_vec(Path("input/2021/07.txt"), Comma);

        assert_eq!(find_cheapest_horizontal_position(crabs, identity), 348_996);
    }

    #[test]
    fn test_2_sample() {
        let crabs = to_vec(Raw("16,1,2,0,4,2,7,1,2,14"), Comma);

        assert_eq!(
            find_cheapest_horizontal_position(crabs, triangle_number),
            168
        );
    }

    #[test]
    fn test_2() {
        let crabs = to_vec(Path("input/2021/07.txt"), Comma);

        assert_eq!(
            find_cheapest_horizontal_position(crabs, triangle_number),
            98_231_647
        );
    }
}
