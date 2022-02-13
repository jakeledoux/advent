pub fn part_one(input: &'static str) -> isize {
    input
        .trim()
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .sum()
}

pub fn part_two(input: &'static str) -> usize {
    let mut floor = 0;
    for (i, n) in input
        .trim()
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .enumerate()
    {
        floor += n;
        if floor == -1 {
            return i + 1;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("(())"), 0);
        assert_eq!(part_one("))((((("), 3);
        assert_eq!(part_one(")())())"), -3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(")"), 1);
        assert_eq!(part_two("()())"), 5);
    }
}
