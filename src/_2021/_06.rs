pub fn parse_lantern_fish_histogram(input: Vec<usize>) -> Vec<u128> {
    input.iter().fold(vec![0; 9], |mut acc, &i| {
        acc[i] += 1;
        acc
    })
}

pub fn advance_lantern_fish_days(mut hist: Vec<u128>, days: i32) -> u128 {
    for _ in 0..days {
        hist.rotate_left(1);
        // Every 0 spawned exactly one fish. In other words, the number of new parents is equal to the
        // number of new children.
        // Parents should reset to 6 as opposed to new children being 8
        //
        // Therefore:
        // 6 = the old 7's + the new parents
        hist[6] += hist[8];
    }

    hist.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::{to_vec, Input::*, Separator::*};
    #[test]
    fn test_1_sample() {
        let input = to_vec(Raw("3,4,3,1,2"), Comma);

        let lantern_fish = parse_lantern_fish_histogram(input);

        let total = advance_lantern_fish_days(lantern_fish, 80);

        assert_eq!(total, 5_934);
    }

    #[test]
    fn test_1() {
        let input = to_vec(Path("input/2021/06.txt"), Comma);

        let lantern_fish = parse_lantern_fish_histogram(input);

        let total = advance_lantern_fish_days(lantern_fish, 80);

        assert_eq!(total, 363_101);
    }

    #[test]
    fn test_2_sample() {
        let input = to_vec(Raw("3,4,3,1,2"), Comma);

        let lantern_fish = parse_lantern_fish_histogram(input);

        let total = advance_lantern_fish_days(lantern_fish, 256);

        assert_eq!(total, 26_984_457_539);
    }

    #[test]
    fn test_2() {
        let input = to_vec(Path("input/2021/06.txt"), Comma);

        let lantern_fish = parse_lantern_fish_histogram(input);

        let total = advance_lantern_fish_days(lantern_fish, 256);

        assert_eq!(total, 1_644_286_074_024);
    }
}
