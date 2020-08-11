use crate::tasks::saga::Saga;

pub fn test_main() {
  let saga = Saga::new();

  println!("{:?}", saga);
}
