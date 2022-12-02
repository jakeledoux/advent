fn compare(a: usize, b: usize) -> Outcome {
    if a == b {
        Outcome::Draw
    } else if a == (b + 1) % 3 {
        Outcome::Win
    } else {
        Outcome::Lose
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn from_index(index: usize) -> Self {
        [Self::Lose, Self::Draw, Self::Win][index]
    }

    fn points(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }

    fn rig(&self, other: usize) -> usize {
        match self {
            Outcome::Lose => (other + 2) % 3,
            Outcome::Draw => other,
            Outcome::Win => (other + 1) % 3,
        }
    }
}

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    input
        .iter()
        .map(|(other, me)| {
            let outcome = compare(*me, *other);
            me + 1 + outcome.points()
        })
        .sum()
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    input
        .iter()
        .map(|(other, outcome)| {
            let outcome = Outcome::from_index(*outcome);
            let me = outcome.rig(*other);
            me + 1 + outcome.points()
        })
        .sum()
}

fn parse_input(input: &'static str) -> Vec<(usize, usize)> {
    let parse = |s| match s {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => panic!("invalid input"),
    };
    input
        .lines()
        .map(|s| {
            let (a, b) = s.split_once(' ').unwrap();
            (parse(a), parse(b))
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
