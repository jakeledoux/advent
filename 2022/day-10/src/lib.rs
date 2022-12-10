use std::str::FromStr;

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

#[derive(Debug)]
struct Crt {
    pixels: [Pixel; CRT_WIDTH * CRT_HEIGHT],
}

impl Default for Crt {
    fn default() -> Self {
        Self {
            pixels: [Pixel::default(); CRT_WIDTH * CRT_HEIGHT],
        }
    }
}

impl Crt {
    pub fn draw(&self) -> String {
        let mut buf = String::with_capacity(CRT_WIDTH * CRT_HEIGHT + CRT_HEIGHT);
        for (i, pixel) in self.pixels.iter().enumerate() {
            if i > 0 && i % CRT_WIDTH == 0 {
                buf.push('\n')
            }
            buf.push((*pixel).into());
        }
        buf
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
enum Pixel {
    On,
    #[default]
    Off,
}

impl From<Pixel> for char {
    fn from(pixel: Pixel) -> Self {
        match pixel {
            Pixel::On => '#',
            Pixel::Off => '.',
        }
    }
}

#[derive(Debug)]
struct Cpu {
    x: i32,
    pc: usize,
    cycle: usize,
    state: CpuState,
    instructions: Vec<Instruction>,
    crt: Crt,
    signal_strengths: Vec<i32>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            x: 1,
            pc: 0,
            cycle: 0,
            state: CpuState::default(),
            instructions: Vec::with_capacity(256),
            crt: Crt::default(),
            signal_strengths: Vec::with_capacity(64),
        }
    }
}

impl Cpu {
    pub fn run(&mut self, instructions: &[Instruction]) {
        self.instructions.extend_from_slice(instructions);
        while !self.state.is_complete() {
            self.step();
        }
    }

    pub fn step(&mut self) {
        self.cycle += 1;

        // reached end of program
        if self.pc >= self.instructions.len() {
            self.state = CpuState::Complete;
        }

        // prepare next instruction
        if self.state.is_idle() {
            self.state = CpuState::Calculating {
                cycles_remaining: self.instructions[self.pc].cycles(),
            }
        }

        // check signal strengths
        if (self.cycle as i32 - 20) % 40 == 0 {
            self.signal_strengths.push(self.cycle as i32 * self.x);
        }

        // draw to CRT
        let pixel = self.cycle - 1;
        if ((self.x - 1)..=(self.x + 1)).contains(&(pixel as i32 % 40)) {
            self.crt.pixels[pixel] = Pixel::On;
        }

        // make progress on next instruction
        if let CpuState::Calculating {
            ref mut cycles_remaining,
        } = self.state
        {
            *cycles_remaining -= 1;

            if cycles_remaining == &0 {
                // execute instruction
                match self.instructions[self.pc] {
                    Instruction::Addx(n) => self.x += n,
                    Instruction::Noop => {}
                }

                // step forward
                self.state = CpuState::Idle;
                self.pc += 1;
            }
        }
    }
}

#[derive(Default, Clone, Copy, Debug, Hash)]
enum CpuState {
    #[default]
    Idle,
    Calculating {
        cycles_remaining: usize,
    },
    Complete,
}

impl CpuState {
    #[must_use]
    fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    #[must_use]
    fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    #[must_use]
    pub fn cycles(&self) -> usize {
        match self {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split(' ').collect::<Vec<_>>()[..] {
            ["addx", n] => Self::Addx(n.parse().unwrap(/* TODO: don't unwrap */)),
            ["noop"] => Self::Noop,
            _ => return Err(()),
        })
    }
}

pub fn part_one(input: &'static str) -> i32 {
    let input = parse_input(input);
    let mut cpu = Cpu::default();
    cpu.run(&input);
    cpu.signal_strengths.into_iter().sum()
}

pub fn part_two(input: &'static str) -> String {
    let input = parse_input(input);
    let mut cpu = Cpu::default();
    cpu.run(&input);
    cpu.crt.draw()
}

fn parse_input(input: &'static str) -> Vec<Instruction> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 13140);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(SAMPLE),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
            "
            .trim()
        );
    }
}
