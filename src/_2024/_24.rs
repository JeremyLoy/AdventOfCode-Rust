use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Gate {
    pub input_a: String,
    pub input_b: String,
    pub output: String,
    pub operation_str: String,
    pub operation: fn(bool, bool) -> bool,
}

pub struct Machine {
    pub input_queue: Vec<String>,
    pub input_wires_to_gates: HashMap<String, Vec<Gate>>,
    pub wires_to_values: HashMap<String, bool>,
    pub gates: Vec<Gate>,
    pub swaps: HashMap<String, String>,
}

impl Machine {
    pub fn find_swaps(
        &mut self,
        number_of_pairs: usize,
        operation: fn(usize, usize) -> usize,
        number_of_bits: usize,
    ) -> String {
        let outputs = self
            .gates
            .iter()
            .map(|gate| gate.output.clone())
            .filter(|output| output.starts_with('z'))
            .collect_vec();

        let pairs: Vec<(String, String)> = outputs
            .iter()
            .combinations(2)
            .map(|v| (v[0].clone(), v[1].clone()))
            .collect_vec();

        let combs = pairs.iter().combinations(number_of_pairs).filter(|c| {
            let mut set = HashSet::new();
            for (a, b) in c {
                if !set.insert(a) || !set.insert(b) {
                    return false;
                }
            }
            true
        });

        'swap: for swap in combs {
            for x in 0..(1 << number_of_bits) {
                for y in 0..(1 << number_of_bits) {
                    let target = operation(x, y);
                    self.reset(x, y, number_of_bits);
                    self.swaps = swap
                        .iter()
                        .flat_map(|(a, b)| vec![(a.clone(), b.clone()), (b.clone(), a.clone())])
                        .collect();
                    self.run();

                    if target != self.number('z') {
                        continue 'swap;
                    }
                }
            }

            return swap
                .iter()
                .map(|(a, b)| vec![a, b])
                .flatten()
                .sorted()
                .join(",");
        }
        panic!("no solution found");
    }
    pub fn reset(&mut self, new_x: usize, new_y: usize, bits: usize) {
        // x and y are never any outputs. This safely clears all outputs while retaining the original
        // input
        // self.wires_to_values
        //     .retain(|key, _| key.starts_with('x') || key.starts_with('y'));
        self.wires_to_values.clear();
        for i in 0..bits {
            self.wires_to_values
                .insert(format!("x{:02}", i), new_x & (1 << i) != 0);
            self.wires_to_values
                .insert(format!("y{:02}", i), new_y & (1 << i) != 0);
        }

        // clear the temporary output overrides for p2
        self.swaps.clear();

        // technically not necessary since run() fully drains input_queue
        self.input_queue.clear();
        // add all of x and y to the input queue
        self.input_queue
            .extend(self.wires_to_values.keys().cloned());
    }
    pub fn run(&mut self) {
        while let Some(wire) = self.input_queue.pop() {
            let gates = self.input_wires_to_gates.get(&wire);
            if gates.is_none() {
                continue;
            }
            for gate in gates.unwrap() {
                let a = self.wires_to_values.get(&gate.input_a);
                let b = self.wires_to_values.get(&gate.input_b);
                match (a, b) {
                    (Some(&a), Some(&b)) => {
                        let op = gate.operation;
                        let output = op(a, b);

                        // for pt 2, conditionally swap the outputs
                        let output_wire = self.swaps.get(&gate.output).unwrap_or(&gate.output);

                        self.wires_to_values.insert(output_wire.to_string(), output);
                        self.input_queue.push(output_wire.clone());
                    }
                    _ => continue,
                };
            }
        }
    }

    pub fn number(&self, starts_with: char) -> usize {
        self.wires_to_values
            .iter()
            .filter(|(wire, _)| wire.starts_with(starts_with))
            .collect_vec()
            .iter()
            .sorted_by_key(|(wire, _)| wire)
            .rev()
            .map(|(_, value)| if **value { 1 } else { 0 })
            .fold(0, |acc, x| (acc << 1) | x)
    }
}

