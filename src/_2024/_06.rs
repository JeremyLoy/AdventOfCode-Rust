use anyhow::Result;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Error, Formatter};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let point = Point { x, y };
                let ch = self.map.get(&point).ok_or(Error)?;
                write!(f, "{ch}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Grid {
    map: HashMap<Point, char>,
    max_x: i32,
    max_y: i32,
    start: (Point, Direction),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[allow(clippy::missing_errors_doc)]
pub fn parse(input: &str) -> Result<Grid> {
    let mut map = HashMap::new();
    let mut start = (Point { x: 0, y: 0 }, Direction::Up);
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            match ch {
                '^' => start = (point, Direction::Up),
                '>' => start = (point, Direction::Right),
                '<' => start = (point, Direction::Left),
                'v' => start = (point, Direction::Down),
                _ => (),
            }
            map.insert(point, ch);
            max_x = max_x.max(x as i32);
        }
        max_y = max_y.max(y as i32);
    }
    Ok(Grid {
        map,
        max_x,
        max_y,
        start,
    })
}

pub fn cycles(grid: &mut Grid) -> usize {
    // walk it once to mutate the original grid, setting walked points
    travel_and_count(grid).unwrap();
    let counter = Arc::new(AtomicUsize::new(0));
    (0..=grid.max_y)
        .into_par_iter()
        .map(|y| {
            (0..=grid.max_x)
                .into_par_iter()
                .filter_map(|x| {
                    // Define the adjacent points around (x, y)
                    let adjacent_points = [
                        Point { x: x - 1, y },
                        Point { x: x + 1, y },
                        Point { x, y: y - 1 },
                        Point { x, y: y + 1 },
                    ];

                    // at least one adjacent point must be on the walked path
                    if adjacent_points.iter().any(|&adj| {
                        if let Some(&c) = grid.map.get(&adj) {
                            c != '.' && c != '#'
                        } else {
                            false
                        }
                    }) {
                        let mut new_grid = Grid {
                            start: grid.start,
                            max_x: grid.max_x,
                            max_y: grid.max_y,
                            map: grid.map.clone(),
                        };
                        new_grid.map.insert(Point { x, y }, '#');

                        counter.fetch_add(1, Relaxed);
                        if travel_and_count(&mut new_grid).is_none() {
                            return Some(1);
                        }
                    }
                    None
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn travel_and_count(grid: &mut Grid) -> Option<usize> {
    let (mut current, mut direction) = (grid.start.0, grid.start.1);
    let mut travelled_positions = HashSet::new();
    let mut cycle_detector = HashSet::new();

    while let Some(&cur_char) = grid.map.get(&current) {
        if cycle_detector.contains(&(current, direction)) {
            return None;
        }
        cycle_detector.insert((current, direction));
        travelled_positions.insert(current);
        match cur_char {
            '.' => grid.map.insert(
                current,
                match direction {
                    Direction::Up | Direction::Down => '|',
                    Direction::Right | Direction::Left => '-',
                },
            ),
            '|' | '-' => grid.map.insert(current, '+'),
            _ => None,
        };
        let next = match direction {
            Direction::Up => Point {
                x: current.x,
                y: current.y - 1,
            },
            Direction::Down => Point {
                x: current.x,
                y: current.y + 1,
            },
            Direction::Left => Point {
                x: current.x - 1,
                y: current.y,
            },
            Direction::Right => Point {
                x: current.x + 1,
                y: current.y,
            },
        };

        match grid.map.get(&next) {
            Some('#') => {
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
            }
            _ => {
                current = next;
            }
        }
    }

    Some(travelled_positions.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    const INPUT: &str = include_str!("../../input/2024/06.txt");

    #[test]
    fn test_1_sample() {
        let mut grid = parse(SAMPLE).unwrap();
        let steps = travel_and_count(&mut grid).unwrap();

        assert_eq!(steps, 41);
    }

    #[test]
    fn test_1() {
        let mut grid = parse(INPUT).unwrap();
        let steps = travel_and_count(&mut grid).unwrap();

        assert_eq!(steps, 5_086);
    }

    #[test]
    fn test_2_sample() {
        let mut grid = parse(SAMPLE).unwrap();
        let count = cycles(&mut grid);

        assert_eq!(count, 6);
    }

    #[test]
    fn test_2() {
        let mut grid = parse(INPUT).unwrap();
        let count = cycles(&mut grid);

        assert_eq!(count, 1_770);
    }
}
