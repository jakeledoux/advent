use std::{num::ParseIntError, str::FromStr};

use anyhow::anyhow;
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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => OpKind::Add,
            "*" => OpKind::Mul,
            _ => return Err(anyhow!("invalid operator")),
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

enum WorryManagement {
    Div,
    Mod,
}

fn find_monkey_business(
    monkeys: &mut [Monkey],
    rounds: usize,
    worry_management: WorryManagement,
) -> usize {
    let product_of_divisors: usize = monkeys.iter().map(|m| m.test.div_by).product();

    // avoid re-allocating this temporary vec
    let mut move_items = Vec::with_capacity(32);
    for _ in 0..rounds {
        // have to iterate over indices to allow mutably borrowing recipients later
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            for mut item in monkey.items.drain(..) {
                item = monkey.op.exec(item);
                item = match worry_management {
                    WorryManagement::Div => item / 3,
                    WorryManagement::Mod => item % product_of_divisors,
                };
                move_items.push((monkey.test.exec(item), item));
                monkey.inspected += 1;
            }
            // `&mut monkey` is dropped

            // transfer items
            for (recipient, item) in move_items.drain(..) {
                monkeys[recipient].items.push(item);
            }
        }
    }

    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

pub fn part_one(input: &'static str) -> usize {
    let mut monkeys = parse_input(input);
    find_monkey_business(&mut monkeys, 20, WorryManagement::Div)
}

pub fn part_two(input: &'static str) -> usize {
    let mut monkeys = parse_input(input);
    find_monkey_business(&mut monkeys, 10_000, WorryManagement::Mod)
}

fn parse_input(input: &'static str) -> Vec<Monkey> {
    // ugliest parser of all time
    input
        .split("\n\n")
        .map(|s| {
            let lines: Vec<_> = s.lines().skip(1).map(|s| s.trim()).collect();
            let items: Vec<usize> = lines[0]
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|s| Ok(s.parse()?))
                .collect::<anyhow::Result<_>>()?;
            let (op_kind, rhs) = lines[1]
                .split_once("= old ")
                .unwrap()
                .1
                .split_once(' ')
                .unwrap();
            let op = Op {
                kind: op_kind.parse()?,
                rhs: rhs.parse()?,
            };
            let test = Test {
                div_by: lines[2].split(' ').last().unwrap().parse()?,
                pass: lines[3].split(' ').last().unwrap().parse()?,
                fail: lines[4].split(' ').last().unwrap().parse()?,
            };

            Ok(Monkey {
                items,
                op,
                test,
                inspected: 0,
            })
        })
        .collect::<anyhow::Result<_>>()
        .unwrap()
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
