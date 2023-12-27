use itertools::Itertools;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::str::FromStr;
use Module::{Broadcaster, Conjunction, FlipFlop};
use Pulse::{High, Low};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Pulse {
    Low,
    High,
}
#[derive(Copy, Clone)]
pub enum Power {
    On,
    Off,
}
pub enum Module {
    FlipFlop(Power),
    Conjunction(Rc<RefCell<HashMap<String, Pulse>>>),
    Broadcaster,
}

pub struct Machine(
    HashMap<String, (Module, Vec<String>)>,
    Vec<(String, Pulse, String)>,
);

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| {
                line.split_once(" -> ")
                    .ok_or("unable to split line".to_string())
            })
            .map_ok(|(module, second)| {
                let destinations = second
                    .split(',')
                    .map(str::trim)
                    .map(str::to_string)
                    .collect();
                if module == "broadcaster" {
                    Ok((module.to_string(), (Broadcaster, destinations)))
                } else {
                    let mut chars = module.chars();
                    let module = match chars.next() {
                        Some('%') => FlipFlop(Power::Off),
                        Some('&') => Conjunction(Rc::new(RefCell::new(HashMap::new()))),
                        _ => return Err(format!("{module} is not a valid module")),
                    };
                    Ok((chars.collect(), (module, destinations)))
                }
            })
            .flatten()
            .collect::<Result<HashMap<String, (Module, Vec<String>)>, _>>()?;

        let conjunctions_by_key: HashMap<String, Rc<RefCell<HashMap<String, Pulse>>>> = map
            .iter()
            .filter_map(|it| {
                if let Conjunction(map) = &it.1 .0 {
                    Some((it.0.clone(), Rc::clone(map)))
                } else {
                    None
                }
            })
            .collect();

        for (key, (_, destinations)) in &map {
            for destination in destinations {
                if let Some(conj) = conjunctions_by_key.get(destination) {
                    conj.borrow_mut().insert(key.clone(), Low);
                }
            }
        }

        let start_list = map
            .get("broadcaster")
            .expect("must have broadcaster")
            .1
            .iter()
            .map(|dest| ("broadcaster".to_string(), Low, dest.clone()))
            .collect::<Vec<(String, Pulse, String)>>();

        Ok(Machine(map, start_list))
    }
}

impl Machine {
    pub fn press_button(
        map: &mut HashMap<String, (Module, Vec<String>)>,
        starting_queue: &mut VecDeque<(String, Pulse, String)>,
        times: u64,
    ) -> u64 {
        let mut low_count = 0;
        let mut high_count = 0;

        for _ in 0..times {
            low_count += 1; // for the button -> broadcaster
            let mut queue = starting_queue.clone();
            Machine::press(map, &mut queue, &mut low_count, &mut high_count);
        }

        low_count * high_count
    }
    pub fn times_for_rx(
        map: &mut HashMap<String, (Module, Vec<String>)>,
        starting_queue: &mut VecDeque<(String, Pulse, String)>,
    ) -> u64 {
        let mut count = 0;
        let mut rx_hit_once = false;
        while !rx_hit_once {
            count += 1;
            let mut queue = starting_queue.clone();
            rx_hit_once = Machine::press(map, &mut queue, &mut 0, &mut 0);
        }
        count
    }
    fn press(
        map: &mut HashMap<String, (Module, Vec<String>)>,
        queue: &mut VecDeque<(String, Pulse, String)>,
        low_count: &mut u64,
        high_count: &mut u64,
    ) -> bool {
        let mut rx_hit_low_times = 0;
        let mut rx_hit_high_times = 0;
        while let Some((source, signal, dest)) = queue.pop_front() {
            match signal {
                High => *high_count += 1,
                Low => *low_count += 1,
            };
            match (signal, dest.as_str()) {
                (High, "rx") => rx_hit_high_times += 1,
                (Low, "rx") => rx_hit_low_times += 1,
                _ => (),
            }
            if let Some((module, destinations)) = map.get_mut(&dest) {
                match (module, signal) {
                    (m @ FlipFlop(Power::Off), Low) => {
                        *m = FlipFlop(Power::On);
                        queue.extend(
                            destinations
                                .iter()
                                .map(|n| (dest.clone(), High, n.to_string())),
                        );
                    }
                    (m @ FlipFlop(Power::On), Low) => {
                        *m = FlipFlop(Power::Off);
                        queue.extend(
                            destinations
                                .iter()
                                .map(|n| (dest.clone(), Low, n.to_string())),
                        );
                    }
                    (FlipFlop(_), High) => (),
                    (Conjunction(seen), _) => {
                        seen.borrow_mut().insert(source, signal);
                        if seen.borrow().values().all(|pulse| *pulse == High) {
                            queue.extend(
                                destinations
                                    .iter()
                                    .map(|n| (dest.clone(), Low, n.to_string())),
                            );
                        } else {
                            queue.extend(
                                destinations
                                    .iter()
                                    .map(|n| (dest.clone(), High, n.to_string())),
                            );
                        }
                    }
                    (Broadcaster, _) => panic!("shouldn't have visited broadcaster twice"),
                }
            }
        }
        rx_hit_high_times == 0 && rx_hit_low_times == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const SAMPLE_2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    const INPUT: &str = include_str!("../../input/2023/20.txt");

    #[test]
    fn test_1_sample() {
        let mut machine: Machine = SAMPLE.parse().unwrap();
        let mut queue: VecDeque<(String, Pulse, String)> = machine
            .1
            .into_iter()
            .map(|it| (it.0, it.1, it.2.clone()))
            .collect::<VecDeque<_>>();

        let mut machine_2: Machine = SAMPLE_2.parse().unwrap();
        let mut queue_2: VecDeque<(String, Pulse, String)> = machine_2
            .1
            .into_iter()
            .map(|it| (it.0, it.1, it.2.clone()))
            .collect::<VecDeque<_>>();

        assert_eq!(
            Machine::press_button(&mut machine.0, &mut queue, 1_000),
            32_000_000
        );
        assert_eq!(
            Machine::press_button(&mut machine_2.0, &mut queue_2, 1_000),
            11_687_500
        );
    }

    #[test]
    fn test_1() {
        let mut machine: Machine = INPUT.parse().unwrap();
        let mut queue: VecDeque<(String, Pulse, String)> = machine
            .1
            .into_iter()
            .map(|it| (it.0, it.1, it.2.clone()))
            .collect::<VecDeque<_>>();

        assert_eq!(
            Machine::press_button(&mut machine.0, &mut queue, 1_000),
            777_666_211
        );
    }

    #[test]
    #[ignore]
    fn test_2() {
        let mut machine: Machine = INPUT.parse().unwrap();
        let mut queue: VecDeque<(String, Pulse, String)> = machine
            .1
            .into_iter()
            .map(|it| (it.0, it.1, it.2.clone()))
            .collect::<VecDeque<_>>();

        assert_eq!(
            Machine::times_for_rx(&mut machine.0, &mut queue),
            32_000_000
        );
    }
}
