use std::fs::File;

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

#[derive(Debug, PartialEq)]
pub struct Redirection {
  file_descriptor: FileDescriptor,
  redirect_kind: RedirectKind,
}

impl Redirection {
  pub fn new(file_descriptor: FileDescriptor, redirect_kind: RedirectKind) -> Self {
    Self { file_descriptor, redirect_kind }
  }
}
