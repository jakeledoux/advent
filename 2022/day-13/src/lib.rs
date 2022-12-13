/// this solution is allocation city. as the days go on I have less and less interest in cleaning
/// these things up after solving the puzzle. get ready for some _real_ ugly code next week.
use std::cmp::Ordering;

use itertools::Itertools;
use serde_json::{json, Value};

fn compare(left: &[Value], right: &[Value]) -> Ordering {
    for (left, right) in left.iter().zip(right) {
        // both ints
        if let (Some(left), Some(right)) = (left.as_i64(), right.as_i64()) {
            if left == right {
                continue;
            } else {
                return left.cmp(&right);
            }
        }

        // coerce to lists
        let left = left
            .as_array()
            .cloned()
            .unwrap_or_else(|| vec![left.to_owned()]);
        let right = right
            .as_array()
            .cloned()
            .unwrap_or_else(|| vec![right.to_owned()]);

        // both lists
        let ord = compare(&left, &right);
        if !ord.is_eq() {
            return ord;
        }
    }

    // no verdict made, tiebreaker checks if the lengths are different
    left.len().cmp(&right.len())
}

pub fn part_one(input: &'static str) -> usize {
    parse_input(input)
        .iter()
        .tuples()
        .map(|(left, right)| compare(left, right))
        .enumerate()
        .filter_map(|(i, ord)| ord.is_lt().then_some(i + 1))
        .sum()
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);

    let divider_packets = [vec![json! {[2]}], vec![json! {[6]}]];
    input
        .into_iter()
        .chain(divider_packets.iter().cloned())
        .sorted_by(|left, right| compare(left, right))
        .enumerate()
        .filter_map(|(i, packet)| divider_packets.contains(&packet).then_some(i + 1))
        .product()
}

fn parse_input(input: &'static str) -> Vec<Vec<Value>> {
    input
        .lines()
        .filter_map(|s| serde_json::from_str(s).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 140);
    }
}
