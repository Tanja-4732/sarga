#[derive(Debug)]
pub struct Instruction<T, C>
where
  T: Fn() -> Result<(), ()>,
  C: Fn() -> (),
{
  state: InstructionState,
  transaction: T,
  compensation: C,
}

impl<T, C> Instruction<T, C>
where
  T: Fn() -> Result<(), ()>,
  C: Fn() -> (),
{
  /// Create a new instruction from a given transaction and a compensation
  pub fn new(transaction: T, compensation: C) -> Instruction<T, C> {
    Instruction {
      state: InstructionState::ScheduledRun,
      transaction,
      compensation,
    }
  }

  /// Executes the instruction in a blocking fashion
  pub fn start(&self) -> Result<&Self, ()> {
    self.transaction.run();

    Ok(self)

    // Err(())

    // TODO Start the transaction, await the result, and return
  }
}

/// The current state of the instruction
#[derive(Debug)]
pub enum InstructionState {
  ScheduledRun,
  ScheduledAbort,
  Running,
  Aborting,
  Finished,
  Aborted,
}
