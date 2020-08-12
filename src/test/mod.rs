use crate::tasks::compensation::Compensation;
use crate::tasks::instruction::Instruction;
use crate::tasks::saga::AbortType;
use crate::tasks::saga::Saga;
use crate::tasks::transaction::Transaction;

pub fn test_main() {
  let mut saga = Saga::new(AbortType::Backward)
    .add_instruction(Instruction::new(Transaction {}, Compensation {}))
    .add_instruction(Instruction::new(Transaction {}, Compensation {}))
    .add_instruction(Instruction::new(Transaction {}, Compensation {}))
    .add_instruction(Instruction::new(Transaction {}, Compensation {}));

  println!("{:?}", saga);

  let res = saga.start_blocking();

  println!("{:?}", res);
  println!("{:?}", saga);
}
