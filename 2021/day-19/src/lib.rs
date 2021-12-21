use std::collections::{HashMap, HashSet};

use euclid::{vec3, Angle, Rotation3D, UnknownUnit, Vector3D};
use itertools::Itertools;
use rayon::prelude::*;

type Point = Vector3D<i64, UnknownUnit>;
type Rotation = (usize, usize);

#[derive(Clone)]
struct Scanner {
    points: Vec<Point>,
}

fn rotate(point: &Point, rotation: Rotation) -> Point {
    let pitch = [-90, 90, 0, 0, 0, 0][rotation.0];
    let yaw = [0, 0, 90, 180, -90, 0][rotation.0];
    let roll = rotation.1 * 90;
    Rotation3D::euler(
        Angle::degrees(yaw as f64),
        Angle::degrees(pitch as f64),
        Angle::degrees(roll as f64),
    )
    .transform_vector3d(point.to_f64())
    .round()
    .to_i64()
}

fn get_rotations() -> Vec<Rotation> {
    let mut rotations = Vec::new();
    for face in 0..6 {
        for roll in 0..4 {
            rotations.push((face, roll));
        }
    }
    rotations
}

fn find_overlap(
    known: &[Point],
    unknown: &[Point],
    threshold: usize,
) -> Option<(Point, Vec<Point>)> {
    get_rotations().into_par_iter().find_map_any(|rotation| {
        let rotated_unknown = unknown
            .iter()
            .map(|point| rotate(point, rotation))
            .collect_vec();

        let offsets: HashMap<Point, usize> = known
            .iter()
            .flat_map(|a| rotated_unknown.iter().map(|b| *b - *a))
            .counts();

        if let Some(offset) = offsets
            .into_par_iter()
            .find_map_any(|(offset, count)| (count >= threshold).then(|| offset))
        {
            return Some((
                offset,
                rotated_unknown
                    .into_iter()
                    .map(|point| point - offset)
                    .collect(),
            ));
        } else {
            None
        }
    })
}

fn align_scanners(scanners: &[Scanner]) -> Option<(Vec<Point>, Vec<Point>)> {
    let mut scanners = scanners.to_vec();
    let scanner = scanners.remove(0);
    let mut beacons: Vec<Point> = scanner.points;
    let mut scanner_positions: Vec<Point> = vec![vec3(0, 0, 0)];

    while !scanners.is_empty() {
        let prev_length = scanners.len();
        let overlaps: Vec<_> = scanners
            .iter()
            .enumerate()
            .par_bridge()
            .filter_map(|(idx, unknown_scanner)| {
                find_overlap(&beacons, &unknown_scanner.points, 12).map(|result| (idx, result))
            })
            .collect();
        for (idx, (offset, corrected_points)) in
            overlaps.into_iter().sorted_by_key(|(i, _)| *i).rev()
        {
            beacons.extend(corrected_points);
            beacons = beacons
                .into_iter()
                .collect::<HashSet<Point>>()
                .into_iter()
                .collect_vec();
            scanner_positions.push(offset);
            scanners.remove(idx);
        }
        if prev_length == scanners.len() {
            return None;
        }
    }

    Some((scanner_positions, beacons))
}

pub fn part_one(input: &'static str) -> usize {
    let scanners = parse_input(input);
    let (_scanners, beacons) = align_scanners(&scanners).expect("puzzle input is solvable");
    beacons.len()
}

pub fn part_two(input: &'static str) -> usize {
    let scanners = parse_input(input);
    let (scanners, _beacons) = align_scanners(&scanners).expect("puzzle input is solvable");
    scanners
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs())
        .max()
        .unwrap() as usize
}

fn parse_input(input: &'static str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .map(|scanner| {
            Scanner {
                points: scanner
                    .lines()
                    .skip(1) // "--- scanner N ---" header
                    .map(|coords| {
                        let coords = coords.split(',').map(|n| n.parse().unwrap()).collect_vec();
                        vec3(coords[0], coords[1], *coords.get(2).unwrap_or(&0))
                    })
                    .collect(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 79);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 3621);
    }
}
