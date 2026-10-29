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