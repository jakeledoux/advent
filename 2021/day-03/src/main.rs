#![allow(dead_code)]

use array2d::Array2D;
use itertools::Itertools;

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

fn part_one(input: &Array2D<char>) -> usize {
    let gamma = input
        .columns_iter()
        .map(|col| *col.counts().iter().max_by_key(|(_k, &v)| v).unwrap().0)
        .collect::<String>();
    let epsilon = input
        .columns_iter()
        .map(|col| *col.counts().iter().min_by_key(|(_k, &v)| v).unwrap().0)
        .collect::<String>();
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();
    gamma * epsilon
}

fn part_two(input: &Array2D<char>) -> usize {
    let mut oxygen_candidates = input
        .rows_iter()
        .map(|row_iter| row_iter.collect_vec())
        .collect_vec();
    for i in 0..input.column_len() {
        let most_common_bit = *oxygen_candidates
            .iter()
            .map(|row| row[i])
            .counts()
            .iter()
            .max_by(|a, b| {
                if a.1 == b.1 {
                    a.0.cmp(b.0)
                } else {
                    a.1.cmp(b.1)
                }
            })
            .unwrap()
            .0;
        oxygen_candidates = oxygen_candidates
            .into_iter()
            .filter(|row| row[i] == most_common_bit)
            .collect();
        if oxygen_candidates.len() == 1 {
            break;
        }
    }

    let mut co2_candidates = input
        .rows_iter()
        .map(|row_iter| row_iter.collect_vec())
        .collect_vec();

    for i in 0..input.column_len() {
        let most_common_bit = *co2_candidates
            .iter()
            .map(|row| row[i])
            .counts()
            .iter()
            .max_by(|a, b| {
                if a.1 == b.1 {
                    a.0.cmp(b.0).reverse()
                } else {
                    a.1.cmp(b.1).reverse()
                }
            })
            .unwrap()
            .0;
        co2_candidates = co2_candidates
            .into_iter()
            .filter(|row| row[i] == most_common_bit)
            .collect();
        if co2_candidates.len() == 1 {
            break;
        }
    }

    let oxygen_rating =
        usize::from_str_radix(&oxygen_candidates[0].iter().copied().collect::<String>(), 2)
            .unwrap();
    let co2_rating =
        usize::from_str_radix(&co2_candidates[0].iter().copied().collect::<String>(), 2).unwrap();
    oxygen_rating * co2_rating
}

fn main() {
    let input: Vec<Vec<_>> = INPUT
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
