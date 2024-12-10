use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn neighbors(self) -> [Point; 4] {
        [
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn parse(input: &str) -> Result<HashMap<Point, u8>> {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if let Some(value) = ch.to_digit(10) {
                grid.insert(
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    value as u8,
                );
            } else {
                return Err(anyhow!(
                    "Invalid character in input at line {}, column {}: '{}'",
                    y,
                    x,
                    ch
                ));
            }
        }
    }
    Ok(grid)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScoringMethod {
    Unique9s,
    UniquePaths,
}

pub fn calculate_path_scores(grid: &HashMap<Point, u8>, scoring_method: ScoringMethod) -> usize {
    fn traverse_trail(
        grid: &HashMap<Point, u8>,
        point: Point,
        current_value: u8,
        path: &mut Vec<Point>,
        p1: &mut HashSet<Point>,
        p2: &mut usize,
    ) {
        if current_value == 9 {
            // p1 is the unique 9s seen per trailhead
            // p2 is the total number of paths to a 9 from a trailhead
            p1.insert(point);
            *p2 += 1;
            return;
        }

        for neighbor in point.neighbors() {
            if let Some(&neighbor_value) = grid.get(&neighbor) {
                if neighbor_value == current_value + 1 {
                    path.push(neighbor);
                    traverse_trail(grid, neighbor, neighbor_value, path, p1, p2);
                    path.pop();
                }
            }
        }
    }

    grid.iter()
        .filter(|(_, &value)| value == 0)
        .map(|(&point, &value)| {
            let mut p1: HashSet<_> = HashSet::new();
            let mut p2 = 0;
            traverse_trail(grid, point, value, &mut vec![point], &mut p1, &mut p2);
            (p1, p2)
        })
        .map(|(seen, score)| match scoring_method {
            ScoringMethod::Unique9s => seen.len(),
            ScoringMethod::UniquePaths => score,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    const INPUT: &str = include_str!("../../input/2024/10.txt");

    #[test]
    fn test_1_sample() {
        let input = parse(SAMPLE).unwrap();
        let score = calculate_path_scores(&input, ScoringMethod::Unique9s);

        assert_eq!(score, 36);
    }

    #[test]
    fn test_1() {
        let input = parse(INPUT).unwrap();
        let score = calculate_path_scores(&input, ScoringMethod::Unique9s);

        assert_eq!(score, 841);
    }

    #[test]
    fn test_2_sample() {
        let input = parse(SAMPLE).unwrap();
        let score = calculate_path_scores(&input, ScoringMethod::UniquePaths);

        assert_eq!(score, 81);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT).unwrap();
        let score = calculate_path_scores(&input, ScoringMethod::UniquePaths);

        assert_eq!(score, 1_875);
    }
}
