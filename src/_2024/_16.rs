use anyhow::{anyhow, Result};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::{BinaryHeap, HashSet};
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::Up => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

pub struct Maze {
    pub start: Point,
    pub end: Point,
    pub grid: HashMap<Point, char>,
}

#[derive(PartialEq, Eq)]
struct Node {
    point: Point,
    cost: i32,
    priority: i32,
    direction: Direction,
    path: Vec<Point>,
}

// Flip the ordering to make BinaryHeap a min-heap/priority queue
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Maze {
    pub fn walk_from_start(&self) -> Option<(i32, i32)> {
        let mut priority_queue = BinaryHeap::new();
        let mut visited = HashMap::new();
        let mut cost = i32::MAX;
        let mut unique_tiles: HashSet<Point> = HashSet::new();

        // Initialize the heap with the starting position and direction Right
        priority_queue.push(Node {
            point: self.start,
            cost: 0,
            priority: self.manhattan(self.start),
            direction: Direction::Right,
            path: vec![self.start],
        });

        while let Some(current) = priority_queue.pop() {
            // If we reached the end point
            if current.point == self.end {
                if current.cost < cost {
                    unique_tiles.clear();
                    cost = current.cost;
                }
                if current.cost == cost {
                    for tile in &current.path {
                        unique_tiles.insert(*tile);
                    }
                }
            }

            // Skip if we've been here at a lower cost
            if visited
                .get(&(current.point, current.direction))
                .filter(|&&c| c < current.cost)
                .is_some()
            {
                continue;
            }

            visited.insert((current.point, current.direction), current.cost);

            // Generate neighbors
            for &next_direction in &[
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let next_point = current.point + next_direction;

                // Skip if next_point is not walkable (e.g., walls)
                if let Some('#') = self.grid.get(&next_point) {
                    continue;
                }

                let direction_change_cost = if next_direction == current.direction {
                    0
                } else {
                    1_000
                };
                let next_cost = current.cost + 1 + direction_change_cost;

                let mut next_path: Vec<_> = current.path.clone();
                next_path.push(next_point);
                priority_queue.push(Node {
                    point: next_point,
                    cost: next_cost,
                    priority: next_cost + self.manhattan(next_point),
                    direction: next_direction,
                    path: next_path,
                });
            }
        }

        Some((cost, unique_tiles.len() as i32))
    }

    fn manhattan(&self, point: Point) -> i32 {
        (self.end.x - point.x).abs() + (self.end.y - point.y).abs()
    }
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = HashMap::new();
        let mut start = None;
        let mut end = None;

        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };
                grid.insert(point, ch);

                match ch {
                    'S' => start = Some(point),
                    'E' => end = Some(point),
                    _ => (),
                }
            }
        }

        let start = start.ok_or_else(|| anyhow!("Missing start point 'S'"))?;
        let end = end.ok_or_else(|| anyhow!("Missing end point 'E'"))?;

        Ok(Maze { start, end, grid })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const SAMPLE_2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";
    const INPUT: &str = include_str!("../../input/2024/16.txt");

    #[test]
    fn test_1_sample() {
        let input: Maze = SAMPLE.parse().unwrap();
        let score = input.walk_from_start().unwrap().0;

        assert_eq!(score, 7_036);
    }

    #[test]
    fn test_1_sample_2() {
        let input: Maze = SAMPLE_2.parse().unwrap();
        let score = input.walk_from_start().unwrap().0;

        assert_eq!(score, 11_048);
    }

    #[test]
    fn test_1() {
        let input: Maze = INPUT.parse().unwrap();
        let score = input.walk_from_start().unwrap().0;

        assert_eq!(score, 90_440);
    }

    #[test]
    fn test_2_sample() {
        let input: Maze = SAMPLE.parse().unwrap();
        let score = input.walk_from_start().unwrap().1;

        assert_eq!(score, 45);
    }

    #[test]
    fn test_2_sample_2() {
        let input: Maze = SAMPLE_2.parse().unwrap();
        let score = input.walk_from_start().unwrap().1;

        assert_eq!(score, 64);
    }

    #[test]
    fn test_2() {
        let input: Maze = INPUT.parse().unwrap();
        let score = input.walk_from_start().unwrap().1;

        assert_eq!(score, 479);
    }
}
