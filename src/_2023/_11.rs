use itertools::Itertools;
use std::cmp::max;
use std::str::FromStr;

pub struct Universe {
    height: u64,
    width: u64,
    galaxies: Vec<Point>,
}

impl FromStr for Universe {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut universe = Universe {
            height: 0,
            width: 0,
            galaxies: Vec::new(),
        };
        s.lines().enumerate().for_each(|(y, line)| {
            universe.height = max(universe.height, y as u64);
            line.chars().enumerate().for_each(|(x, c)| {
                universe.width = max(universe.width, x as u64);
                if c == '#' {
                    universe.galaxies.push(Point {
                        x: x as u64,
                        y: y as u64,
                    });
                }
            });
        });
        Ok(universe)
    }
}

impl Universe {
    pub fn expand(&mut self, times_larger: u64) {
        let times_larger = max(times_larger - 1, 1);
        let mut none_rows = Vec::new();
        let mut none_columns = Vec::new();
        for i in 0..=self.height {
            if !self.galaxies.iter().any(|g| g.y == i) {
                none_rows.push(i);
            }
        }
        for i in 0..=self.width {
            if !self.galaxies.iter().any(|g| g.x == i) {
                none_columns.push(i);
            }
        }
        for row in none_rows.iter().rev() {
            self.galaxies
                .iter_mut()
                .filter(|g| g.y > *row)
                .for_each(|g| {
                    g.y += times_larger;
                });
        }
        for column in none_columns.iter().rev() {
            self.galaxies
                .iter_mut()
                .filter(|g| g.x > *column)
                .for_each(|g| {
                    g.x += times_larger;
                });
        }
    }

    pub fn sum_shortest_galaxy_paths(&self) -> u64 {
        self.galaxies
            .iter()
            .combinations(2)
            .map(|comb| manhattan_distance(*comb[0], *comb[1]))
            .sum::<u64>()
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    x: u64,
    y: u64,
}
pub fn manhattan_distance(a: Point, b: Point) -> u64 {
    (a.x.abs_diff(b.x)) + a.y.abs_diff(b.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    const INPUT: &str = include_str!("../../input/2023/11.txt");

    #[test]
    fn test_1_sample() {
        let mut universe = SAMPLE.parse::<Universe>().unwrap();
        universe.expand(1);

        assert_eq!(universe.sum_shortest_galaxy_paths(), 374);
    }

    #[test]
    fn test_1() {
        let mut universe = INPUT.parse::<Universe>().unwrap();
        universe.expand(1);

        assert_eq!(universe.sum_shortest_galaxy_paths(), 9_608_724);
    }

    #[test]
    fn test_2_sample() {
        let mut universe = SAMPLE.parse::<Universe>().unwrap();
        universe.expand(10);
        assert_eq!(universe.sum_shortest_galaxy_paths(), 1_030);

        let mut universe = SAMPLE.parse::<Universe>().unwrap();
        universe.expand(100);
        assert_eq!(universe.sum_shortest_galaxy_paths(), 8_410);
    }

    #[test]
    fn test_2() {
        let mut universe = INPUT.parse::<Universe>().unwrap();
        universe.expand(1_000_000);

        assert_eq!(universe.sum_shortest_galaxy_paths(), 904_633_799_472);
    }
}
