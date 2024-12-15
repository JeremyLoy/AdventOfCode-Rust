use anyhow::{anyhow, Result};
use gif::{Encoder, Frame, Repeat};
use std::collections::HashSet;
use std::fs::File;

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

// this function created the gif neighboring this file
// I then upscaled it with imagemagick instead of trying to do so in the code
// convert _14.gif -filter point -resize 800% _14.gif
#[allow(clippy::field_reassign_with_default)]
pub fn write_file(mut roombas: Vec<(i32, i32, i32, i32)>) {
    let width: i32 = 101;
    let height: i32 = 103;
    let total_frames = 120;

    // start on the answer
    let start = 7_502;
    for roomba in &mut roombas {
        advance(roomba, start, width, height);
    }

    let palette = &[
        0x00, 0x00, 0x00, // Black (RGB)
        0xFF, 0xFF, 0xFF, // White (RGB)
    ];

    let mut roomba_set = roombas
        .iter()
        .map(|r| (r.0, r.1))
        .collect::<HashSet<(i32, i32)>>();

    let mut image = File::create("_14.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, width as u16, height as u16, palette).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    for f in start..start + total_frames / 2 {
        let mut frame = Frame::default();
        frame.width = width as u16;
        frame.height = height as u16;
        frame.delay = 2;

        let mut pixels = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let found = roomba_set.contains(&(x, y));
                pixels.push(u8::from(found));
            }
        }

        frame.buffer = pixels.into();
        encoder.write_frame(&frame).unwrap();

        // linger on the answer for half of the .gif
        if f == start {
            for _ in 0..total_frames / 2 {
                encoder.write_frame(&frame).unwrap();
            }
        }

        roomba_set.clear();
        for roomba in &mut roombas {
            advance(roomba, 1, width, height);
            roomba_set.insert((roomba.0, roomba.1));
        }
    }
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
