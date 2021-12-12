use cached::proc_macro::cached;
use itertools::Itertools;
use petgraph::graphmap::UnGraphMap;

type CaveMap<'a> = UnGraphMap<&'a str, ()>;

fn recursive_solver<'a, F: Fn(&str, &[&str]) -> bool>(
    history: &[&'a str],
    target: &'a str,
    map: &'a CaveMap,
    filter: &F,
) -> Vec<Vec<&'a str>> {
    let mut paths = vec![];
    if let Some(pos) = history.last() {
        if *pos == target {
            return vec![history.to_vec()];
        }
        paths.extend(
            map.neighbors(pos)
                .filter(|neighbor| filter(neighbor, history))
                .flat_map(|neighbor| {
                    let new_history = [history, &[neighbor]].concat();
                    recursive_solver(&new_history, target, map, filter)
                }),
        );
    }
    paths
}

fn make_filter(base_limit: usize, secondary_limit: usize) -> impl Fn(&str, &[&str]) -> bool {
    move |n: &str, history: &[&str]| -> bool {
        let counts = history
            .iter()
            .copied()
            .filter(|e| e.chars().all(char::is_lowercase))
            .counts();
        let limit = if n == "start" || counts.values().contains(&secondary_limit) {
            base_limit
        } else {
            secondary_limit
        };
        let instances = counts.get(n).copied().unwrap_or(0);
        instances < limit
    }
}

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    recursive_solver(&["start"], "end", &input, &make_filter(1, 1)).len()
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    recursive_solver(&["start"], "end", &input, &make_filter(1, 2)).len()
}

#[cached]
fn parse_input(input: &'static str) -> CaveMap {
    input
        .lines()
        .map(|edge| edge.split_once('-').unwrap())
        .fold(CaveMap::new(), |mut map, edge| {
            map.add_edge(edge.0, edge.1, ());
            map
        })
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one_sample() {
        assert_eq!(part_one(SAMPLE), 226);
    }

    #[test]
    fn test_part_two_sample() {
        assert_eq!(part_two(SAMPLE), 3509);
    }
}
