#![allow(dead_code)]

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug)]
enum Group {
    Parens,
    Square,
    Curly,
    Angle,
}

impl TryFrom<char> for Group {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' | ')' => Ok(Group::Parens),
            '[' | ']' => Ok(Group::Square),
            '{' | '}' => Ok(Group::Curly),
            '<' | '>' => Ok(Group::Angle),
            _ => Err("Invalid group character"),
        }
    }
}

impl PartialEq<char> for Group {
    fn eq(&self, other: &char) -> bool {
        match self {
            Group::Parens => "()".contains(*other),
            Group::Square => "[]".contains(*other),
            Group::Curly => "{}".contains(*other),
            Group::Angle => "<>".contains(*other),
        }
    }
}

fn find_syntax_errors(line: &str) -> Option<usize> {
    let mut stack: Vec<Group> = Vec::new();
    for character in line.chars() {
        if "{[(<".contains(character) {
            stack.push(character.try_into().unwrap());
        } else if stack.last().unwrap().eq(&character) {
            stack.pop();
        } else {
            return Some(error_points(character).unwrap());
        }
    }
    None
}

fn autocomplete(line: &str) -> Option<usize> {
    let mut stack: Vec<Group> = Vec::new();
    for character in line.chars() {
        if "{[(<".contains(character) {
            stack.push(character.try_into().unwrap());
        } else if stack.last().unwrap().eq(&character) {
            stack.pop();
        } else {
            return None;
        }
    }
    Some(
        stack
            .iter()
            .rev()
            .copied()
            .map(autocomplete_points)
            .fold(0, |total, points| total * 5 + points),
    )
}

fn autocomplete_points(group: Group) -> usize {
    match group {
        Group::Parens => 1,
        Group::Square => 2,
        Group::Curly => 3,
        Group::Angle => 4,
    }
}

fn error_points(value: char) -> Option<usize> {
    match value {
        '(' | ')' => Some(3),
        '[' | ']' => Some(57),
        '{' | '}' => Some(1197),
        '<' | '>' => Some(25137),
        _ => None,
    }
}

fn part_one(input: &[&str]) -> usize {
    input.iter().copied().filter_map(find_syntax_errors).sum()
}

fn part_two(input: &[&str]) -> usize {
    let mut points: Vec<usize> = input.iter().copied().filter_map(autocomplete).collect();
    points.sort_unstable();
    points[(points.len() + 1) / 2 - 1]
}

fn main() {
    let input: Vec<_> = SAMPLE
        .lines()
        .filter_map(|s| match s.trim() {
            "" => None,
            _ => Some(s.trim()),
        })
        .collect();

    dbg!(part_one(&input));
    dbg!(part_two(&input));
}
