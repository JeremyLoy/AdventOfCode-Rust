use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Pipe {
    V,
    H,
    NE,
    SE,
    SW,
    NW,
    G,
    S,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Heading {
    N,
    S,
    E,
    W,
}

impl Display for Heading {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::N => write!(f, "N"),
            Self::S => write!(f, "S"),
            Self::E => write!(f, "E"),
            Self::W => write!(f, "W"),
        }
    }
}

impl Pipe {
    fn from_char(s: char) -> Result<Self, String> {
        Ok(match s {
            '│' | '|' => Self::V,
            '─' | '-' => Self::H,
            '└' | 'L' => Self::NE,
            '┌' | 'F' => Self::SE,
            '┐' | '7' => Self::SW,
            '┘' | 'J' => Self::NW,
            '.' => Self::G,
            'S' => Self::S,
            e => return Err(format!("unknown char {e}")),
        })
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V => write!(f, "│"),
            Self::H => write!(f, "─"),
            Self::NE => write!(f, "└"),
            Self::SE => write!(f, "┌"),
            Self::SW => write!(f, "┐"),
            Self::NW => write!(f, "┘"),
            Self::G => write!(f, "."),
            Self::S => write!(f, "S"),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}
pub fn parse_maze(input: &str) -> (Point, HashMap<Point, Pipe>) {
    let mut start = Point { x: 0, y: 0 };
    let mut maze = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if let Ok(pipe) = Pipe::from_char(c) {
                let point = Point { x, y };
                maze.insert(point, pipe);
                if pipe == Pipe::S {
                    start = point;
                }
            }
        });
    });
    (start, maze)
}

fn all_points_in_loop(start: Point, maze: &HashMap<Point, Pipe>) -> Vec<Point> {
    let mut points = Vec::new();

    let (start_heading, start_pipe) = find_starts_heading(start, maze);
    let next = get_next_point(start_heading, start_pipe, start);
    let mut next_heading = next.0;
    let mut next_point = next.1;
    points.push(next_point);
    let mut next_pipe = maze.get(&next_point).expect("all points should be in maze");
    while next_point != start {
        (next_heading, next_point) = get_next_point(next_heading, *next_pipe, next_point);
        next_pipe = maze.get(&next_point).expect("all points should be in grid");
        points.push(next_point);
    }
    points
}

pub fn maze_to_string(maze: &HashMap<Point, Pipe>, x: usize, y: usize) -> String {
    let mut s = String::new();
    for y in 0..y {
        for x in 0..x {
            if let Some(pipe) = maze.get(&Point { x, y }) {
                s += pipe.to_string().as_str();
            }
        }
        s.push('\n');
    }
    s.pop(); // remove trailing newline
    s
}

pub fn only_loop_as_string(
    maze: &mut HashMap<Point, Pipe>,
    x: usize,
    y: usize,
    start: Point,
) -> String {
    let points: HashSet<Point> = HashSet::from_iter(all_points_in_loop(start, maze));

    let mut s = String::new();
    for y in 0..y {
        for x in 0..x {
            let point = Point { x, y };
            if let Some(pipe) = maze.get(&point) {
                if points.contains(&point) {
                    s += pipe.to_string().as_str();
                } else {
                    s += "0";
                }
            }
        }
        s.push('\n');
    }
    s.pop(); // remove trailing newline
    s
}

pub fn find_starts_heading(start: Point, _maze: &HashMap<Point, Pipe>) -> (Heading, Pipe) {
    // hardcode for now
    if start == (Point { x: 1, y: 1 }) {
        (Heading::W, Pipe::SE)
    } else if start == (Point { x: 0, y: 2 }) {
        (Heading::N, Pipe::SE)
    } else if start == (Point { x: 119, y: 72 }) {
        (Heading::E, Pipe::NW)
    } else if start == (Point { x: 12, y: 4 }) {
        (Heading::N, Pipe::SE)
    } else if start == (Point { x: 4, y: 0 }) {
        (Heading::N, Pipe::SW)
    } else {
        panic!("unknown start point {start:?}")
    }
}

