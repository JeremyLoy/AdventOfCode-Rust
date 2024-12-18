use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::{BinaryHeap, HashSet};
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.x.cmp(&other.x) {
            std::cmp::Ordering::Equal => self.y.cmp(&other.y),
            other => other,
        }
    }
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

pub struct MemorySpace {
    pub width: i32,
    pub height: i32,
    pub fallen: HashSet<Point>,
    pub bytes: Vec<Point>,
}

impl MemorySpace {
    pub fn new(width: i32, height: i32, input: &str) -> Self {
        let bytes = input
            .lines()
            .map(|l| {
                let (x, y) = l.split_once(',').unwrap();
                Point {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect();

        MemorySpace {
            width,
            height,
            bytes,
            fallen: HashSet::new(),
        }
    }

    pub fn fall(&mut self, idx: usize) {
        self.fallen.clear();
        self.bytes[..=idx].iter().for_each(|p| {
            self.fallen.insert(*p);
        });
    }

    fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x <= self.width && point.y >= 0 && point.y <= self.height
    }

    pub fn most_fallen(&mut self) -> Option<Point> {
        let mut low = 0;
        let mut high = self.bytes.len() - 1;
        let mut result = None;

        while low <= high {
            let mid = (low + high) >> 1;
            self.fall(mid);
            if self.shortest_path().is_none() {
                result = Some(mid);
                high = mid - 1; // Search in the lower half
            } else {
                low = mid + 1; // Search in the upper half
            }
        }

        result.map(|idx| self.bytes[idx])
    }

    pub fn shortest_path(&self) -> Option<i32> {
        fn manhattan_distance(a: Point, b: Point) -> i32 {
            (a.x - b.x).abs() + (a.y - b.y).abs()
        }

        let start = Point { x: 0, y: 0 };
        let goal = Point {
            x: self.width,
            y: self.height,
        };

        // Directions to move
        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(Reverse((0, 0, start))); // (num_steps + manhattan, num_steps, point)

        let mut min_steps_by_points = HashMap::new();
        min_steps_by_points.insert(start, 0);

        while let Some(Reverse((_, current_steps, current))) = priority_queue.pop() {
            if current == goal {
                return Some(current_steps);
            }

            for &direction in &directions {
                let neighbor = current + direction;

                // Ignore points out of bounds or inaccessible ('#')
                if !self.in_bounds(neighbor) || self.fallen.contains(&neighbor) {
                    continue;
                }

                let tentative_next_steps = current_steps + 1;
                if tentative_next_steps < *min_steps_by_points.get(&neighbor).unwrap_or(&i32::MAX) {
                    min_steps_by_points.insert(neighbor, tentative_next_steps);

                    let heuristic_with_steps =
                        tentative_next_steps + manhattan_distance(neighbor, goal);
                    priority_queue.push(Reverse((
                        heuristic_with_steps,
                        tentative_next_steps,
                        neighbor,
                    )));
                }
            }
        }

        None // No path found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
    const INPUT: &str = include_str!("../../input/2024/18.txt");

    #[test]
    fn test_1_sample() {
        let mut memory_space = MemorySpace::new(6, 6, SAMPLE);
        memory_space.fall(12 - 1);
        let steps = memory_space.shortest_path().unwrap();

        assert_eq!(steps, 22);
    }

    #[test]
    fn test_1() {
        let mut memory_space = MemorySpace::new(70, 70, INPUT);
        memory_space.fall(1_024 - 1);
        let steps = memory_space.shortest_path().unwrap();

        assert_eq!(steps, 250);
    }

    #[test]
    fn test_2_sample() {
        let mut memory_space = MemorySpace::new(6, 6, SAMPLE);
        let point = memory_space.most_fallen().unwrap();
        assert_eq!(point, Point { x: 6, y: 1 });
    }

    #[test]
    fn test_2() {
        let mut memory_space = MemorySpace::new(70, 70, INPUT);
        let point = memory_space.most_fallen().unwrap();
        assert_eq!(point, Point { x: 56, y: 8 });
    }
}
