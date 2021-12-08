#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

type SignalPattern = HashSet<char>;

fn digits() -> Vec<HashSet<char>> {
    [
        vec!['a', 'b', 'c', 'e', 'f', 'g'],
        vec!['c', 'f'],
        vec!['a', 'c', 'd', 'e', 'g'],
        vec!['a', 'c', 'd', 'f', 'g'],
        vec!['b', 'c', 'd', 'f'],
        vec!['a', 'b', 'd', 'f', 'g'],
        vec!['a', 'b', 'd', 'e', 'f', 'g'],
        vec!['a', 'c', 'f'],
        vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        vec!['a', 'b', 'c', 'd', 'f', 'g'],
    ]
    .iter()
    .map(|v| HashSet::from_iter(v.iter().copied()))
    .collect()
}

fn identify_digit(signals: &SignalPattern) -> Option<u8> {
    match signals.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

fn reduce_options(
    taken: &HashMap<char, char>,
    mut remaining: HashMap<char, HashSet<char>>,
    test_cases: &[SignalPattern],
) -> Option<HashMap<char, char>> {
    let mut taken = taken.to_owned();

    let mut changes_made = 1;
    while changes_made > 0 {
        changes_made = 0;
        // Remove taken from remaining
        remaining.iter_mut().for_each(|(_k, v)| {
            *v = v
                .difference(&taken.values().copied().collect())
                .copied()
                .collect()
        });

        // Add single-option remnants to taken
        remaining = remaining
            .into_iter()
            .filter(|(k, v)| {
                if v.len() == 1 {
                    changes_made += 1;
                    taken.insert(*k, v.into_iter().copied().next().unwrap());
                    false
                } else {
                    true
                }
            })
            .collect();

        // Ensure no duplicates taken
        if taken.values().counts().values().any(|&v| v > 1) {
            return None;
        }
    }

    // Walk back up if options depleted for any remnant
    if remaining.iter().any(|(_k, v)| v.len() == 0) {
        return None;
    }

    // Check if remnants have been depleted
    if remaining.len() == 0 {
        return Some(taken);
    }

    // Iterate through remaining possibilities
    for (&remnant, remnant_options) in remaining.iter() {
        for &remnant_option in remnant_options {
            let (mut taken, mut remaining) = (taken.clone(), remaining.clone());
            taken.insert(remnant, remnant_option);
            remaining.remove(&remnant).unwrap();

            // Recurse through implications
            if let Some(result) = reduce_options(&taken, remaining, test_cases) {
                if check_valid_mapping(test_cases, &result) {
                    return Some(result);
                }
            }
        }
    }
    None
}

fn part_one(input: &[(Vec<SignalPattern>, Vec<SignalPattern>)]) -> usize {
    input
        .iter()
        .flat_map(|(_patterns, output)| {
            output.iter().filter_map(identify_digit).collect::<Vec<_>>()
        })
        .count()
}

fn part_two(input: &[(Vec<SignalPattern>, Vec<SignalPattern>)]) -> usize {
    input
        .iter()
        .map(|(patterns, output)| {
            let options = filter_obvious(&patterns);
            let brute_forced_map = reduce_options(&HashMap::new(), options, patterns).unwrap();

            let digits = crate::digits();
            let answer: usize = output
                .iter()
                .map(|digi_pattern| {
                    let corrected_pattern: HashSet<char> = digi_pattern
                        .iter()
                        .map(|c| *brute_forced_map.get(c).unwrap())
                        .collect();
                    digits.iter().position(|d| d == &corrected_pattern).unwrap()
                })
                .join("")
                .parse()
                .unwrap();
            dbg!(answer)
        })
        .sum()
}

fn check_valid_mapping(inputs: &[SignalPattern], map: &HashMap<char, char>) -> bool {
    let digits = crate::digits();
    inputs
        .iter()
        .all(|pattern| digits.contains(&pattern.iter().map(|c| *map.get(c).unwrap()).collect()))
}

fn filter_obvious(patterns: &[SignalPattern]) -> HashMap<char, HashSet<char>> {
    let mut options: HashMap<char, HashSet<char>> = ('a'..='g')
        .map(|a| (a, ('a'..='g').collect::<HashSet<_>>()))
        .collect();

    let digits = crate::digits();

    for pattern in patterns {
        let possibilities = match pattern.len() {
            2 => &digits[1],
            3 => &digits[7],
            4 => &digits[4],
            _ => &digits[8],
        };
        for &character in pattern {
            let option = options.get_mut(&character).unwrap();
            *option = option.intersection(possibilities).copied().collect();
        }
    }

    options
}

fn main() {
    let input: Vec<_> = SAMPLE
        .lines()
        .filter_map(|s| {
            if let Some((patterns, output)) = s.trim().split_once('|') {
                let parse_pattern = |p: &str| {
                    p.split_whitespace()
                        .map(|p| p.chars().collect::<SignalPattern>())
                        .collect()
                };
                Some((parse_pattern(patterns), parse_pattern(output)))
            } else {
                None
            }
        })
        .collect();

    // dbg!(part_one(&input));
    dbg!(part_two(&input));
}
