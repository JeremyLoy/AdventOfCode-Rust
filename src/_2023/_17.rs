use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::ops::Add;
use Heading::{Down, Left, Right, Up};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct State {
    position: Point,
    heading: Heading,
    sequential_steps_in_heading: u32,
}

impl Add<Heading> for State {
    type Output = State;

    fn add(self, rhs: Heading) -> Self::Output {
        State {
            position: self.position + rhs,
            heading: rhs,
            sequential_steps_in_heading: if self.heading == rhs {
                self.sequential_steps_in_heading + 1
            } else {
                1
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Work {
    state: State,
    heat_loss: u32,
}

impl Ord for Work {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for Work {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn djikstra(
    city: &City,
    start: Point,
    goal: Point,
    min_steps: u32,
    is_valid_next_move: fn(State, Heading) -> bool,
) -> Option<u32> {
    // A min-heap of states to explore, with the ones having the lowest cost at the top.
    let mut frontier: BinaryHeap<Work> = BinaryHeap::new();
    let mut seen: HashSet<State> = HashSet::new();

    // let initial_cost = 0;
    let initial_state = State {
        position: start,
        heading: Right,
        sequential_steps_in_heading: 0,
    };
    frontier.push(Work {
        state: initial_state,
        heat_loss: 0,
    });
    seen.insert(initial_state);

    while let Some(current) = frontier.pop() {
        if current.state.position == goal && current.state.sequential_steps_in_heading >= min_steps
        {
            // Early exit: the shortest path to goal is found.
            return Some(current.heat_loss);
        }

        // cannot move backwards
        let valid_headings = match current.state.heading {
            Up => [Up, Right, Left],
            Down => [Right, Down, Left],
            Left => [Up, Down, Left],
            Right => [Up, Right, Down],
        };
        // Examine all neighbors of the current position.
        for heading in valid_headings {
            let next_state = current.state + heading;
            if let Some(new_cost) = city.grid.get(&next_state.position) {
                if seen.get(&next_state).is_none() && is_valid_next_move(current.state, heading) {
                    seen.insert(next_state);
                    frontier.push(Work {
                        state: next_state,
                        heat_loss: current.heat_loss + new_cost,
                    });
                }
            }
        }
    }

    None // No path found
}

pub struct City {
    grid: HashMap<Point, u32>,
    width: usize,
    height: usize,
}

impl City {
    pub fn min_heat_loss(&self) -> u32 {
        djikstra(
            self,
            Point { x: 0, y: 0 },
            Point {
                x: self.width as isize,
                y: self.height as isize,
            },
            1,
            |s: State, h: Heading| s.sequential_steps_in_heading < 3 || s.heading != h,
        )
        .expect("there should always be a path to the goal")
    }
    pub fn min_heat_loss_ultra(&self) -> u32 {
        djikstra(
            self,
            Point { x: 0, y: 0 },
            Point {
                x: self.width as isize,
                y: self.height as isize,
            },
            4,
            |s: State, h: Heading| {
                if s.sequential_steps_in_heading > 9 {
                    s.heading != h
                } else if s.sequential_steps_in_heading < 4 {
                    s.heading == h
                } else {
                    true
                }
            },
        )
        .expect("there should always be a path to the goal")
    }
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.height {
            for x in 0..=self.width {
                f.write_char(
                    char::from_digit(
                        *self
                            .grid
                            .get(&Point {
                                x: x as isize,
                                y: y as isize,
                            })
                            .expect("all points should exist"),
                        10,
                    )
                    .expect("every weight should be a valid char"),
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Default)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Heading {
    Up,
    Down,
    Left,
    Right,
}

pub fn parse(input: &str) -> City {
    let mut grid = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    input.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, weight)| {
            height = usize::max(height, y);
            width = usize::max(width, x);
            grid.insert(
                Point {
                    x: x as isize,
                    y: y as isize,
                },
                weight.to_digit(10).expect("all weights should be digits"),
            );
        });
    });
    City {
        grid,
        width,
        height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    const SAMPLE_2: &str = "\
111111111111
999999999991
999999999991
999999999991
999999999991";
    const INPUT: &str = include_str!("../../input/2023/17.txt");

    #[test]
    fn test_1_sample() {
        let input = parse(SAMPLE);

        println!("{input}");

        assert_eq!(input.min_heat_loss(), 102);
    }

    #[test]
    fn test_1() {
        let input = parse(INPUT);

        assert_eq!(input.min_heat_loss(), 843);
    }

    #[test]
    fn test_2_sample() {
        let input = parse(SAMPLE);
        let input_2 = parse(SAMPLE_2);

        assert_eq!(input.min_heat_loss_ultra(), 94);
        assert_eq!(input_2.min_heat_loss_ultra(), 71);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT);

        assert_eq!(input.min_heat_loss_ultra(), 1_017);
    }
}
