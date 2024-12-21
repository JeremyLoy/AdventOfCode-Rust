use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn neighbors(&self) -> Vec<Point> {
        vec![
            Point {
                x: self.x,
                y: self.y - 1,
            }, // North
            Point {
                x: self.x,
                y: self.y + 1,
            }, // South
            Point {
                x: self.x - 1,
                y: self.y,
            }, // West
            Point {
                x: self.x + 1,
                y: self.y,
            }, // East
        ]
    }
}

pub struct Racetrack {
    pub start: Point,
    pub end: Point,
    pub height: usize,
    pub width: usize,
    pub walls: HashSet<Point>,
}

fn manhattan(p1: Point, p2: Point) -> usize {
    (p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y)) as usize
}

impl Racetrack {
    pub fn solve(&mut self, cheat_size: usize, min_time_saved: usize) -> usize {
        let original_track = self.get_track();

        let cheats = self.find_cheats(&original_track, cheat_size, min_time_saved);

        cheats
            .iter()
            // there is no need to redo any sort of pathfinding
            // the amount of time saved is the index of cheat start - index of cheat end - manhattan(start,end)
            //
            // #####      #####
            // #...#      #...#
            // #S#E#  --> #S-E#
            // #####      #####
            //
            // J would be 4 and i would be 0. The manhattan distance between the two is 2
            // 4 - 2 = 2 seconds saved
            .filter(|(i, j, distance)| j - i - distance >= min_time_saved)
            .count()
    }
    pub fn find_cheats(
        &self,
        path: &[Point],
        cheat_size: usize,
        min_time_saved: usize,
    ) -> Vec<(usize, usize, usize)> {
        let mut cheats = Vec::new();
        let last_possible_cheat = path.len() - 1 - min_time_saved;

        for (i, &p1) in path[..last_possible_cheat].iter().enumerate() {
            // only look forward along the path, and you can skip some amount
            // of checking so that it is not quite n^2
            //
            // We want at least min_time_saved, so there is no need to check anything on path beyond that index
            //
            // Additionally, the shortest possible cheat looks like this
            //
            // #####      #####
            // #...#      #...#
            // #S#E#  --> #S-E#
            // #####      #####
            //
            // the minimum possible j is i + 4. i + 1->3 will never be a cheat
            // - i+1 is the immediate neighbor and would pass through no walls
            // - i+2 and i+3 could be reached through a wall, but would not save time
            for (j, &p2) in path.iter().enumerate().skip(i + 4) {
                let distance = manhattan(p1, p2);
                if distance <= cheat_size {
                    cheats.push((i, j, distance));
                }
            }
        }

        cheats
    }
    pub fn print_path(&self, path: &HashSet<Point>) {
        for y in 0..=self.height as i32 {
            for x in 0..=self.width as i32 {
                let point = Point { x, y };
                if self.start == point {
                    print!("S");
                } else if self.end == point {
                    print!("E");
                } else if self.walls.contains(&point) {
                    print!("#");
                } else if path.contains(&point) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!(); // Move to the next line in the grid
        }
    }
    pub fn in_bounds(&self, point: &Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && point.x <= self.width as i32
            && point.y <= self.height as i32
    }

    // simplified floodfill as there is only ever one path with no
    pub fn get_track(&self) -> Vec<Point> {
        let mut track = Vec::new();
        let mut seen = HashSet::new();
        track.push(self.start);
        seen.insert(self.start);

        let mut current = self.start;
        while let Some(next) = current
            .neighbors()
            .into_iter()
            .find(|p| self.in_bounds(p) && !self.walls.contains(p) && !seen.contains(p))
        {
            current = next;
            track.push(next);
            seen.insert(next);
        }
        track
    }
}

pub fn parse(input: &str) -> Racetrack {
    let mut walls = HashSet::new();
    let mut start = None;
    let mut end = None;
    let mut height = 0;
    let mut width = 0;

    for (y, line) in input.lines().enumerate() {
        height = height.max(y);
        for (x, c) in line.chars().enumerate() {
            width = width.max(x);
            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            match c {
                '#' => {
                    walls.insert(point);
                }
                'S' => {
                    start = Some(point);
                }
                'E' => {
                    end = Some(point);
                }
                _ => {}
            }
        }
    }

    Racetrack {
        start: start.expect("No start point"),
        end: end.expect("No end point"),
        height,
        width,
        walls,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";
    const INPUT: &str = include_str!("../../input/2024/20.txt");

    #[test]
    fn test_1_sample() {
        let mut racetrack = parse(SAMPLE);

        assert_eq!(racetrack.solve(2, 2), 44);
    }

    #[test]
    fn test_1() {
        let mut racetrack = parse(INPUT);

        assert_eq!(racetrack.solve(2, 100), 1_399);
    }

    #[test]
    fn test_2_sample() {
        let mut racetrack = parse(SAMPLE);

        assert_eq!(racetrack.solve(20, 50), 285);
    }

    #[test]
    fn test_2() {
        let mut racetrack = parse(INPUT);

        assert_eq!(racetrack.solve(20, 100), 994_807);
    }
}
