use super::Run;

pub struct Echo;

impl Run for Echo {
  fn run(args: Vec<String>) {
    println!("{}", args[1..].join(" "));
  }
}