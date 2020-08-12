use crate::tasks::instruction::Instruction;

/// A saga is a list of instructions, leading to eventual consistency
#[derive(Debug)]
pub struct Saga {
  instructions: Vec<Instruction>,
  state: State,
  abort_type: AbortType,
}

impl Saga {
  /// Create a new saga with a given name
  pub fn new(abort_type: AbortType) -> Saga {
    Saga {
      instructions: Vec::new(),
      state: State::Pending,
      abort_type,
    }
  }

  /// Appends an instruction to the end of the saga
  pub fn add_instruction(mut self, instruction: Instruction) -> Self {
    match self.state {
      State::Pending => {
        self.instructions.push(instruction);
        self
      }
      _ => panic!("Cannot append instructions after starting"),
    }
  }

  /// Starts the saga in a blocking fashion
  pub fn start_blocking(&mut self) {
    // TODO start all instructions in a loop
    // TODO Set the state inside of the loop
    self.state = State::Running(1);
    // TODO Return something
  }
}

/// The current state of the saga
#[derive(Debug)]
enum State {
  Pending,
  Running(usize),
  Aborting(usize),
  Finished,
  Aborted,
}

/// The way the saga should be aborted, if an instruction fails
#[derive(Debug)]
pub enum AbortType {
  Forward,
  Backward,
}
