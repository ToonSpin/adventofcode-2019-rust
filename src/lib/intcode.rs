#[derive(Debug)]
enum ParameterMode {
    Immediate,
    Position
}

#[derive(Debug)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl Opcode {
    fn from(i: i32) -> Opcode {
        match i {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Halt,
            _ => { panic!("Unknown opcode: {}", i) },
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    param1: ParameterMode,
    param2: ParameterMode,
    param3: ParameterMode,
}

impl Instruction {
    fn from(mut i: i32) -> Instruction {
        let opcode = Opcode::from(i % 100);
        i /= 100;
        let param1 = if i % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
        i /= 10;
        let param2 = if i % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
        i /= 10;
        let param3 = if i % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };

        Instruction {
            opcode,
            param1,
            param2,
            param3
        }
    }
}

#[derive(Debug)]
enum ProgramState {
    Running,
    WaitingForInput,
    Halted,
}

/// Contains an Intcode program.
pub struct Program {
    program: Vec<i32>,
    sp: usize,
    input: Vec<i32>,
    input_pos: usize,
    output: Vec<i32>,
    output_pos: usize,
    state: ProgramState,
}

impl Program {
    /// Creates a new Intcode program.
    ///
    /// The `Program` returned will start out as Running.
    pub fn new(program_vec: &Vec<i32>) -> Program {
        Program {
            program: program_vec.clone(),
            sp: 0,
            input: Vec::new(),
            input_pos: 0,
            output: Vec::new(),
            output_pos: 0,
            state: ProgramState::Running
        }
    }

    /// Adds a value to the program's input queue.
    pub fn push_input(&mut self, i: i32) {
        self.input.push(i);
        if let ProgramState::WaitingForInput = self.state {
            self.state = ProgramState::Running;
        }
    }

    fn push_output(&mut self, i: i32) {
        self.output.push(i);
    }

    fn get_input(&mut self) -> i32 {
        let result = self.input[self.input_pos];
        self.input_pos += 1;
        result
    }

    /// Return `true` if and only if this program's output queue is not empty.
    pub fn has_output(&mut self) -> bool {
        self.output_pos < self.output.len()
    }

    /// Returns the last output generated, even if it has already been consumed.
    /// If no outputs have been generated yet, this will return `None`.
    ///
    /// # Example
    /// ```
    /// let mut program = intcode::Program::new(&vec![4, 5, 4, 6, 99, 1, 2]);
    /// program.run_till_halted_or_blocked();
    /// assert_eq!(program.get_output(), Some(1));
    /// assert_eq!(program.get_output(), Some(2));
    /// assert_eq!(program.get_output(), None);
    /// assert_eq!(program.last_output(), Some(2));
    /// ```
    pub fn last_output(&mut self) -> Option<i32> {
        if self.output.len() > 0 {
            Some(self.output[self.output.len() - 1])
        } else {
            None
        }
    }

    /// Consumes and returns the first output in the output queue that has not
    /// been consumed. If no outputs have been generated yet, this will return
    /// `None`.
    ///
    /// # Example
    /// ```
    /// let mut program = intcode::Program::new(&vec![4, 5, 4, 6, 99, 1, 2]);
    /// program.run_till_halted_or_blocked();
    /// assert_eq!(program.get_output(), Some(1));
    /// assert_eq!(program.get_output(), Some(2));
    /// assert_eq!(program.get_output(), None);
    /// assert_eq!(program.last_output(), Some(2));
    /// ```
    pub fn get_output(&mut self) -> Option<i32> {
        if self.has_output() {
            self.output_pos += 1;
            Some(self.output[self.output_pos - 1])
        } else {
            None
        }
    }

    fn increase_sp(&mut self) {
        let instruction = Instruction::from(self.program[self.sp]);
        self.sp += match instruction.opcode {
            Opcode::Add => 4,
            Opcode::Multiply => 4,
            Opcode::Input => 2,
            Opcode::Output => 2,
            Opcode::JumpIfTrue => 3,
            Opcode::JumpIfFalse => 3,
            Opcode::LessThan => 4,
            Opcode::Equals => 4,
            Opcode::Halt => 0,
        }
    }

