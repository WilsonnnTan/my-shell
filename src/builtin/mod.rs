pub mod command;
pub mod echo;
pub mod exit;
pub mod r#type;

pub trait Run {
  fn run(args: Vec<&str>);
}
