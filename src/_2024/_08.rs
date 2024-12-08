use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub data: HashMap<char, Vec<(i32, i32)>>,
}

impl Grid {
    pub fn point_in_bounds(&self, (x, y): &(i32, i32)) -> bool {
        *x >= 0 && *x <= self.width && *y >= 0 && *y <= self.height
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn parse(input: &str) -> Result<Grid> {
    let mut height = 0;
    let mut width = 0;
    let mut data = HashMap::new();
    input.lines().enumerate().for_each(|(y, row)| {
        height = height.max(y);
        row.chars().enumerate().for_each(|(x, c)| {
            width = width.max(x);
            if c != '.' {
                data.entry(c)
                    .or_insert_with(Vec::new)
                    .push((x as i32, y as i32));
            }
        });
    });
    Ok(Grid {
        width: width as i32,
        height: height as i32,
        data,
    })
}

pub fn solve(input: &Grid) -> usize {
    input
        .data
        .iter()
        .flat_map(|(_, points)| {
            points.iter().permutations(2).map(|pairs| {
                let (a, b) = (pairs[0], pairs[1]);
                let (vx, vy) = (b.0 - a.0, b.1 - a.1);
                (b.0 + vx, b.1 + vy)
            })
        })
        .filter(|p| input.point_in_bounds(p))
        .unique()
        .count()
}

pub fn solve2(input: &Grid) -> usize {
    input
        .data
        .iter()
        .flat_map(|(_, points)| {
            points.iter().permutations(2).flat_map(|pairs| {
                let (a, b) = (*pairs[0], *pairs[1]);
                // include the initial points in the result
                let mut all_in_bounds = vec![a, b];
                let (vx, vy) = (b.0 - a.0, b.1 - a.1);
                let mut next = (b.0 + vx, b.1 + vy);
                while input.point_in_bounds(&next) {
                    all_in_bounds.push(next);
                    next = (next.0 + vx, next.1 + vy);
                }
                all_in_bounds
            })
        })
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    const SAMPLE_2: &str = "\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
";

    const INPUT: &str = include_str!("../../input/2024/08.txt");

    #[test]
    fn test_1_sample() {
        let input = parse(SAMPLE).unwrap();
        let result = solve(&input);

        assert_eq!(result, 14);
    }

    #[test]
    fn test_1() {
        let input = parse(INPUT).unwrap();
        let result = solve(&input);

        assert_eq!(result, 332);
    }

    #[test]
    fn test_2_sample() {
        let input = parse(SAMPLE).unwrap();
        let result = solve2(&input);

        assert_eq!(result, 34);
    }

    #[test]
    fn test_2_sample_2() {
        let input = parse(SAMPLE_2).unwrap();
        let result = solve2(&input);

        assert_eq!(result, 9);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT).unwrap();
        let result = solve2(&input);

        assert_eq!(result, 1_174);
    }
}
