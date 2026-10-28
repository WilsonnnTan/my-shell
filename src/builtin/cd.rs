use super::{Run};
use std::{path::Path, env};

pub struct Cd;

impl Run for Cd {
  fn run(args: Vec<&str>) {
    let Some(&dir_path) = args.get(1) else {
      eprintln!("cd: expected 1 args got 0");
      return;
    };

    let dir_path = Path::new(dir_path);
    match dir_path.is_dir() {
      true => {
        if let Err(e) = env::set_current_dir(dir_path) {
          eprintln!("cd: {}: {}", dir_path.display(), e);
        }
      },
      false => eprintln!("cd: {}: No such file or directory", dir_path.display()),
    }
  }
}