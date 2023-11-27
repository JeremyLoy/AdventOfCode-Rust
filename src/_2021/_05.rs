use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn parse_line_to_point(point_str: &str) -> Option<Self> {
        let (x_str, y_str) = point_str.split_once(",")?;
        let x = x_str.trim().parse::<i32>().ok()?;
        let y = y_str.trim().parse::<i32>().ok()?;
        Some(Point { x, y })
    }

    pub fn parse_line_to_pair(line: &str) -> Option<(Self, Self)> {
        let (start_str, end_str) = line.split_once("->")?;
        let start_point = Self::parse_line_to_point(start_str)?;
        let end_point = Self::parse_line_to_point(end_str)?;
        Some((start_point, end_point))
    }

    pub fn parse_batch(lines: impl Iterator<Item = String>) -> impl Iterator<Item = (Self, Self)> {
        lines
            .into_iter()
            .filter_map(|line| Self::parse_line_to_pair(&line))
    }
}

pub enum Diagonals {
    Include,
    Exclude,
}

pub fn plot_points(
    points: impl Iterator<Item = (Point, Point)>,
    plot_diagonals: Diagonals,
) -> HashMap<Point, i32> {
    let mut grid = HashMap::new();
    for (mut start, end) in points {
        if matches!(plot_diagonals, Diagonals::Exclude) && start.x != end.x && start.y != end.y {
            continue;
        }
        while start.x != end.x || start.y != end.y {
            let count = grid.entry(start).or_insert(0);
            *count += 1;

            if start.x < end.x {
                start.x += 1;
            }
            if start.x > end.x {
                start.x -= 1;
            }

            if start.y < end.y {
                start.y += 1;
            }
            if start.y > end.y {
                start.y -= 1;
            }
        }
        let count = grid.entry(start).or_insert(0);
        *count += 1;
    }
    grid
}

pub fn count_overlapping_points(grid: HashMap<Point, i32>) -> i32 {
    grid.into_iter().fold(0, |mut count, (_point, value)| {
        if value > 1 {
            count += 1;
        }
        count
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::{to_lines, Input::*};
    #[test]
    fn test_1_sample() {
        let input = to_lines(Raw("
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        "));

        let grid = plot_points(Point::parse_batch(input), Diagonals::Exclude);

        assert_eq!(count_overlapping_points(grid), 5);
    }

    #[test]
    fn test_1() {
        let input = to_lines(Path("input/2021/5.txt"));

        let grid = plot_points(Point::parse_batch(input), Diagonals::Exclude);

        assert_eq!(count_overlapping_points(grid), 8_111);
    }

    #[test]
    fn test_2_sample() {
        let input = to_lines(Raw("
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        "));

        let grid = plot_points(Point::parse_batch(input), Diagonals::Include);

        assert_eq!(count_overlapping_points(grid), 12);
    }

    #[test]
    fn test_2() {
        let input = to_lines(Path("input/2021/5.txt"));

        let grid = plot_points(Point::parse_batch(input), Diagonals::Include);

        assert_eq!(count_overlapping_points(grid), 22_088);
    }
}
