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