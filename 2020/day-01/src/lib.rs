use itertools::Itertools;

pub fn expense_transform(items: Vec<usize>) -> Option<usize> {
    (items.iter().sum::<usize>() == 2020).then(|| items.iter().product())
}

pub fn part_one(input: &'static str) -> usize {
    parse_input(input)
        .into_iter()
        .combinations(2)
        .find_map(expense_transform)
        .unwrap()
}

pub fn part_two(input: &'static str) -> usize {
    parse_input(input)
        .into_iter()
        .combinations(3)
        .find_map(expense_transform)
        .unwrap()
}

fn parse_input(input: &'static str) -> Vec<usize> {
    input.lines().map(|n| n.trim().parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 514579);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 241861950);
    }
}
