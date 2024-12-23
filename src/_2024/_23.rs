use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split('-').collect_tuple().unwrap();
        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }
    graph
}

pub fn p2(graph: &HashMap<&str, HashSet<&str>>) -> String {
    let mut max_clique = vec![];

    let keys = graph.keys().collect_vec();

    for &&key in &keys {
        let mut clique = vec![key];

        for &&neighbor in &keys {
            if neighbor != key
                && clique
                    .iter()
                    .all(|&member| graph[member].contains(neighbor))
            {
                clique.push(neighbor);
            }
        }

        if clique.len() > max_clique.len() {
            max_clique = clique;
        }
    }

    // Join the clique into a comma-separated string for the result
    max_clique.iter().sorted().join(",")
}

pub fn p1(graph: &HashMap<&str, HashSet<&str>>) -> usize {
    let t_keys = graph.keys().filter(|k| k.starts_with('t')).collect_vec();

    let mut result = vec![];

    for t_key in &t_keys {
        if let Some(connections) = graph.get(*t_key) {
            for connection in connections.iter().combinations(2) {
                let (a, b) = (connection[0], connection[1]);
                if !graph.get(a).unwrap().contains(b) {
                    continue;
                }
                let mut v = vec![a, b, t_key];
                v.sort();
                result.push(v);
            }
        }
    }

    result.into_iter().unique().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
    const INPUT: &str = include_str!("../../input/2024/23.txt");

    #[test]
    fn test_1_sample() {
        let graph = parse(SAMPLE);
        let count = p1(&graph);

        assert_eq!(count, 7);
    }

    #[test]
    fn test_1() {
        let graph = parse(INPUT);
        let count = p1(&graph);

        assert_eq!(count, 1_240);
    }

    #[test]
    fn test_2_sample() {
        let graph = parse(SAMPLE);
        let count = p2(&graph);

        assert_eq!(count, "co,de,ka,ta");
    }

    #[test]
    fn test_2() {
        let graph = parse(INPUT);
        let count = p2(&graph);

        assert_eq!(count, "am,aq,by,ge,gf,ie,mr,mt,rw,sn,te,yi,zb");
    }
}
