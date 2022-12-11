#![allow(unused)]

use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;

#[derive(Debug)]
struct Op {
    kind: OpKind,
    rhs: Rhs,
}

impl Op {
    pub fn exec(&self, lhs: usize) -> usize {
        let rhs = match self.rhs {
            Rhs::Old => lhs,
            Rhs::Literal(n) => n,
        };

        match self.kind {
            OpKind::Add => lhs + rhs,
            OpKind::Mul => lhs * rhs,
        }
    }
}

#[derive(Debug)]
enum OpKind {
    Add,
    Mul,
}

impl FromStr for OpKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => OpKind::Add,
            "*" => OpKind::Mul,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
enum Rhs {
    Old,
    Literal(usize),
}

impl FromStr for Rhs {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "old" => Rhs::Old,
            n => Rhs::Literal(n.parse()?),
        })
    }
}

#[derive(Debug)]
struct Test {
    div_by: usize, // test divisibility by
    pass: usize,   // throw to monkey[i] on true
    fail: usize,   // throw to monkey[i] on false
}

impl Test {
    pub fn exec(&self, worry_level: usize) -> usize {
        if worry_level % self.div_by == 0 {
            self.pass
        } else {
            self.fail
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Op,
    test: Test,
    inspected: usize,
}

pub fn part_one(input: &'static str) -> usize {
    let mut monkeys = parse_input(input);

    for round in 0..20 {
        // have to iterate over indices to allow mutably borrowing later
        for monkey_i in 0..monkeys.len() {
            let mut move_items = Vec::with_capacity(8);
            {
                let monkey = &mut monkeys[monkey_i];
                for item in &monkey.items {
                    // inspects item
                    let mut item = monkey.op.exec(*item);
                    // bored with item
                    item = item / 3;
                    move_items.push((monkey.test.exec(item), item));
                }
                monkey.inspected += monkey.items.len();
                monkey.items.clear();
            }
            for (recipient, item) in move_items {
                monkeys[recipient].items.push(item);
            }
        }
    }

    monkeys
        .into_iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

pub fn part_two(input: &'static str) -> usize {
    let mut monkeys = parse_input(input);

    let product: usize = monkeys.iter().map(|m| m.test.div_by).product();
    for round in 0..10_000 {
        // have to iterate over indices to allow mutably borrowing later
        for monkey_i in 0..monkeys.len() {
            let mut move_items = Vec::with_capacity(8);
            {
                let monkey = &mut monkeys[monkey_i];
                for item in &monkey.items {
                    // inspects item
                    let mut item = monkey.op.exec(*item);
                    // bored with item
                    item %= product;
                    move_items.push((monkey.test.exec(item), item));
                }
                monkey.inspected += monkey.items.len();
                monkey.items.clear();
            }
            for (recipient, item) in move_items {
                monkeys[recipient].items.push(item);
            }
        }
    }

    monkeys
        .into_iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn parse_input(input: &'static str) -> Vec<Monkey> {
    input
        .split("\n\n")
        // Monkey
        .map(|s| {
            let lines: Vec<_> = s.lines().skip(1).map(|s| s.trim()).collect();

            let items: Vec<usize> = lines[0]
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();

            let (op_kind, rhs) = lines[1]
                .split_once("= old ")
                .unwrap()
                .1
                .split_once(" ")
                .unwrap();
            let op = Op {
                kind: op_kind.parse().unwrap(),
                rhs: rhs.parse().unwrap(),
            };

            let test = Test {
                div_by: lines[2].split(" ").last().unwrap().parse().unwrap(),
                pass: lines[3].split(" ").last().unwrap().parse().unwrap(),
                fail: lines[4].split(" ").last().unwrap().parse().unwrap(),
            };

            Monkey {
                items,
                op,
                test,
                inspected: 0,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 10605);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 2713310158);
    }
}
