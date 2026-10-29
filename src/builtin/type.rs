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
