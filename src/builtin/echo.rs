use super::Run;

pub struct Echo;

impl Run for Echo {
  fn run(args: Vec<String>) {
    println!("{}", args[1..].join(" "));
  }
}

#[cfg(test)]
mod tests {
  use predicates::str::contains;

  #[test]
  fn echo_test() {
    assert_cmd::Command::cargo_bin("codecrafters-shell").unwrap()
      .write_stdin("echo hello world\nexit")
      .assert()
      .success()
      .stdout(contains("hello world"));
  }
}