use std::{env, path::PathBuf};

#[cfg(unix)]
use std::{fs, os::unix::fs::PermissionsExt};

use super::{echo::Echo, exit::Exit, r#type::Type, Run};

pub struct Command;

impl Command {
  // check file execute permission
  #[cfg(unix)]
  fn is_executeable (file_path: &PathBuf) -> bool {
    let mode = fs::metadata(file_path).unwrap().permissions().mode();
    if mode & 0o111 != 0 {
      return true;
    }
    false
  }

  // windows doesnt need to check execute permission
  #[cfg(windows)]
  fn is_executeable (_: &PathBuf) -> bool {
    true
  }

  // Search executable in PATH
  pub fn search_executable(command: &str) -> Option<PathBuf>{
    // use OsString to support all Os buf
    let path_var= env::var_os("PATH").unwrap();
    let paths= env::split_paths(&path_var);

    for path in paths {
      let file_path = path.join(command);
      if file_path.is_file() && Self::is_executeable(&file_path) {
        return Some(file_path);
      }
    }

    None
  }
}

impl Run for Command {
  fn run(args: Vec<&str>) {
    // no args case
    let Some(&command) = args.get(0) else { return };

    match command {
      "exit" => Exit::run(args),
      "echo" => Echo::run(args),
      "type" => Type::run(args),
      _ => match Self::search_executable(command) {
        Some(_) => {
          // Run command with executable file
        }
        None => println!("{}: command not found", command),
      }
    }
  }
}
