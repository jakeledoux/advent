use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    input
        .into_iter()
        .map(|(a, b)| {
            *a.intersection(&b)
                .next()
                .expect("one intersection is guaranteed")
        })
        .sum()
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    input
        .into_iter()
        .map(|(a, b)| a.union(&b).copied().collect::<HashSet<usize>>())
        .chunks(3)
        .into_iter()
        .flat_map(|chunk| {
            chunk
                .into_iter()
                .reduce(|a, b| a.intersection(&b).copied().collect())
                .expect("one intersection is guaranteed")
        })
        .sum()
}

fn parse_input(input: &'static str) -> Vec<(HashSet<usize>, HashSet<usize>)> {
    let parse = |compartment: &str| {
        HashSet::from_iter(
            compartment
                .chars()
                .map(|c| c as usize - if c.is_uppercase() { 38 } else { 96 }),
        )
    };
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| (parse(a), parse(b)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 157);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 70);
    }
}
