use std::io::stdin;

macro_rules! impl_from {
    ( $for_type:ty, $( $type:ty ),* ) => {
        $(
            impl From<$type> for $for_type {
                fn from(v: $type) -> Self {
                    <$for_type>::new(v.into())
                }
            }
        )*
    };
}

macro_rules! impl_try_from {
    ( $for_type:ty, $( $type:ty ),* ) => {
        $(
            impl TryFrom<$type> for $for_type {
                type Error = std::num::TryFromIntError;

                fn try_from(v: $type) -> Result<Self, Self::Error> {
                    Ok(<$for_type>::new(v.try_into()?))
                }
            }
        )*
    };
}

macro_rules! impl_try_into {
    ( $for_type:ty, $( $type:ty ),* ) => {
        $(
            impl TryFrom<$for_type> for $type {
                type Error = std::num::TryFromIntError;

                fn try_from(v: $for_type) -> Result<Self, Self::Error> {
                    <$type>::try_from(v.0)
                }
            }
        )*
    };
}

macro_rules! impl_into {
    ( $for_type:ty, $( $type:ty ),* ) => {
        $(
            impl From<$for_type> for $type {
                fn from(v: $for_type) -> $type {
                    <$type>::from(v.0)
                }
            }
        )*
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Int(i64);

impl Int {
    pub fn new(value: i64) -> Self {
        Self(value)
    }
}

impl_from!(Int, i64, i32, i16, i8);
impl_try_from!(Int, isize, usize, u64, u32, u16, u8);
impl_try_into!(Int, isize, i32, i16, i8, usize, u64, u32, u16, u8);
impl_into!(Int, i64);
impl_into!(&Int, i64);

impl From<Int> for bool {
    fn from(v: Int) -> Self {
        v != 0
    }
}

impl From<bool> for Int {
    fn from(v: bool) -> Self {
        Int(v as i64)
    }
}

impl std::cmp::PartialEq<i64> for Int {
    fn eq(&self, other: &i64) -> bool {
        self.0.eq(other)
    }
}

impl std::str::FromStr for Int {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>().map(Int)
    }
}

impl std::fmt::Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::ops::Add for Int {
    type Output = Int;

    fn add(self, rhs: Self) -> Self::Output {
        Int(self.0 + rhs.0)
    }
}

impl std::ops::Mul for Int {
    type Output = Int;

    fn mul(self, rhs: Self) -> Self::Output {
        Int(self.0 * rhs.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputMode {
    Stdin,
    Buffer(Vec<Int>),
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Stdin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OutputMode {
    Stdout,
    Buffer(Vec<Int>),
}

impl Default for OutputMode {
    fn default() -> Self {
        Self::Stdout
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ParameterMode {
    Position,
    Immediate,
}

impl Default for ParameterMode {
    fn default() -> Self {
        Self::Position
    }
}

impl TryFrom<Int> for ParameterMode {
    type Error = ();

    fn try_from(value: Int) -> Result<Self, Self::Error> {
        Ok(match value.0 {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Opcode {
    Add, // Add
    Mul, // Multiply
    Str, // Store
    Out, // Output
    Bot, // Branch on true
    Bof, // Branch on false
    Clt, // Check less than
    Ceq, // Check equal
    Hlt, // Halt
}

impl TryFrom<Int> for Opcode {
    type Error = (); // TODO: Custom errors

    fn try_from(value: Int) -> Result<Self, Self::Error> {
        Ok(match value.0 {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::Str,
            4 => Self::Out,
            5 => Self::Bot,
            6 => Self::Bof,
            7 => Self::Clt,
            8 => Self::Ceq,
            99 => Self::Hlt,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Instruction {
    opcode: Opcode,
    modes: [ParameterMode; 3],
}

impl Instruction {
    pub fn arg_len(&self) -> usize {
        match self.opcode {
            Opcode::Add => 3,
            Opcode::Mul => 3,
            Opcode::Str => 1,
            Opcode::Out => 1,
            Opcode::Hlt => 0,
            Opcode::Bot => 2,
            Opcode::Bof => 2,
            Opcode::Clt => 3,
            Opcode::Ceq => 3,
        }
    }
}

impl TryFrom<Int> for Instruction {
    type Error = (); // TODO: Custom errors

    fn try_from(value: Int) -> Result<Self, Self::Error> {
        let value = format!("{:05}", value);
        let (modes, opcode) = value.split_at(3);
        let modes: Vec<ParameterMode> = modes
            .chars()
            .map(|c| ParameterMode::try_from(Int(c.to_digit(10).unwrap() as i64)).unwrap())
            .rev()
            .collect();
        let opcode = Opcode::try_from(opcode.parse::<Int>().unwrap())?;
        Ok(Self {
            opcode,
            modes: modes[..3].try_into().unwrap(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Memory {
    words: Vec<Int>,
}

impl Memory {
    pub fn len(&self) -> usize {
        self.words.len()
    }

    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<Int> {
        (index < self.len()).then(|| self.words[index])
    }
}

impl std::ops::Index<usize> for Memory {
    type Output = Int;

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
            .collect::<Result<Vec<Int>, _>>()
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
    input: InputMode,
    output: OutputMode,
}

impl IntTerpreter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_memory(mut self, memory: Memory) -> Self {
        self.memory = Some(memory);
        self
    }

    pub fn with_input<T>(mut self, input: &[T]) -> Self
    where
        Int: From<T>,
        T: Copy,
    {
        let vecc = input.iter().map(|t| Int::from(*t)).collect();
        self.input = InputMode::Buffer(vecc);
        self
    }

    pub fn with_buffer(mut self) -> Self {
        self.output = OutputMode::Buffer(Vec::new());
        self
    }

    pub fn with_stdout(mut self) -> Self {
        self.output = OutputMode::Stdout;
        self
    }

    pub fn set_memory(&mut self, memory: Memory) {
        self.memory = Some(memory);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.ip = 0;
        self.status = Status::Ready;
        if let Some(output) = self.output_mut() {
            output.clear();
        }
        if let Some(input) = self.input_mut() {
            input.clear();
        }
    }

    pub fn eval_arg(&self, instruction: &Instruction, args: &[Int], argi: usize) -> Int {
        let arg = args[argi];
        match instruction.modes[argi] {
            ParameterMode::Position => self.memory.as_ref().unwrap()[arg.0 as usize],
            ParameterMode::Immediate => arg,
        }
    }

    pub fn read_memory(&self, address: usize) -> Result<Int, ()> {
        if let Some(ref memory) = self.memory {
            if address < memory.len() {
                return Ok(memory[address]);
            }
        }
        Err(())
    }

    pub fn write_memory(&mut self, value: Int, address: Int) {
        if let Some(ref mut memory) = self.memory {
            memory[address.0 as usize] = value;
        }
    }

    pub fn next_instruction(&mut self) -> Result<Instruction, ()> {
        let instruction = Instruction::try_from(self.read_memory(self.ip)?)?;
        self.ip += 1;
        Ok(instruction)
    }

    pub fn consume_args(&mut self, instruction: &Instruction) -> Result<Vec<Int>, ()> {
        let args = (self.ip..self.ip + instruction.arg_len())
            .filter_map(|i| self.read_memory(i).ok())
            .collect::<Vec<Int>>();

        if args.len() == instruction.arg_len() {
            self.ip += instruction.arg_len();
            Ok(args)
        } else {
            Err(())
        }
    }

    pub fn take_input(&mut self) -> Option<Int> {
        match self.input {
            InputMode::Stdin => {
                let mut buf = String::new();
                stdin().read_line(&mut buf).unwrap();
                buf.parse().ok()
            }
            InputMode::Buffer(ref mut input) => (!input.is_empty()).then(|| input.remove(0)),
        }
    }

    pub fn intput(&self) -> Option<&Vec<Int>> {
        match self.input {
            InputMode::Buffer(ref buf) => Some(buf),
            _ => None,
        }
    }

    pub fn input_mut(&mut self) -> Option<&mut Vec<Int>> {
        match self.input {
            InputMode::Buffer(ref mut buf) => Some(buf),
            _ => None,
        }
    }

    pub fn output(&self) -> Option<&Vec<Int>> {
        match self.output {
            OutputMode::Buffer(ref buf) => Some(buf),
            _ => None,
        }
    }

    pub fn output_mut(&mut self) -> Option<&mut Vec<Int>> {
        match self.output {
            OutputMode::Buffer(ref mut buf) => Some(buf),
            _ => None,
        }
    }

    fn write_output(&mut self, value: Int) {
        match self.output {
            OutputMode::Stdout => println!("{}", value),
            OutputMode::Buffer(ref mut buf) => buf.push(value),
        }
    }

    pub fn step(&mut self) -> Status {
        if self.status.is_ready() {
            // Program has finished executing
            if let Ok(instr) = self.next_instruction() {
                // Consume arguments
                if let Ok(args) = self.consume_args(&instr) {
                    self.status = match instr.opcode {
                        Opcode::Add => {
                            let sum =
                                self.eval_arg(&instr, &args, 0) + self.eval_arg(&instr, &args, 1);
                            self.write_memory(sum, args[2]);
                            Status::Ready
                        }
                        Opcode::Mul => {
                            let product =
                                self.eval_arg(&instr, &args, 0) * self.eval_arg(&instr, &args, 1);
                            self.write_memory(product, args[2]);
                            Status::Ready
                        }
                        Opcode::Str => {
                            if let Some(value) = self.take_input() {
                                self.write_memory(value, args[0]);
                                Status::Ready
                            } else {
                                Status::Error
                            }
                        }
                        Opcode::Out => {
                            self.write_output(self.eval_arg(&instr, &args, 0));
                            Status::Ready
                        }
                        Opcode::Bot => {
                            if self.eval_arg(&instr, &args, 0).into() {
                                if let Ok(ptr) = self.eval_arg(&instr, &args, 1).try_into() {
                                    self.ip = ptr;
                                    Status::Ready
                                } else {
                                    Status::Error
                                }
                            } else {
                                Status::Ready
                            }
                        }
                        Opcode::Bof => {
                            if !bool::from(self.eval_arg(&instr, &args, 0)) {
                                if let Ok(ptr) = self.eval_arg(&instr, &args, 1).try_into() {
                                    self.ip = ptr;
                                    Status::Ready
                                } else {
                                    Status::Error
                                }
                            } else {
                                Status::Ready
                            }
                        }
                        Opcode::Clt => {
                            let cmp =
                                self.eval_arg(&instr, &args, 0) < self.eval_arg(&instr, &args, 1);
                            self.write_memory(cmp.into(), args[2]);
                            Status::Ready
                        }
                        Opcode::Ceq => {
                            let cmp =
                                self.eval_arg(&instr, &args, 0) == self.eval_arg(&instr, &args, 1);
                            self.write_memory(cmp.into(), args[2]);
                            Status::Ready
                        }
                        Opcode::Hlt => Status::Finished,
                    }
                } else {
                    self.status = Status::Error;
                }
            } else {
                self.status = Status::Finished;
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

    pub fn get(&self, index: usize) -> Option<Int> {
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

    #[test]
    fn test_cmp() {
        for (program, input, result) in [
            ("3,9,8,9,10,9,4,9,99,-1,8", 8, 1),
            ("3,9,7,9,10,9,4,9,99,-1,8", 8, 0),
            ("3,3,1108,-1,8,3,4,3,99", 8, 1),
            ("3,3,1107,-1,8,3,4,3,99", 8, 0),
            ("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 8, 1000),
            ("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 7, 999),
            ("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 9, 1001),
        ] {
            let memory = Memory::try_from(program).unwrap();
            let mut interpreter = IntTerpreter::new()
                .with_memory(memory)
                .with_input(&[input])
                .with_buffer();
            interpreter.execute();
            assert!(!matches!(interpreter.status, Status::Error));
            assert_eq!(*interpreter.output().unwrap().last().unwrap(), result);
        }
    }

    fn run_program(raw_program: &str) -> Memory {
        let memory = Memory::try_from(raw_program).unwrap();
        let mut interpreter = IntTerpreter::new().with_memory(memory);
        interpreter.execute();
        assert!(!matches!(interpreter.status, Status::Error));
        interpreter.memory.unwrap()
    }

    #[test]
    fn test_instruction() {
        let n = Int(1002);
        let instr = Instruction::try_from(n).unwrap();
        assert_eq!(instr.opcode, Opcode::Mul);
        assert_eq!(
            instr.modes,
            [
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position
            ]
        );
    }
}
