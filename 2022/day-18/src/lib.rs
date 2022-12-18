use std::{collections::HashSet, ops};

use itertools::Itertools;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3 {
    const MAX: Self = Self {
        x: std::isize::MAX,
        y: std::isize::MAX,
        z: std::isize::MAX,
    };
    const MIN: Self = Self {
        x: std::isize::MIN,
        y: std::isize::MIN,
        z: std::isize::MIN,
    };

    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn sides(&self) -> impl Iterator<Item = Self> + '_ {
        (0..3).flat_map(move |dimension| {
            [-1, 1].into_iter().map(move |direction| {
                let mut side = *self;
                match dimension {
                    0 => side.x += direction,
                    1 => side.y += direction,
                    2 => side.z += direction,
                    _ => unreachable!(),
                }
                side
            })
        })
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

fn find_bounds<'a>(cubes: impl IntoIterator<Item = &'a Vec3>) -> (Vec3, Vec3) {
    let mut min = Vec3::MAX;
    let mut max = Vec3::MIN;

    for &cube in cubes {
        min = min.min(cube);
        max = max.max(cube);
    }

    (min - Vec3::new(1, 1, 1), max + Vec3::new(1, 1, 1))
}

fn in_bounds(cube: Vec3, bounds: (Vec3, Vec3)) -> bool {
    (bounds.0.x..=bounds.1.x).contains(&cube.x)
        && (bounds.0.y..=bounds.1.y).contains(&cube.y)
        && (bounds.0.z..=bounds.1.z).contains(&cube.z)
}

pub fn part_one(input: &'static str) -> usize {
    let cubes = parse_input(input);
    cubes
        .iter()
        .flat_map(|cube| cube.sides().filter(|side| !cubes.contains(side)))
        .count()
}

pub fn part_two(input: &'static str) -> usize {
    let cubes = parse_input(input);
    let bounds = find_bounds(&cubes);

    let mut queue = vec![bounds.0];
    let mut checked: HashSet<Vec3> = HashSet::new();
    let mut surface = 0;

    loop {
        let Some(cube) = queue.pop() else {
            break;
        };

        if checked.contains(&cube) {
            continue;
        }

        if cubes.contains(&cube) {
            surface += 1;
        } else {
            for side in cube.sides() {
                if in_bounds(side, bounds) {
                    queue.insert(0, side);
                }
            }
            checked.insert(cube);
        }
    }

    surface
}

fn parse_input(input: &'static str) -> HashSet<Vec3> {
    input
        .lines()
        .map(|s| {
            let (x, y, z) = s
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Vec3 { x, y, z }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 64);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 58);
    }
}
