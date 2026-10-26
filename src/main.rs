#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let command = command.trim().to_string();
        if command == "exit" {
            break
        }
        println!("{command}: command not found")
    }
}
