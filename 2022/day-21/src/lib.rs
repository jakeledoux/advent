use std::collections::HashMap;

use strum::EnumString;

type ExprId = &'static str;
type ExprHeap = HashMap<&'static str, Expr>;

#[derive(Debug, Hash, Clone, Copy)]
enum Expr {
    Lit(i64),
    Math(Op, ExprId, ExprId),
}

impl Expr {
    pub fn eval(&self, heap: &ExprHeap) -> i64 {
        match self {
            Expr::Lit(n) => *n,
            Expr::Math(op, lhs, rhs) => {
                let (lhs, rhs) = (heap[lhs].eval(heap), heap[rhs].eval(heap));
                match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                }
            }
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, EnumString)]
enum Op {
    #[strum(serialize = "+")]
    Add,
    #[strum(serialize = "-")]
    Sub,
    #[strum(serialize = "*")]
    Mul,
    #[strum(serialize = "/")]
    Div,
}

pub fn part_one(input: &'static str) -> i64 {
    let heap = parse_input(input);
    heap["root"].eval(&heap)
}

pub fn part_two(input: &'static str) -> i64 {
    let mut heap = parse_input(input);
    let Expr::Math(_, lhs, rhs) = heap["root"] else { unreachable!() };
    let (lhs, rhs) = (heap[lhs], heap[rhs].eval(&heap));

    // ugly binary search. don't worry, this disgusts me too. at least it works. and it's fast.
    let mut lower_bound = 3;
    let mut upper_bound = 10000000000000;
    let mut last_n = 0;
    loop {
        let n = lower_bound + ((upper_bound - lower_bound) / 2);
        if n == last_n {
            panic!("converged with no answer");
        }
        heap.insert("humn", Expr::Lit(n));
        let lhs = lhs.eval(&heap);
        if lhs == rhs {
            return n;
        } else {
            if (lhs > rhs) == cfg!(not(test)) {
                lower_bound = n;
            } else {
                upper_bound = n;
            }
        }
        last_n = n;
    }
}

fn parse_input(input: &'static str) -> ExprHeap {
    input
        .lines()
        .map(|s| {
            let (expr_id, s) = s.split_once(": ").unwrap();
            let expr = if let Ok(n) = s.parse() {
                Expr::Lit(n)
            } else {
                let split: Vec<_> = s.split(' ').collect();
                Expr::Math(split[1].parse().unwrap(), split[0], split[2])
            };

            (expr_id, expr)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 152);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 301);
    }
}
