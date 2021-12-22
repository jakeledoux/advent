// use euclid::{box3d, Box3D, Point3D, UnknownUnit};
use regex::Regex;

// type Cube = Box3D<isize, UnknownUnit>;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Clone, Copy)]
struct Cube {
    min: Point,
    max: Point,
}

impl Cube {
    pub fn new(
        min_x: isize,
        min_y: isize,
        min_z: isize,
        max_x: isize,
        max_y: isize,
        max_z: isize,
    ) -> Self {
        Self {
            min: Point {
                x: min_x,
                y: min_y,
                z: min_z,
            },
            max: Point {
                x: max_x,
                y: max_y,
                z: max_z,
            },
        }
    }

    pub fn volume(&self) -> isize {
        let x = self.max.x - self.min.x + 1;
        let y = self.max.y - self.min.y + 1;
        let z = self.max.z - self.min.z + 1;
        x * y * z
    }

    pub fn intersection(&self, other: &Cube) -> Option<Cube> {
        let min = Point {
            x: self.min.x.max(other.min.x),
            y: self.min.y.max(other.min.y),
            z: self.min.z.max(other.min.z),
        };
        let max = Point {
            x: self.max.x.min(other.max.x),
            y: self.max.y.min(other.max.y),
            z: self.max.z.min(other.max.z),
        };

        (min.x <= max.x && min.y <= max.y && min.z <= max.z).then(|| Cube { min, max })
    }
}

#[derive(Debug)]
pub enum InstructionKind {
    Positive,
    Negative,
}

impl InstructionKind {
    /// Returns `true` if the instruction kind is [`Positive`].
    ///
    /// [`Positive`]: InstructionKind::Positive
    pub fn is_positive(&self) -> bool {
        matches!(self, Self::Positive)
    }

    /// Returns `true` if the instruction kind is [`Negative`].
    ///
    /// [`Negative`]: InstructionKind::Negative
    pub fn is_negative(&self) -> bool {
        matches!(self, Self::Negative)
    }
}

#[derive(Debug)]
pub struct Instruction {
    kind: InstructionKind,
    cube: Cube,
}

pub fn execute_sequence(instructions: &[Instruction]) -> usize {
    let mut positive_cubes: Vec<Cube> = Vec::new();
    let mut negative_cubes: Vec<Cube> = Vec::new();

    for Instruction { kind, cube } in instructions {
        let new_negative: Vec<Cube> = positive_cubes
            .iter()
            .filter_map(|other| cube.intersection(other))
            .collect();
        let new_positive: Vec<Cube> = negative_cubes
            .iter()
            .filter_map(|other| cube.intersection(other))
            .collect();

        if kind.is_positive() {
            positive_cubes.push(*cube);
        }
        positive_cubes.extend(new_positive);
        negative_cubes.extend(new_negative);
    }

    let volume: isize = positive_cubes.into_iter().map(|cube| cube.volume()).sum();
    let neg_volume: isize = negative_cubes.into_iter().map(|cube| cube.volume()).sum();
    (volume - neg_volume) as usize
}

pub fn part_one(input: &'static str) -> usize {
    let instructions = parse_input(input);
    let bounds = Cube::new(-50, -50, -50, 50, 50, 50);
    let instructions: Vec<Instruction> = instructions
        .into_iter()
        .filter_map(|mut instruction| {
            if let Some(intersection) = instruction.cube.intersection(&bounds) {
                instruction.cube = intersection;
                Some(instruction)
            } else {
                None
            }
        })
        .collect();

    execute_sequence(&instructions)
}

pub fn part_two(input: &'static str) -> usize {
    let instructions = parse_input(input);
    execute_sequence(&instructions)
}

fn parse_input(input: &'static str) -> Vec<Instruction> {
    let bound_pattern: Regex =
        Regex::new(r#"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)"#)
            .unwrap();

    input
        .lines()
        .map(|s| {
            let groups = bound_pattern.captures(s).unwrap();
            let coords = groups
                .iter()
                .skip(2)
                .map(|s| s.unwrap().as_str().parse().unwrap());
            let min: Vec<isize> = coords.clone().step_by(2).collect();
            let max: Vec<isize> = coords.skip(1).step_by(2).collect();
            let cube = Cube::new(min[0], min[1], min[2], max[0], max[1], max[2]);

            Instruction {
                kind: match groups.get(1).unwrap().as_str() {
                    "on" => InstructionKind::Positive,
                    "off" => InstructionKind::Negative,
                    _ => unreachable!(),
                },
                cube,
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
        assert_eq!(part_one(SAMPLE), 590784);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE.trim()), 39769202357779);
    }
}
