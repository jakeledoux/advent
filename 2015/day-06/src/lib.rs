use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Rectangle {
    start: Point,
    end: Point,
}

impl Rectangle {
    pub fn contains(&self, point: &Point) -> bool {
        (point.x >= self.start.x && point.x <= self.end.x)
            && (point.y >= self.start.y && point.y <= self.end.y)
    }
}

#[derive(Debug)]
enum InstructionKind {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    pub kind: InstructionKind,
    pub rect: Rectangle,
}

enum Behaviour {
    PartOne,
    PartTwo,
}

#[derive(Debug, Clone, Copy)]
struct Light(usize);

impl Light {
    pub fn turn_on(&mut self, behaviour: &Behaviour) {
        match behaviour {
            Behaviour::PartOne => self.0 = 1,
            Behaviour::PartTwo => self.0 += 1,
        }
    }

    pub fn turn_off(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }

    pub fn toggle(&mut self, behaviour: &Behaviour) {
        match behaviour {
            Behaviour::PartOne => self.0 = if self.is_on() { 0 } else { 1 },
            Behaviour::PartTwo => self.0 += 2,
        }
    }

    pub fn execute(&mut self, instruction_kind: &InstructionKind, behaviour: &Behaviour) {
        match instruction_kind {
            InstructionKind::On => self.turn_on(behaviour),
            InstructionKind::Off => self.turn_off(),
            InstructionKind::Toggle => self.toggle(behaviour),
        };
    }

    #[must_use]
    fn is_on(&self) -> bool {
        self.0 > 0
    }
}

struct LightMatrix {
    width: usize,
    lights: Vec<Light>,
}

impl LightMatrix {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            lights: Vec::from_iter(std::iter::repeat(Light(0)).take(width * height)),
        }
    }

    pub fn execute(&mut self, instruction: &Instruction, behaviour: &Behaviour) {
        self.lights
            .iter_mut()
            .enumerate()
            .filter_map(|(i, light)| {
                let x = i % self.width;
                let y = i / self.width;
                let point = Point::new(x, y);
                instruction.rect.contains(&point).then(|| light)
            })
            .for_each(|light| light.execute(&instruction.kind, behaviour))
    }

    pub fn count_lit(&self) -> usize {
        self.lights.iter().map(|Light(brightness)| brightness).sum()
    }
}

pub fn part_one(input: &'static str) -> usize {
    let instructions = parse_input(input);
    let mut matrix = LightMatrix::new(1000, 1000);
    instructions
        .iter()
        .for_each(|instr| matrix.execute(instr, &Behaviour::PartOne));
    matrix.count_lit()
}

pub fn part_two(input: &'static str) -> usize {
    let instructions = parse_input(input);
    let mut matrix = LightMatrix::new(1000, 1000);
    instructions
        .iter()
        .for_each(|instr| matrix.execute(instr, &Behaviour::PartTwo));
    matrix.count_lit()
}

fn parse_input(input: &'static str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|s| match s.trim() {
            "" => None,
            _ => Some(s.trim()),
        })
        .map(|line| {
            let kind = if line.starts_with("turn on") {
                InstructionKind::On
            } else if line.starts_with("turn off") {
                InstructionKind::Off
            } else if line.starts_with("toggle") {
                InstructionKind::Toggle
            } else {
                unreachable!()
            };

            let captures: Vec<usize> = Regex::new(r#"(\d+),(\d+) through (\d+),(\d+)"#)
                .unwrap()
                .captures(line)
                .unwrap()
                .iter()
                .skip(1)
                .map(|n| n.unwrap().as_str().parse().unwrap())
                .collect();
            let rect = Rectangle {
                start: Point::new(captures[0], captures[1]),
                end: Point::new(captures[2], captures[3]),
            };

            Instruction { kind, rect }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        const SAMPLE: &str = "
turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500
";
        assert_eq!(part_one(SAMPLE), 998_996);
    }

    #[test]
    fn test_part_two() {
        const SAMPLE: &str = "
turn on 0,0 through 0,0
toggle 0,0 through 999,999
";
        assert_eq!(part_two(SAMPLE), 2000001);
    }
}
