use super::Run;

pub struct Exit;

impl Run for Exit {
  fn run(_args: Vec<String>) {
    std::process::exit(0);
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn exit_test() {
    assert_cmd::Command::cargo_bin("codecrafters-shell").unwrap()
    .write_stdin("exit")
    .assert()
    .success();
  }
}