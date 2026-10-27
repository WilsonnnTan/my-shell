#[allow(unused_imports)]
use std::io::{self, Write};
use my_shell::run_command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let command: Vec<&str> = command.trim().split_whitespace().collect();
        run_command(command);
    }
}
