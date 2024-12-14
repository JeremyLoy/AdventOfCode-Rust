use anyhow::{anyhow, Result};
use std::collections::HashSet;

#[allow(clippy::missing_errors_doc)]
pub fn parse(input: &str) -> Result<Vec<(i32, i32, i32, i32)>> {
    use regex::Regex;

    let re = Regex::new(r"-?\d+")?;
    let numbers: Vec<i32> = re
        .find_iter(input)
        .map(|m| {
            m.as_str()
                .parse::<i32>()
                .map_err(|e| anyhow!("failed to parse number {e}"))
        })
        .collect::<Result<Vec<i32>>>()?;

    Ok(numbers
        .chunks(4)
        .map(|chunk| (chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect())
}

fn advance(roomba: &mut (i32, i32, i32, i32), seconds: i32, width: i32, height: i32) {
    let (x, y, vx, vy) = roomba;

    // scale the vector by seconds
    *x += *vx * seconds;
    *y += *vy * seconds;

    // adjust for wraparound
    *x %= width;
    *y %= height;
    if *x < 0 {
        *x += width;
    }
    if *y < 0 {
        *y += height;
    }
}

pub fn p1(mut roombas: Vec<(i32, i32, i32, i32)>, times: i32, width: i32, height: i32) -> usize {
    let mid_width = width / 2;
    let mid_height = height / 2;

    let filtered: Vec<(i32, i32)> = roombas
        .iter_mut()
        .map(|roomba| {
            advance(roomba, times, width, height);
            (roomba.0, roomba.1)
        })
        .filter(|(x, y)| *x != mid_width && *y != mid_height)
        .collect();

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for (x, y) in filtered {
        if x < mid_width && y < mid_height {
            q1 += 1;
        } else if x >= mid_width && y < mid_height {
            q2 += 1;
        } else if x < mid_width && y >= mid_height {
            q3 += 1;
        } else {
            q4 += 1;
        }
    }
    q1 * q2 * q3 * q4
}

//  This solution only works because of an abstract property of the puzzle where no robots overlap
// in the solution. I initially guessed that all robots would be used in the formation of the tree, tried it,
// and got the correct number.
//
// It is not true that all robots are used though, so I feel like that is a mistake in the input generation.
// I imagine that the intent behind this puzzle was some sort of cluster search algorithm given the quadrants solution in p1.
//
// ###############################
// #.............................#
// #.............................#
// #.............................#
// #.............................#
// #..............#..............#
// #.............###.............#
// #............#####............#
// #...........#######...........#
// #..........#########..........#
// #............#####............#
// #...........#######...........#
// #..........#########..........#
// #.........###########.........#
// #........#############........#
// #..........#########..........#
// #.........###########.........#
// #........#############........#
// #.......###############.......#
// #......#################......#
// #........#############........#
// #.......###############.......#
// #......#################......#
// #.....###################.....#
// #....#####################....#
// #.............###.............#
// #.............###.............#
// #.............###.............#
// #.............................#
// #.............................#
// #.............................#
// #.............................#
// ###############################
pub fn p2(mut roombas: Vec<(i32, i32, i32, i32)>) -> usize {
    let width = 101;
    let height = 103;
    let mut seconds = 0;
    let mut seen = HashSet::new();

    'outer: loop {
        seconds += 1;
        seen.clear();
        for roomba in &mut roombas {
            advance(roomba, 1, width, height);
        }
        // significant speed improvement, about 3-4x
        // as soon as a single overlap occurs, re-loop.
        for &(x, y, ..) in &roombas {
            let new = seen.insert((x, y));
            if !new {
                continue 'outer;
            }
        }
        break;
    }
    seconds
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
    const INPUT: &str = include_str!("../../input/2024/14.txt");

    #[test]
    fn test_1_sample() {
        let roombas = parse(SAMPLE).unwrap();
        let answer = p1(roombas, 100, 11, 7);
        assert_eq!(answer, 12);
    }

    #[test]
    fn test_1() {
        let roombas = parse(INPUT).unwrap();
        let answer = p1(roombas, 100, 101, 103);

        assert_eq!(answer, 226_179_492);
    }

    #[test]
    fn test_2() {
        let roombas = parse(INPUT).unwrap();
        let seconds = p2(roombas);

        assert_eq!(seconds, 7_502);
    }
}
