use crate::tasks::compensation::Compensation;
use crate::tasks::transaction::Transaction;

#[derive(Debug)]
pub struct Instruction {
  state: InstructionState,
  transaction: Transaction,
  compensation: Compensation,
}

impl Instruction {
  /// Create a new instruction from a given transaction and a compensation
  pub fn new(transaction: Transaction, compensation: Compensation) -> Instruction {
    Instruction {
      state: InstructionState::ScheduledRun,
      transaction,
      compensation,
    }
  }

  /// Executes the instruction in a blocking fashion
  pub fn start_blocking(&self) -> Result<&Self, ()> {
    self.transaction.run_blocking();

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
