use intterpreter::{IntTerpreter, Memory};

pub mod intterpreter;

pub fn part_one(input: &'static str) -> i64 {
    let memory = parse_input(input);
    let mut computer = IntTerpreter::new()
        .with_memory(memory)
        .with_input(&[1])
        .with_buffer();
    computer.execute();
    computer.output().unwrap().last().unwrap().into()
}

pub fn part_two(input: &'static str) -> i64 {
    let memory = parse_input(input);
    let mut computer = IntTerpreter::new()
        .with_memory(memory)
        .with_input(&[5])
        .with_buffer();
    computer.execute();
    computer.output().unwrap().last().unwrap().into()
}

fn parse_input(input: &'static str) -> Memory {
    Memory::try_from(input).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 12440243);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 15486302);
    }
}
