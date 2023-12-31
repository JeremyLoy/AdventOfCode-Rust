use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn neighbors(self) -> [Point; 4] {
        [
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
        ]
    }
}

pub struct SnowIsland {
    grid: HashMap<Point, char>,
    height: i32,
    width: i32,
}

impl FromStr for SnowIsland {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut height = 0;
        let mut width = 0;
        let grid = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                height = height.max(y as i32 + 1);
                width = width.max(line.len() as i32);
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        Point {
                            x: x as i32,
                            y: y as i32,
                        },
                        c,
                    )
                })
            })
            .collect::<HashMap<_, _>>();

        Ok(SnowIsland {
            grid,
            height,
            width,
        })
    }
}

impl SnowIsland {
    pub fn longest_climbing_path(&self) -> usize {
        0
    }
    pub fn longest_path(&self) -> usize {
        let start = Point { x: 1, y: 0 };
        let goal = Point {
            x: self.width - 2,
            y: self.height - 1,
        };

        let path = self.dfs(start, goal).into_iter().collect::<HashSet<_>>();

        // for y in 0..self.height {
        //     for x in 0..self.width {
        //         let point = &Point { x, y };
        //         if path.contains(point) {
        //             print!("O");
        //         } else {
        //             print!("{}", self.grid.get(point).unwrap());
        //         }
        //     }
        //     println!();
        // }

        // start doesn't count as taking a step
        path.len() - 1
    }
    fn dfs(&self, start: Point, goal: Point) -> Vec<Point> {
        let mut path_stack = VecDeque::new();
        let mut visited = HashSet::new();
        let mut longest_path = Vec::new();

        path_stack.push_back(vec![start]);

        while let Some(path) = path_stack.pop_front() {
            let current = *path.last().expect("no path should be empty");

            if !visited.insert(current) {
                continue;
            }

            if current == goal && path.len() > longest_path.len() {
                longest_path = path.clone();
            }

            for neighbor in self.valid_neighbors(current) {
                let mut new_path = path.clone();
                new_path.push(neighbor);
                path_stack.push_front(new_path);
            }
        }
        longest_path
    }
    fn valid_neighbors(&self, point: Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        for neighbor in point.neighbors() {
            match self.grid.get(&neighbor) {
                Some('.') => neighbors.push(neighbor),
                Some('>') => {
                    if neighbor.x == point.x + 1 {
                        neighbors.push(neighbor);
                    }
                }
                Some('<') => {
                    if neighbor.x == point.x - 1 {
                        neighbors.push(neighbor);
                    }
                }
                Some('v') => {
                    if neighbor.y == point.y + 1 {
                        neighbors.push(neighbor);
                    }
                }
                Some('^') => {
                    if neighbor.y == point.y - 1 {
                        neighbors.push(neighbor);
                    }
                }
                Some('#' | _) | None => (),
            }
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    const INPUT: &str = include_str!("../../input/2023/23.txt");

    #[test]
    fn test_1_sample() {
        let island: SnowIsland = SAMPLE.parse().unwrap();

        assert_eq!(island.longest_path(), 94);
    }

    #[test]
    fn test_1() {
        let island: SnowIsland = INPUT.parse().unwrap();

        assert_eq!(island.longest_path(), 2_334);
    }

    #[test]
    #[ignore]
    fn test_2_sample() {
        let island: SnowIsland = SAMPLE.parse().unwrap();

        assert_eq!(island.longest_climbing_path(), 154);
    }

    #[test]
    #[ignore]
    fn test_2() {
        let island: SnowIsland = INPUT.parse().unwrap();

        assert_eq!(island.longest_climbing_path(), 1 + 1);
    }
}
