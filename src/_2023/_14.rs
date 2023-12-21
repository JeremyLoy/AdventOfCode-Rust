use std::fmt;
use std::fmt::Display;
use std::ops::Range;
use Direction::{East, North, South, West};

pub struct Dish {
    grid: Vec<Vec<char>>,
}

impl Display for Dish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, row) in self.grid.iter().enumerate() {
            for &c in row {
                write!(f, "{c}")?;
            }
            if i != self.grid.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

fn get_windows(column: &[char]) -> Vec<Range<usize>> {
    let mut in_sequence = false;
    let mut start = None;
    let mut windows = vec![];
    for (i, ch) in column.iter().enumerate() {
        match ch {
            '#' => {
                if in_sequence {
                    in_sequence = false;
                    if let Some(s) = start {
                        windows.push(s..i);
                    }
                }
            }
            _ => {
                if !in_sequence {
                    in_sequence = true;
                    start = Some(i);
                }
            }
        }
    }
    if in_sequence {
        if let Some(s) = start {
            windows.push(s..column.len());
        }
    }
    windows
}

impl Dish {
    pub fn load(&self) -> i32 {
        let mut sum = 0;
        for x in 0..self.grid[0].len() {
            for y in 0..self.grid.len() {
                let rock = self.grid[y][x];
                if rock == 'O' {
                    sum += self.grid.len() - y;
                }
            }
        }
        sum as i32
    }
    pub fn shift(&mut self, direction: Direction) {
        match direction {
            North => {
                for x in 0..self.grid[0].len() {
                    let mut column: Vec<char> = self.grid.iter().map(|row| row[x]).rev().collect();

                    let windows = get_windows(&column);

                    for window in windows {
                        column[window].sort_unstable();
                    }

                    for (y, c) in column.iter().rev().enumerate() {
                        self.grid[y][x] = *c;
                    }
                }
            }
            East => {
                for row in &mut self.grid {
                    let windows = get_windows(row);

                    for window in windows {
                        row[window].sort_unstable();
                    }
                }
            }
            West => {
                for row in &mut self.grid {
                    let windows = get_windows(row);

                    for window in windows {
                        row[window].sort_unstable_by(|a, b| b.cmp(a));
                    }
                }
            }
            South => {
                for x in 0..self.grid[0].len() {
                    let mut column: Vec<char> = self.grid.iter().map(|row| row[x]).rev().collect();

                    let windows = get_windows(&column);

                    for window in windows {
                        // column[window].sort_unstable();
                        column[window].sort_unstable_by(|a, b| b.cmp(a));
                    }

                    for (y, c) in column.iter().rev().enumerate() {
                        self.grid[y][x] = *c;
                    }
                }
            }
        }
    }

    pub fn spin_cycle(&mut self, times: i32) {
        for _ in 0..times {
            self.shift(North);
            self.shift(West);
            self.shift(South);
            self.shift(East);
        }
    }
}

pub fn parse(input: &str) -> Dish {
    Dish {
        grid: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    const SHIFTED_NORTH: &str = "\
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    const INPUT: &str = include_str!("../../input/2023/14.txt");

    #[test]
    fn test_load() {
        let input = parse(SHIFTED_NORTH);

        assert_eq!(input.load(), 136);
    }

    #[test]
    fn test_cycle() {
        let mut input = parse(SAMPLE);

        input.spin_cycle(1);

        assert_eq!(
            input.to_string(),
            "\
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
        );

        input.spin_cycle(1);

        assert_eq!(
            input.to_string(),
            "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
        );

        input.spin_cycle(1);

        assert_eq!(
            input.to_string(),
            "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
        );
    }

    #[test]
    fn test_1_sample() {
        let mut input = parse(SAMPLE);

        input.shift(North);

        assert_eq!(input.load(), 136);
    }

    #[test]
    fn test_1() {
        let mut input = parse(INPUT);

        input.shift(North);

        assert_eq!(input.load(), 103_614);
    }

    #[test]
    fn test_2_sample() {
        let mut input = parse(SAMPLE);

        input.spin_cycle(1_000);

        assert_eq!(input.load(), 64);
    }

    #[test]
    fn test_2() {
        let mut input = parse(INPUT);

        input.spin_cycle(1_000);
        // input.spin_cycle(1_000_000_000);

        assert_eq!(input.load(), 83_790);
    }
}
