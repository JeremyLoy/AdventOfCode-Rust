use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub struct Button {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prize {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Machine {
    pub a: Button,
    pub b: Button,
    pub prize: Prize,
}

pub fn parse(input: &str) -> Result<Vec<Machine>> {
    let re = regex::Regex::new(r".*X[=+-](\d+).*Y[=+-](\d+)")?;
    let machines = input
        .split("\n\n")
        .enumerate()
        .map(|(machine_num, machine)| {
            let ((ax, ay), (bx, by), (px, py)) = machine
                .lines()
                .enumerate()
                .map(|(line_num, l)| {
                    re.captures(l)
                        .ok_or(anyhow!(
                            "failed to parse X Y coordinates group {machine_num} line {line_num}"
                        ))
                        .and_then(|caps| Ok((caps[1].parse()?, caps[2].parse()?)))
                })
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .collect_tuple()
                .ok_or(anyhow!(
                    "could not split machine {machine_num} into 3 lines"
                ))?;

            Ok(Machine {
                a: Button { x: ax, y: ay },
                b: Button { x: bx, y: by },
                prize: Prize { x: px, y: py },
            })
        })
        .collect();
    machines
}

fn determinant(a: i64, b: i64, c: i64, d: i64) -> i64 {
    // | a b |
    // |  x  |
    // | c d |
    a * d - b * c
}

impl Machine {
    // Cramer's rule for solving a system of two equations, apparently.
    // IDK I never watched Seinfeld
    //
    // This only works because there is either 0 or 1 solution for the Machine.
    pub fn min_tokens(&self) -> i64 {
        // | ax bx |
        // | ay by |
        let d = determinant(self.a.x, self.b.x, self.a.y, self.b.y);
        // | px py |
        // | bx by |
        let a_times = determinant(self.prize.x, self.prize.y, self.b.x, self.b.y) / d;
        // | ax ay |
        // | px py |
        let b_times = determinant(self.a.x, self.a.y, self.prize.x, self.prize.y) / d;

        if self.press(a_times, b_times) == (self.prize.x, self.prize.y) {
            (a_times * 3) + b_times
        } else {
            0
        }
    }

    #[inline]
    fn press(&self, a: i64, b: i64) -> (i64, i64) {
        (self.a.x * a + self.b.x * b, self.a.y * a + self.b.y * b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    const INPUT: &str = include_str!("../../input/2024/13.txt");

    #[test]
    fn test_1_sample() {
        let input = parse(SAMPLE).unwrap();
        let min_tokens: i64 = input.iter().map(Machine::min_tokens).sum();

        assert_eq!(min_tokens, 480);
    }

    #[test]
    fn test_1() {
        let input = parse(INPUT).unwrap();
        let min_tokens: i64 = input.iter().map(Machine::min_tokens).sum();

        assert_eq!(min_tokens, 28_753);
    }

    #[test]
    fn test_2() {
        let mut input = parse(INPUT).unwrap();
        let min_tokens: i64 = input
            .iter_mut()
            .map(|machine| {
                machine.prize.x += 10_000_000_000_000;
                machine.prize.y += 10_000_000_000_000;
                machine.min_tokens()
            })
            .sum();

        assert_eq!(min_tokens, 102_718_967_795_500);
    }
}
