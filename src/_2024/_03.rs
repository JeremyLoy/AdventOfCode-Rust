use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
pub enum Instruction {
    Do,
    Dont,
    Mul(i32, i32),
}

pub fn parse(input: &str) -> Result<Vec<Instruction>> {
    let re = Regex::new(
        r"(?x)
              (?P<mul>mul\((\d{1,3}),(\d{1,3})\))|
              (?P<do>do\(\))|
              (?P<dont>don't\(\))
              ",
    )?;
    re.captures_iter(input)
        .map(|cap| {
            if cap.name("mul").is_some() {
                let left = cap[2].parse()?;
                let right = cap[3].parse()?;
                Ok(Instruction::Mul(left, right))
            } else if cap.name("do").is_some() {
                Ok(Instruction::Do)
            } else if cap.name("dont").is_some() {
                Ok(Instruction::Dont)
            } else {
                Err(anyhow::anyhow!("Unexpected instruction format"))
            }
        })
        .collect()
}

pub fn uncorrupt_instructions(instructions: &[Instruction]) -> i32 {
    instructions
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mul(left, right) => left * right,
            Instruction::Do | Instruction::Dont => 0,
        })
        .sum()
}

pub fn uncorrupt_instructions_with_conditionals(instructions: &[Instruction]) -> i32 {
    instructions
        .iter()
        .scan(true, |enabled, instruction| match instruction {
            Instruction::Mul(left, right) if *enabled => Some(left * right),
            Instruction::Mul(_, _) => Some(0),
            Instruction::Dont => {
                *enabled = false;
                Some(0)
            }
            Instruction::Do => {
                *enabled = true;
                Some(0)
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    const INPUT: &str = include_str!("../../input/2024/03.txt");

    #[test]
    fn test_1_sample() {
        let instructions = parse(SAMPLE).unwrap();
        let sum = uncorrupt_instructions(&instructions);

        assert_eq!(sum, 161);
    }

    #[test]
    fn test_1() {
        let instructions = parse(INPUT).unwrap();
        let sum = uncorrupt_instructions(&instructions);

        assert_eq!(sum, 170_778_545);
    }

    #[test]
    fn test_2_sample() {
        let instructions = parse(SAMPLE_2).unwrap();
        let sum = uncorrupt_instructions_with_conditionals(&instructions);

        assert_eq!(sum, 48);
    }

    #[test]
    fn test_2() {
        let instructions = parse(INPUT).unwrap();
        let sum = uncorrupt_instructions_with_conditionals(&instructions);

        assert_eq!(sum, 82_868_252);
    }
}
