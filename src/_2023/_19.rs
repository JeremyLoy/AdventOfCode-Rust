use crate::_2023::_19::Rule::{Accept, Comparison, Destination, Reject};
use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

pub struct System {
    workflows: HashMap<String, Vec<Rule>>,
    parts: Vec<HashMap<char, i32>>,
}

impl System {
    pub fn process(&self) -> i32 {
        self.parts
            .iter()
            .filter(|part| self.process_part(part))
            .map(|part| part.values().sum::<i32>())
            .sum()
    }
    pub fn process_part(&self, part: &HashMap<char, i32>) -> bool {
        let mut cur = "in";
        loop {
            let rules = self
                .workflows
                .get(cur)
                .unwrap_or_else(|| panic!("workflow {cur} does not exist"));
            for rule in rules {
                match rule {
                    Accept => return true,
                    Reject => return false,
                    Destination(destination) => {
                        cur = destination.as_str();
                        break;
                    }
                    Comparison {
                        rating,
                        condition,
                        value,
                        destination,
                    } => {
                        let gt = |a, b| a > b;
                        let lt = |a, b| a < b;
                        let op = if condition == ">" { gt } else { lt };
                        let part_value = part
                            .get(rating)
                            .unwrap_or_else(|| panic!("rating {rating} is not in xmas"));
                        if op(*part_value, *value) {
                            match destination.as_str() {
                                "A" => return true,
                                "R" => return false,
                                _ => cur = destination.as_str(),
                            };
                            break;
                        }
                    }
                }
            }
        }
    }
}

pub struct Workflow {
    key: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key, rules) = s.split_once('{').ok_or("unable to split workflow")?;
        let rules = rules
            .trim_end_matches('}')
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Workflow {
            key: key.to_string(),
            rules,
        })
    }
}

pub enum Rule {
    Comparison {
        rating: char,
        condition: String,
        value: i32,
        destination: String,
    },
    Destination(String),
    Accept,
    Reject,
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            let (value, destination) = s[2..]
                .split_once(':')
                .ok_or(format!("unable to split {s} into amount and destination"))?;
            Ok(Comparison {
                rating: s.chars().next().expect("string should not be empty"),
                condition: s[1..=1].to_string(),
                value: value.parse()?,
                destination: destination.to_string(),
            })
        } else {
            Ok(match s {
                "A" => Accept,
                "R" => Reject,
                _ => Destination(s.to_string()),
            })
        }
    }
}

fn parse_parts(s: &str) -> Result<HashMap<char, i32>, Box<dyn Error>> {
    s.trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
        .map(|s| s.split_once('=').ok_or("ratings must be splittable"))
        .map_ok(|(key, value)| {
            let value = value.parse::<i32>()?;
            Ok((key.chars().next().unwrap_or_default(), value))
        })
        .flatten()
        .collect()
}

impl FromStr for System {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<System, Box<dyn Error>> {
        let (workflows, parts) = input
            .split_once("\n\n")
            .ok_or("could not split workflows and parts")?;
        let workflows = workflows
            .lines()
            .map(str::parse)
            .map_ok(|workflow: Workflow| (workflow.key, workflow.rules))
            .collect::<Result<_, _>>()?;
        let parts = parts
            .lines()
            .map(parse_parts)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(System { workflows, parts })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    const INPUT: &str = include_str!("../../input/2023/19.txt");

    #[test]
    fn test_1_sample() {
        let system: System = SAMPLE.parse().unwrap();

        assert_eq!(system.process(), 19_114);
    }

    #[test]
    fn test_1() {
        let system: System = INPUT.parse().unwrap();

        assert_eq!(system.process(), 280_909);
    }

    #[test]
    #[ignore]
    fn test_2_sample() {
        let system: System = SAMPLE.parse().unwrap();

        assert_eq!(system.process(), 1 + 1);
    }

    #[test]
    #[ignore]
    fn test_2() {
        let system: System = INPUT.parse().unwrap();

        assert_eq!(system.process(), 1 + 1);
    }
}
