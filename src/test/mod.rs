use crate::core::compensation::Compensation;
use crate::core::instruction::Instruction;
use crate::core::saga::AbortType;
use crate::core::saga::Saga;
use crate::core::transaction::Transaction;

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
