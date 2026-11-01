/// This is the module to run the command, process the special operator, and search for executeable
use std::{
  env, 
  path::{Path, PathBuf}, 
  process,
  process::Output, 
  ffi::OsString,
};

#[cfg(unix)]
use std::{fs, os::unix::fs::PermissionsExt};

use crate::builtin::OutputExt;

use super::{
  echo::Echo, 
  exit::Exit, 
  r#type::Type, 
  pwd::Pwd,
  cd::Cd,
  Redirection,
  Token,
  Run
};

use anyhow::{Result, bail, Context};


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
      eprintln!("Could not read $PATH");
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

  pub fn run(args: Vec<Token>) -> Result<()> {
    let mut seperated_command: Vec<String> = Vec::new();
    let mut redirect: Vec<(Redirection, PathBuf)> = Vec::new();

    // convert the Token type into String and Redirection
    // seperate redirection and the actual command and arg
    let mut iter = args.into_iter().peekable();    
    while let Some(arg) = iter.next() {
      match arg {
        Token::Redirection(redir) => {
          // store the Redirection and the next args by taking it as a path file
          // if no path file then we return an error
          match iter.next() {
            Some(Token::Word(path)) => { redirect.push((redir, PathBuf::from(path))) },
            _ => { bail!("Missing file specification after redirection operator"); }
          }
        },
        Token::Word(word) => {
          seperated_command.push(word);
        }
      }
    }

    // return Ok if user doesn't type anything
    let Some(command) = seperated_command.get(0) else { return Ok(()) };

    let mut command_output: Output = Output::build_output(0, String::new(), String::new());

    match command.as_str() {
      "exit" => command_output = Exit::run(seperated_command),
      "echo" => command_output = Echo::run(seperated_command),
      "type" => command_output = Type::run(seperated_command),
      "pwd" => command_output = Pwd::run(seperated_command),
      "cd" => command_output = Cd::run(seperated_command),
      _ => match Self::search_executable(command) {
        Some(file_path) => {
          // Run command with executable file
          command_output = process::Command::new(command)
            .current_dir(file_path.parent().unwrap_or(Path::new(".")))
            .args(&seperated_command[1..])
            .output()
            .context("Command recognized but failed to run")?;
        }
        None => println!("{}: command not found", command),
      }
    }

    let mut iter = redirect.into_iter();

    while let Some((redir, path)) = iter.next() {
      redir.apply(&command_output, path)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::{Command, env};

  #[test]
  fn search_executable_test() {
    let temp_path = tempfile::tempdir().unwrap();

    // set $PATH temporary and restore it at the end
    let old_path = env::var_os("PATH").unwrap_or_default();
    let mut new_path: Vec<std::path::PathBuf> = env::split_paths(&old_path).collect();
    new_path.extend(vec![temp_path.path().to_path_buf()]);
    let new_path = env::join_paths(new_path).unwrap();
    unsafe { env::set_var("PATH", new_path) }

    // create temp exe file
    let file1 = tempfile::Builder::new()
      .prefix("my test exe file")
      .rand_bytes(0)
      .tempfile_in(&temp_path.path())
      .unwrap();

    #[cfg(unix)]
    {
      use std::{fs, os::unix::fs::PermissionsExt};
      fs::set_permissions(file1.path(), fs::Permissions::from_mode(0o111)).unwrap();
    }

    assert_eq!(
      Command::search_executable("my test exe file").unwrap().as_path(),
      file1.path()
    );

    assert!(Command::search_executable("nonexist_command").is_none());

    // restore the original PATH for the test process
    unsafe { env::set_var("PATH", old_path) }
  }
}
