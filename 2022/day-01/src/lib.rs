use itertools::Itertools;

fn map_sum(i: impl IntoIterator<Item = Vec<u32>>) -> impl Iterator<Item = u32> {
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

fn parse_input(input: &'static str) -> Vec<Vec<u32>> {
    input.lines().fold(vec![vec![]], |mut acc, mut s| {
        s = s.trim();
        if s.is_empty() {
            acc.push(vec![])
        } else if let Ok(calories) = s.parse() {
            acc.last_mut()
                .expect(r#"AOC input is of type: `"" | number`"#)
                .push(calories);
        }

        acc
    })
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
