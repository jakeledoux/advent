use std::collections::HashMap;

use cached::proc_macro::cached;
use vector2d::Vector2D;

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

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    let orthogonal_lines: Vec<Line> = input
        .iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .copied()
        .collect();
    Line::find_intersections(&orthogonal_lines).len()
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    Line::find_intersections(&input).len()
}

#[cached]
fn parse_input(input: &'static str) -> Vec<Line> {
    input
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
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 5);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 12);
    }
}
