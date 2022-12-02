use strum_macros::EnumString;

#[derive(Clone, Copy, PartialEq, Eq, EnumString)]
enum Move {
    #[strum(serialize = "A", serialize = "X")]
    Rock,
    #[strum(serialize = "B", serialize = "Y")]
    Paper,
    #[strum(serialize = "C", serialize = "Z")]
    Scissors,
}

impl Move {
    fn from_index(index: usize) -> Self {
        [Move::Rock, Move::Paper, Move::Scissors][index % 3]
    }

    fn index(&self) -> usize {
        match self {
            Move::Rock => 0,
            Move::Paper => 1,
            Move::Scissors => 2,
        }
    }

    fn compare(&self, other: &Self) -> Outcome {
        if self == other {
            Outcome::Draw
        } else if self.index() == (other.index() + 1) % 3 {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }
}

#[derive(PartialEq, Eq, EnumString)]
enum Outcome {
    #[strum(serialize = "X")]
    Lose,
    #[strum(serialize = "Y")]
    Draw,
    #[strum(serialize = "Z")]
    Win,
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
        Move::from_index(match self {
            Outcome::Lose => other.index() + 2,
            Outcome::Draw => return *other,
            Outcome::Win => other.index() + 1,
        })
    }
}

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    input
        .iter()
        .map(|(other, (me, _outcome))| {
            let outcome = me.compare(other);
            me.index() + 1 + outcome.points()
        })
        .sum()
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    input
        .iter()
        .map(|(other, (_me, outcome))| {
            let me = outcome.rig(other);
            me.index() + 1 + outcome.points()
        })
        .sum()
}

fn parse_input(input: &'static str) -> Vec<(Move, (Move, Outcome))> {
    input
        .lines()
        .map(|s| {
            let (a, b) = s.split_once(' ').unwrap();
            (a.parse().unwrap(), (b.parse().unwrap(), b.parse().unwrap()))
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
