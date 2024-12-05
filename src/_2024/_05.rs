use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
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

    pub fn is_sorted_by(&self, ordering_rules: &HashSet<OrderingRule>) -> bool {
        self.0
            .iter()
            .is_sorted_by(|x, y| ordering_rules.contains(&OrderingRule((**x, **y))))
    }

    pub fn sort_by(&mut self, ordering_rules: &HashSet<OrderingRule>) {
        self.0.sort_by(|x, y| {
            if ordering_rules.contains(&OrderingRule((*x, *y))) {
                Ordering::Less
            } else if ordering_rules.contains(&OrderingRule((*y, *x))) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn parse(input: &str) -> Result<(HashSet<OrderingRule>, Vec<Page>)> {
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
        .collect::<Result<HashSet<OrderingRule>>>()?;

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

pub fn p1(ordering_rules: &HashSet<OrderingRule>, pages: &[Page]) -> u32 {
    pages
        .iter()
        .filter(|page| page.is_sorted_by(ordering_rules))
        .map(Page::middle)
        .sum()
}

#[allow(clippy::manual_inspect)]
pub fn p2(ordering_rules: &HashSet<OrderingRule>, pages: &mut [Page]) -> u32 {
    pages
        .iter_mut()
        .filter(|page| !page.is_sorted_by(ordering_rules))
        .map(|page| {
            page.sort_by(ordering_rules);
            page
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
        let (ordering_rules, mut pages) = parse(SAMPLE).unwrap();
        let sum = p2(&ordering_rules, &mut pages);

        assert_eq!(sum, 123);
    }

    #[test]
    fn test_2() {
        let (ordering_rules, mut pages) = parse(INPUT).unwrap();
        let sum = p2(&ordering_rules, &mut pages);

        assert_eq!(sum, 4_884);
    }
}
