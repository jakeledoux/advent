use std::collections::HashMap;

use cached::proc_macro::cached;
use itertools::Itertools;
use petgraph::graphmap::DiGraphMap;

type RuleMap = HashMap<(char, char), char>;
type PolyMap = DiGraphMap<char, usize>;

#[derive(Clone)]
struct Polymer {
    map: PolyMap,
    counts: HashMap<char, usize>,
    rules: RuleMap,
}

impl Polymer {
    pub fn step(&mut self) {
        let mut new_map = self.map.clone();
        self.map.all_edges().for_each(|(a, b, &count)| {
            // Remove these instances of A -> B edge from new map
            Polymer::decrease_edge(&mut new_map, a, b, count);
            // Find C and increment its counter by number of new edges
            let &c = self.rules.get(&(a, b)).unwrap();
            let old_count = self.counts.get(&c).unwrap_or(&0);
            let c_count = old_count + count;
            self.counts.insert(c, c_count);
            // Add or update edges A -> C, C -> B to new map
            Polymer::increase_edge(&mut new_map, a, c, count);
            Polymer::increase_edge(&mut new_map, c, b, count);
        });
        self.map = new_map;
    }

    fn increase_edge(map: &mut PolyMap, a: char, b: char, count: usize) {
        let weight = map.edge_weight(a, b).unwrap_or(&0) + count;
        map.add_edge(a, b, weight);
    }

    fn decrease_edge(map: &mut PolyMap, a: char, b: char, count: usize) {
        let weight = map.edge_weight(a, b).unwrap() - count;
        map.add_edge(a, b, weight);
    }
}

pub fn part_one(input: &'static str) -> usize {
    let mut polymer = parse_input(input);
    (0..10).for_each(|_| polymer.step());
    let (min, max) = polymer.counts.into_values().minmax().into_option().unwrap();
    max - min
}

pub fn part_two(input: &'static str) -> usize {
    let mut polymer = parse_input(input);
    (0..40).for_each(|_| polymer.step());
    let (min, max) = polymer.counts.into_values().minmax().into_option().unwrap();
    max - min
}

#[cached]
fn parse_input(input: &'static str) -> Polymer {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let polymap = template
        .chars()
        .tuple_windows()
        .fold(PolyMap::new(), |mut polymer, (a, b)| {
            let weight = polymer.edge_weight(a, b).unwrap_or(&0) + 1;
            polymer.add_edge(a, b, weight);
            polymer
        });
    let counts = template.chars().counts();
    let rules = rules
        .lines()
        .map(|line| {
            let (edge, node) = line.split_once(" -> ").unwrap();
            let edge = edge.chars().collect_tuple().unwrap();
            (edge, node.chars().next().unwrap())
        })
        .collect();
    Polymer {
        map: polymap,
        counts,
        rules,
    }
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 1588);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 2188189693529);
    }
}
