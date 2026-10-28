#[allow(unused_imports)]
use std::io::{self, Write};
use my_shell::builtin::{command::Command, Run};
use anyhow::Result;

fn main() -> Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush()?;

        let mut args: String = String::new();
        io::stdin().read_line(&mut args)?;

        let args: Vec<&str> = args.trim().split_whitespace().collect();
        Command::run(args);
    }
}
