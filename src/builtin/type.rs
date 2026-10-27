use super::Run;

pub struct Type;

impl Run for Type {
  fn run(args: &[&str]) {
    let built_in_commands = ["echo", "exit", "type"];

    match args.get(1) {
      None => {}
      Some(command) => {
        if built_in_commands.contains(command) {
          println!("{} is a shell builtin",command)
        } else {
          println!("{}: not found", command)
        }
      }
    }
  }
}