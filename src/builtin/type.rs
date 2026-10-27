use super::Run;

pub struct Type;

impl Run for Type {
  fn run(args: &[&str]) {
    let built_in_commands = ["echo", "exit", "type"];

    if built_in_commands.contains(&args[1]) {
      println!("{} is a shell builtin", args[1])
    } else {
      println!("{}: not found", args[1])
    }
  }
}