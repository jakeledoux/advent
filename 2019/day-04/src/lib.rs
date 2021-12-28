use std::{borrow::Borrow, ops::RangeInclusive};

pub fn runs<T: Eq>(seq: &[T]) -> Vec<usize> {
    let mut runs = Vec::new();
    let mut current_run = 1;
    seq.windows(2).for_each(|win| {
        if win[0] == win[1] {
            current_run += 1;
        } else {
            runs.push(current_run);
            current_run = 1;
        }
    });
    runs.push(current_run);

    runs
}

pub fn is_valid(pass: &usize) -> bool {
    let digits: Vec<char> = pass.borrow().to_string().chars().collect();
    digits.len() == 6
        && runs(&digits).into_iter().any(|len| len >= 2)
        && digits.windows(2).all(|win| win[0] <= win[1])
}

pub fn is_valid_part_two(pass: &usize) -> bool {
    let digits: Vec<char> = pass.borrow().to_string().chars().collect();
    digits.len() == 6
        && runs(&digits).into_iter().any(|len| len == 2)
        && digits.windows(2).all(|win| win[0] <= win[1])
}

pub fn part_one(input: &'static str) -> usize {
    parse_input(input).filter(is_valid).count()
}

pub fn part_two(input: &'static str) -> usize {
    // Incorrect: 535
    parse_input(input).filter(is_valid_part_two).count()
}

fn parse_input(input: &'static str) -> RangeInclusive<usize> {
    if let [start, end] = input
        .trim()
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()[..]
    {
        start..=end
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::{is_valid, is_valid_part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(is_valid(&567899), true);
        assert_eq!(is_valid(&556789), true);
        assert_eq!(is_valid(&111111), true);
        assert_eq!(is_valid(&223450), false);
        assert_eq!(is_valid(&123789), false);
        assert_eq!(is_valid(&123789), false);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(is_valid_part_two(&556789), true);
        assert_eq!(is_valid_part_two(&111111), false);
        assert_eq!(is_valid_part_two(&223450), false);
        assert_eq!(is_valid_part_two(&123789), false);
        assert_eq!(is_valid_part_two(&112233), true);
        assert_eq!(is_valid_part_two(&123444), false);
        assert_eq!(is_valid_part_two(&111122), true);
    }
}
