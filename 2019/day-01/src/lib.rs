pub fn part_one(input: &'static str) -> isize {
    let input = parse_input(input);
    input.iter().map(|n| n / 3 - 2).sum()
}

pub fn part_two(input: &'static str) -> isize {
    let input = parse_input(input);
    input
        .iter()
        .map(|&(mut n)| {
            let mut total = 0;
            loop {
                n = n / 3 - 2;
                if n > 0 {
                    total += n;
                } else {
                    break;
                }
            }
            total
        })
        .sum()
}

fn parse_input(input: &'static str) -> Vec<isize> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 34_241);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 51_316);
    }
}
