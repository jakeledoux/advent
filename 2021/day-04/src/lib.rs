use std::str::FromStr;

use anyhow::anyhow;
use cached::proc_macro::cached;

const BOARD_WIDTH: usize = 5;

/// Represents whether a space in a bingo board has been crossed off.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BingoSpace {
    Crossed,
    Blank,
}

/// Represents a stateful bingo board
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct BingoBoard {
    numbers: [usize; 25],
    status: [BingoSpace; 25],
}

impl BingoBoard {
    /// Initialize from row-wise flat array
    pub fn new(numbers: &[usize; 25]) -> Self {
        Self {
            numbers: *numbers,
            status: [BingoSpace::Blank; 25],
        }
    }

    /// Cross off `n` if on board
    pub fn update(&mut self, n: usize) {
        if let Some(i) = self.numbers.iter().position(|&e| e == n) {
            self.status[i] = BingoSpace::Crossed;
        }
    }

    /// Returns `true` if the board has a bingo
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
                .all(|space| matches!(space, BingoSpace::Crossed))
            {
                return true;
            }
        }
        false
    }

    /// Calculate winning score
    fn score(&self, winning_number: usize) -> usize {
        self.numbers
            .iter()
            .zip(self.status.iter())
            .filter_map(|(&n, status)| match status {
                BingoSpace::Blank => Some(n),
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

pub fn part_one(input: &'static str) -> usize {
    let (numbers, boards) = parse_input(input);
    let mut numbers = numbers.iter().copied();
    let mut boards = boards.to_vec();
    numbers
        .find_map(move |n| {
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

pub fn part_two(input: &'static str) -> usize {
    let (numbers, boards) = parse_input(input);
    let mut numbers = numbers.iter().copied();
    let mut boards = boards.to_vec();
    loop {
        let n = numbers.next().unwrap();
        let mut board = boards[0];
        boards = boards
            .iter()
            .filter_map(|&(mut board)| {
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

#[cached]
fn parse_input(input: &'static str) -> (Vec<usize>, Vec<BingoBoard>) {
    let input: Vec<_> = input.lines().map(|s| s.trim()).collect();
    // Get number sequence
    let numbers: Vec<usize> = input[0].split(',').map(|s| s.parse().unwrap()).collect();
    // Treat the remaining lines as board definitions
    let boards = &input[1..];
    let boards: Vec<BingoBoard> = boards
        .split(|s| s.is_empty())
        .filter_map(|slice| {
            let board = slice.join(" ");
            match board.is_empty() {
                false => Some(board.parse().unwrap()),
                true => None,
            }
        })
        .collect();
    (numbers, boards)
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 4512);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 1924);
    }
}
