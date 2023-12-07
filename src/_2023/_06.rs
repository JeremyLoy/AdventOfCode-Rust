#[derive(Debug, PartialOrd, PartialEq)]
pub struct Race {
    duration: u64,
    record: u64,
}

impl Race {
    pub fn one_from_iterator(mut i: impl Iterator<Item = String>) -> Option<Race> {
        let (duration, record) = (i.next()?, i.next()?);

        let (_, duration) = duration.split_once(':')?;
        let (_, record) = record.split_once(':')?;

        let duration = duration
            .replace(char::is_whitespace, "")
            .parse::<u64>()
            .ok()?;
        let record = record
            .replace(char::is_whitespace, "")
            .parse::<u64>()
            .ok()?;

        Some(Race { duration, record })
    }
}

impl Race {
    pub fn from_iterator(mut i: impl Iterator<Item = String>) -> Option<Vec<Race>> {
        let (durations, records) = (i.next()?, i.next()?);

        let (_, durations) = durations.split_once(':')?;
        let (_, records) = records.split_once(':')?;

        let durations = durations
            .split_whitespace()
            .map(str::trim)
            .map(str::parse::<u64>)
            .filter_map(Result::ok);
        let records = records
            .split_whitespace()
            .map(str::trim)
            .map(str::parse::<u64>)
            .filter_map(Result::ok);

        Some(
            durations
                .zip(records)
                .map(|(duration, record)| Race { duration, record })
                .collect(),
        )
    }

    pub fn winning_permutations(&self) -> u64 {
        let is_winner = |held_duration: &u64| {
            let remaining_time = self.duration - held_duration;
            let distance_travelled = remaining_time * held_duration;
            distance_travelled > self.record
        };
        let find_smallest_winner_binary_search = |start: u64, end: u64| {
            let mut left = start;
            let mut right = end;

            while left < right {
                let mid = left + (right - left) / 2;

                if is_winner(&mid) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }

            if left < end && is_winner(&left) {
                left
            } else {
                start
            }
        };
        let find_largest_winner_binary_search = |start: u64, end: u64| {
            let mut left = start;
            let mut right = end;

            while left < right {
                let mid = left + (right - left) / 2;

                if is_winner(&mid) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            if right > 0 && is_winner(&(right - 1)) {
                right - 1
            } else {
                end
            }
        };
        let start_pos = find_smallest_winner_binary_search(1, self.duration - 1);
        let end_pos = find_largest_winner_binary_search(1, self.duration - 1);
        end_pos - start_pos + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::to_lines;
    use crate::input_parsing::Input::{Path, Raw};

    #[test]
    fn test_from_iterator() {
        let input = Raw("\
Time:      7  15   30
Distance:  9  40  200
");
        let expected = vec![
            Race {
                duration: 7,
                record: 9,
            },
            Race {
                duration: 15,
                record: 40,
            },
            Race {
                duration: 30,
                record: 200,
            },
        ];

        assert_eq!(expected, Race::from_iterator(to_lines(input)).unwrap());
    }

    #[test]
    fn test_one_from_iterator() {
        let input = Raw("\
Time:      7  15   30
Distance:  9  40  200
");
        let expected = Race {
            duration: 71_530,
            record: 940_200,
        };

        assert_eq!(expected, Race::one_from_iterator(to_lines(input)).unwrap());
    }

    #[test]
    fn test_1_sample() {
        let input = Raw("\
Time:      7  15   30
Distance:  9  40  200
");

        let races = Race::from_iterator(to_lines(input)).unwrap();

        assert_eq!(
            races
                .iter()
                .map(Race::winning_permutations)
                .product::<u64>(),
            288
        );
    }

    #[test]
    fn test_1() {
        let input = Path("input/2023/06.txt");

        let races = Race::from_iterator(to_lines(input)).unwrap();

        assert_eq!(
            races
                .iter()
                .map(Race::winning_permutations)
                .product::<u64>(),
            741_000
        );
    }

    #[test]
    fn test_2_sample() {
        let input = Raw("\
Time:      7  15   30
Distance:  9  40  200
");

        let race = Race::one_from_iterator(to_lines(input)).unwrap();

        assert_eq!(race.winning_permutations(), 71_503);
    }

    #[test]
    fn test_2() {
        let input = Path("input/2023/06.txt");

        let race = Race::one_from_iterator(to_lines(input)).unwrap();

        assert_eq!(race.winning_permutations(), 38_220_708);
    }
}
