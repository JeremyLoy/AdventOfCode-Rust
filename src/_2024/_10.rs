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
        trail_head: Point,
        start_elevation: u8,
    ) -> (HashSet<Point>, usize) {
        let mut p1 = HashSet::new();
        let mut p2 = 0;
        let mut stack = vec![(trail_head, start_elevation)];
        while let Some((point, elevation)) = stack.pop() {
            for neighbor in point.neighbors() {
                if let Some(&neighbor_value) = grid.get(&neighbor) {
                    if neighbor_value == elevation + 1 {
                        stack.push((neighbor, neighbor_value));
                    }
                }
            }
            if elevation == 9 {
                // p1 is the unique 9s seen per trailhead
                // p2 is the total number of paths to a 9 from a trailhead
                p1.insert(point);
                p2 += 1;
            }
        }
        (p1, p2)
    }

    grid.iter()
        .filter(|(_, &value)| value == 0)
        .map(|(&trail_head, &value)| {
            let (p1, p2) = traverse_trail(grid, trail_head, value);

            match scoring_method {
                ScoringMethod::Unique9s => p1.len(),
                ScoringMethod::UniquePaths => p2,
            }
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
