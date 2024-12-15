use anyhow::{anyhow, Result};
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}
impl Sub<Direction> for Point {
    type Output = Point;

    fn sub(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::Up => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Right => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}
impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, direction: Direction) -> Point {
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

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_char(c: char) -> Result<Self> {
        match c {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(anyhow!("char {c} is not a direction")),
        }
    }

    pub fn vec_from_str(s: &str) -> Result<Vec<Self>> {
        s.chars()
            .filter(|c| !c.is_whitespace())
            .map(Self::from_char)
            .collect()
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Tile {
    Wall,
    Box,
    Robot,
    Empty,
    BoxLeft,
    BoxRight,
}

impl std::str::FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('@') => Ok(Tile::Robot),
            Some('#') => Ok(Tile::Wall),
            Some('.') => Ok(Tile::Empty),
            Some('O') => Ok(Tile::Box),
            Some('[') => Ok(Tile::BoxLeft),
            Some(']') => Ok(Tile::BoxRight),
            Some(c) => Err(anyhow!("unexpected character: {c}")),
            None => Err(anyhow!("empty string")),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Robot => '@',
            Tile::Wall => '#',
            Tile::Empty => '.',
            Tile::Box => 'O',
            Tile::BoxLeft => '[',
            Tile::BoxRight => ']',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug)]
pub struct Warehouse {
    grid: HashMap<Point, Tile>,
    directions: Vec<Direction>,
}

impl Warehouse {
    pub fn expand(&mut self) {
        let mut new_grid = HashMap::new();

        for (point, tile) in &self.grid {
            let point = Point {
                x: point.x * 2,
                y: point.y,
            };
            match tile {
                Tile::Wall => {
                    // Add two walls
                    new_grid.insert(point, Tile::Wall);
                    new_grid.insert(
                        Point {
                            x: point.x + 1,
                            y: point.y,
                        },
                        Tile::Wall,
                    );
                }
                Tile::Robot => {
                    // Add Robot and Empty
                    new_grid.insert(point, Tile::Robot);
                    new_grid.insert(
                        Point {
                            x: point.x + 1,
                            y: point.y,
                        },
                        Tile::Empty,
                    );
                }
                Tile::Box => {
                    // Add BoxLeft and BoxRight as Box for simplicity (adjust if further enums are needed)
                    new_grid.insert(point, Tile::BoxLeft);
                    new_grid.insert(
                        Point {
                            x: point.x + 1,
                            y: point.y,
                        },
                        Tile::BoxRight,
                    );
                }
                Tile::Empty => {
                    // Add two Empty tiles
                    new_grid.insert(point, Tile::Empty);
                    new_grid.insert(
                        Point {
                            x: point.x + 1,
                            y: point.y,
                        },
                        Tile::Empty,
                    );
                }
                // these tiles do not exist pre-expansion
                Tile::BoxLeft | Tile::BoxRight => unreachable!(),
            }
        }

        // Update the warehouse grid with the expanded one
        self.grid = new_grid;
    }
    pub fn advance_robot(&mut self) {
        let mut robot_position = self
            .robot_position()
            .expect("Robot not found in the warehouse grid");

        for direction in &self.directions {
            let mut potential_moves = Vec::new();
            let mut current_position = robot_position;

            loop {
                let new_position = current_position + *direction;

                if let Some(tile) = self.grid.get(&new_position) {
                    match tile {
                        Tile::Wall => break, // Stop when a wall is encountered
                        Tile::Box => potential_moves.push((new_position, *tile)),
                        Tile::Empty => {
                            potential_moves.push((new_position, *tile));
                            break;
                        }
                        Tile::BoxLeft | Tile::BoxRight | Tile::Robot => unreachable!(),
                    }
                } else {
                    break; // Stop if the position is out of bounds
                }
                current_position = new_position;
            }
            // if the immediate next position is a wall, or if the last position is not Empty
            // then the robot cannot advance and move boxes
            if potential_moves.is_empty() || potential_moves.last().unwrap().1 == Tile::Box {
                continue;
            }

            for (position, _) in potential_moves[1..].iter().rev() {
                self.grid.insert(*position, Tile::Box);
            }
            self.grid
                .insert(potential_moves.first().unwrap().0, Tile::Robot);
            self.grid.insert(robot_position, Tile::Empty);
            robot_position = potential_moves.first().unwrap().0;
        }
    }

    fn robot_position(&self) -> Option<Point> {
        self.grid.iter().find_map(|(point, tile)| {
            if *tile == Tile::Robot {
                Some(*point)
            } else {
                None
            }
        })
    }

