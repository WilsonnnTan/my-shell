mod builtin;
use builtin::{Run, echo::Echo, exit::Exit, r#type::Type};

pub fn run_command(command: Vec<&str>) {
  match command[0] {
    "exit" => Exit::run(&command),
    "echo" => Echo::run(&command),
    "type" => Type::run(&command),
    _ => println!("{}: command not found", command[0]),
  }
}