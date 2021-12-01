#![allow(dead_code)]

use itertools::Itertools;

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

fn part_one<T: Clone + Ord, I: Iterator<Item = T>>(numbers: I) -> usize {
    numbers.tuple_windows().filter(|(a, b)| b > a).count()
}

fn part_two(numbers: &[usize]) -> usize {
    part_one(
        numbers
            .windows(3)
            .map(|window| window.iter().sum::<usize>()),
    )
}

fn main() {
    let numbers = INPUT
        .lines()
        .filter_map(|s| s.trim().parse().ok())
        .collect_vec();

    dbg!(part_one(numbers.iter()));
    dbg!(part_two(&numbers));
}
