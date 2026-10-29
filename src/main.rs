#[allow(unused_imports)]
use std::io::{self, Write};
use my_shell::builtin::{
    command::Command,
    Run,
    parser::Parser
};
use anyhow::Result;

fn main() -> Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush()?;

        let mut args: String = String::new();
        io::stdin().read_line(&mut args)?;
        // use parser from library instead of hardcode on main.rs
        let args: Vec<String> = Parser::parse(args).unwrap();
        Command::run(args);
    }
}
