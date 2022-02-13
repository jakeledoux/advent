use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Santa {
    x: isize,
    y: isize,
}

impl Santa {
    pub fn fly(&mut self, direction: char) {
        match direction {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            _ => {}
        }
    }
}

fn houses(steps: impl Iterator<Item = char>) -> HashSet<Santa> {
    steps
        .fold(
            (HashSet::from([Santa::default()]), Santa::default()),
            |(mut houses, mut santa), step| {
                santa.fly(step);
                houses.insert(santa);
                (houses, santa)
            },
        )
        .0
}

pub fn part_one(input: &'static str) -> usize {
    houses(input.chars()).len()
}

pub fn part_two(input: &'static str) -> usize {
    houses(input.chars().skip(1).step_by(2))
        .union(&houses(input.chars().step_by(2)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(">"), 2);
        assert_eq!(part_one("^>v<"), 4);
        assert_eq!(part_one("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("^v"), 3);
        assert_eq!(part_two("^>v<"), 3);
        assert_eq!(part_two("^v^v^v^v^v"), 11);
    }
}
