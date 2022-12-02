use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl From<Move> for Outcome {
    fn from(m: Move) -> Self {
        match m {
            Move::Rock => Outcome::Lose,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Win,
        }
    }
}

impl Outcome {
    fn points(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }

    fn rig(&self, other: &Move) -> Move {
        [Move::Rock, Move::Paper, Move::Scissors]
            .into_iter()
            .find(|m| m.compare(other) == *self)
            .unwrap()
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl Move {
    fn index(&self) -> usize {
        match self {
            Move::Rock => 0,
            Move::Paper => 1,
            Move::Scissors => 2,
        }
    }

    fn compare(&self, other: &Self) -> Outcome {
        if self == other {
            return Outcome::Draw;
        }
        let self_beats = match self {
            Move::Rock => 2,
            Move::Paper => 0,
            Move::Scissors => 1,
        };
        if self_beats == other.index() {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }
}

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    input
        .iter()
        .map(|(other, me)| {
            let outcome = me.compare(other);
            return me.index() + 1 + outcome.points();
        })
        .sum()
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    input
        .iter()
        .map(|(other, me)| {
            let outcome: Outcome = (*me).into();
            let me = outcome.rig(other);
            return me.index() + 1 + outcome.points();
        })
        .sum()
}

fn parse_input(input: &'static str) -> Vec<(Move, Move)> {
    input
        .lines()
        .filter_map(|s| match s.trim() {
            "" => None,
            _ => Some(s.trim()),
        })
        .map(|s| {
            let (a, b) = s.split_once(" ").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 15);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 12);
    }
}
