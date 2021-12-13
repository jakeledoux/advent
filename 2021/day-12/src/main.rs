use lib::{part_one, part_two};

mod lib;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one answer: {}", part_one(INPUT));
    println!("Part two answer: {}", part_two(INPUT));
}