pub fn parse(input: &str) -> Machine {
    let (input_queue, gates) = input.split_once("\n\n").unwrap();
    let wires_to_values: HashMap<String, bool> = input_queue
        .lines()
        .map(|line| {
            let (input, value) = line.split_once(": ").unwrap();
            (input.to_string(), value == "1")
        })
        .collect();
    let input_queue = wires_to_values.keys().cloned().collect();
    let gates = gates
        .lines()
        .map(|line| {
            let (input_a, operation_str, input_b, _, output) =
                line.split_whitespace().collect_tuple().unwrap();
            let operation = match operation_str {
                "AND" => |a, b| a && b,
                "XOR" => |a, b| a != b,
                "OR" => |a, b| a || b,
                operation => panic!("unhandled operation {operation}"),
            };
            Gate {
                input_a: input_a.to_string(),
                input_b: input_b.to_string(),
                output: output.to_string(),
                operation,
                operation_str: operation_str.to_string(),
            }
        })
        .collect_vec();
    let mut input_wires_to_gates = HashMap::new();
    for gate in &gates {
        input_wires_to_gates
            .entry(gate.input_a.clone())
            .or_insert_with(Vec::new)
            .push(gate.clone());
        input_wires_to_gates
            .entry(gate.input_b.clone())
            .or_insert_with(Vec::new)
            .push(gate.clone());
    }
    Machine {
        input_queue,
        input_wires_to_gates,
        wires_to_values,
        gates,
        swaps: HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    use std::ops::BitAnd;

    const SAMPLE: &str = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

    const SAMPLE_2: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

    const SAMPLE_3: &str = "\
x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00
";

    const INPUT: &str = include_str!("../../input/2024/24.txt");

    #[test]
    fn test_1_sample() {
        let mut machine = parse(SAMPLE);
        machine.run();

        assert_eq!(machine.number('z'), 4);
    }

    #[test]
    fn test_1_sample_2() {
        let mut machine = parse(SAMPLE_2);

        machine.run();

        assert_eq!(machine.number('z'), 2024);
    }

    #[test]
    fn test_1() {
        let mut machine = parse(INPUT);
        machine.run();

        assert_eq!(machine.number('z'), 64_755_511_006_320);
    }

    #[test]
    fn test_2_sample() {
        let mut machine = parse(SAMPLE_3);
        let swaps = machine.find_swaps(2, usize::bitand, 6);

        assert_eq!(swaps, "z00,z01,z02,z05");
    }

    #[test]
    fn test_2() {
        let mut machine = parse(INPUT);
        let mut swaps = HashMap::new();
        swaps.insert("sbg".to_string(), "z19".to_string());
        swaps.insert("z19".to_string(), "sbg".to_string());
        swaps.insert("djg".to_string(), "z12".to_string());
        swaps.insert("z12".to_string(), "djg".to_string());
        swaps.insert("dsd".to_string(), "z37".to_string());
        swaps.insert("z37".to_string(), "dsd".to_string());
        swaps.insert("hjm".to_string(), "mcq".to_string());
        swaps.insert("mcq".to_string(), "hjm".to_string());

        machine.reset(1, 1, 45);
        machine.swaps = swaps.clone();

        // 1. Edge Cases
        let test_cases = vec![
            (0, 0),                                   // Both inputs are zero
            (0, (1usize << 44) - 1),                  // x = 0, y = max 44-bit value
            ((1usize << 44) - 1, 0),                  // x = max 44-bit value, y = 0
            ((1usize << 44) - 1, (1usize << 44) - 1), // x = max 44-bit value, y = max 44-bit value
            (1, 1),                                   // Minimal non-zero values
            ((1usize << 43), (1usize << 43)),         // Carry propagates to the high bit
            ((1usize << 43) - 1, 1),                  // Lower bits are max, single carry
        ];

        for (x, y) in test_cases {
            let expected = (x + y) & ((1usize << 45) - 1); // Expected value (mask to 45 bits)
            machine.reset(x, y, 45);
            machine.swaps = swaps.clone();
            machine.run();
            let result = machine.number('z');
            assert_eq!(
                result, expected,
                "Failed for inputs x = {:#046b}, y = {:#046b} and expected z = {:#046b} but got result = {:#046b}",
                x, y, expected, result
            );
        }

        let mut rng = thread_rng();
        for _ in 0..1_000 {
            // 1000 random tests, adjust count depending on performance
            let x: usize = rng.gen_range(0..(1usize << 44));
            let y: usize = rng.gen_range(0..(1usize << 44));
            let expected = (x + y) & ((1usize << 45) - 1); // Expected value (simulate 45-bit addition)
            machine.reset(x, y, 45);
            machine.swaps = swaps.clone();
            machine.run();
            let result = machine.number('z');
            assert_eq!(
                result, expected,
                "Failed for inputs x = {:#046b}, y = {:#046b} and expected z = {:#046b} but got result = {:#046b}",
                x, y, expected, result
            );
        }
    }
}
