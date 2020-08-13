#[derive(Debug)]
pub struct Transaction {}

impl Transaction {
  pub fn run_blocking(&self) -> Result<&Self, ()> {
    Ok(self)
  }
}
