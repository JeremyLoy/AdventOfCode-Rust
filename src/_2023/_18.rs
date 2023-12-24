use crate::_2023::_18::Direction::{D, L, R, U};
use std::collections::HashSet;
use std::error::Error;
use std::ops::Add;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum Direction {
    R,
    D,
    L,
    U,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => U,
            "D" => D,
            "L" => L,
            "R" => R,
            _ => return Err(format!("{s} is not a valid Direction")),
        })
    }
}

pub struct DigPlan {
    direction: Direction,
    amount: u32,
}

impl DigPlan {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        let mut s = s.split_whitespace();
        Ok(DigPlan {
            direction: s.next().ok_or("no direction found")?.parse()?,
            amount: s.next().ok_or("no amount found")?.parse()?,
        })
    }
    fn parse_swapped(s: &str) -> Result<Self, Box<dyn Error>> {
        let s = s.split_whitespace();
        let mut s = s.skip(2);
        let rgb = s.next().ok_or("must have had rgb string")?;
        let rgb = rgb.trim_end_matches(')').trim_start_matches("(#");
        let amount = &rgb[0..5];
        let amount = u32::from_str_radix(amount, 16)?;
        let direction = &rgb[5..6];
        let direction = match direction {
            "0" => R,
            "1" => D,
            "2" => L,
            "3" => U,
            _ => {
                return Err(format!("{direction} is not a valid Direction").into());
            }
        };
        Ok(DigPlan { direction, amount })
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            U => Point {
                x: self.x,
                y: self.y - 1,
            },
            D => Point {
                x: self.x,
                y: self.y + 1,
            },
            L => Point {
                x: self.x - 1,
                y: self.y,
            },
            R => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

pub fn parse(input: &str) -> Vec<DigPlan> {
    input.lines().flat_map(DigPlan::parse).collect()
}

pub fn parse_swapped(input: &str) -> Vec<DigPlan> {
    input.lines().flat_map(DigPlan::parse_swapped).collect()
}

pub fn cubic_meters_of_laval(dig_plan: Vec<DigPlan>) -> i64 {
    let mut perimeter: Vec<Point> = Vec::new();
    let mut current = Point { x: 0, y: 0 };
    perimeter.push(current);
    for plan in dig_plan {
        for _ in 0..plan.amount {
            current = current + plan.direction;
            perimeter.push(current);
        }
    }
    // flood_fill(&mut perimeter, Point { x: 1, y: 1 });
    // perimeter.len()
    internal_area(&perimeter) + perimeter.len() as i64
}

pub fn internal_area(perimeter: &[Point]) -> i64 {
    // Shoelace formula for calculating the area of a polygon
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut sum = 0i64;
    for i in 0..perimeter.len() - 1 {
        let (x1, y1) = (i64::from(perimeter[i].x), i64::from(perimeter[i].y));
        let (x2, y2) = (i64::from(perimeter[i + 1].x), i64::from(perimeter[i + 1].y));

        sum += x1 * y2 - x2 * y1;
    }
    let area = sum / 2;

    // Pick's Theorem rearranged slightly so that we solve for the number of interior points given the area
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    area - (perimeter.len() as i64 - 1) / 2
}

// I'm not actually using this but I figured
pub fn flood_fill(perimeter: &mut HashSet<Point>, start: Point) {
    // 4 directions: up, down, left, right
    let directions = [U, D, L, R];

    let mut stack = vec![start];
    while let Some(point) = stack.pop() {
        if !perimeter.contains(&point) {
            perimeter.insert(point);
            for direction in directions {
                stack.push(point + direction);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    const INPUT: &str = include_str!("../../input/2023/18.txt");

    #[test]
    fn test_1_sample() {
        let dig_plan = parse(SAMPLE);

        assert_eq!(cubic_meters_of_laval(dig_plan), 62);
    }

    #[test]
    fn test_1() {
        let dig_plan = parse(INPUT);

        assert_eq!(cubic_meters_of_laval(dig_plan), 48_503);
    }

    #[test]
    fn test_2_sample() {
        let dig_plan = parse_swapped(SAMPLE);

        assert_eq!(cubic_meters_of_laval(dig_plan), 952_408_144_115);
    }

    #[test]
    fn test_2() {
        let dig_plan = parse_swapped(INPUT);

        assert_eq!(cubic_meters_of_laval(dig_plan), 148_442_153_147_147);
    }
}
