use super::{Run, command::Command};

const BUILTIN_COMMANDS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

pub struct Type;

impl Run for Type {
  fn run(args: Vec<String>) {
    let Some(command) = args.get(1) else { 
      eprintln!("type: expected 1 args got 0");
      return;
    };

    if BUILTIN_COMMANDS.contains(&command.as_str()) {
      println!("{} is a shell builtin",command)
    } else if let Some(file_path) = Command::search_executable(command){
      println!("{} is {}", command, file_path.display());
    } else {
      println!("{}: not found", command);
    }
  }
}

#[cfg(test)]
mod tests {
  use predicates::str::contains;

  #[test]
  fn expect_atleast_1_argument() {
    assert_cmd::Command::cargo_bin("codecrafters-shell").unwrap()
      .write_stdin("type\nexit")
      .assert()
      .success()
      .stderr(contains("type: expected 1 args got 0"));
  }

  #[test]
  fn type_builtin_test() {
    assert_cmd::Command::cargo_bin("codecrafters-shell").unwrap()
      .write_stdin("type echo\nexit")
      .assert()
      .success()
      .stdout(contains("echo is a shell builtin"));
  }

  #[test]
  fn type_not_found_test() {
    assert_cmd::Command::cargo_bin("codecrafters-shell").unwrap()
      .write_stdin("type unregistered_command\nexit")
      .assert()
      .success()
      .stdout(contains("unregistered_command: not found"));
  }

  // implement test for searching executable
}