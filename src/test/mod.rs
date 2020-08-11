use crate::tasks::instruction::Instruction;
use crate::tasks::saga::AbortType;
use crate::tasks::saga::Saga;

pub fn test_main() {
  let mut saga = Saga::new(AbortType::Backward).add_instruction(Instruction {});

  saga.add_instruction(Instruction {});

  println!("{:?}", saga);
}
