pub fn hash_seq(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .map(|seq| seq.chars().fold(0, hash))
        .sum()
}

pub fn hash(cur: u64, c: char) -> u64 {
    ((cur + (c as u64)) * 17) % 256
}

pub fn focusing_power(seq: &str) -> u64 {
    let mut boxes: Vec<Vec<(&str, &str)>> = vec![Vec::new(); 256];
    seq.trim().split(',').for_each(|step| {
        let parts: Vec<&str> = step.split_terminator(['=', '-'].as_ref()).collect();
        let label = parts.first().expect("there must be a label");
        let focal_length = parts.last().expect("parts should be non-empty");
        let idx = hash_seq(label);
        let boxx = boxes
            .get_mut(idx as usize)
            .expect("every box is pre-initialized");
        if parts.len() > 1 {
            if let Some(pos) = boxx
                .iter_mut()
                .position(|(cur_label, _)| label == cur_label)
            {
                boxx[pos].1 = focal_length;
            } else {
                boxx.push((label, *focal_length));
            }
        } else {
            boxx.retain(|(cur_label, _)| label != cur_label);
        }
    });
    boxes
        .iter()
        .enumerate()
        .fold(0, |mut acc, (mut box_idx, boxx)| {
            box_idx += 1;
            boxx.iter()
                .enumerate()
                .for_each(|(mut lens_idx, (_, focal_length))| {
                    lens_idx += 1;
                    acc += (box_idx
                        * focal_length
                            .parse::<usize>()
                            .expect("focal_length is a number")
                        * lens_idx) as u64;
                });
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";
    const INPUT: &str = include_str!("../../input/2023/15.txt");

    #[test]
    fn test_hash() {
        assert_eq!("HASH".chars().fold(0, hash), 52);
    }
    #[test]
    fn test_1_sample() {
        let input = hash_seq(SAMPLE);

        assert_eq!(input, 1_320);
    }

    #[test]
    fn test_1() {
        let input = hash_seq(INPUT);

        assert_eq!(input, 511_215);
    }

    #[test]
    fn test_2_sample() {
        let input = focusing_power(SAMPLE);

        assert_eq!(input, 145);
    }

    #[test]
    fn test_2() {
        let input = focusing_power(INPUT);

        assert_eq!(input, 236_057);
    }
}
