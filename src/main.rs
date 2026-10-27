#[allow(unused_imports)]
use std::io::{self, Write};
use my_shell::builtin::{command::Command, Run};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut args: String = String::new();
        io::stdin().read_line(&mut args).unwrap();

        let args: Vec<&str> = args.trim().split_whitespace().collect();
        Command::run(args);
    }
}
