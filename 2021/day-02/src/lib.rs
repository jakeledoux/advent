use cached::proc_macro::cached;

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

pub fn part_one(input: &'static str) -> isize {
    let input = parse_input(input);
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

pub fn part_two(input: &'static str) -> isize {
    let input = parse_input(input);
    let mut sub = Sub::default();
    sub.follow_directions(&input);
    sub.pos * sub.depth
}

#[cached]
fn parse_input(input: &'static str) -> Vec<&'static str> {
    input
        .lines()
        .filter_map(|s| match s.trim() {
            "" => None,
            _ => Some(s.trim()),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 150);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 900);
    }
}
