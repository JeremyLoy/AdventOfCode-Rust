use itertools::Itertools;

pub fn count_1478(input: Box<dyn Iterator<Item = String>>) -> i32 {
    input
        .map(|line| line.split_once("|").unwrap().1.trim().to_owned())
        .map(|output| {
            output.split_whitespace().fold(0, |mut count, digit| {
                if [2, 3, 4, 7].contains(&(digit.len() as i32)) {
                    count += 1
                }
                count
            })
        })
        .sum()
}

pub const DIGIT_MASKS: [(char, u8); 7] = [
    ('a', 0b01000000),
    ('b', 0b00100000),
    ('c', 0b00010000),
    ('d', 0b00001000),
    ('e', 0b00000100),
    ('f', 0b00000010),
    ('g', 0b00000001),
];
pub fn get_bit(c: char) -> u8 {
    for (ch, bit) in DIGIT_MASKS.iter() {
        if *ch == c {
            return *bit;
        }
    }
    0
}
pub fn signal_to_mask(s: &str) -> u8 {
    let mut mask = 0;
    for ch in s.chars() {
        mask |= get_bit(ch);
    }
    mask
}

pub fn overlaps(a: u8, b: u8) -> bool {
    a & b == b
}

pub fn determine_output(row: &str) -> i32 {
    let signals = row
        .split_whitespace()
        .map(|s| s.trim())
        .filter(|s| *s != "|")
        .map(signal_to_mask)
        .collect_vec();
    let (signals, output) = signals.split_at(10);

    let mut digit_to_mask = [0; 10];
    digit_to_mask[1] = *signals
        .iter()
        .find(|signal| signal.count_ones() == 2)
        .unwrap();
    digit_to_mask[4] = *signals
        .iter()
        .find(|signal| signal.count_ones() == 4)
        .unwrap();
    digit_to_mask[7] = *signals
        .iter()
        .find(|signal| signal.count_ones() == 3)
        .unwrap();
    digit_to_mask[8] = *signals
        .iter()
        .find(|signal| signal.count_ones() == 7)
        .unwrap();

    digit_to_mask[3] = *signals
        .iter()
        .filter(|signal| signal.count_ones() == 5)
        .find(|signal| overlaps(**signal, digit_to_mask[1]))
        .unwrap();

    digit_to_mask[9] = *signals
        .iter()
        .filter(|signal| signal.count_ones() == 6)
        .find(|signal| overlaps(**signal, digit_to_mask[3]))
        .unwrap();

    digit_to_mask[0] = *signals
        .iter()
        .filter(|signal| signal.count_ones() == 6)
        .filter(|signal| **signal != digit_to_mask[9])
        .filter(|signal| overlaps(**signal, digit_to_mask[7]))
        .find(|signal| overlaps(**signal, digit_to_mask[1]))
        .unwrap();

    digit_to_mask[6] = *signals
        .iter()
        .filter(|signal| signal.count_ones() == 6)
        .filter(|signal| **signal != digit_to_mask[9])
        .find(|signal| **signal != digit_to_mask[0])
        .unwrap();

    digit_to_mask[5] = *signals
        .iter()
        .filter(|signal| signal.count_ones() == 5)
        .find(|signal| overlaps(digit_to_mask[6], **signal))
        .unwrap();

    digit_to_mask[2] = *signals
        .iter()
        .filter(|signal| signal.count_ones() == 5)
        .filter(|signal| **signal != digit_to_mask[5])
        .find(|signal| **signal != digit_to_mask[3])
        .unwrap();

    output
        .iter()
        .filter_map(|o| digit_to_mask.iter().find_position(|s| **s == *o))
        .map(|i| i.0.to_string())
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::{to_lines, Input::*};

    #[test]
    fn test_determine_output() {
        assert_eq!(determine_output("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), 5353);
        assert_eq!(determine_output("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"), 8394);
        assert_eq!(determine_output("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"), 9781);
        assert_eq!(
            determine_output(
                "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"
            ),
            1197
        );
        assert_eq!(determine_output("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"), 9361);
        assert_eq!(determine_output("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"), 4873);
        assert_eq!(determine_output("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"), 8418);
        assert_eq!(determine_output("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"), 4548);
        assert_eq!(determine_output("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"), 1625);
        assert_eq!(
            determine_output(
                "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"
            ),
            8717
        );
        assert_eq!(
            determine_output(
                "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            ),
            4315
        );
    }

    #[test]
    fn test_8_1_sample() {
        let input = Raw("
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        ");

        let signal = to_lines(input);

        assert_eq!(count_1478(signal), 26);
    }

    #[test]
    fn test_8_1() {
        let input = to_lines(Path("input/2021/8.txt"));

        assert_eq!(count_1478(input), 530);
    }

    #[test]
    fn test_8_2_sample() {
        let input = to_lines(Raw("
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "));

        assert_eq!(input.map(|l| determine_output(&l)).sum::<i32>(), 61_229);
    }

    #[test]
    fn test_8_2() {
        let input = to_lines(Path("input/2021/8.txt"));

        assert_eq!(input.map(|l| determine_output(&l)).sum::<i32>(), 1_051_087);
    }
}
