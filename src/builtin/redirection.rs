use std::{
  process::Output,
  path::PathBuf,
};

#[derive(PartialEq, Debug)]
#[repr(u8)]
pub enum FileDescriptor { 
  Stdin = 0,
  Stdout = 1,
  Stderr = 2,
}

#[derive(PartialEq, Debug)]
pub enum RedirectKind {
  Read = 0,
  Write = 1,
  Append = 2,
}

// the reason I dont include PathBuf in Redirection because
// redirection operator is a standalone and there is also a case where
// "missing file specification after redirection operator" error
#[derive(Debug, PartialEq)]
pub struct Redirection {
  file_descriptor: FileDescriptor,
  redirect_kind: RedirectKind,
}

impl Redirection {
  pub fn new(file_descriptor: FileDescriptor, redirect_kind: RedirectKind) -> Self {
    Self { file_descriptor, redirect_kind }
  }

  pub fn apply(&self, output: Output, file_path: PathBuf) {
    todo!()
  }
}
