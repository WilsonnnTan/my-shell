use std::{
  process::Output,
  path::PathBuf,
  fs::OpenOptions,
  io::Write,
};

use anyhow::Result;

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

  pub fn apply(&self, output: &Output, file_path: PathBuf) -> Result<()> {
    let mut file_open_options = OpenOptions::new();

    match self.redirect_kind {
      RedirectKind::Read => { file_open_options.read(true); },
      RedirectKind::Write => { file_open_options.create(true).truncate(true).write(true); },
      RedirectKind::Append => { file_open_options.create(true).append(true); },
    }

    let mut f = file_open_options.open(file_path)?;

    match self.file_descriptor {
      FileDescriptor::Stdout => { f.write_all(&output.stdout)? },
      FileDescriptor::Stderr => { f.write_all(&output.stderr)? },
      _ => {},
    }

    Ok(())
  }
}

// write unittest for apply