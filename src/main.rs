#[allow(unused_imports)]
use std::io::{self, Write};
use my_shell::builtin::{
    command::Command,
    parser::Parser,
    Token,
};
use anyhow::Result;

fn main() -> Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush()?;

        let mut args: String = String::new();
        io::stdin().read_line(&mut args)?;
        // use parser from library instead of hardcode on main.rs
        let args: Vec<Token> = Parser::parse(args).unwrap_or_else( |e| {
            eprintln!("failed to parse: {}", e);
            Vec::new()
        });
        
        Command::run(args).unwrap_or_else(|e| {
            eprintln!("failed to run command: {}", e);
        });
    }
}
