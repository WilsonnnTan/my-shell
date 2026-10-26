#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    while true {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let command = command.trim();
        println!("{command}: command not found")
    }
}
