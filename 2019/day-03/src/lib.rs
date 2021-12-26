use std::ops::RangeInclusive;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    pub fn manhattan_distance(&self, other: &Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }

    pub fn len(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    fn negative(&self) -> bool {
        self.x < 0 || self.y < 0
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug)]
struct Intersection {
    pos: Pos,
    steps: (usize, usize),
}

#[derive(Debug)]
enum Line {
    Horizontal {
        x: RangeInclusive<isize>,
        y: isize,
        steps: (usize, usize),
    },
    Vertical {
        x: isize,
        y: RangeInclusive<isize>,
        steps: (usize, usize),
    },
}

impl Line {
    pub fn index_step(&self, idx: usize) -> Option<usize> {
        let &(lhs, rhs) = match self {
            Self::Horizontal { steps, .. } => steps,
            Self::Vertical { steps, .. } => steps,
        };
        (idx <= abs_diff(lhs, rhs)).then(|| {
            lhs.min(rhs)
                + if lhs > rhs {
                    abs_diff(lhs, rhs) - idx
                } else {
                    idx
                }
        })
    }

    pub fn intersection(&self, other: &Line) -> Option<Intersection> {
        match self {
            Line::Horizontal { x: x_range, y, .. } => match other {
                Line::Vertical { x, y: y_range, .. }
                    if x_range.contains(x) && y_range.contains(y) =>
                {
                    Some(Intersection {
                        pos: Pos { x: *x, y: *y },
                        steps: (
                            self.index_step((x - x_range.start()) as usize).unwrap(),
                            other.index_step((y - y_range.start()) as usize).unwrap(),
                        ),
                    })
                }
                _ => None,
            },
            Line::Vertical { x, y: y_range, .. } => match other {
                Line::Horizontal { x: x_range, y, .. }
                    if x_range.contains(x) && y_range.contains(y) =>
                {
                    Some(Intersection {
                        pos: Pos { x: *x, y: *y },
                        steps: (
                            self.index_step((y - y_range.start()) as usize).unwrap(),
                            other.index_step((x - x_range.start()) as usize).unwrap(),
                        ),
                    })
                }
                _ => None,
            },
        }
    }
}

#[derive(Debug)]
struct Wire {
    lines: Vec<Line>,
}

impl Wire {
    pub fn intersections(&self, other: &Wire) -> Vec<Intersection> {
        self.lines
            .iter()
            .flat_map(|line| {
                other
                    .lines
                    .iter()
                    .filter_map(|other_line| line.intersection(other_line))
                    .filter(|inter| !inter.pos.is_zero())
            })
            .collect()
    }
}

impl From<&str> for Wire {
    fn from(s: &str) -> Self {
        let mut lines = Vec::new();
        let mut pos = Pos::zero();
        let mut steps = 0;
        for dir in s.split(',').map(|movement| {
            let (dir, dist) = movement.split_at(1);
            let dist = dist.parse::<isize>().unwrap();
            match dir {
                "U" => Pos { x: 0, y: -dist },
                "D" => Pos { x: 0, y: dist },
                "L" => Pos { x: -dist, y: 0 },
                "R" => Pos { x: dist, y: 0 },
                _ => unreachable!(),
            }
        }) {
            // horizontal line
            if dir.y == 0 {
                let (lhs_steps, rhs_steps) = (steps, steps + dir.x.abs() as usize);
                let (start, end) = (pos.x, pos.x + dir.x);
                lines.push(Line::Horizontal {
                    x: start.min(end)..=start.max(end),
                    y: pos.y,
                    steps: if dir.negative() {
                        (rhs_steps, lhs_steps)
                    } else {
                        (lhs_steps, rhs_steps)
                    },
                });
            }
            // vertical line
            else {
                let (lhs_steps, rhs_steps) = (steps, steps + dir.y.abs() as usize);
                let (start, end) = (pos.y, pos.y + dir.y);
                lines.push(Line::Vertical {
                    x: pos.x,
                    y: start.min(end)..=start.max(end),
                    steps: if dir.negative() {
                        (rhs_steps, lhs_steps)
                    } else {
                        (lhs_steps, rhs_steps)
                    },
                });
            }

            steps += dir.len();
            pos += dir;
        }

        Wire { lines }
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    a.max(b) - a.min(b)
}

pub fn part_one(input: &'static str) -> usize {
    let wires = parse_input(input);
    wires[0]
        .intersections(&wires[1])
        .iter()
        .map(|inter| inter.pos.manhattan_distance(&Pos::zero()))
        .min()
        .unwrap()
}

pub fn part_two(input: &'static str) -> usize {
    let wires = parse_input(input);
    wires[0]
        .intersections(&wires[1])
        .iter()
        .map(|inter| inter.steps.0 + inter.steps.1)
        .min()
        .unwrap()
}

fn parse_input(input: &'static str) -> [Wire; 2] {
    let mut wires = input.lines().map(Wire::from);
    [wires.next().unwrap(), wires.next().unwrap()]
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
        assert_eq!(
            part_one("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            159
        );
        assert_eq!(
            part_one(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("R8,U5,L5,D3\nU7,R6,D4,L4"), 30);
        assert_eq!(
            part_two("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            610
        );
        assert_eq!(
            part_two(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
