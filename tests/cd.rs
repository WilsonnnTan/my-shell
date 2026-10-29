#[cfg(test)]
mod tests {
  use predicates::str::contains;

  #[test]
  #[cfg(windows)]
  fn cd_home_test() {
    let home = tempfile::tempdir().unwrap();
    let not_home = tempfile::tempdir().unwrap();

    assert_cmd::Command::cargo_bin("codecrafters-shell").unwrap()
      .current_dir(not_home.path())
      .env("HOME", home.path())
      .env("HOMEPATH", home.path())
      .write_stdin("cd ~\npwd\nexit")
      .assert()
      .success()
      .stdout(contains(home.path().display().to_string()));
  }
}