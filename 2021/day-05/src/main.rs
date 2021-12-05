#![allow(dead_code)]

use std::collections::HashMap;

use vector2d::Vector2D;

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    start: Vector2D<isize>,
    end: Vector2D<isize>,
}

impl Line {
    fn new(start: Vector2D<isize>, end: Vector2D<isize>) -> Self {
        Self { start, end }
    }

    // Finds all intersections and returns their coordinates
    fn find_intersections(lines: &[Self]) -> Vec<(isize, isize)> {
        lines
            .iter()
            .map(Line::rasterize)
            .fold(
                HashMap::new(),
                |mut counter: HashMap<(isize, isize), usize>, pixels| {
                    for pixel in pixels {
                        counter.insert(pixel, *counter.get(&pixel).unwrap_or(&0) + 1);
                    }
                    counter
                },
            )
            .into_iter()
            .filter_map(|(coords, v)| (v > 1).then(|| coords))
            .collect()
    }

    /// Calculates all coordinates this line occupies
    fn rasterize(&self) -> Vec<(isize, isize)> {
        let slope = self.end - self.start;
        let steps = slope.x.abs().max(slope.y.abs());
        (0..=steps)
            .map(|step| {
                (
                    self.start.x + (slope.x / steps) * step,
                    self.start.y + (slope.y / steps) * step,
                )
            })
            .collect()
    }
}

fn part_one(input: &[Line]) -> usize {
    let orthogonal_lines: Vec<Line> = input
        .iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .copied()
        .collect();
    Line::find_intersections(&orthogonal_lines).len()
}

fn part_two(input: &[Line]) -> usize {
    Line::find_intersections(input).len()
}

fn main() {
    // Parse input into `Line`s
    let input: Vec<Line> = INPUT
        .lines()
        .filter_map(|s| {
            if let Some((start, end)) = s.split_once(" -> ") {
                let [start, end] = [start, end].map(|pos| {
                    let (x, y) = pos.split_once(',').unwrap();
                    Vector2D::new(x.parse().unwrap(), y.parse().unwrap())
                });
                return Some(Line::new(start, end));
            }
            None
        })
        .collect();

    dbg!(part_one(&input));
    dbg!(part_two(&input));
}
