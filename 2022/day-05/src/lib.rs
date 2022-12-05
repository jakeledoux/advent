use itertools::Itertools;

const EXPECT_AOC: &str = "Advent of Code input will never be malformed";

type Unit = char;

#[derive(Debug, Default)]
pub struct Supplies {
    stacks: [Vec<Unit>; 9],
}

impl Supplies {
    pub fn transfer(&mut self, count: usize, from_stack: usize, to_stack: usize, method: Method) {
        let mut buffer: Vec<_> = (0..count)
            .map(|_| self.stacks[from_stack].pop().expect(EXPECT_AOC))
            .collect();
        if matches!(method, Method::Group) {
            buffer.reverse();
        }
        self.stacks[to_stack].append(&mut buffer);
    }

    pub fn skim(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.last())
            .collect()
    }
}

pub enum Method {
    Single,
    Group,
}

pub fn part_one(input: &'static str) -> String {
    let (mut supplies, instructions) = parse_input(input);
    for (count, from, to) in instructions {
        supplies.transfer(count, from - 1, to - 1, Method::Single);
    }
    supplies.skim()
}

pub fn part_two(input: &'static str) -> String {
    let (mut supplies, instructions) = parse_input(input);
    for (count, from, to) in instructions {
        supplies.transfer(count, from - 1, to - 1, Method::Group);
    }
    supplies.skim()
}

fn parse_input(input: &'static str) -> (Supplies, Vec<(usize, usize, usize)>) {
    let (state, instructions) = input.split_once("\n\n").expect(EXPECT_AOC);
    let mut supplies = Supplies::default();

    // find all crates and push them into their stacks
    state.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .enumerate()
            .filter(|(_i, c)| c.is_alphabetic())
            .for_each(|(i, c)| supplies.stacks[i / 4].push(c));
    });

    // filter to words that parse into ints and collect those into one tuple per line
    // "move 2 from 1 to 3" -> (3, 1, 3)
    let instructions = instructions
        .lines()
        .map(|s| {
            s.split(' ')
                .filter_map(|s| s.parse().ok())
                .collect_tuple()
                .expect(EXPECT_AOC)
        })
        .collect();

    (supplies, instructions)
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), "CMZ");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), "MCD");
    }
}
