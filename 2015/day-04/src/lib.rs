fn mine_adventcoin(prefix: &str, zeroes: usize) -> usize {
    let zeroes: String = "0".repeat(zeroes);
    (0..)
        .map(|n| {
            (
                n,
                format!("{:x}", md5::compute(format!("{}{}", prefix.trim(), n))),
            )
        })
        .find_map(|(n, hash)| hash.starts_with(&zeroes).then(|| n))
        .unwrap()
}

pub fn part_one(input: &'static str) -> usize {
    mine_adventcoin(input, 5)
}

pub fn part_two(input: &'static str) -> usize {
    mine_adventcoin(input, 6)
}

#[cfg(test)]
mod tests {
    use super::part_one;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("abcdef"), 609043);
        assert_eq!(part_one("pqrstuv"), 1048970);
    }
}
