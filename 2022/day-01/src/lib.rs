use itertools::Itertools;
use smallvec::SmallVec;

fn map_sum(
    i: impl IntoIterator<Item = impl IntoIterator<Item = u32>>,
) -> impl Iterator<Item = u32> {
    i.into_iter().map(|v| v.into_iter().sum())
}

pub fn part_one(input: &'static str) -> u32 {
    let input = parse_input(input);
    map_sum(input).max().unwrap()
}

pub fn part_two(input: &'static str) -> u32 {
    let input = parse_input(input);
    map_sum(input).sorted().rev().take(3).sum()
}

fn parse_input(input: &'static str) -> Vec<SmallVec<[u32; 32]>> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|s| s.parse().expect(r#"AOC input is of type: `"" | number`"#))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 24000);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 45000);
    }
}
