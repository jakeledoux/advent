#![allow(dead_code)]

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

fn load_generation(fish: &[usize]) -> [usize; 9] {
    fish.iter().fold([0; 9], |mut gen, &f| {
        gen[f] += 1;
        gen
    })
}

fn advance_generation(gen: &[usize; 9]) -> [usize; 9] {
    let mut new_gen = *gen;
    new_gen.rotate_left(1);
    new_gen[6] += gen[0];
    new_gen
}

fn simulate_population(fish: &[usize], generations: usize) -> usize {
    (0..generations)
        .fold(load_generation(fish), |gen, _| advance_generation(&gen))
        .into_iter()
        .sum()
}

fn part_one(input: &[usize]) -> usize {
    simulate_population(input, 80)
}

fn part_two(input: &[usize]) -> usize {
    simulate_population(input, 256)
}

fn main() {
    let input: Vec<usize> = SAMPLE
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    dbg!(part_one(&input));
    dbg!(part_two(&input));
}
