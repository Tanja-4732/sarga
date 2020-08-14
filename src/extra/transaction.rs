#[derive(Debug)]
pub struct Transaction {}

impl Transaction {
  pub fn run(&self) -> Result<&Self, ()> {
    Ok(self)
  }
}
