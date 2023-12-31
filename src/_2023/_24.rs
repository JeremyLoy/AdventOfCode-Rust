use itertools::Itertools;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct Hailstone {
    px: i128,
    py: i128,
    pz: i128,
    vx: i128,
    vy: i128,
    vz: i128,
}

impl Hailstone {
    pub fn intersection(&self, other: &Hailstone) -> Option<(f64, f64)> {
        let x1 = self.px;
        let x2 = self.px + self.vx;
        let x3 = other.px;
        let x4 = other.px + other.vx;
        let y1 = self.py;
        let y2 = self.py + self.vy;
        let y3 = other.py;
        let y4 = other.py + other.vy;

        // https://en.wikipedia.org/wiki/Line-line_intersection#Given_two_points_on_each_line
        let denom = ((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4));

        // lines are parallel or coincident
        if denom == 0 {
            return None;
        }

        #[allow(clippy::cast_precision_loss)]
        let px = ((((x1 * y2) - (y1 * x2)) * (x3 - x4)) - ((x1 - x2) * ((x3 * y4) - (y3 * x4))))
            as f64
            / denom as f64;

        #[allow(clippy::cast_precision_loss)]
        let py = ((((x1 * y2) - (y1 * x2)) * (y3 - y4)) - ((y1 - y2) * ((x3 * y4) - (y3 * x4))))
            as f64
            / denom as f64;

        Some((px, py))
    }
    pub fn intersects(&self, other: &Hailstone, area: (i128, i128)) -> bool {
        let Some(intersection) = self.intersection(other) else {
            return false;
        };
        #[allow(clippy::cast_precision_loss)]
        let area = (area.0 as f64, area.1 as f64);
        let intersects_in_area = intersection.0 >= area.0
            && intersection.0 <= area.1
            && intersection.1 >= area.0
            && intersection.1 <= area.1;

        #[allow(clippy::cast_precision_loss)]
        let self_intersects_future = (intersection.0 - self.px as f64) / self.vx as f64 >= 0f64;
        #[allow(clippy::cast_precision_loss)]
        let other_intersects_future = (intersection.0 - other.px as f64) / other.vx as f64 >= 0f64;
        intersects_in_area && self_intersects_future && other_intersects_future
    }
}

impl FromStr for Hailstone {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (px, py, pz, vx, vy, vz) = s
            .split('@')
            .flat_map(|s| s.split(','))
            .map(str::trim)
            .flat_map(str::parse)
            .collect_tuple()
            .ok_or("tuple should have exactly 6 items")?;

        Ok(Hailstone {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        })
    }
}

/// Parses a string and returns a vector of `Hailstone` structs.
///
/// # Arguments
///
/// * `input` - A string slice containing input to be parsed
///
/// # Returns
///
/// * `Result<Vec<Hailstone>, &'static str>` - A result containing a vector of `Hailstone`
/// structs if parsing is successful, or an error message if parsing fails.
///
/// # Errors
///
/// * if any row doesn't contain exactly 6 numbers, no results are return
pub fn parse(input: &str) -> Result<Vec<Hailstone>, &'static str> {
    input.lines().map(str::parse).collect()
}

pub fn intersections_in_area(hailstones: &[Hailstone], area: (i128, i128)) -> usize {
    hailstones
        .iter()
        .combinations(2)
        .map(|h| h[0].intersects(h[1], area))
        .filter(|it| *it)
        .count()
}

pub fn magic_rock(_hailstones: &[Hailstone]) -> Hailstone {
    Hailstone {
        px: 0,
        py: 0,
        pz: 0,
        vx: 0,
        vy: 0,
        vz: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    const INPUT: &str = include_str!("../../input/2023/24.txt");

    #[test]
    fn test_1_sample() {
        let hailstones = parse(SAMPLE).unwrap();

        assert_eq!(intersections_in_area(&hailstones, (7, 27)), 1 + 1);
    }

    #[test]
    fn test_1() {
        let hailstones = parse(INPUT).unwrap();

        assert_eq!(
            intersections_in_area(&hailstones, (200_000_000_000_000, 400_000_000_000_000)),
            16_018
        );
    }

    #[test]
    #[ignore]
    fn test_2_sample() {
        let hailstones = parse(SAMPLE).unwrap();

        assert_eq!(
            magic_rock(&hailstones),
            Hailstone {
                px: 24,
                py: 13,
                pz: 10,
                vx: -3,
                vy: 1,
                vz: 2
            }
        );
    }

    #[test]
    #[ignore]
    fn test_2() {
        let hailstones = parse(INPUT).unwrap();

        assert_eq!(
            magic_rock(&hailstones),
            Hailstone {
                px: 24,
                py: 13,
                pz: 10,
                vx: -3,
                vy: 1,
                vz: 2
            }
        );
    }
}
