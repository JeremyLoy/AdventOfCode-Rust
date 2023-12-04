use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Schematic {
    grid: HashMap<Point, char>,
    point_to_int: HashMap<Point, i32>,
    parts: Vec<(Point, Point, i32)>,
}

impl Schematic {
    pub fn from_lines(input: impl Iterator<Item = String>) -> Option<Schematic> {
        let input: Vec<String> = input.collect();

        let width = input.first()?.len();
        let height = input.len();
        let mut grid = HashMap::new();
        let mut point_to_int = HashMap::new();
        let mut parts: Vec<(Point, Point, i32)> = Vec::new();

        input.iter().enumerate().for_each(|(y, row)| {
            row.chars().enumerate().for_each(|(x, char)| {
                grid.insert(
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    char,
                );
            });
        });

        for y in 0..height {
            let mut building_number = false;
            let mut number: Vec<char> = Vec::new();
            let mut start = Point { x: 0, y: 0 };
            for x in 0..width {
                let current = Point {
                    x: x as i32,
                    y: y as i32,
                };
                if let Some(c) = grid.get(&current) {
                    if c.is_numeric() {
                        if !building_number {
                            start = current;
                            building_number = true;
                        }
                        number.push(*c);
                    } else {
                        if building_number {
                            let end = Point {
                                x: x as i32 - 1,
                                y: y as i32,
                            };
                            let n = number.iter().collect::<String>().parse().unwrap();
                            parts.push((start, end, n));
                            for x in start.x..=end.x {
                                point_to_int.insert(Point { x, y: y as i32 }, n);
                            }
                            number.clear();
                        }
                        building_number = false;
                    }
                }
            }
            // base case - important if the number goes all the way until the right wall
            if building_number {
                let end = Point {
                    x: width as i32,
                    y: y as i32,
                };
                let n = number.iter().collect::<String>().parse().unwrap();
                parts.push((start, end, n));
                for x in start.x..=end.x {
                    point_to_int.insert(Point { x, y: y as i32 }, n);
                }
            }
        }

        Some(Schematic {
            grid,
            point_to_int,
            parts,
        })
    }
    pub fn collect_part_numbers(&self) -> Vec<i32> {
        self.parts
            .iter()
            .filter(|(start, end, _n)| self.has_adjacent_symbol(start, end))
            .map(|p| p.2)
            .collect()
    }

    pub fn sum_of_gear_ratios(&self) -> i32 {
        self.find_gears()
            .iter()
            .map(|gear| self.get_unique_surrounding_numbers(gear))
            .filter(|v| v.len() == 2)
            .map(|v| v.first().unwrap() * v.last().unwrap())
            .sum()
    }

    fn get_surrounding_box(start: &Point, end: &Point) -> Vec<Point> {
        let mut points = Vec::new();
        //x +/- 1 to include diagonals
        for x in start.x - 1..=end.x + 1 {
            // the entire line above
            points.push(Point { x, y: start.y - 1 });
            // the entire line below
            points.push(Point { x, y: start.y + 1 });
        }
        // inline left
        points.push(Point {
            x: start.x - 1,
            y: start.y,
        });
        // inline right
        points.push(Point {
            x: end.x + 1,
            y: start.y,
        });

        points
    }

    fn find_gears(&self) -> Vec<Point> {
        self.grid
            .iter()
            .filter(|(_p, c)| **c == '*')
            .map(|(point, _c)| *point)
            .collect()
    }

    fn has_adjacent_symbol(&self, start: &Point, end: &Point) -> bool {
        Self::get_surrounding_box(start, end)
            .iter()
            .filter_map(|p| self.grid.get(p))
            .any(|c| !c.is_ascii_digit() && *c != '.')
    }

    fn get_unique_surrounding_numbers(&self, gear: &Point) -> Vec<i32> {
        Self::get_surrounding_box(gear, gear)
            .iter()
            .filter_map(|p| self.point_to_int.get(p))
            .copied()
            .collect::<HashSet<i32>>()
            .iter()
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::to_lines;
    use crate::input_parsing::Input::{Path, Raw};

    #[test]
    fn test_1_sample() {
        let input = Raw("\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
");
        let schematic = Schematic::from_lines(to_lines(input)).unwrap();

        assert_eq!(schematic.collect_part_numbers().iter().sum::<i32>(), 4_361);
    }

    #[test]
    fn test_1() {
        let input = Path("input/2023/03.txt");

        let schematic = Schematic::from_lines(to_lines(input)).unwrap();

        assert_eq!(
            schematic.collect_part_numbers().iter().sum::<i32>(),
            539_590
        );
    }

    #[test]
    fn test_2_sample() {
        let input = Raw("\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
");

        let schematic = Schematic::from_lines(to_lines(input)).unwrap();

        assert_eq!(schematic.sum_of_gear_ratios(), 467_835);
    }

    #[test]
    fn test_2() {
        let input = Path("input/2023/03.txt");

        let schematic = Schematic::from_lines(to_lines(input)).unwrap();

        assert_eq!(schematic.sum_of_gear_ratios(), 80_703_636);
    }
}
