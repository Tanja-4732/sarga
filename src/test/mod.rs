use crate::core::instruction::Instruction;
use crate::core::saga::AbortType;
use crate::core::saga::Saga;
// use crate::extra::compensation::Compensation;
// use crate::extra::transaction::Transaction;

// #[tokio::main]
pub async fn test_main() {
  let mut saga = Saga::new(AbortType::Backward)
    .add_instruction(Instruction::new(|| Ok(()), || ()))
    .add_instruction(Instruction::new(|| Ok(()), || ()))
    .add_instruction(Instruction::new(|| Ok(()), || ()))
    .add_instruction(Instruction::new(|| Ok(()), || ()));

  println!("{:?}", saga);

  let res = saga.start();

  println!("{:?}", res);
  println!("{:?}", saga);
}
