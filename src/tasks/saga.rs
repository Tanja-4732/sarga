use crate::tasks::instruction::Instruction;

#[derive(Debug)]
pub struct Saga {
  instructions: Vec<Instruction>,
  state: State,
  name: String,
}

impl Saga {
  pub fn with_name(name: String) -> Saga {
    Saga {
      name,
      instructions: Vec::new(),
      state: State::Pending,
    }
  }

  pub fn add_instruction(&mut self, instruction: Instruction) -> &mut Saga {
    self.instructions.push(instruction);
    self
  }
}

#[derive(Debug)]
enum State {
  Pending,
  Running(usize),
  Aborting(usize),
  Finished,
  Aborted,
}
