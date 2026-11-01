use super::{Run, OutputExt, command::Command};
use std::process::{Output};

const BUILTIN_COMMANDS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

pub struct Type;

impl Run for Type {
  fn run(args: Vec<String>) -> Output {
    let mut stdout: String = String::new();
    let mut stderr: String = String::new();
    let mut status_code: i32 = 0;

    let Some(command) = args.get(1) else { 
      status_code = 1;
      stderr = format!("type: expected 1 args got 0 \n");
      eprint!("{}", stderr);

      return Output::build_output(status_code, stdout, stderr)
    };

    if BUILTIN_COMMANDS.contains(&command.as_str()) {
      stdout = format!("{} is a shell builtin", command);
    } else if let Some(file_path) = Command::search_executable(command){
      stdout = format!("{} is {}", command, file_path.display());
    } else {
      stdout = format!("{}: not found", command);
    }

    print!("{}", stdout);

    Output::build_output(status_code, stdout, stderr)
  }
}
