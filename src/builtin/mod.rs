pub mod parser;
pub mod command;
pub mod echo;
pub mod exit;
pub mod r#type;
pub mod pwd;
pub mod cd;
pub mod redirection;

use std::process::{Output, ExitStatus};
use crate::builtin::redirection::{
  Redirection,
};

// implement exit status porter
#[cfg(windows)]
pub fn exit_status(code: i32) -> ExitStatus{
  use std::os::windows::process::ExitStatusExt;
  ExitStatus::from_raw(code as u32)
}

#[cfg(unix)]
pub fn exit_status(code: i32) -> ExitStatus{
  use std::os::unix::process::ExitStatusExt;
  ExitStatus::from_raw(code << 8)
}

// Implement Output constructor for std::process::Output
trait OutputExt {
  fn build_output(status_code: i32, stdout: String, stderr: String) -> Self;
}

impl OutputExt for Output {
  fn build_output(status_code: i32, stdout: String, stderr: String) -> Self {
    Self {
      status: exit_status(status_code),
      stdout: stdout.into_bytes(),
      stderr: stderr.into_bytes(),
    }
  }
}

// Token for parser
#[derive(Debug, PartialEq)]
pub enum Token {
  Word(String),
  Redirection(Redirection),
}

pub trait Run {
  fn run(args: Vec<String>) -> Output;
}
