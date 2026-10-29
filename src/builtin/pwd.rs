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

#[cfg(test)]
mod tests {
  use predicates::str::contains;

  #[test]
  fn pwd_test() {
    let dir = tempfile::tempdir().unwrap();
    let expected = dir.path();
    assert_cmd::Command::cargo_bin("codecrafters-shell").unwrap()
      .current_dir(expected)
      .write_stdin("pwd\nexit")
      .assert()
      .success()
      .stdout(contains(expected.display().to_string()));
  }
}