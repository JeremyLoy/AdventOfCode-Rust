use itertools::Itertools;
use rayon::prelude::*;
use std::error::Error;
use std::ops::Range;
use std::str::FromStr;

pub struct Almanac {
    seeds: Vec<u64>,
    seed_ranges: Vec<Range<u64>>,
    maps: Vec<Vec<AlmanacEntry>>,
}

/// Note: Range doesn't implement Copy so this cannot implement Copy
#[derive(Clone)]
pub struct AlmanacEntry {
    source: Range<u64>,
    destination: Range<u64>,
}

impl FromStr for AlmanacEntry {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let (destination, source, length) = (
            s.next().ok_or("expected a destination")?,
            s.next().ok_or("expected a source")?,
            s.next().ok_or("expected a length")?,
        );
        let (destination, source, length) = (
            destination.parse::<u64>()?,
            source.parse::<u64>()?,
            length.parse::<u64>()?,
        );

        let (destination, source) = (destination..destination + length, source..source + length);

        Ok(AlmanacEntry {
            source,
            destination,
        })
    }
}

impl Almanac {
    pub fn new(mut s: impl Iterator<Item = String>) -> Option<Almanac> {
        let seeds = s.next()?;
        let seeds: Vec<u64> = seeds
            .split_once(':')?
            .1
            .split_whitespace()
            .filter_map(|s| s.trim().parse::<u64>().ok())
            .collect();

        let seed_ranges: Vec<Range<u64>> = seeds
            .iter()
            .tuples()
            .map(|(start, end)| *start..*start + *end)
            .collect();

        let mut maps: Vec<Vec<AlmanacEntry>> = Vec::new();
        let mut current_map: Vec<AlmanacEntry> = Vec::new();

        s.next(); // consume first delimiter

        s.for_each(|row| {
            let row = row.trim();
            if row.ends_with(':') {
                maps.push(current_map.clone());
                current_map = Vec::new();
                return;
            }
            if let Ok(entry) = row.parse() {
                current_map.push(entry);
            }
        });
        maps.push(current_map);

        Some(Almanac {
            seeds,
            seed_ranges,
            maps,
        })
    }

    pub fn get_location(&self, seed: u64) -> u64 {
        self.maps.iter().fold(seed, |location, map| {
            if let Some(entry) = map.iter().find(|entry| entry.source.contains(&location)) {
                entry.destination.start + (location - entry.source.start)
            } else {
                location
            }
        })
    }

    pub fn get_seed(&self, location: u64) -> u64 {
        self.maps.iter().rev().fold(location, |seed, map| {
            if let Some(entry) = map.iter().find(|entry| entry.destination.contains(&seed)) {
                entry.source.start + (seed - entry.destination.start)
            } else {
                seed
            }
        })
    }

    pub fn lowest_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| self.get_location(*seed))
            .min()
            .expect("non-empty seeds")
    }

    pub fn lowest_location_over_ranges(&self) -> u64 {
        (0..u64::MAX)
            .into_par_iter()
            .find_first(|location| {
                let seed = self.get_seed(*location);
                let found = self.seed_ranges.iter().find(|range| range.contains(&seed));
                found.is_some()
            })
            .unwrap_or(u64::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::to_lines;
    use crate::input_parsing::Input::{Path, Raw};

    #[test]
    fn test_1_sample() {
        let input = Raw("\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
");

        let almanac = Almanac::new(to_lines(input)).unwrap();

        assert_eq!(almanac.lowest_location(), 35);
    }

    #[test]
    fn test_1() {
        let input = Path("input/2023/05.txt");

        let almanac = Almanac::new(to_lines(input)).unwrap();

        assert_eq!(almanac.lowest_location(), 107_430_936);
    }

    #[test]
    fn test_2_sample() {
        let input = Raw("\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
");

        let almanac = Almanac::new(to_lines(input)).unwrap();

        assert_eq!(almanac.lowest_location_over_ranges(), 46);
    }

    #[test]
    #[ignore]
    fn test_2() {
        let input = Path("input/2023/05.txt");

        let almanac = Almanac::new(to_lines(input)).unwrap();

        assert_eq!(almanac.lowest_location_over_ranges(), 23_738_616);
    }
}
