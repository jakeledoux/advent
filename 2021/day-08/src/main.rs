#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::io::stdout;

use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, execute};
use itertools::Itertools;
use lazy_static::lazy_static;

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");
const VISUALIZE: bool = option_env!("VISUALIZE").is_some();
static mut ITERATIONS: u128 = 0;

type SignalPattern = HashSet<char>;
type WiringPermutations = HashMap<char, HashSet<char>>;
type WireMap = HashMap<char, char>;

lazy_static! {
    static ref DIGITS: [SignalPattern; 10] = {
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
        .map(|v| SignalPattern::from_iter(v.iter().copied()))
        .collect_vec()
        .try_into()
        .expect("This cannot fail.")
    };
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
    taken: &WireMap,
    mut remaining: WiringPermutations,
    test_cases: &[SignalPattern],
) -> Option<WireMap> {
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
                    taken.insert(*k, v.iter().copied().next().unwrap());
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
    if remaining.iter().any(|(_k, v)| v.is_empty()) {
        return None;
    }

    // Check if remnants have been depleted
    if remaining.is_empty() {
        if VISUALIZE {
            std::thread::sleep(std::time::Duration::from_millis(1));
            let iterations = unsafe { ITERATIONS }.to_string();
            execute!(
                stdout(),
                SetForegroundColor(Color::DarkCyan),
                Print(format!(" {}  ", taken.values().join("")).to_string()),
                Print("          ".to_string()),
                SetForegroundColor(Color::DarkGrey),
                Print(format!(
                    "  {}",
                    std::iter::repeat('0').take(10 - iterations.len()).join("")
                )),
                ResetColor,
                Print(format!("{}", iterations).to_string()),
                cursor::MoveToColumn(11),
                SetForegroundColor(Color::Black),
            )
            .unwrap();
        }
        return if check_valid_mapping(test_cases, &taken) {
            if VISUALIZE {
                execute!(
                    stdout(),
                    SetBackgroundColor(Color::DarkGreen),
                    Print("  PASSED  ".to_string()),
                    ResetColor,
                    cursor::MoveToColumn(33),
                )
                .unwrap();
            }
            Some(taken)
        } else {
            if VISUALIZE {
                execute!(
                    stdout(),
                    SetBackgroundColor(Color::DarkRed),
                    Print("  FAILED  ".to_string()),
                    ResetColor,
                    cursor::MoveToColumn(0),
                )
                .unwrap();
            }
            None
        };
    }

    // Iterate through remaining possibilities
    for (&remnant, remnant_options) in remaining.iter() {
        for &remnant_option in remnant_options {
            let (mut taken, mut remaining) = (taken.clone(), remaining.clone());
            taken.insert(remnant, remnant_option);
            remaining.remove(&remnant).unwrap();

            // Recurse through implications
            unsafe {
                ITERATIONS += 1;
            }
            if let Some(result) = reduce_options(&taken, remaining, test_cases) {
                if check_valid_mapping(test_cases, &result) {
                    return Some(result);
                }
            }
        }
    }
    None
}

fn check_valid_mapping(inputs: &[SignalPattern], map: &WireMap) -> bool {
    inputs
        .iter()
        .all(|pattern| DIGITS.contains(&pattern.iter().map(|c| *map.get(c).unwrap()).collect()))
}

fn filter_obvious(patterns: &[SignalPattern]) -> WiringPermutations {
    let mut options: WiringPermutations = ('a'..='g')
        .map(|a| (a, ('a'..='g').collect::<SignalPattern>()))
        .collect();

    for pattern in patterns {
        let possibilities = match pattern.len() {
            2 => &DIGITS[1],
            3 => &DIGITS[7],
            4 => &DIGITS[4],
            _ => &DIGITS[8],
        };
        for &character in pattern {
            let option = options.get_mut(&character).unwrap();
            *option = option.intersection(possibilities).copied().collect();
        }
    }

    options
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
            let options = filter_obvious(patterns);
            let brute_forced_map = reduce_options(&WireMap::new(), options, patterns).unwrap();

            let num = output
                .iter()
                .map(|digi_pattern| {
                    let corrected_pattern: SignalPattern = digi_pattern
                        .iter()
                        .map(|c| *brute_forced_map.get(c).unwrap())
                        .collect();
                    DIGITS.iter().position(|d| d == &corrected_pattern).unwrap()
                })
                .join("")
                .parse::<usize>()
                .unwrap();

            if VISUALIZE {
                unsafe {
                    ITERATIONS = 0;
                }
                execute!(stdout(), Print(format!("   {:^6}\n", num).to_string()),).unwrap();
            }
            num
        })
        .sum()
}

fn main() {
    let input: Vec<_> = INPUT
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

    if VISUALIZE {
        execute!(
            stdout(),
            cursor::Hide,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            Print(" abcdefg |  status  | iterations | result \n".to_string()),
            Print("------------------------------------------\n".to_string()),
        )
        .unwrap();

        let result = part_two(&input);
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("------------------------------------------\n".to_string()),
            SetForegroundColor(Color::White),
            SetBackgroundColor(Color::DarkBlue),
            Print(" Result: "),
            ResetColor,
            Print("  "),
            Print(format!("{}", result)),
            ResetColor,
            cursor::Show
        )
        .unwrap();
    } else {
        dbg!(part_one(&input));
        dbg!(part_two(&input));
    }
}
