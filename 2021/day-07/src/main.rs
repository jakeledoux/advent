#![allow(dead_code)]

use std::ops::Range;

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

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

fn part_one(input: &[usize]) -> usize {
    find_range(input)
        .unwrap()
        .map(|n| fuel_required(n, input, BurnRate::Linear))
        .min()
        .unwrap()
}

fn part_two(input: &[usize]) -> usize {
    find_range(input)
        .unwrap()
        .map(|n| fuel_required(n, input, BurnRate::Triangular))
        .min()
        .unwrap()
}

fn main() {
    let input: Vec<usize> = INPUT
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    dbg!(part_one(&input));
    dbg!(part_two(&input));
}
