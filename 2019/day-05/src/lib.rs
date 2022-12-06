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
