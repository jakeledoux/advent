#![allow(dead_code)]

use std::num::ParseIntError;

use array2d::Array2D;
use itertools::Itertools;

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

enum Commonality {
    Most,
    Least,
}

impl Commonality {
    /// Compare two values according to desired commonality
    ///
    /// For example, if `self` is `Commonality::Least` then the lesser value will be selected.
    pub fn cmp<T: Ord>(&self, a: &T, b: &T) -> std::cmp::Ordering {
        match self {
            Commonality::Most => a.cmp(b),
            Commonality::Least => b.cmp(a),
        }
    }
}

/// Find the most common value in an iterator
fn most_common_value<I: Iterator<Item = T>, T: Ord + Copy + std::hash::Hash>(
    iter: I,
    commonality: Commonality,
) -> T {
    *iter
        .counts()
        .iter()
        .max_by(|(_, va), (_, vb)| commonality.cmp(va, vb))
        .unwrap()
        .0
}

/// Iteratively filters collection by column commonality until one value remains.
fn reduce_find<T: Copy + Ord + std::hash::Hash>(
    collection: &[Vec<T>],
    commonality: Commonality,
) -> Option<Vec<T>> {
    if let Some(first) = collection.first() {
        let mut collection = collection.to_vec();

        for i in 0..first.len() {
            // Find the most common value at index `i`.
            let most_common_value = *collection
                .iter()
                .map(|row| row[i])
                .counts()
                .iter()
                .max_by(|a, b| {
                    if a.1 == b.1 {
                        commonality.cmp(a.0, b.0)
                    } else {
                        commonality.cmp(a.1, b.1)
                    }
                })
                .unwrap()
                .0;

            // Filter to rows that contain `most_common_value` in the correct index.
            collection = collection
                .into_iter()
                .filter(|row| row[i] == most_common_value)
                .collect_vec();

            // Reduced to one item
            if collection.len() == 1 {
                return Some(collection[0].clone());
            }
        }
    }
    // Either `collection` is empty or there are multiple rows that meet the criteria.
    None
}

/// Convert an array of `char` to `String` then parse into usize assuming base 2.
fn char_array_to_bin(chars: &[char]) -> Result<usize, ParseIntError> {
    usize::from_str_radix(&chars.iter().collect::<String>(), 2)
}

fn part_one(input: &Array2D<char>) -> usize {
    let gamma = input
        .columns_iter()
        .map(|v| most_common_value(v, Commonality::Most))
        .collect::<String>();
    let epsilon = input
        .columns_iter()
        .map(|v| most_common_value(v, Commonality::Least))
        .collect::<String>();
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();
    gamma * epsilon
}

fn part_two(input: &Array2D<char>) -> usize {
    let input = input
        .rows_iter()
        .map(|row_iter| row_iter.copied().collect_vec())
        .collect_vec();

    let oxygen_rating =
        char_array_to_bin(&reduce_find(&input, Commonality::Most).unwrap()).unwrap();
    let co2_rating = char_array_to_bin(&reduce_find(&input, Commonality::Least).unwrap()).unwrap();

    oxygen_rating * co2_rating
}

fn main() {
    let input: Vec<Vec<_>> = SAMPLE
        .lines()
        .filter_map(|s| match s.trim() {
            "" => None,
            _ => Some(s.trim().chars().collect()),
        })
        .collect();

    let input = Array2D::from_rows(&input);

    dbg!(part_one(&input));
    dbg!(part_two(&input));
}
