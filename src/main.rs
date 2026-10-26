#[allow(unused_imports)]
use std::io::{self, Write};
use my_shell::built_in::{Run, echo::Echo, exit::Exit};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let command: Vec<&str> = command.trim().split_whitespace().collect();

        // refactor later using match on lib.rs
        if command[0] == "exit" {
            Exit::run(&command);
        }
        else if command[0] == "echo" {
            Echo::run(&command);
        }
        else {
            println!("{}: command not found", command[0])
        }
    }
}