    // similar logic to p1 for left and right. Just move every position over
    // p2 is a bit more tricky, I do a BFS search and stop successfully if all leaves are Empty.
    // if any leaf is a wall, nothing moves.
    // if all leaves are empty, then each tile is moved forward in the direction once.
    // sorting the tiles to move by farthest in the target direction makes the swaps trivial
    pub fn advance_robot_expanded(&mut self) {
        for direction in &self.directions {
            // inefficient, but I haven't figured out a way to cache it
            let robot_position = self
                .robot_position()
                .expect("Robot not found in the warehouse grid");
            // moving left and right is  nearly identical to p1. Haven't figured out a way around copy/paste here yet.
            if matches!(direction, Direction::Left | Direction::Right) {
                let mut pieces_to_move = VecDeque::new();
                let mut queue = VecDeque::new();
                queue.push_back(robot_position);

                while let Some(current_position) = queue.pop_front() {
                    let current_tile = self
                        .grid
                        .get(&current_position)
                        .expect("all positions should be in the grid");
                    match current_tile {
                        Tile::Empty => {
                            break;
                        }
                        Tile::Wall => {
                            // Stop the search for all branches since a wall is encountered
                            pieces_to_move.clear();
                            break;
                        }
                        Tile::Robot | Tile::BoxLeft | Tile::BoxRight => {
                            pieces_to_move.push_front(current_position);
                            queue.push_back(current_position + *direction);
                        }
                        Tile::Box => unreachable!("Box should not exist on an expanded Warehouse"),
                    }
                }
                for position in &pieces_to_move {
                    let current_tile = *self
                        .grid
                        .get(position)
                        .expect("every position to move should be in the grid");

                    self.grid.insert(*position + *direction, current_tile);
                    self.grid.insert(*position, Tile::Empty);
                }
            } else {
                // BFS
                let mut queue = VecDeque::new();
                let mut seen = HashSet::new();

                // Vector to keep track of the pieces to move during BFS
                // Add the new position
                let mut pieces_to_move: Vec<Point> = Vec::new();
                queue.push_back(robot_position);

                while let Some(current_position) = queue.pop_front() {
                    if !seen.insert(current_position) {
                        continue;
                    }
                    let current_tile = self
                        .grid
                        .get(&current_position)
                        .expect("all positions should be in the grid");
                    match current_tile {
                        Tile::Robot => {
                            pieces_to_move.push(current_position);
                            queue.push_back(current_position + *direction);
                            continue;
                        }
                        Tile::Empty => {
                            continue;
                        }
                        Tile::BoxLeft => {
                            pieces_to_move.push(current_position);
                            queue.push_back(current_position + *direction);
                            queue.push_back(current_position + Direction::Right);
                        }
                        Tile::BoxRight => {
                            pieces_to_move.push(current_position);
                            queue.push_back(current_position + *direction);
                            queue.push_back(current_position + Direction::Left);
                        }
                        Tile::Wall => {
                            // Stop the search for all branches since a wall is encountered
                            pieces_to_move.clear();
                            break;
                        }
                        Tile::Box => unreachable!("Box should not exist on an expanded Warehouse"),
                    }
                }

                pieces_to_move.sort_by_key(|position| match direction {
                    Direction::Up => position.y,
                    Direction::Down => -position.y,
                    _ => unreachable!("Direction should only be Up or Down in this context"),
                });
                for position in &pieces_to_move {
                    let current_tile = *self
                        .grid
                        .get(position)
                        .expect("every position to move should be in the grid");

                    self.grid.insert(*position + *direction, current_tile);
                    self.grid.insert(*position, Tile::Empty);
                }
            }
        }
    }
    pub fn sum_gps(&self) -> i32 {
        self.grid
            .iter()
            .map(|(point, tile)| {
                if matches!(tile, Tile::Box | Tile::BoxLeft) {
                    point.y * 100 + point.x
                } else {
                    0
                }
            })
            .sum()
    }
}

impl std::fmt::Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.grid.keys().map(|p| p.x).max().unwrap_or(0);
        let max_y = self.grid.keys().map(|p| p.y).max().unwrap_or(0);

        for y in 0..=max_y {
            for x in 0..=max_x {
                let tile = self.grid.get(&Point { x, y }).unwrap_or(&Tile::Empty);
                write!(f, "{tile}")?;
            }
            writeln!(f)?; // Newline after each row
        }

        Ok(())
    }
}

impl std::str::FromStr for Warehouse {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (grid_str, directions) = s
            .split_once("\n\n")
            .ok_or(anyhow!("could not split input into grid and directions"))?;

        let mut grid = HashMap::new();

        for (y, line) in grid_str.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };
                grid.insert(point, Tile::from_str(&ch.to_string())?);
            }
        }

        let directions = Direction::vec_from_str(directions)?;

        Ok(Warehouse { grid, directions })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    const SAMPLE_2: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const SAMPLE_3: &str = "\
    #######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    const SAMPLE_4: &str = "\
####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################

^";
    const INPUT: &str = include_str!("input/2024/15.txt");

    #[test]
    fn test_1_sample() {
        let mut warehouse: Warehouse = SAMPLE.parse().unwrap();
        warehouse.advance_robot();

        assert_eq!(warehouse.sum_gps(), 10_092);
    }

    #[test]
    fn test_1_sample_2() {
        let mut warehouse: Warehouse = SAMPLE_2.parse().unwrap();
        warehouse.advance_robot();

        assert_eq!(warehouse.sum_gps(), 2_028);
    }

    #[test]
    fn test_1() {
        let mut warehouse: Warehouse = INPUT.parse().unwrap();
        warehouse.advance_robot();

        assert_eq!(warehouse.sum_gps(), 1_437_174);
    }

    #[test]
    fn test_2_sample() {
        let mut warehouse: Warehouse = SAMPLE.parse().unwrap();
        warehouse.expand();
        warehouse.advance_robot_expanded();

        assert_eq!(warehouse.sum_gps(), 9_021);
    }

    #[test]
    fn test_2_sample_3() {
        let mut warehouse: Warehouse = SAMPLE_3.parse().unwrap();
        warehouse.expand();
        warehouse.advance_robot_expanded();

        assert_eq!(warehouse.sum_gps(), 618);
    }

    #[test]
    fn test_2_sample_4() {
        let warehouse: Warehouse = SAMPLE_4.parse().unwrap();

        assert_eq!(warehouse.sum_gps(), 9_021);
    }

    #[test]
    fn test_2() {
        let mut warehouse: Warehouse = INPUT.parse().unwrap();
        warehouse.expand();
        warehouse.advance_robot_expanded();

        assert_eq!(warehouse.sum_gps(), 1_437_468);
    }
}
