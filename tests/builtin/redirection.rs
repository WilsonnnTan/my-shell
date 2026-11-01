#[cfg(test)]
mod tests {
  #[test]
  fn redirection_test() {
    todo!()
  }
}

// write case for:
// echo hi > test.txt; echo hi >test.txt
// > test.txt; nonexist_command > test.txt
// type 1> test.txt
// type 2> test.txt
// echo hi >> test.txt; type 2>> test.txt
// echo hi > nonexist_dir/test.txt