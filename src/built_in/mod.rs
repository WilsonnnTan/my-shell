pub mod echo;
pub mod exit;

pub trait Run {
  fn run(args: &[&str]);
}