use std::collections::HashSet;

fn find_marker(stream: &[char], length: usize) -> Option<usize> {
    let mut marker = HashSet::<&char>::with_capacity(length);
    stream
        .windows(length)
        .position(|window| {
            marker.clear();
            marker.extend(window);
            marker.len() == length
        })
        .map(|i| i + length)
}

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    find_marker(&input, 4).unwrap()
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    find_marker(&input, 14).unwrap()
}

fn parse_input(input: &'static str) -> Vec<char> {
    input.trim().chars().collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 7);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 19);
    }
}
