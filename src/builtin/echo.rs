use super::Run;

pub struct Echo;

impl Run for Echo {
  fn run(args: &[&str]) {
    println!("{}", args[1..].join(" "));
  }
}