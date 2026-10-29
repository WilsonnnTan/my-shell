use super::{Run};
use std::{env, path::PathBuf};
use anyhow::{Result, Context};

pub struct Cd;

impl Cd {
  #[cfg(windows)]
  fn get_home_var() -> Result<String> {
    return env::var("HOMEPATH").context("failed to read $HOMEPATH variable");
  }

  #[cfg(unix)]
  fn get_home_var() -> Result<String> {
    return env::var("HOME").context("failed to read $HOME variable");
  }
}

impl Run for Cd {
  fn run(args: Vec<String>) {
    let Some(dir_path) = args.get(1) else {
      eprintln!("cd: expected 1 args got 0");
      return;
    };

    let dir_path = match dir_path.as_str() {
      "~" => match Self::get_home_var() {
        Ok(home_dir) => PathBuf::from(home_dir),
        Err(e) => {
          eprintln!("cd: {}", e);
          return;
        },
      },
      _ => PathBuf::from(dir_path),
    };

    match dir_path.is_dir() {
      true => {
        if let Err(e) = env::set_current_dir(&dir_path) {
          eprintln!("cd: {}: {}", dir_path.display(), e);
        }
      },
      false => eprintln!("cd: {}: No such file or directory", dir_path.display()),
    }
  }
}

#[cfg(test)]
mod tests {
  use predicates::str::contains;

  #[test]
  #[cfg(windows)]
  fn cd_home_test() {
    let home = tempfile::tempdir().unwrap();
    let not_home = tempfile::tempdir().unwrap();

    assert_cmd::Command::cargo_bin("codecrafters-shell").unwrap()
      .current_dir(not_home.path())
      .env("HOME", home.path())
      .env("HOMEPATH", home.path())
      .write_stdin("cd ~\npwd\nexit")
      .assert()
      .success()
      .stdout(contains(home.path().display().to_string()));
  }
}