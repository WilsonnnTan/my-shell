use super::{Run, OutputExt};
use std::{env, path::PathBuf, process::Output};
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
  fn run(args: Vec<String>) -> Output {
    let stdout: String = String::new();
    let mut stderr: String = String::new();
    let mut status_code: i32 = 0;

    let Some(dir_path) = args.get(1) else {
      status_code = 1;
      stderr = format!("cd: expected 1 args got 0");
      eprint!("{}", stderr);
      
      return Output::build_output(status_code, stdout, stderr)
    };

    let dir_path = match dir_path.as_str() {
      "~" => match Self::get_home_var() {
        Ok(home_dir) => PathBuf::from(home_dir),
        Err(e) => {
          status_code = 1;
          stderr = format!("cd: {}", e);
          eprint!("{}", stderr);

          return Output::build_output(status_code, stdout, stderr)
        },
      },
      _ => PathBuf::from(dir_path),
    };

    match dir_path.is_dir() {
      true => {
        if let Err(e) = env::set_current_dir(&dir_path) {
          status_code = 1;
          stderr = format!("cd: {}: {}", dir_path.display(), e);
          eprint!("{}", stderr);
        }
      },
      false => {
        status_code = 1;
        stderr = format!("cd: {}: No such file or directory", dir_path.display());
        eprint!("{}", stderr);
      }
    }

    return Output::build_output(status_code, stdout, stderr)
  }
}
