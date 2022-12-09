use std::collections::HashSet;

use derive_more::{Add, AddAssign, Sub, SubAssign};
use strum_macros::EnumString;

#[derive(Clone, Copy, Default, Hash, Debug, Add, AddAssign, Sub, SubAssign, PartialEq, Eq)]
struct Pos(isize, isize);

impl Pos {
    pub fn must_move(self, other: Self) -> Option<Pos> {
        let dist = self - other;

        // diagonal jump
        if dist.0.abs() > 1 && dist.1.abs() > 1 {
            Some(Pos(dist.0 / 2, dist.1 / 2))
        }
        // orthagonal jump
        else if dist.1 > 1 {
            Some(Dir::Down.into())
        } else if dist.1 < -1 {
            Some(Dir::Up.into())
        } else if dist.0 > 1 {
            Some(Dir::Right.into())
        } else if dist.0 < -1 {
            Some(Dir::Left.into())
        }
        // no move necessary
        else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Rope<const N: usize> {
    head: Pos,
    tails: [Pos; N],
    tail_visited: HashSet<Pos>,
}

impl<const N: usize> Default for Rope<N> {
    fn default() -> Self {
        Self {
            head: Pos::default(),
            tails: [Pos::default(); N],
            tail_visited: HashSet::from([Pos::default()]),
        }
    }
}

impl<const N: usize> Rope<N> {
    pub fn update(&mut self, dir: Dir, steps: usize) {
        let dir = Pos::from(dir);
        for _ in 0..steps {
            self.head += dir;
            let mut leader = self.head;

            for tail in self.tails.iter_mut() {
                if let Some(dir) = tail.must_move(leader) {
                    *tail = leader + dir;
                }
                leader = *tail;
            }
            self.tail_visited.insert(leader);
        }
    }
}

#[derive(Debug, Clone, Copy, EnumString)]
enum Dir {
    #[strum(serialize = "U")]
    Up,
    #[strum(serialize = "D")]
    Down,
    #[strum(serialize = "L")]
    Left,
    #[strum(serialize = "R")]
    Right,
}

impl From<Dir> for Pos {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::Up => Pos(0, -1),
            Dir::Down => Pos(0, 1),
            Dir::Left => Pos(-1, 0),
            Dir::Right => Pos(1, 0),
        }
    }
}

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    let mut rope = Rope::<1>::default();
    for (dir, steps) in input {
        rope.update(dir, steps);
    }
    rope.tail_visited.len()
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    let mut rope = Rope::<9>::default();
    for (dir, steps) in input {
        rope.update(dir, steps);
    }
    rope.tail_visited.len()
}

fn parse_input(input: &'static str) -> Vec<(Dir, usize)> {
    input
        .lines()
        .map(|s| {
            let (dir, steps) = s.split_once(' ').unwrap();
            (dir.parse().unwrap(), steps.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");
    const LONG_SAMPLE: &str = include_str!("../sample_2.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 13);
    }

    #[test]
    fn test_part_two_short() {
        assert_eq!(part_two(SAMPLE), 1);
    }

    #[test]
    fn test_part_two_long() {
        assert_eq!(part_two(LONG_SAMPLE), 36);
    }
}
