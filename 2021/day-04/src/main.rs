#![allow(dead_code)]

use std::str::FromStr;

use anyhow::anyhow;

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

const BOARD_WIDTH: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BingoSpace {
    Checked,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct BingoBoard {
    numbers: [usize; 25],
    status: [BingoSpace; 25],
}

impl BingoBoard {
    pub fn new(numbers: &[usize; 25]) -> Self {
        Self {
            numbers: *numbers,
            status: [BingoSpace::Empty; 25],
        }
    }

    pub fn update(&mut self, n: usize) {
        if let Some(i) = self.numbers.iter().position(|&e| e == n) {
            self.status[i] = BingoSpace::Checked;
        }
    }

    pub fn check_won(&self) -> bool {
        let rows: Vec<Vec<_>> = self
            .status
            .chunks(BOARD_WIDTH)
            .map(|row| row.into())
            .collect();
        let cols: Vec<Vec<_>> = (0..BOARD_WIDTH)
            .map(|i| {
                self.status
                    .iter()
                    .copied()
                    .skip(i)
                    .step_by(BOARD_WIDTH)
                    .collect()
            })
            .collect();
        for group in rows.iter().chain(cols.iter()) {
            if group
                .iter()
                .all(|space| matches!(space, BingoSpace::Checked))
            {
                return true;
            }
        }
        false
    }

    fn score(&self, winning_number: usize) -> usize {
        self.numbers
            .iter()
            .zip(self.status.iter())
            .filter_map(|(&n, status)| match status {
                BingoSpace::Empty => Some(n),
                _ => None,
            })
            .sum::<usize>()
            * winning_number
    }
}

impl FromStr for BingoBoard {
    type Err = anyhow::Error;

    fn from_str(board: &str) -> anyhow::Result<Self> {
        match board
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
        {
            Ok(numbers) => Ok(BingoBoard::new(&numbers)),
            Err(_) => Err(anyhow!("Malformed board")),
        }
    }
}

fn part_one(input: &[&str]) -> usize {
    let numbers: Vec<usize> = input[0].split(',').map(|s| s.parse().unwrap()).collect();
    let boards = &input[1..];
    let mut boards: Vec<BingoBoard> = boards
        .split(|s| s.is_empty())
        .filter_map(|slice| {
            let board = slice.join(" ");
            match board.is_empty() {
                false => Some(board.parse().unwrap()),
                true => None,
            }
        })
        .collect();

    numbers
        .iter()
        .find_map(move |&n| {
            for board in boards.iter_mut() {
                board.update(n);
                if board.check_won() {
                    return Some(board.score(n));
                }
            }
            None
        })
        .unwrap()
}

fn part_two(input: &[&str]) -> usize {
    let mut numbers = input[0].split(',').map(|s| s.parse().unwrap());
    let boards = &input[1..];
    let mut boards: Vec<BingoBoard> = boards
        .split(|s| s.is_empty())
        .filter_map(|slice| {
            let board = slice.join(" ");
            match board.is_empty() {
                false => Some(board.parse().unwrap()),
                true => None,
            }
        })
        .collect();

    loop {
        let n = numbers.next().unwrap();
        let mut board = boards[0];
        boards = boards
            .iter_mut()
            .filter_map(|&mut mut board| {
                board.update(n);
                if board.check_won() {
                    None
                } else {
                    Some(board)
                }
            })
            .collect();
        if boards.is_empty() {
            board.update(n);
            break board.score(n);
        }
    }
}

fn main() {
    let input: Vec<_> = INPUT.lines().map(|s| s.trim()).collect();

    dbg!(part_one(&input));
    dbg!(part_two(&input));
}
