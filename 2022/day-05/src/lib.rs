use itertools::Itertools;

#[derive(Debug, Default)]
pub struct Supplies {
    stacks: [Vec<Unit>; 9],
}

impl Supplies {
    pub fn push(&mut self, stack: usize, unit: Unit) {
        self.stacks[stack].push(unit)
    }

    pub fn pop(&mut self, stack: usize) -> Option<Unit> {
        self.stacks[stack].pop()
    }

    pub fn transfer(&mut self, from_stack: usize, to_stack: usize) {
        if let Some(unit) = self.pop(from_stack) {
            self.push(to_stack, unit);
        }
    }

    pub fn bulk_transfer(&mut self, count: usize, from_stack: usize, to_stack: usize) {
        let mut temp_stack = Vec::new();
        for _ in 0..count {
            if let Some(unit) = self.pop(from_stack) {
                temp_stack.push(unit);
            }
        }

        for unit in temp_stack.into_iter().rev() {
            self.push(to_stack, unit);
        }
    }

    pub fn skim(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.last())
            .collect()
    }
}

type Unit = char;

pub fn part_one(input: &'static str) -> String {
    let (mut supplies, instructions) = parse_input(input);
    for (count, from, to) in instructions {
        for _ in 0..count {
            supplies.transfer(from - 1, to - 1);
        }
    }
    supplies.skim()
}

pub fn part_two(input: &'static str) -> String {
    let (mut supplies, instructions) = parse_input(input);
    for (count, from, to) in instructions {
        supplies.bulk_transfer(count, from - 1, to - 1);
    }
    supplies.skim()
}

fn parse_input(input: &'static str) -> (Supplies, Vec<(usize, usize, usize)>) {
    let (state, instructions) = input.split_once("\n\n").unwrap();
    let state = state.lines().rev().skip(1);
    let mut supplies = Supplies::default();
    state.for_each(|line| {
        line.chars()
            .enumerate()
            .filter(|(_i, c)| c.is_alphabetic())
            .for_each(|(i, c)| supplies.push(i / 4, c));
    });

    (
        supplies,
        instructions
            .lines()
            .map(|s| {
                s.split(' ')
                    .filter_map(|s| s.parse().ok())
                    .collect_tuple()
                    .unwrap()
            })
            .collect(),
    )
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