    fn param(&self, param: usize) -> i32{
        let instruction = Instruction::from(self.program[self.sp]);
        let value = self.program[self.sp + param];

        let mode = match param {
            1 => instruction.param1,
            2 => instruction.param2,
            3 => instruction.param3,
            _ => unreachable!()
        };

        match mode {
            ParameterMode::Immediate => { value },
            ParameterMode::Position => { self.program[value as usize] },
        }
    }

    fn execute_instruction(&mut self) {
        let instruction = Instruction::from(self.program[self.sp]);
        let mut bump_sp = true;

        if let ProgramState::Halted = self.state {
            panic!("Attempted to run a halted program.");
        }

        match instruction.opcode {
            Opcode::Add => {
                let pos = self.program[(self.sp + 3) as usize];
                self.program[pos as usize] = self.param(1) + self.param(2);
            }
            Opcode::Multiply => {
                let pos = self.program[(self.sp + 3) as usize];
                self.program[pos as usize] = self.param(1) * self.param(2);
            }
            Opcode::Input => {
                if self.input.len() > self.input_pos {
                    let pos = self.program[(self.sp + 1) as usize];
                    self.program[pos as usize] = self.get_input();
                } else {
                    bump_sp = false;
                    self.state = ProgramState::WaitingForInput;
                }
            }
            Opcode::Output => {
                self.push_output(self.param(1));
            }
            Opcode::JumpIfTrue => {
                if self.param(1) != 0 {
                    bump_sp = false;
                    self.sp = self.param(2) as usize;
                }
            }
            Opcode::JumpIfFalse => {
                if self.param(1) == 0 {
                    bump_sp = false;
                    self.sp = self.param(2) as usize;
                }
            }
            Opcode::LessThan => {
                let pos = self.program[(self.sp + 3) as usize];
                self.program[pos as usize] = if self.param(1) < self.param(2) { 1 } else { 0 };
            }
            Opcode::Equals => {
                let pos = self.program[(self.sp + 3) as usize];
                self.program[pos as usize] = if self.param(1) == self.param(2) { 1 } else { 0 };
            }
            Opcode::Halt => {
                self.state = ProgramState::Halted;
            }
        }
        if bump_sp {
            self.increase_sp();
        }
    }

    /// Returns `true` if and only if the program is in the "halted" state. This
    /// can only happen if the appropriate opcode has been executed.
    pub fn halted(&mut self) -> bool {
        match self.state {
            ProgramState::Running => false,
            ProgramState::Halted => true,
            ProgramState::WaitingForInput => false,
        }
    }

    /// Returns `true` if and only if the program is in the "halted" state, or
    /// is waiting for input.
    pub fn halted_or_blocked(&mut self) -> bool {
        match self.state {
            ProgramState::Running => false,
            ProgramState::Halted => true,
            ProgramState::WaitingForInput => true,
        }
    }

    /// Starts running the program until it can't run any further.
    ///
    /// This will go through the instructions of the program until it halts, or
    /// encounters an "input" opcode but has no input. If the latter happens,
    /// then you can call this method again after supplying input to make the
    /// program resume execution.
    ///
    /// # Panics
    ///
    /// Panics if the program is in "halted" state when the method is called, or
    /// if an unknown opcode is encountered.
    ///
    /// # Example
    /// ```
    /// let mut program = intcode::Program::new(&vec![3, 5, 4, 5, 99, 0]);
    /// program.run_till_halted_or_blocked();
    ///
    /// assert_eq!(program.get_output(), None);
    /// assert!(!program.halted());
    ///
    /// program.push_input(123);
    /// program.run_till_halted_or_blocked();
    ///
    /// assert_eq!(program.get_output(), Some(123));
    /// assert!(program.halted());
    /// ```
    pub fn run_till_halted_or_blocked(&mut self) {
        while !self.halted_or_blocked() {
            self.execute_instruction();
        }
    }
}
