use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Returns the neighbors of the point (N, S, E, W)
    pub fn neighbors(&self) -> [Point; 4] {
        [
            Point {
                x: self.x,
                y: self.y - 1,
            }, // North
            Point {
                x: self.x,
                y: self.y + 1,
            }, // South
            Point {
                x: self.x + 1,
                y: self.y,
            }, // East
            Point {
                x: self.x - 1,
                y: self.y,
            }, // West
        ]
    }

    // returns the 3 Points diagonal to this point
    // in order to check if it is a corner.
    // eg N+NE+E, S+SW+W
    pub fn corners(&self) -> [[Point; 3]; 4] {
        [
            [
                Point {
                    x: self.x,
                    y: self.y - 1,
                },
                Point {
                    x: self.x + 1,
                    y: self.y - 1,
                },
                Point {
                    x: self.x + 1,
                    y: self.y,
                },
            ], // NE
            [
                Point {
                    x: self.x,
                    y: self.y - 1,
                },
                Point {
                    x: self.x - 1,
                    y: self.y - 1,
                },
                Point {
                    x: self.x - 1,
                    y: self.y,
                },
            ], // NW
            [
                Point {
                    x: self.x + 1,
                    y: self.y,
                },
                Point {
                    x: self.x + 1,
                    y: self.y + 1,
                },
                Point {
                    x: self.x,
                    y: self.y + 1,
                },
            ], // SE
            [
                Point {
                    x: self.x - 1,
                    y: self.y,
                },
                Point {
                    x: self.x - 1,
                    y: self.y + 1,
                },
                Point {
                    x: self.x,
                    y: self.y + 1,
                },
            ], // SW
        ]
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn parse(input: &str) -> HashMap<Point, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    (
                        Point {
                            x: x as i32,
                            y: y as i32,
                        },
                        c,
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn find_vegetable_plots(grid: &HashMap<Point, char>) -> Vec<(Vec<Point>, char)> {
    let mut visited = HashSet::new();
    let mut plots: Vec<(Vec<Point>, char)> = Vec::new();

    for (&point, &value) in grid {
        if visited.contains(&point) {
            continue; // Already visited this point
        }

        let mut plot = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_front(point);

        while let Some(current) = queue.pop_back() {
            if visited.contains(&current) {
                continue; // Already visited this neighbor
            }

            visited.insert(current);
            plot.push(current);

            for neighbor in current.neighbors() {
                if !visited.contains(&neighbor) {
                    if let Some(&neighbor_value) = grid.get(&neighbor) {
                        if neighbor_value == value {
                            queue.push_front(neighbor);
                        }
                    }
                }
            }
        }

        if !plot.is_empty() {
            plots.push((plot, value));
        }
    }

    plots
}

pub fn perimeter(grid: &HashMap<Point, char>, group: &[Point], vegetable: char) -> usize {
    group
        .iter()
        .map(|point| {
            4 - point
                .neighbors()
                .iter()
                .filter(|p| grid.get(p).is_some_and(|v| *v == vegetable))
                .count()
        })
        .sum()
}

pub fn count_corners(grid: &HashMap<Point, char>, plot: &[Point], vegetable: char) -> usize {
    let mut corners = 0;

    for point in plot {
        for corner in point.corners() {
            let (left, corner, right) = corner.iter().map(|p| grid.get(p)).collect_tuple().unwrap();

            // ...
            // VV.
            // VV.   NE case, checking the middle
            let is_exterior = (left.is_some_and(|c| *c != vegetable) || left.is_none())
                & (right.is_some_and(|v| *v != vegetable) || right.is_none());

            // VV.
            // VVV
            // VVV NE case, checking the middle
            let is_interior = left.is_some_and(|c| *c == vegetable)
                && left == right
                && (corner.is_none() || corner.is_some_and(|c| *c != vegetable));

            if is_exterior || is_interior {
                corners += 1;
            }
        }
    }

    corners
}

pub fn sum_perimeter_area(grid: &HashMap<Point, char>, groupings: &[(Vec<Point>, char)]) -> usize {
    groupings
        .iter()
        .map(|(group, vegetable)| {
            let area = group.len();
            let perimeter = perimeter(grid, group, *vegetable);
            perimeter * area
        })
        .sum()
}

pub fn sum_sides_area(grid: &HashMap<Point, char>, plots: &[(Vec<Point>, char)]) -> usize {
    plots
        .iter()
        .map(|(plot, vegetable)| {
            let area = plot.len();
            let num_sides = count_corners(grid, plot, *vegetable);
            num_sides * area
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
    const SAMPLE_2: &str = "\
AAAA
BBCD
BBCC
EEEC
";
    const SAMPLE_3: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
    const SAMPLE_4: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
    const SAMPLE_5: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
    const INPUT: &str = include_str!("../../input/2024/12.txt");

    #[test]
    fn test_1_sample() {
        let input = parse(SAMPLE);
        let plots = find_vegetable_plots(&input);
        let sum = sum_perimeter_area(&input, &plots);

        assert_eq!(sum, 1_930);
    }

    #[test]
    fn test_1() {
        let input = parse(INPUT);
        let plots = find_vegetable_plots(&input);
        let sum = sum_perimeter_area(&input, &plots);

        assert_eq!(sum, 1_421_958);
    }

    #[test]
    fn test_2_sample() {
        let input = parse(SAMPLE);
        let plots = find_vegetable_plots(&input);
        let sum = sum_sides_area(&input, &plots);

        assert_eq!(sum, 1_206);
    }

    #[test]
    fn test_2_sample_2() {
        let input = parse(SAMPLE_2);
        let plots = find_vegetable_plots(&input);
        let sum = sum_sides_area(&input, &plots);

        assert_eq!(sum, 80);
    }

    #[test]
    fn test_2_sample_3() {
        let input = parse(SAMPLE_3);
        let plots = find_vegetable_plots(&input);
        let sum = sum_sides_area(&input, &plots);

        assert_eq!(sum, 236);
    }

    #[test]
    fn test_2_sample_4() {
        let input = parse(SAMPLE_4);
        let plots = find_vegetable_plots(&input);
        let sum = sum_sides_area(&input, &plots);

        assert_eq!(sum, 368);
    }

    #[test]
    fn test_2_sample_5() {
        let input = parse(SAMPLE_5);
        let plots = find_vegetable_plots(&input);
        let sum = sum_sides_area(&input, &plots);

        assert_eq!(sum, 436);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT);
        let plots = find_vegetable_plots(&input);
        let sum = sum_sides_area(&input, &plots);

        assert_eq!(sum, 885_394);
    }
}