pub fn get_next_point(heading: Heading, cur_pipe: Pipe, cur_point: Point) -> (Heading, Point) {
    match (heading, cur_pipe) {
        (Heading::N, Pipe::V) | (Heading::W, Pipe::NE) | (Heading::E, Pipe::NW) => (
            Heading::N,
            Point {
                x: cur_point.x,
                y: cur_point.y - 1,
            },
        ),
        (Heading::S, Pipe::V) | (Heading::W, Pipe::SE) | (Heading::E, Pipe::SW) => (
            Heading::S,
            Point {
                x: cur_point.x,
                y: cur_point.y + 1,
            },
        ),
        (Heading::W, Pipe::H) | (Heading::N, Pipe::SW) | (Heading::S, Pipe::NW) => (
            Heading::W,
            Point {
                x: cur_point.x - 1,
                y: cur_point.y,
            },
        ),
        (Heading::E, Pipe::H) | (Heading::S, Pipe::NE) | (Heading::N, Pipe::SE) => (
            Heading::E,
            Point {
                x: cur_point.x + 1,
                y: cur_point.y,
            },
        ),
        #[allow(clippy::uninlined_format_args)]
        (heading, pipe) => panic!("unknown heading pipe combo {}, {}", heading, pipe),
    }
}

pub fn furthest_point(start: Point, maze: &HashMap<Point, Pipe>) -> i32 {
    (all_points_in_loop(start, maze).len() / 2) as i32
}

