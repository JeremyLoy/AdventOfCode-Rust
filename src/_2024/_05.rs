use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct OrderingRule((u8, u8));

#[derive(Debug)]
pub struct Page(Vec<u8>);

impl Page {
    pub fn middle(&self) -> u32 {
        let length = self.0.len();
        assert_ne!(length, 0, "Cannot get middle element of an empty Vec");
        assert_ne!(
            length % 2,
            0,
            "Cannot get middle element of a Vec with an even number of elements"
        );
        u32::from(self.0[length / 2])
    }
    pub fn applies(&self, ordering_rule: &OrderingRule) -> bool {
        self.ordering_rule_idx(ordering_rule).is_none()
    }
    pub fn ordering_rule_idx(&self, ordering_rule: &OrderingRule) -> Option<(usize, usize)> {
        let (left, right) = ordering_rule.0;

        let index_of_left = self.0.iter().position(|&value| value == left);
        let index_of_right = self.0.iter().position(|&value| value == right);

        match (index_of_left, index_of_right) {
            (Some(l), Some(r)) if l > r => Some((l, r)),
            _ => None,
        }
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn parse(input: &str) -> Result<(Vec<OrderingRule>, Vec<Page>)> {
    let (ordering_rules, pages) = input
        .split_once("\n\n")
        .ok_or(anyhow!("could not split on ordering rules and pages"))?;

    let ordering_rules = ordering_rules
        .lines()
        .map(|line| {
            line.split_once('|')
                .ok_or(anyhow!("could not split on |"))
                .map(|(a, b)| Ok((a.parse()?, b.parse()?)))?
        })
        .map_ok(OrderingRule)
        .collect::<Result<Vec<OrderingRule>>>()?;

    let pages = pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().map_err(|e: ParseIntError| anyhow!(e)))
                .collect::<Result<Vec<_>>>()
        })
        .map_ok(Page)
        .collect::<Result<Vec<Page>>>()?;

    Ok((ordering_rules, pages))
}

pub fn p1(ordering_rules: &[OrderingRule], pages: &[Page]) -> u32 {
    pages
        .iter()
        .filter(|page| {
            ordering_rules
                .iter()
                .all(|ordering_rule| page.applies(ordering_rule))
        })
        .map(Page::middle)
        .sum()
}

pub fn p2(ordering_rules: &[OrderingRule], pages: &[Page]) -> u32 {
    pages
        .iter()
        .filter(|page| {
            !ordering_rules
                .iter()
                .all(|ordering_rule| page.applies(ordering_rule))
        })
        .map(|page| {
            let mut new_page = Page(page.0.clone());
            while !ordering_rules
                .iter()
                .all(|ordering_rule| new_page.applies(ordering_rule))
            {
                if let Some(Some((from_index, to_index)), ..) = ordering_rules
                    .iter()
                    .map(|or| new_page.ordering_rule_idx(or))
                    .find(Option::is_some)
                {
                    // Remove the element from the current position
                    let element = new_page.0.remove(from_index);

                    // Adjust the target index if removing has shifted it
                    let adjusted_index = if from_index < to_index {
                        to_index - 1
                    } else {
                        to_index
                    };

                    // Insert the element at the new position
                    new_page.0.insert(adjusted_index, element);
                }
            }

            new_page
        })
        .map(|p| p.middle())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    const INPUT: &str = include_str!("../../input/2024/05.txt");

    #[test]
    fn test_1_sample() {
        let (ordering_rules, pages) = parse(SAMPLE).unwrap();
        let sum = p1(&ordering_rules, &pages);

        assert_eq!(sum, 143);
    }

    #[test]
    fn test_1() {
        let (ordering_rules, pages) = parse(INPUT).unwrap();
        let sum = p1(&ordering_rules, &pages);

        assert_eq!(sum, 6_041);
    }

    #[test]
    fn test_2_sample() {
        let (ordering_rules, pages) = parse(SAMPLE).unwrap();
        let sum = p2(&ordering_rules, &pages);

        assert_eq!(sum, 123);
    }

    #[test]
    fn test_2() {
        let (ordering_rules, pages) = parse(INPUT).unwrap();
        let sum = p2(&ordering_rules, &pages);

        assert_eq!(sum, 4_884);
    }
}
