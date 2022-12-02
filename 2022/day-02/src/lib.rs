pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    input
        .iter()
        .map(|(other, me)| {
            me + 1
                + if me == other {
                    3
                } else if *me == (other + 1) % 3 {
                    6
                } else {
                    0
                }
        })
        .sum()
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    input
        .iter()
        .map(|(other, outcome)| ((other + [2, 0, 1][*outcome]) % 3) + 1 + (outcome * 3))
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
