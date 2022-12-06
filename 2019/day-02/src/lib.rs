pub mod intcode {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum Opcode {
        Add,
        Mul,
        Hlt,
    }

    impl Opcode {
        pub fn arg_len(&self) -> usize {
            match self {
                Self::Add => 3,
                Self::Mul => 3,
                Self::Hlt => 0,
            }
        }
    }

    impl TryFrom<usize> for Opcode {
        type Error = (); // TODO: Custom errors

        fn try_from(value: usize) -> Result<Self, Self::Error> {
            Ok(match value {
                1 => Self::Add,
                2 => Self::Mul,
                99 => Self::Hlt,
                _ => return Err(()),
            })
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Memory {
        words: Vec<usize>,
    }

    impl Memory {
        pub fn len(&self) -> usize {
            self.words.len()
        }

        pub fn is_empty(&self) -> bool {
            self.words.is_empty()
        }

        pub fn get(&self, index: usize) -> Option<usize> {
            (index < self.len()).then(|| self.words[index])
        }
    }

    impl std::ops::Index<usize> for Memory {
        type Output = usize;

        fn index(&self, index: usize) -> &Self::Output {
            self.words.index(index)
        }
    }

    impl std::ops::IndexMut<usize> for Memory {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            self.words.index_mut(index)
        }
    }

    impl TryFrom<&str> for Memory {
        type Error = (); // TODO: Custom errors

        fn try_from(s: &str) -> Result<Self, Self::Error> {
            if let Ok(words) = s
                .split(',')
                .filter_map(|mut s| {
                    s = s.trim();
                    if !s.is_empty() {
                        Some(s.parse())
                    } else {
                        None
                    }
                })
                .collect::<Result<Vec<usize>, _>>()
            {
                Ok(Memory { words })
            } else {
                Err(())
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Status {
        Ready,
        Finished,
        Error,
    }

    impl Status {
        /// Returns `true` if the status is [`Ready`].
        ///
        /// [`Ready`]: Status::Ready
        pub fn is_ready(&self) -> bool {
            matches!(self, Self::Ready)
        }

        /// Returns `true` if the status is [`Finished`].
        ///
        /// [`Finished`]: Status::Finished
        pub fn is_finished(&self) -> bool {
            matches!(self, Self::Finished)
        }

        /// Returns `true` if the status is [`Error`].
        ///
        /// [`Error`]: Status::Error
        pub fn is_error(&self) -> bool {
            matches!(self, Self::Error)
        }
    }

    impl Default for Status {
        fn default() -> Self {
            Self::Ready
        }
    }

    #[derive(Debug, Default)]
    pub struct IntTerpreter {
        ip: usize,
        memory: Option<Memory>,
        status: Status,
    }

    impl IntTerpreter {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_memory(memory: Memory) -> Self {
            Self {
                memory: Some(memory),
                ..Default::default()
            }
        }

        pub fn set_memory(&mut self, memory: Memory) {
            self.memory = Some(memory);
            self.reset();
        }

        pub fn reset(&mut self) {
            self.ip = 0;
            self.status = Status::Ready;
        }

        pub fn step(&mut self) -> Status {
            if self.status.is_ready() {
                if let Some(ref mut memory) = self.memory {
                    // Program has finished executing
                    if self.ip >= memory.len() {
                        self.status = Status::Finished;
                    } else if let Ok(opcode) = Opcode::try_from(memory[self.ip]) {
                        self.ip += 1;

                        // Consume arguments
                        let n_args = opcode.arg_len();
                        if let Some(args) = (self.ip..self.ip + n_args)
                            .map(|i| memory.get(i))
                            .collect::<Option<Vec<usize>>>()
                        {
                            assert_eq!(args.len(), n_args);
                            self.ip += n_args;

                            self.status = match opcode {
                                Opcode::Add => {
                                    memory[args[2]] = memory[args[0]] + memory[args[1]];
                                    Status::Ready
                                }
                                Opcode::Mul => {
                                    memory[args[2]] = memory[args[0]] * memory[args[1]];
                                    Status::Ready
                                }
                                Opcode::Hlt => Status::Finished,
                            }
                        } else {
                            self.status = Status::Error;
                        }
                    } else {
                        self.status = Status::Error;
                    }
                }
            }
            self.status
        }

        pub fn execute(&mut self) -> Status {
            while self.step().is_ready() {}
            self.status
        }

        pub fn execute_until(&mut self, _f: ()) -> Status {
            todo!()
        }

        pub fn get(&self, index: usize) -> Option<usize> {
            if let Some(memory) = &self.memory {
                memory.get(index)
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_general() {
            for (program, result) in [
                ("1,0,0,0,99", "2,0,0,0,99"),
                ("2,3,0,3,99", "2,3,0,6,99"),
                ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
                ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"),
            ] {
                assert_eq!(run_program(program), Memory::try_from(result).unwrap());
            }
        }

        fn run_program(raw_program: &str) -> Memory {
            let memory = Memory::try_from(raw_program).unwrap();
            let mut interpreter = IntTerpreter::with_memory(memory);
            interpreter.execute();
            interpreter.memory.unwrap()
        }
    }
}

use intcode::*;

pub fn part_one(input: &'static str) -> usize {
    let mut memory = Memory::try_from(input).unwrap();
    memory[1] = 12;
    memory[2] = 2;
    let mut interpreter = IntTerpreter::with_memory(memory);
    interpreter.execute();
    interpreter.get(0).unwrap()
}

pub fn part_two(input: &'static str) -> usize {
    let base_memory = Memory::try_from(input).unwrap();

    const TARGET: usize = 19690720;

    for (noun, verb) in (0..128).flat_map(move |a| (0..128).map(move |b| (a, b))) {
        let mut memory = base_memory.clone();
        memory[1] = noun;
        memory[2] = verb;
        let mut interpreter = IntTerpreter::with_memory(memory);
        interpreter.execute();
        let output = interpreter.get(0).unwrap();

        if output == TARGET {
            return 100 * noun + verb;
        }
    }
    panic!("Failed");
}
