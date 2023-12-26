use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

pub struct System {
    workflows: HashMap<String, Vec<Rule>>,
    parts: Vec<Part>,
}

impl System {
    pub fn process(&self) -> i32 {
        let parts = self.parts.iter().filter(|part| self.process_part(part));
        parts.map(Part::score).sum()
    }
    pub fn process_part(&self, part: &Part) -> bool {
        let gt = |a, b| a > b;
        let lt = |a, b| a < b;
        let mut cur = "in";
        while cur != "A" && cur != "R" {
            let rules = self
                .workflows
                .get(cur)
                .unwrap_or_else(|| panic!("workflow {cur} does not exist"));
            for rule in rules {
                let op = if rule.condition == ">" { gt } else { lt };
                match rule.rating.as_str() {
                    "x" => {
                        if op(part.x, rule.value) {
                            cur = rule.destination.as_str();
                            break;
                        }
                    }
                    "m" => {
                        if op(part.m, rule.value) {
                            cur = rule.destination.as_str();
                            break;
                        }
                    }
                    "a" => {
                        if op(part.a, rule.value) {
                            cur = rule.destination.as_str();
                            break;
                        }
                    }
                    "s" => {
                        if op(part.s, rule.value) {
                            cur = rule.destination.as_str();
                            break;
                        }
                    }
                    _ => cur = rule.destination.as_str(),
                }
            }
        }
        match cur {
            "A" => true,
            "R" => false,
            _ => panic!("somehow broke out of process_part without accept or rejection"),
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
pub struct Rule {
    rating: String,
    condition: String,
    value: i32,
    destination: String,
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            let (value, destination) = s[2..]
                .split_once(':')
                .ok_or(format!("unable to split {s} into amount and destination"))?;
            Ok(Rule {
                rating: s[0..=0].to_string(),
                condition: s[1..=1].to_string(),
                value: value.parse()?,
                destination: destination.to_string(),
            })
        } else {
            Ok(Rule {
                rating: String::default(),
                condition: String::default(),
                value: 0,
                destination: s.to_string(),
            })
        }
    }
}

pub struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    pub fn score(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(|s| &s[2..])
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        if parts.len() != 4 {
            return Err(format!("unexpected number of parts {}", parts.len()).into());
        }
        Ok(Part {
            x: parts[0],
            m: parts[1],
            a: parts[2],
            s: parts[3],
        })
    }
}
impl FromStr for System {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<System, Box<dyn Error>> {
        let (workflows, parts) = input
            .split_once("\n\n")
            .ok_or("could not split workflows and parts")?;
        let workflow_lines = workflows.lines();
        let mut workflows = HashMap::new();
        for workflow in workflow_lines {
            let workflow: Workflow = workflow.parse()?;
            workflows.insert(workflow.key, workflow.rules);
        }
        let parts = parts
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Part>, _>>()?;
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
    fn test_2_sample() {
        let system: System = SAMPLE.parse().unwrap();

        assert_eq!(system.process(), 1 + 1);
    }

    #[test]
    fn test_2() {
        let system: System = INPUT.parse().unwrap();

        assert_eq!(system.process(), 1 + 1);
    }
}
