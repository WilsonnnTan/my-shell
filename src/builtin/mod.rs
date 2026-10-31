pub mod parser;
pub mod command;
pub mod echo;
pub mod exit;
pub mod r#type;
pub mod pwd;
pub mod cd;
pub mod redirection;

use crate::builtin::redirection::{
  Redirection,
};

#[derive(Debug, PartialEq)]
pub enum Token {
  Word(String),
  Redirection(Redirection),
}

pub trait Run {
  fn run(args: Vec<String>);
}
