use super::{Run};
use std::{env};

pub struct Pwd;

impl Run for Pwd {
  fn run(_: Vec<String>) {
    match env::current_dir() {
      Ok(dir) => println!("{}", dir.display()),
      Err(e) => {
        eprintln!("pwd: failed to run pwd: {}", e);
        return;
      }
    }
  }
}