#![allow(dead_code)]

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

fn part_one(numbers: &[usize]) -> usize {
    numbers
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn part_two(numbers: &[usize]) -> usize {
    let numbers: Vec<usize> = numbers
        .windows(3)
        .map(|window| window.into_iter().sum())
        .collect();

    numbers
        .as_slice()
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn main() {
    let numbers: Vec<_> = INPUT
        .split('\n')
        .filter_map(|s| match s.trim() {
            "" => None,
            _ => s.trim().parse::<usize>().ok(),
        })
        .collect();

    dbg!(part_one(&numbers));
    dbg!(part_two(&numbers));
}
