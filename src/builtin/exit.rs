use std::process::Output;
use super::Run;

pub struct Exit;

impl Run for Exit {
  fn run(_args: Vec<String>) -> Output {
    std::process::exit(0);
  }
}
