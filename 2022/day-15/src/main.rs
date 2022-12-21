#![allow(special_module_name)]
use lib::{part_one, part_two};

pub mod lib;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT, 2000000));
    println!("Part two: {}", part_two(INPUT));
}
