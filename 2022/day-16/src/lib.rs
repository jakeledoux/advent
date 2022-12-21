#![allow(unused)]

use std::{
    collections::{BinaryHeap, HashMap},
    fmt,
    str::FromStr,
};

use itertools::Itertools;
use petgraph::{algo, prelude::*};
use regex::Regex;

const ID_BASE: usize = 26;
const ID_OFFSET: usize = 64;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct ValveId(usize);

impl fmt::Debug for ValveId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (a, b) = (self.0 / ID_BASE + ID_OFFSET, self.0 % ID_BASE + ID_OFFSET);
        write!(
            f,
            "{}{}",
            char::from_u32(a as u32).unwrap(),
            char::from_u32(b as u32).unwrap()
        )
    }
}

impl FromStr for ValveId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.chars().map(|c| c as usize - ID_OFFSET).collect_tuple() {
            Ok(ValveId(a * ID_BASE + b))
        } else {
            Err(())
        }
    }
}

#[derive(Default, Debug)]
struct ValveNetwork {
    graph: DiGraphMap<ValveId, usize>,
    flow_rates: HashMap<ValveId, usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    time: usize,
    flow: usize,
    pressure: usize,
    visited: Vec<ValveId>,
    location: ValveId,
}

impl State {
    pub fn travel_to(mut self, valve: ValveId, cost: usize, flow: usize) -> Self {
        self.location = valve;
        self.visited.push(valve);
        self.pressure += self.flow * cost;
        self.flow += flow;
        self.time += cost;
        self
    }
}

impl State {
    pub fn finalize(mut self) -> Self {
        self.pressure += (30 - self.time) * self.flow;
        self.time = 30;
        self
    }
}

impl Default for State {
    fn default() -> Self {
        let start = "AA".parse().unwrap();
        Self {
            time: 0,
            flow: 0,
            pressure: 0,
            visited: vec![start],
            location: start,
        }
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pressure.cmp(&other.pressure)
    }
}

fn move_options<'a>(
    network: &'a ValveNetwork,
    state: &'a State,
) -> impl Iterator<Item = ((ValveId, ValveId, &'a usize), usize)> + 'a {
    network
        .graph
        .edges_directed(state.location, Outgoing)
        .filter(|edge| !state.visited.contains(&edge.1))
        .filter(|edge| state.time + edge.2 <= 30)
        .filter_map(|edge| {
            let flow = network.flow_rates[&edge.1];
            (flow > 0).then_some((edge, flow))
        })
}

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);

    {
        let mut queue = BinaryHeap::from([State::default()]);
        let mut solutions = Vec::new();

        while let Some(state) = queue.pop() {
            for (edge, flow) in move_options(&input, &state) {
                let new_state = state
                    .clone()
                    .travel_to(edge.1, *edge.2, input.flow_rates[&edge.1]);

                if move_options(&input, &new_state).count() == 0 {
                    solutions.push(new_state.finalize());
                } else {
                    queue.push(new_state);
                }
            }
        }

        dbg!(solutions
            .iter()
            .map(|state| state.pressure)
            .sorted()
            .rev()
            .collect_vec());
    }

    1
}

pub fn part_two(input: &'static str) -> usize {
    // let input = parse_input(input);
    0
}

fn parse_input(input: &'static str) -> ValveNetwork {
    let mut valves = ValveNetwork::default();
    let mut raw_graph: DiGraphMap<ValveId, ()> = DiGraphMap::new();
    let mut edges = vec![];
    let pattern = Regex::new(r"([A-Z]{2}).*rate=(\d+);.*valves? ((?:[A-Z]{2}(?:, )?)+)").unwrap();
    input
        .lines()
        .filter_map(|s| pattern.captures(s))
        .for_each(|c| {
            let node = c[1].parse().unwrap();
            raw_graph.add_node(node);
            let old_value = valves.flow_rates.insert(node, c[2].parse().unwrap());
            assert!(old_value.is_none()); // if old_value is some, then ValveId failed to generate
                                          // unique integers
            c[3].split(", ")
                .for_each(|s| edges.push((node, s.parse().unwrap())))
        });

    for (a, b) in edges {
        raw_graph.add_edge(a, b, ());
        raw_graph.add_edge(b, a, ());
    }

    for ((a, b), dist) in algo::floyd_warshall(&raw_graph, |_| 1).unwrap() {
        if a == b {
            continue;
        }

        valves.graph.add_edge(a, b, dist);
        valves.graph.add_edge(b, a, dist);
    }

    valves
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 0);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 0);
    }
}
