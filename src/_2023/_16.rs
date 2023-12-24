use crate::_2023::_16::Heading::{Down, Right, Up};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::ops::Add;
use Heading::Left;

pub struct Contraption {
    grid: HashMap<Point, char>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Heading {
    Up,
    Down,
    Left,
    Right,
}

impl Contraption {
    fn trace(&self, mut point: Point, mut heading: Heading, seen: &mut HashSet<(Point, Heading)>) {
        if seen.contains(&(point, heading)) {
            return;
        }
        while let Some(tile) = self.grid.get(&point) {
            seen.insert((point, heading));
            match (tile, heading) {
                ('|', Right | Left) => {
                    self.trace(point + Up, Up, seen);
                    self.trace(point + Down, Down, seen);
                    return;
                }
                ('-', Up | Down) => {
                    self.trace(point + Left, Left, seen);
                    self.trace(point + Right, Right, seen);
                    return;
                }
                ('\\', Right) | ('/', Left) | ('|' | '.', Down) => {
                    point = point + Down;
                    heading = Down;
                }
                ('\\', Up) | ('/', Down) | ('-' | '.', Left) => {
                    point = point + Left;
                    heading = Left;
                }
                ('\\', Down) | ('/', Up) | ('-' | '.', Right) => {
                    point = point + Right;
                    heading = Right;
                }
                ('\\', Left) | ('/', Right) | ('|' | '.', Up) => {
                    point = point + Up;
                    heading = Up;
                }
                _ => panic!("unhandled tile {tile} and heading {heading:?}"),
            }
        }
    }
    pub fn count_energized(&self) -> usize {
        let mut seen: HashSet<(Point, Heading)> = HashSet::new();
        self.trace(Point { x: 0, y: 0 }, Right, &mut seen);
        let set: HashSet<Point> = seen.iter().map(|(point, _)| *point).collect::<HashSet<_>>();
        set.len()
    }
    fn edges(&self) -> Vec<(Point, Heading)> {
        let mut edges = Vec::new();
        for x in 0..self.width {
            edges.push((
                Point {
                    x: x as isize,
                    y: 0,
                },
                Down,
            ));
            edges.push((
                Point {
                    x: x as isize,
                    y: self.height as isize,
                },
                Up,
            ));
        }
        for y in 0..self.height {
            edges.push((
                Point {
                    x: 0,
                    y: y as isize,
                },
                Right,
            ));
            edges.push((
                Point {
                    x: self.width as isize,
                    y: y as isize,
                },
                Left,
            ));
        }

        edges
    }
    pub fn count_largest_energized(&self) -> usize {
        self.edges()
            .into_par_iter()
            .map(|(point, heading)| {
                let mut seen: HashSet<(Point, Heading)> = HashSet::new();
                self.trace(point, heading, &mut seen);
                let set: HashSet<Point> =
                    seen.iter().map(|(point, _)| *point).collect::<HashSet<_>>();
                set.len()
            })
            .max()
            .unwrap()
    }
}

impl Display for Contraption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.height {
            for x in 0..=self.width {
                f.write_char(
                    *self
                        .grid
                        .get(&Point {
                            x: x as isize,
                            y: y as isize,
                        })
                        .expect("all points should exist"),
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Add<Heading> for Point {
    type Output = Point;

    fn add(self, heading: Heading) -> Self::Output {
        match heading {
            Up => Point {
                x: self.x,
                y: self.y - 1,
            },
            Down => Point {
                x: self.x,
                y: self.y + 1,
            },
            Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Right => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

pub fn parse(input: &str) -> Contraption {
    let mut grid = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    input.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, tile)| {
            height = usize::max(height, y);
            width = usize::max(width, x);
            grid.insert(
                Point {
                    x: x as isize,
                    y: y as isize,
                },
                tile,
            );
        });
    });
    Contraption {
        grid,
        width,
        height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    const INPUT: &str = include_str!("../../input/2023/16.txt");

    #[test]
    fn test_1_sample() {
        let input = parse(SAMPLE);

        assert_eq!(input.count_energized(), 46);
    }

    #[test]
    fn test_1() {
        let input = parse(INPUT);

        assert_eq!(input.count_energized(), 7_415);
    }

    #[test]
    fn test_2_sample() {
        let input = parse(SAMPLE);

        assert_eq!(input.count_largest_energized(), 51);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT);

        assert_eq!(input.count_largest_energized(), 7_943);
    }
}
