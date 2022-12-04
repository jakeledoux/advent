use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct Assignment {
    start: usize,
    end: usize,
}

impl Assignment {
    pub fn contains(&self, other: &Self) -> bool {
        (self.start..=self.end).contains(&other.start)
            && (self.start..=self.end).contains(&other.end)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        other.start <= self.end && other.end >= self.start
    }
}

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    input
        .into_iter()
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count()
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    input.into_iter().filter(|(a, b)| a.overlaps(b)).count()
}

fn parse_input(input: &'static str) -> Vec<(Assignment, Assignment)> {
    input
        .lines()
        .map(|s| {
            s.split(',')
                .map(|s| {
                    let (start, end) = s
                        .split('-')
                        .map(|s| s.parse().expect("input will be valid integers"))
                        .collect_tuple()
                        .expect("input ranges will always be two numbers");
                    Assignment { start, end }
                })
                .collect_tuple()
                .expect("input will always be two ranges")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 4);
    }
}
