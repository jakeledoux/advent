#![allow(unused)]
use std::{
    collections::HashMap,
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

use itertools::Itertools;

const SPAWN: Pos = Pos::new(500, 0);
const SOUTH: Pos = Pos::new(0, 1);
const SOUTHWEST: Pos = Pos::new(-1, 1);
const SOUTHEAST: Pos = Pos::new(1, 1);

enum Part {
    One,
    Two,
}

#[derive(Default)]
struct Cave {
    // the hashmap isn't very performant but I don't have the time to replace it right now
    blocks: HashMap<Pos, Block>,
}

impl Cave {
    pub fn add_block(&mut self, pos: Pos, block: Block) -> Option<Block> {
        self.blocks.insert(pos, block)
    }

    pub fn get(&self, pos: &Pos) -> Block {
        *self.blocks.get(pos).unwrap_or(&Block::Air)
    }

    pub fn bounds(&self) -> (Pos, Pos) {
        let mut min = Pos::new(std::isize::MAX, std::isize::MAX);
        let mut max = Pos::new(std::isize::MIN, std::isize::MIN);

        for pos in self.blocks.keys() {
            min.x = min.x.min(pos.x);
            min.y = min.y.min(pos.y);
            max.x = max.x.max(pos.x);
            max.y = max.y.max(pos.y);
        }

        (min, max)
    }

    pub fn debug_draw(&self) {
        let bounds = self.bounds();
        for y in (bounds.0.y)..=(bounds.1.y) {
            println!();
            for x in (bounds.0.x)..=(bounds.1.x) {
                print!(
                    "{}",
                    match self.get(&Pos::new(x, y)) {
                        Block::Air => '.',
                        Block::Sand => 'o',
                        Block::Rock => '#',
                    }
                )
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Air,
    Sand,
    Rock,
}

impl Block {
    #[must_use]
    fn is_air(&self) -> bool {
        matches!(self, Self::Air)
    }

    #[must_use]
    fn is_sand(&self) -> bool {
        matches!(self, Self::Sand)
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn line_to(self, other: Self) -> impl Iterator<Item = Self> {
        let step = (other - self).clamp(1);
        let mut pos = self;
        std::iter::once(pos).chain(std::iter::from_fn(move || {
            if pos == other {
                return None;
            }
            pos += step;
            Some(pos)
        }))
    }

    pub fn clamp(self, max: isize) -> Self {
        Self {
            x: self.x.min(max).max(-max),
            y: self.y.min(max).max(-max),
        }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(isize, isize)> for Pos {
    fn from(tuple: (isize, isize)) -> Self {
        Pos {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .split(',')
            .into_iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<isize>, _>>()
        {
            Ok(v) if v.len() == 2 => Ok(Self { x: v[0], y: v[1] }),
            _ => Err(()),
        }
    }
}

fn simulate(cave: &mut Cave, part: Part) -> usize {
    let floor = cave.bounds().1.y + 2;
    let mut path = vec![SPAWN];
    for sand_blocks in 0.. {
        'step: loop {
            let pos = *path.last().unwrap();
            if pos.y + 1 < floor {
                for new_pos in [pos + SOUTH, pos + SOUTHWEST, pos + SOUTHEAST] {
                    if cave.get(&new_pos).is_air() {
                        path.push(new_pos);
                        continue 'step;
                    }
                }
            } else if matches!(part, Part::One) {
                // falling infinitely
                return sand_blocks;
            }
            // can't move
            break;
        }
        let sand = path.pop().unwrap();
        if matches!(part, Part::Two) && sand == SPAWN {
            // spawn has filled
            return sand_blocks + 1;
        }
        cave.add_block(sand, Block::Sand);
    }
    unreachable!()
}

pub fn part_one(input: &'static str) -> usize {
    let mut cave = parse_input(input);
    simulate(&mut cave, Part::One)
}

pub fn part_two(input: &'static str) -> usize {
    let mut cave = parse_input(input);
    simulate(&mut cave, Part::Two)
}

fn parse_input(input: &'static str) -> Cave {
    let mut cave = Cave::default();
    for line in input
        .lines()
        .map(|s| s.split(" -> ").map(|s| s.parse::<Pos>().unwrap()))
    {
        for (a, b) in line.tuple_windows() {
            for pos in a.line_to(b) {
                cave.add_block(pos, Block::Rock);
            }
        }
    }

    cave
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::{part_one, part_two, Pos};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 24);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 93);
    }

    #[test]
    fn test_line_to() {
        let a = Pos::new(3, 8);
        let b = Pos::new(6, 8);
        let mut out = vec![a, Pos::new(4, 8), Pos::new(5, 8), b];
        assert_eq!(a.line_to(b).collect_vec(), out);
        out.reverse();
        assert_eq!(b.line_to(a).collect_vec(), out);
    }
}
