use super::{Run, command::Command};

const BUILTIN_COMMANDS: [&str; 3] = ["echo", "exit", "type"];

pub struct Type;

impl Run for Type {
  fn run(args: Vec<&str>) {
    let Some(&command) = args.get(1) else { return };

    if BUILTIN_COMMANDS.contains(&command) {
      println!("{} is a shell builtin",command)
    } else if let Some(file_path) = Command::search_executable(&command){
      println!("{} is {}", command, file_path.display());
    } else {
      println!("{}: not found", command);
    }
  }
}