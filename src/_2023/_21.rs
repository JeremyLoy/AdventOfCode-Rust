use crate::_2023::_21::Plot::{Rock, Soil};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};

pub enum Plot {
    Soil,
    Rock,
}
#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn neighbors(self) -> [Point; 4] {
        [
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }
}

pub struct Garden {
    grid: HashMap<Point, Plot>,
    start: Point,
    size: i64,
}

impl Garden {
    pub fn reachable_soil(&self, steps: usize) -> usize {
        let mut next_queue: RefCell<VecDeque<Point>> = RefCell::new(VecDeque::new());
        let mut current_queue: RefCell<VecDeque<Point>> = RefCell::new(VecDeque::new());
        next_queue.get_mut().push_front(self.start);
        for _ in 0..steps {
            (current_queue, next_queue) = (next_queue, current_queue);
            let mut seen: HashSet<Point> = HashSet::new();
            while let Some(point) = current_queue.get_mut().pop_front() {
                for neighbor in point.neighbors() {
                    if let Some(Soil) = self.grid.get(&neighbor) {
                        if !seen.contains(&neighbor) {
                            next_queue.get_mut().push_back(neighbor);
                            seen.insert(neighbor);
                        }
                    }
                }
            }
        }

        next_queue.get_mut().len()
    }

    pub fn reachable_soil_math(&self, steps: i64) -> i64 {
        let even_steps = steps % 2 == 0;
        let mut seen = HashSet::new();
        for point in points_within_manhattan_distance(self.start, steps, self.size) {
            if let Some(Soil) = self.grid.get(&point) {
                let man_dist_even = manhattan_distance(self.start, point) % 2 == 0;
                if even_steps == man_dist_even {
                    seen.insert(point);
                }
            }
        }
        self.print_seen(&seen);
        seen.len() as i64
    }

    fn print_seen(&self, seen: &HashSet<Point>) {
        for y in 0..self.size {
            let mut row = Vec::new();
            for x in 0..self.size {
                let point = &Point { x, y };
                if seen.get(point).is_some() {
                    row.push('0');
                } else {
                    match self.grid.get(point).unwrap() {
                        Soil => row.push('.'),
                        Rock => row.push('#'),
                    }
                }
            }
            let row: String = row.iter().collect();
            println!("{row}");
        }
    }
}
fn manhattan_distance(a: Point, b: Point) -> u64 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}
fn translate_to_tile(point: Point, grid_size: i64) -> Point {
    Point {
        x: point.x.rem_euclid(grid_size),
        y: point.y.rem_euclid(grid_size),
    }
}
fn points_within_manhattan_distance(
    start: Point,
    steps: i64,
    grid_size: i64,
) -> impl Iterator<Item = Point> {
    (-steps..=steps)
        .flat_map(move |x| {
            (-steps..=steps).filter_map(move |y| {
                if x.abs() + y.abs() <= steps {
                    Some(Point {
                        x: start.x + x,
                        y: start.y + y,
                    })
                } else {
                    None
                }
            })
        })
        .map(move |point| translate_to_tile(point, grid_size))
}

pub fn parse(input: &str) -> Garden {
    let mut grid = HashMap::new();
    let mut start = Point { x: 0, y: 0 };
    let mut size = 0;
    for (y, row) in input.lines().enumerate() {
        size = size.max(row.len());
        for (x, plot) in row.chars().enumerate() {
            let point = Point {
                x: x as i64,
                y: y as i64,
            };
            if plot == '.' {
                grid.insert(point, Soil);
            } else if plot == '#' {
                grid.insert(point, Rock);
            } else if plot == 'S' {
                start = point;
                grid.insert(point, Soil);
            } else {
                panic!("unhandled plot {plot}")
            }
        }
    }
    Garden {
        grid,
        start,
        size: size as i64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    const INPUT: &str = include_str!("../../input/2023/21.txt");

    #[test]
    fn test_1_sample() {
        let garden = parse(SAMPLE);

        assert_eq!(garden.reachable_soil(6), 16);
    }

    #[test]
    fn test_1() {
        let garden = parse(INPUT);

        assert_eq!(garden.reachable_soil(64), 3_816);
    }

    #[test]
    #[ignore]
    fn test_2_sample() {
        let garden = parse(SAMPLE);

        assert_eq!(garden.reachable_soil_math(6), 16);
        assert_eq!(garden.reachable_soil_math(10), 50);
        assert_eq!(garden.reachable_soil_math(100), 6_536);
        assert_eq!(garden.reachable_soil_math(500), 167_004);
        assert_eq!(garden.reachable_soil_math(1_000), 668_697);
        assert_eq!(garden.reachable_soil_math(5_000), 16_733_044);
    }

    #[test]
    #[ignore]
    fn test_2() {
        let garden = parse(INPUT);

        assert_eq!(garden.reachable_soil_math(26_501_365), 3_816);
    }
}
