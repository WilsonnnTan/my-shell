pub mod command;
pub mod echo;
pub mod exit;
pub mod r#type;
pub mod pwd;

pub trait Run {
  fn run(args: Vec<&str>);
}
