const CHECKSUM: [(usize, usize, isize); 7] = [
    (3, 4, -8),
    (5, 6, -4),
    (2, 7, 5),
    (8, 9, 0),
    (10, 11, 2),
    (1, 12, 1),
    (0, 13, -5),
];

pub fn part_one() -> usize {
    const N: [u8; 14] = [9, 8, 4, 9, 1, 9, 5, 9, 9, 9, 7, 9, 9, 4];
    assert!(checksum(&N));
    N.into_iter()
        .map(|d| d.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn part_two() -> usize {
    const N: [u8; 14] = [6, 1, 1, 9, 1, 5, 1, 6, 1, 1, 1, 3, 2, 1];
    assert!(checksum(&N));
    N.into_iter()
        .map(|d| d.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn checksum(model_number: &[u8; 14]) -> bool {
    CHECKSUM
        .iter()
        .all(|&(a, b, addend)| model_number[a] as isize + addend == model_number[b] as isize)
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        part_one();
    }

    #[test]
    fn test_part_two() {
        part_two();
    }
}
