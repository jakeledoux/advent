use lib::part_one;

pub mod lib;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
}
