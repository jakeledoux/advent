#![allow(dead_code)]

use itertools::Itertools;

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

fn part_one(numbers: &[usize]) -> usize {
    numbers
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn part_two(numbers: &[usize]) -> usize {
    numbers
        .windows(3)
        .map(|window| window.iter().sum::<usize>())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn main() {
    let numbers: Vec<usize> = INPUT
        .lines()
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    dbg!(part_one(&numbers));
    dbg!(part_two(&numbers));
}
