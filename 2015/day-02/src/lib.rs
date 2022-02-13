use itertools::Itertools;

pub fn part_one(input: &'static str) -> usize {
    parse_input(input)
        .into_iter()
        .map(|b| {
            let sides: Vec<usize> = b
                .into_iter()
                .combinations(2)
                .map(|side| side.iter().product())
                .collect();
            let extra = sides.iter().min().unwrap();
            sides.iter().map(|side| side * 2).sum::<usize>() + extra
        })
        .sum()
}

pub fn part_two(input: &'static str) -> usize {
    parse_input(input)
        .into_iter()
        .map(|b| {
            b.iter()
                .combinations(2)
                .map(|side| 2 * side.into_iter().sum::<usize>())
                .min()
                .unwrap()
                + b.iter().product::<usize>()
        })
        .sum()
}

fn parse_input(input: &'static str) -> Vec<Vec<usize>> {
    input
        .lines()
        .filter_map(|s| match s.trim() {
            "" => None,
            _ => Some(s.trim()),
        })
        .map(|line| line.split('x').map(|s| s.parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("2x3x4"), 58);
        assert_eq!(part_one("1x1x10"), 43);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("2x3x4"), 34);
        assert_eq!(part_two("1x1x10"), 14);
    }
}
