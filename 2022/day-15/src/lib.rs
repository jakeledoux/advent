#![allow(unused)]

use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Pos,
    end: Pos,
}

impl Line {
    pub fn contains_point(&self, pos: &Pos) -> bool {
        if pos.y != self.start.y || !(self.start.x..=self.end.x).contains(&pos.x) {
            return false;
        }
        true
    }

    pub fn area(&self) -> isize {
        self.end.x - self.start.x
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.start.y != other.start.y {
            return None;
        }

        let intersection = Line {
            start: Pos::new(self.start.x.max(other.start.x), self.start.y),
            end: Pos::new(self.end.x.min(other.start.x), self.start.y),
        };

        (intersection.area() > 0).then_some(intersection)
    }
}

struct Diamond {
    center: Pos,
    size: isize,
}

impl Diamond {
    pub fn line_at(&self, row: isize) -> Option<Line> {
        let y_dist = self.size - self.center.y.abs_diff(row) as isize;
        if y_dist < 0 {
            return None;
        }

        Some(Line {
            start: Pos::new(self.center.x - y_dist, row),
            end: Pos::new(self.center.x + y_dist, row),
        })
    }
}

#[derive(Default)]
struct Survey {
    scanned: Vec<Diamond>,
}

impl Survey {
    pub fn mark(&mut self, diamond: Diamond) {
        self.scanned.push(diamond);
    }

    pub fn bounds(&self) -> (Pos, Pos) {
        let mut min = Pos::new(std::isize::MAX, std::isize::MAX);
        let mut max = Pos::new(std::isize::MIN, std::isize::MIN);

        for diamond in &self.scanned {
            min.x = min.x.min(diamond.center.x - diamond.size);
            min.y = min.y.min(diamond.center.y - diamond.size);
            max.x = max.x.max(diamond.center.x + diamond.size);
            max.y = max.y.max(diamond.center.y + diamond.size);
        }

        (min, max)
    }

    pub fn lines_on(&self, row: isize) -> impl Iterator<Item = Line> + '_ {
        self.scanned
            .iter()
            .filter_map(move |diamond| diamond.line_at(row))
    }

    pub fn scanned_in_row(&self, row: isize) -> isize {
        let lines = self.lines_on(row).collect_vec();
        let intersections = lines
            .iter()
            .tuple_combinations()
            .filter_map(|(a, b)| a.intersection(b))
            .collect_vec();
        dbg!(&intersections);
        dbg!(&lines);
        lines.into_iter().map(|line| line.area()).sum::<isize>()
            - intersections
                .into_iter()
                .map(|line| line.area())
                .sum::<isize>()
    }

    pub fn debug_draw(&self) {
        let bounds = self.bounds();
        for y in (bounds.0.y)..=(bounds.1.y) {
            print!("\n{y:3} ");
            for x in (bounds.0.x)..=(bounds.1.x) {
                print!(
                    "{}",
                    if self
                        .lines_on(y)
                        .any(|line| line.contains_point(&Pos::new(x, y)))
                    {
                        '#'
                    } else {
                        '.'
                    }
                )
            }
        }
        println!()
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

    pub fn manhattan(self, other: Self) -> isize {
        let dist = self - other;
        dist.x.abs() + dist.y.abs()
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

#[derive(Debug, Hash, Clone, Copy)]
struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl Sensor {
    pub fn distance_to_beacon(&self) -> isize {
        self.pos.manhattan(self.beacon)
    }

    pub fn scanned(self) -> impl Iterator<Item = Line> {
        let distance = self.distance_to_beacon();
        (-distance..=distance).map(move |y_dist| {
            let x_dist = distance - y_dist.abs();
            let y = self.pos.y + y_dist;

            let mut start = Pos::new(self.pos.x - x_dist, y);
            let mut end = Pos::new(self.pos.x + x_dist, y);
            if start == self.beacon {
                start.x += 1;
            }
            if end == self.beacon {
                end.x -= 1;
            }
            Line { start, end }
        })
    }
}

pub fn part_one(input: &'static str, row: isize) -> isize {
    let sensors = parse_input(input);
    let mut survey = Survey::default();
    for sensor in &sensors {
        survey.scanned.push(Diamond {
            center: sensor.pos,
            size: sensor.distance_to_beacon(),
        });
    }
    #[cfg(test)]
    survey.debug_draw();
    survey.scanned_in_row(row)
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    0
}

fn parse_input(input: &'static str) -> Vec<Sensor> {
    let pattern = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
    input
        .lines()
        .filter_map(|s| {
            pattern
                .captures_iter(s)
                .map(|c| Pos::new(c[1].parse().unwrap(), c[2].parse().unwrap()))
                .collect_tuple()
        })
        .map(|(sensor, beacon)| Sensor {
            pos: sensor,
            beacon,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE, 10), 26);
        assert!(part_one(INPUT, 2000000) > 236579);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 0);
    }
}