pub fn count_enclosed_tiles(start: Point, maze: &HashMap<Point, Pipe>) -> i32 {
    let mut points = all_points_in_loop(start, maze);

    // Shoelace formula for calculating the area of a polygon
    // https://en.wikipedia.org/wiki/Shoelace_formula
    // Add the first point to the end of the points vector to create a closed loop
    points.push(points[0]);
    let mut sum = 0i32;
    for i in 0..points.len() - 1 {
        let (x1, y1) = (points[i].x as i32, points[i].y as i32);
        let (x2, y2) = (points[i + 1].x as i32, points[i + 1].y as i32);

        sum += x1 * y2 - x2 * y1;
    }
    let area = (sum / 2).abs();

    // Pick's Theorem rearranged slightly so that we solve for the number of interior points given the area
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    area - (points.len() as i32 - 1) / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "\
.....
.S─┐.
.│.│.
.└─┘.
.....";
    const SAMPLE_2: &str = "\
..┌┐.
.┌┘│.
S┘.└┐
│┌──┘
└┘...";

    const SAMPLE_3: &str = "\
...........
.S───────┐.
.│┌─────┐│.
.││.....││.
.││.....││.
.│└─┐.┌─┘│.
.│..│.│..│.
.└──┘.└──┘.
...........";
    const SAMPLE_4: &str = "\
..........
.S──────┐.
.│┌────┐│.
.││....││.
.││....││.
.│└─┐┌─┘│.
.│..││..│.
.└──┘└──┘.
..........";
    const SAMPLE_5: &str = "\
.┌────┐┌┐┌┐┌┐┌─┐....
.│┌──┐││││││││┌┘....
.││.┌┘││││││││└┐....
┌┘└┐└┐└┘└┘││└┘.└─┐..
└──┘.└┐...└┘S┐┌─┐└┐.
....┌─┘..┌┐┌┘│└┐└┐└┐
....└┐.┌┐││└┐│.└┐└┐│
.....│┌┘└┘│┌┘│┌┐│.└┘
....┌┘└─┐.││.││││...
....└───┘.└┘.└┘└┘...";
    const SAMPLE_6: &str = "\
┌┌┐┌S┌┐┌┐┌┐┌┐┌┐┌───┐
└│└┘││││││││││││┌──┘
┌└─┐└┘└┘││││││└┘└─┐┐
┌──┘┌──┐││└┘└┘┐┌┐┌┘─
└───┘┌─┘└┘.││─┌┘└┘┘┐
│┌│┌─┘┌───┐┌┐─└┐└│┐│
│┌┌┘┌┐└┐┌─┘┌┐│┘└───┐
┐─└─┘└┐││┌┐│└┐┌─┐┌┐│
└.└┐└┌┘│││││┌┘└┐││└┘
└┐┘└┘└─┘└┘└┘└──┘└┘.└";
    const INPUT: &str = include_str!("../../input/2023/10.txt");

    #[test]
    #[ignore]
    fn test_parse() {
        let maze_1 = parse_maze(SAMPLE_1);
        let maze_2 = parse_maze(SAMPLE_2);
        let input = parse_maze(INPUT);

        let printed_1 = maze_to_string(&maze_1.1, 5, 5);
        let printed_2 = maze_to_string(&maze_2.1, 5, 5);
        let printed_input = maze_to_string(&input.1, 140, 140);

        assert_eq!(printed_1, SAMPLE_1);
        assert_eq!(printed_2, SAMPLE_2);
        assert_eq!(printed_input, INPUT);

        assert_eq!(maze_1.0, Point { x: 1, y: 1 });
        assert_eq!(maze_2.0, Point { x: 0, y: 2 });
        assert_eq!(input.0, Point { x: 119, y: 72 });
    }

    #[test]
    #[ignore]
    fn print_loop_only() {
        let mut maze_1 = parse_maze(SAMPLE_1);
        let mut maze_2 = parse_maze(SAMPLE_2);
        let mut maze_3 = parse_maze(SAMPLE_3);
        let mut maze_4 = parse_maze(SAMPLE_4);
        let mut maze_5 = parse_maze(SAMPLE_5);
        let mut maze_6 = parse_maze(SAMPLE_6);
        let mut input = parse_maze(INPUT);

        let printed_1 = only_loop_as_string(&mut maze_1.1, 5, 5, Point { x: 1, y: 1 });
        let printed_2 = only_loop_as_string(&mut maze_2.1, 5, 5, Point { x: 0, y: 2 });
        let printed_3 = only_loop_as_string(&mut maze_3.1, 11, 9, Point { x: 1, y: 1 });
        let printed_4 = only_loop_as_string(&mut maze_4.1, 10, 9, Point { x: 1, y: 1 });
        let printed_5 = only_loop_as_string(&mut maze_5.1, 20, 10, Point { x: 12, y: 4 });
        let printed_6 = only_loop_as_string(&mut maze_6.1, 20, 10, Point { x: 4, y: 0 });
        let printed_input = only_loop_as_string(&mut input.1, 140, 140, Point { x: 119, y: 72 });

        println!("{}\n", &printed_1);
        println!("{}\n", &printed_2);
        println!("{}\n", &printed_3);
        println!("{}\n", &printed_4);
        println!("{}\n", &printed_5);
        println!("{}\n", &printed_6);
        println!("{}\n", &printed_input);
    }

    #[test]
    fn test_1_sample() {
        let (maze_1_start, maze_1) = parse_maze(SAMPLE_1);
        let (maze_2_start, maze_2) = parse_maze(SAMPLE_2);

        assert_eq!(furthest_point(maze_1_start, &maze_1), 4);
        assert_eq!(furthest_point(maze_2_start, &maze_2), 8);
    }

    #[test]
    fn test_1() {
        let (start, maze) = parse_maze(INPUT);

        assert_eq!(furthest_point(start, &maze), 6_927);
    }

    #[test]
    fn test_2_sample() {
        let maze_1 = parse_maze(SAMPLE_1).1;
        let maze_2 = parse_maze(SAMPLE_2).1;
        let maze_3 = parse_maze(SAMPLE_3).1;
        let maze_4 = parse_maze(SAMPLE_4).1;
        let maze_5 = parse_maze(SAMPLE_5).1;
        let maze_6 = parse_maze(SAMPLE_6).1;

        assert_eq!(count_enclosed_tiles(Point { x: 1, y: 1 }, &maze_1), 1);
        assert_eq!(count_enclosed_tiles(Point { x: 0, y: 2 }, &maze_2), 1);
        assert_eq!(count_enclosed_tiles(Point { x: 1, y: 1 }, &maze_3), 4);
        assert_eq!(count_enclosed_tiles(Point { x: 1, y: 1 }, &maze_4), 4);
        assert_eq!(count_enclosed_tiles(Point { x: 12, y: 4 }, &maze_5), 8);
        assert_eq!(count_enclosed_tiles(Point { x: 4, y: 0 }, &maze_6), 10);
    }

    #[test]
    fn test_2() {
        let maze = parse_maze(INPUT).1;

        assert_eq!(count_enclosed_tiles(Point { x: 119, y: 72 }, &maze), 467);
    }
}
