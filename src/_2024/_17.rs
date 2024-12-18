use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Computer {
    pub reg_a: u64,
    pub reg_b: u64,
    pub reg_c: u64,
    pub instruction_pointer: usize,
    pub instructions: Vec<u64>,
    pub output: Vec<u64>,
}

impl Computer {
    fn literal_op(&self) -> u64 {
        self.instructions[self.instruction_pointer + 1]
    }
    fn combo_op(&self) -> u64 {
        let op = self.instructions[self.instruction_pointer + 1];
        match op {
            0..=3 => op,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("7 is not a valid combo op"),
            unhandled => panic!("{unhandled:?} is an unknown opcode"),
        }
    }
    fn adv(&mut self) {
        let num = self.reg_a;
        let denom = 2_u64.pow(self.combo_op() as u32);
        self.reg_a = num / denom;
        self.instruction_pointer += 2;
    }
    fn bxl(&mut self) {
        self.reg_b ^= self.literal_op();
        self.instruction_pointer += 2;
    }
    fn bst(&mut self) {
        self.reg_b = self.combo_op() % 8;
        self.instruction_pointer += 2;
    }
    fn jnz(&mut self) {
        if self.reg_a == 0 {
            self.instruction_pointer += 2;
        } else {
            self.instruction_pointer = self.literal_op() as usize;
        }
    }
    fn bxc(&mut self) {
        self.reg_b ^= self.reg_c;
        self.instruction_pointer += 2;
    }
    fn out(&mut self) {
        self.output.push(self.combo_op() % 8);
        self.instruction_pointer += 2;
    }
    fn bdv(&mut self) {
        let num = self.reg_a;
        let denom = 2_u64.pow(self.combo_op() as u32);
        self.reg_b = num / denom;
        self.instruction_pointer += 2;
    }
    fn cdv(&mut self) {
        let num = self.reg_a;
        let denom = 2_u64.pow(self.combo_op() as u32);
        self.reg_c = num / denom;
        self.instruction_pointer += 2;
    }

    pub fn run(&mut self) {
        while let Some(instruction) = self.instructions.get(self.instruction_pointer) {
            match instruction {
                0 => self.adv(),
                1 => self.bxl(),
                2 => self.bst(),
                3 => self.jnz(),
                4 => self.bxc(),
                5 => self.out(),
                6 => self.bdv(),
                7 => self.cdv(),
                unhandled => panic!("{unhandled:?} is an unknown opcode"),
            }
        }
    }

    // work backwards from the last digit, adding 1 repeatedly to increment the first digit
    // multiply by 8 to lock all correct digits
    pub fn min_quine(&mut self) -> u64 {
        // start at 8 and from the 2nd to last digit because the final digit always 0
        let mut quine = 8;
        for idx in (0..self.instructions.len() - 1).rev() {
            loop {
                self.reset();
                self.reg_a = quine;
                self.run();

                if self
                    .output
                    .iter()
                    .zip(self.instructions[idx..].iter())
                    .all(|(a, b)| a == b)
                {
                    if self.output.len() != self.instructions.len() {
                        quine *= 8;
                    }
                    break;
                }
                quine += 1;
            }
        }
        quine
    }

    fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_b = 0;
        self.reg_c = 0;
        self.instruction_pointer = 0;
        self.output.clear();
    }
}

pub fn parse(input: &str) -> Computer {
    let (reg_a, reg_b, reg_c, instructions) = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once(": ").unwrap().1)
        .collect_tuple()
        .unwrap();
    let instructions = instructions
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect_vec();
    let output = Vec::new();
    Computer {
        reg_a: reg_a.parse().unwrap(),
        reg_b: reg_b.parse().unwrap(),
        reg_c: reg_c.parse().unwrap(),
        instruction_pointer: 0,
        instructions,
        output,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
    const SAMPLE_2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";
    const INPUT: &str = include_str!("../../input/2024/17.txt");

    #[test]
    fn test_1_sample() {
        let mut input = parse(SAMPLE);
        input.run();

        assert_eq!(input.output.into_iter().join(","), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_1() {
        let mut input = parse(INPUT);
        input.run();

        assert_eq!(input.output.into_iter().join(","), "2,0,7,3,0,3,1,3,7");
    }

    #[test]
    fn test_2_sample() {
        let mut computer = parse(SAMPLE_2);
        let quine = computer.min_quine();

        assert_eq!(quine, 117_440);
    }

    #[test]
    fn test_2() {
        let mut computer = parse(INPUT);
        let quine = computer.min_quine();

        assert_eq!(quine, 247_839_539_763_386);
    }
}
