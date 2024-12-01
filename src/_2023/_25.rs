use itertools::Itertools;
use rand::prelude::{IteratorRandom, SliceRandom};
use rand::thread_rng;
use rayon::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct WiringDiagram {
    graph: HashMap<String, Vec<String>>,
}
// Removes the primary entry for old_node in the graph
// Because this graph is not directed,
// references to old_node in its own connections are additionally replaced
fn replace_node(graph: &mut HashMap<String, Vec<String>>, old_node: &str, new_node: &str) {
    let olds_connections = graph.remove(old_node).unwrap();
    for connection in olds_connections {
        let back_connections = graph.get_mut(&connection).unwrap();
        for node in back_connections {
            if *node == old_node {
                *node = new_node.to_string();
            }
        }
    }
}

// Combines A and B. 1. a new entry is made in the graph that represents their union.
// Connections for A and B are concatenated under A-B.
// References to A and B are replaced with A-B in its original connections (because the graph is not directed)
fn combine_nodes(graph: &mut HashMap<String, Vec<String>>, a: &str, b: &str) {
    let new_node = format!("{a}-{b}");
    let a_connected = graph.get(a).unwrap().clone();
    let b_connected = graph.get(b).unwrap().clone();

    graph.insert(
        new_node.to_string(),
        a_connected
            .into_iter()
            .filter(|v| v != b)
            .chain(b_connected.into_iter().filter(|v| v != a))
            .collect(),
    );
    replace_node(graph, a, &new_node);
    replace_node(graph, b, &new_node);
}
impl WiringDiagram {
    /*
    https://en.wikipedia.org/wiki/Karger%27s_algorithm
    This is a probabilistic algorithm, and needs to be repeated an unknown number
    of times before returning.

    To reduce computation time, I'm using Rayon's par_iter to run NUM_CPU threads.
    However, Rayon doesn't have any mechanism for early return, so I store the first found value
    in an ArcAtomic. At the beginning of each iteration, I check it and short-circuit by returning if it exists

    I believe that the input is constructed in such a way that there is only one possible
    3 cut solution,and 3 is the minimum, so its all hardcoded for now. It would be trivial to parameterize the
    number of attempts and short circuit though.

    On my Mac this ends up being <100ms give or take.
    */
    pub fn mincut(&self) -> usize {
        let found = Arc::new(AtomicUsize::new(0));
        (0..u128::MAX)
            .into_par_iter()
            .map(|_| {
                let mut rng = thread_rng();
                let found = Arc::clone(&found);
                loop {
                    let mut graph = self.graph.clone();
                    // stop looping once split in half
                    while graph.len() > 2 {
                        // short circuit opportunity
                        if found.load(Ordering::Relaxed) != 0 {
                            return found.load(Ordering::Relaxed);
                        }

                        // a is a random node and b is a random node that is connected to a
                        let a = graph.keys().choose(&mut rng).unwrap().clone();
                        let b = graph
                            .get(&a)
                            .and_then(|v| v.choose(&mut rng))
                            .unwrap()
                            .clone();

                        combine_nodes(&mut graph, &a, &b);
                    }

                    let ((a, connections), (b, _)) = graph.iter().collect_tuple().unwrap();

                    // connections.len() is the number of cuts
                    // its always the same length for A and B since there are only two nodes remaining, so just
                    // check one
                    if connections.len() == 3 {
                        // we joined the keys using '-' so count them and add 1 to derive
                        // the number of nodes comprising each half of the graph
                        let a_node_count = a.chars().filter(|c| *c == '-').count() + 1;
                        let b_node_count = b.chars().filter(|c| *c == '-').count() + 1;
                        // multiplying them is the AoC solution but not actually part of the algo.
                        let answer = a_node_count * b_node_count;
                        // Cache so that other threads return early
                        found.store(answer, Ordering::Relaxed);
                        return answer;
                    }
                }
            })
            .find_first(|_| true)
            .unwrap()
    }
}

impl FromStr for WiringDiagram {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        for line in s.lines() {
            let Some((key, values)) = line.split_once(':') else {
                return Err("unable to split components");
            };
            for value in values.split_whitespace() {
                graph
                    .entry(key.to_string())
                    .or_default()
                    .push(value.to_string());
                graph
                    .entry(value.to_string())
                    .or_default()
                    .push(key.to_string());
            }
        }

        Ok(WiringDiagram { graph })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
    const INPUT: &str = include_str!("../../input/2023/25.txt");

    #[test]
    fn test_1_sample() {
        let wiring: WiringDiagram = SAMPLE.parse().unwrap();

        assert_eq!(wiring.mincut(), 54);
    }

    #[test]
    fn test_1() {
        let wiring: WiringDiagram = INPUT.parse().unwrap();

        assert_eq!(wiring.mincut(), 520_380);
    }
}
