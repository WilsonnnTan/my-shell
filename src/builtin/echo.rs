use std::process::Output;
use super::{Run, OutputExt};


pub struct Echo;

impl Run for Echo {
  fn run(args: Vec<String>) -> Output {
    let stdout = format!("{}\n", args[1..].join(" "));
    let stderr = "".to_string();
    let status_code: i32 = 0;

    print!("{}", stdout);

    Output::build_output(status_code, stdout, stderr)
  }
}
