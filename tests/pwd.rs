#[cfg(test)]
mod tests {
  use predicates::str::contains;

  #[test]
  fn pwd_test() {
    let dir = tempfile::tempdir().unwrap();
    let expected = dir.path();
    assert_cmd::Command::cargo_bin("codecrafters-shell").unwrap()
      .current_dir(expected)
      .write_stdin("pwd\nexit")
      .assert()
      .success()
      .stdout(contains(expected.display().to_string()));
  }
}