pub fn eric_mod(mut number: isize, dividend: usize) -> usize {
    number = number.rem_euclid(dividend as isize);
    if number == 0 {
        dividend
    } else {
        number as usize
    }
}

pub fn decrypt(encrypted: &[isize], iterations: usize) -> isize {
    let mut decrypted: Vec<usize> = (0..encrypted.len()).collect();
    for _ in 0..iterations {
        for (original_i, number) in encrypted.iter().enumerate() {
            let i = decrypted.iter().position(|n| n == &original_i).unwrap();
            decrypted.remove(i);
            decrypted.insert(eric_mod(i as isize + number, decrypted.len()), original_i);
        }
    }
    decrypted
        .iter()
        .cycle()
        .skip(
            decrypted
                .iter()
                .map(|i| encrypted[*i])
                .position(|n| n == 0)
                .unwrap(),
        )
        .step_by(1000)
        .skip(1)
        .take(3)
        .map(|i| encrypted[*i])
        .sum()
}

pub fn part_one(input: &'static str) -> isize {
    let encrypted = parse_input(input);
    decrypt(&encrypted, 1)
}

pub fn part_two(input: &'static str) -> isize {
    let mut encrypted = parse_input(input);
    encrypted.iter_mut().for_each(|n| *n *= 811589153);
    decrypt(&encrypted, 10)
}

fn parse_input(input: &'static str) -> Vec<isize> {
    input
        .lines()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 1623178306);
    }
}
