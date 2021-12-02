#![allow(dead_code)]

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up(isize),
    Down(isize),
    Forward(isize),
}

impl Direction {
    pub fn parse(direction: &str) -> Option<Self> {
        if let Some((command, units)) = direction.split_once(' ') {
            if let Ok(units) = units.parse() {
                return match command.to_lowercase().trim() {
                    "up" => Some(Direction::Up(units)),
                    "down" => Some(Direction::Down(units)),
                    "forward" => Some(Direction::Forward(units)),
                    _ => None,
                };
            }
        }
        None
    }
}

#[derive(Default, Debug)]
struct Sub {
    pub pos: isize,
    pub depth: isize,
    pub aim: isize,
}

impl Sub {
    pub fn follow_directions(&mut self, directions: &[&str]) {
        directions
            .iter()
            .map(|line| Direction::parse(line).unwrap())
            .for_each(|dir| match dir {
                Direction::Up(units) => self.aim -= units,
                Direction::Down(units) => self.aim += units,
                Direction::Forward(units) => {
                    self.pos += units;
                    self.depth += self.aim * units;
                }
            });
    }
}

fn part_one(input: &[&str]) -> isize {
    let mut sub = Sub::default();
    input
        .iter()
        .map(|line| Direction::parse(line).unwrap())
        .for_each(|dir| match dir {
            Direction::Up(units) => sub.depth -= units,
            Direction::Down(units) => sub.depth += units,
            Direction::Forward(units) => sub.pos += units,
        });
    sub.pos * sub.depth
}

fn part_two(input: &[&str]) -> isize {
    let mut sub = Sub::default();
    sub.follow_directions(input);
    sub.pos * sub.depth
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
