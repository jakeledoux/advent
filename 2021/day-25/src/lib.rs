use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cucumber {
    Right,
    Down,
}

impl Cucumber {
    pub fn as_char(&self) -> char {
        match self {
            Self::Down => 'v',
            Self::Right => '>',
        }
    }
}

impl TryFrom<&char> for Cucumber {
    type Error = ();

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            'v' => Ok(Self::Down),
            '>' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    y: usize,
    x: usize,
}

impl From<(usize, usize)> for Pos {
    fn from(p: (usize, usize)) -> Self {
        Pos { x: p.0, y: p.1 }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SeaFloor {
    cukes: HashMap<Pos, Cucumber>,
    width: usize,
    height: usize,
}

impl SeaFloor {
    pub fn step(&mut self) -> usize {
        let mut moves = 0;

        // east-herd
        self.cukes = self
            .cukes
            .iter()
            .map(|(&pos, &cuke)| match cuke {
                Cucumber::Right => {
                    let new_pos = Pos {
                        x: (pos.x + 1) % self.width,
                        y: pos.y,
                    };
                    if self.get_cell(&new_pos).is_none() {
                        moves += 1;
                        (new_pos, cuke)
                    } else {
                        (pos, cuke)
                    }
                }
                _ => (pos, cuke),
            })
            .collect();

        // south-herd
        self.cukes = self
            .cukes
            .iter()
            .map(|(&pos, &cuke)| match cuke {
                Cucumber::Down => {
                    let new_pos = Pos {
                        x: pos.x,
                        y: (pos.y + 1) % self.height,
                    };
                    if self.get_cell(&new_pos).is_none() {
                        moves += 1;
                        (new_pos, cuke)
                    } else {
                        (pos, cuke)
                    }
                }
                _ => (pos, cuke),
            })
            .collect();

        moves
    }

    pub fn get_cell(&self, pos: &Pos) -> Option<&Cucumber> {
        self.cukes.get(pos)
    }
}

pub fn part_one(input: &'static str) -> usize {
    let mut sea_floor = parse_input(input);
    let mut steps = 0;
    loop {
        steps += 1;
        let moves = sea_floor.step();
        if moves == 0 {
            break steps;
        }
    }
}

fn parse_input(input: &'static str) -> SeaFloor {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let cukes: HashMap<Pos, Cucumber> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if let Ok(cucumber) = Cucumber::try_from(&c) {
                    Some((Pos { x, y }, cucumber))
                } else {
                    None
                }
            })
        })
        .collect();

    SeaFloor {
        cukes,
        width,
        height,
    }
}

#[cfg(test)]
mod tests {
    use crate::part_one;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 58);
    }
}
