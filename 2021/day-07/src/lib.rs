use std::ops::Range;

use cached::proc_macro::cached;

enum BurnRate {
    Linear,
    Triangular,
}

fn triangle(n: usize) -> usize {
    (n * (n + 1)) / 2
}

fn find_range(collection: &[usize]) -> Option<Range<usize>> {
    let iter = collection.iter().copied();
    if let (Some(min), Some(max)) = (iter.clone().min(), iter.max()) {
        Some(min..max)
    } else {
        None
    }
}

fn fuel_required(target: usize, positions: &[usize], burn_rate: BurnRate) -> usize {
    positions
        .iter()
        .map(|&n| {
            let distance = (n as isize - target as isize).abs() as usize;
            match burn_rate {
                BurnRate::Linear => distance,
                BurnRate::Triangular => triangle(distance),
            }
        })
        .sum()
}

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    find_range(&input)
        .unwrap()
        .map(|n| fuel_required(n, &input, BurnRate::Linear))
        .min()
        .unwrap()
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    find_range(&input)
        .unwrap()
        .map(|n| fuel_required(n, &input, BurnRate::Triangular))
        .min()
        .unwrap()
}

#[cached]
fn parse_input(input: &'static str) -> Vec<usize> {
    input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 37);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 168);
    }
}
