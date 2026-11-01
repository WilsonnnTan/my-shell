use super::{Run, OutputExt};
use std::{env, process::Output};

pub struct Pwd;

impl Run for Pwd {
  fn run(_: Vec<String>) -> Output {
    let mut stdout: String = String::new();
    let mut stderr: String = String::new();
    let mut status_code: i32 = 0;

    match env::current_dir() {
      Ok(dir) => {
        stdout = format!("{}\n", dir.display());
        print!("{}", stdout);
      },
      Err(e) => {
        status_code = 1;
        stderr = format!("pwd: failed to run pwd: {e}\n");
        eprint!("{}", stderr);
      }
    }

    Output::build_output(status_code, stdout, stderr)
  }
}
