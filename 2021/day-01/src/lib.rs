use itertools::Itertools;

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    input.iter().tuple_windows().filter(|(a, b)| b > a).count()
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    input
        .windows(3)
        .map(|window| window.iter().sum::<usize>())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn parse_input(input: &'static str) -> Vec<usize> {
    input
        .lines()
        .filter_map(|s| s.trim().parse().ok())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 7);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 5);
    }
}
