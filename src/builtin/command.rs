use std::{
  env, 
  path::{Path, PathBuf}, 
  process, 
  ffi::OsString
};

#[cfg(unix)]
use std::{fs, os::unix::fs::PermissionsExt};

use super::{
  echo::Echo, 
  exit::Exit, 
  r#type::Type, 
  pwd::Pwd,
  cd::Cd,
  Run
};

pub struct Command;

impl Command {
  // check file execute permission
  #[cfg(unix)]
  fn is_executeable (file_path: &PathBuf) -> bool {
    match fs::metadata(file_path) {
      Ok(meta) => meta.permissions().mode() & 0o111 != 0,
      Err(e) => {
        eprintln!("Failed to read {} permission {}", file_path.display(), e);
        false
      }
    }
  }

  // windows doesnt need to check execute permission
  #[cfg(windows)]
  fn is_executeable (_: &PathBuf) -> bool {
    true
  }

  // Search executable in PATH
  pub fn search_executable(command: &str) -> Option<PathBuf>{
    // use OsString to support all Os buf
    let path_var= env::var_os("PATH").unwrap_or_else( || {
      eprintln!("could not read $PATH");
      OsString::new()
    });
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
      "pwd" => Pwd::run(args),
      "cd" => Cd::run(args),
      _ => match Self::search_executable(command) {
        Some(file_path) => {
          // Run command with executable file
          let out = process::Command::new(command)
            .current_dir(file_path.parent().unwrap_or(Path::new(".")))
            .args(&args[1..])
            .output()
            .expect("Exectuable exist but failed to run");

          print!("{}", String::from_utf8_lossy(&out.stdout));
          eprint!("{}", String::from_utf8_lossy(&out.stderr));
        }
        None => println!("{}: command not found", command),
      }
    }
  }
}
