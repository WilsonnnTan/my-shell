use super::Run;

pub struct Exit;

impl Run for Exit {
  fn run(_args: Vec<&str>) {
    std::process::exit(0);
  }
}