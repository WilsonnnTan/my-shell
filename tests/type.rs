#[cfg(test)]
mod tests {
  use predicates::str::contains;

  #[test]
  fn expect_atleast_1_argument() {
    assert_cmd::Command::cargo_bin("codecrafters-shell").unwrap()
      .write_stdin("type\nexit")
      .assert()
      .success()
      .stderr(contains("type: expected 1 args got 0"));
  }

  #[test]
  fn type_builtin_test() {
    assert_cmd::Command::cargo_bin("codecrafters-shell").unwrap()
      .write_stdin("type echo\nexit")
      .assert()
      .success()
      .stdout(contains("echo is a shell builtin"));
  }

  #[test]
  fn type_not_found_test() {
    assert_cmd::Command::cargo_bin("codecrafters-shell").unwrap()
      .write_stdin("type unregistered_command\nexit")
      .assert()
      .success()
      .stdout(contains("unregistered_command: not found"));
  }
}