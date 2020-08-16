use crate::core::instruction::Instruction;

/// A saga is a list of instructions, leading to eventual consistency
#[derive(Debug)]
pub struct Saga<T,C>
where
  T: Fn() -> Result<(), ()>,
  C: Fn() -> (),
{
  instructions: Vec<Instruction<T, C>>,
  state: SagaState,
  abort_type: AbortType,
}

impl Saga {
  /// Create a new saga with a given name
  pub fn new(abort_type: AbortType) -> Saga {
    Saga {
      instructions: Vec::new(),
      state: SagaState::Pending,
      abort_type,
    }
  }

  /// Appends an instruction to the end of the saga if it has not been started yet
  pub fn add_instruction(mut self, instruction:  ) -> Self {
    match self.state {
      // Add the requested instruction to the saga
      SagaState::Pending => {
        self.instructions.push(instruction);
        self
      }

      // Prevent the modification of the saga after starting
      _ => panic!("Cannot append instructions after starting"),
    }
  }

  /// Starts the saga in a blocking fashion
  pub fn start(&mut self) -> Result<(), ()> {
    for i in 0..self.instructions.len() {
      self.state = SagaState::Running(i);

      match self.instructions[i].start() {
        Ok(_) => (),
        Err(()) => return self.abort(i),
      };
    }

    self.state = SagaState::Finished;
    Ok(())
  }

  fn abort(&mut self, i: usize) -> Result<(), ()> {
    self.state = SagaState::Aborting(i);
    Ok(())
  }

  fn abort_forwards(&mut self) {}
  fn abort_backwards(&mut self) {}
}

/// The current state of the saga
#[derive(Debug)]
enum SagaState {
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
