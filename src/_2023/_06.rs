use std::fs::remove_dir_all;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Race {
    duration: u64,
    record: u64,
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
        (1..self.duration).fold(0, |i, held_duration| {
            let remaining_time = self.duration - held_duration;
            let distance_travelled = remaining_time * held_duration;
            if distance_travelled > self.record {
                i + 1
            } else {
                i
            }
        })
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
        assert_eq!(
            Race {
                duration: 71_530,
                record: 940_200
            }
            .winning_permutations(),
            71_503
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Race {
                duration: 47_847_467,
                record: 207_139_412_091_014,
            }
            .winning_permutations(),
            38_220_708
        );
    }
}
