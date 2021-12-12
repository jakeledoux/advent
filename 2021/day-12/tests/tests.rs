use day_12::{part_one, part_two};

const SAMPLE: &str = include_str!("../sample.txt");

#[test]
fn test_part_one_sample() {
    assert_eq!(part_one(SAMPLE), 226);
}

#[test]
fn test_part_two_sample() {
    assert_eq!(part_two(SAMPLE), 3509);
}
