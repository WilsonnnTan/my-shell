use anyhow::{Result, bail};

pub struct Parser;

impl Parser {
  pub fn parse(args: String) -> Result<Vec<String>> {
    let mut parsed_args: Vec<String> = Vec::new();
    let mut arg: String = String::new();
    let mut in_single_quote: bool = false;

    for c in args.chars() {
      match c {
        '\'' => {
          in_single_quote = !in_single_quote;
        },
        c if c.is_whitespace() && !in_single_quote => {
          if arg.len() > 0 {
            parsed_args.push(arg);
            arg = String::new();
          }
        },
        c => {
          arg.push(c);
        },
      }
    }

    if in_single_quote {
      bail!("uncompleted single quote");
    }

    if arg.len() > 0 {
      parsed_args.push(arg);
    }
    
    Ok(parsed_args)
  }
}

#[cfg(test)]
mod tests {
  mod single_quotes {
    use super::super::Parser;

    #[test]
    fn spaces_are_preserved_within_quotes() {
      assert_eq!(
        Parser::parse(String::from("echo 'hello    world'")).unwrap(),
        vec!["echo", "hello    world"]
      );
    }

    #[test]
    fn consecutive_spaces_are_collapsed_unless_quoted() {
      assert_eq!(
        Parser::parse(String::from("echo hello    world")).unwrap(),
        vec!["echo", "hello", "world"]
      );
    }

    #[test]
    fn adjacent_quoted_strings_with_space() {
      assert_eq!(
        Parser::parse(String::from("echo 'hello' 'world'")).unwrap(),
        vec!["echo", "hello", "world"]
      );
    }

    #[test]
    fn adjacent_quoted_strings_are_concatenated() {
      assert_eq!(
        Parser::parse(String::from("echo 'hello''world'")).unwrap(),
        vec!["echo", "helloworld"]
      );
    }

    #[test]
    fn adjacent_quoted_and_unquoted_strings_are_concatenated() {
      assert_eq!(
        Parser::parse(String::from("echo 'hello'world")).unwrap(),
        vec!["echo", "helloworld"]
      );
    }

    #[test]
    fn empty_quotes_are_ignored() {
      assert_eq!(
        Parser::parse(String::from("echo hello''world")).unwrap(),
        vec!["echo", "helloworld"]
      );
    }

    #[test]
    #[should_panic(expected = "uncompleted single quote")]
    fn uncompleted_single_quote() {
      Parser::parse(String::from("echo hello'world")).unwrap();
    }
  }

  
  mod double_quotes {
    use super::super::Parser;

    #[test]
    fn spaces_are_preserved_within_quotes() {
      assert_eq!(
        Parser::parse(String::from(r#"echo "hello    world""#)).unwrap(),
        vec!["echo", "hello    world"]
      );
    }

    #[test]
    fn consecutive_spaces_are_collapsed_unless_quoted() {
      assert_eq!(
        Parser::parse(String::from("echo hello    world")).unwrap(),
        vec!["echo", "hello", "world"]
      );
    }

    #[test]
    fn adjacent_quoted_strings_with_space() {
      assert_eq!(
        Parser::parse(String::from(r#"echo "hello" "world""#)).unwrap(),
        vec!["echo", "hello", "world"]
      );
    }

    #[test]
    fn adjacent_quoted_strings_are_concatenated() {
      assert_eq!(
        Parser::parse(String::from(r#"echo "hello""world""#)).unwrap(),
        vec!["echo", "helloworld"]
      );
    }

    #[test]
    fn adjacent_quoted_and_unquoted_strings_are_concatenated() {
      assert_eq!(
        Parser::parse(String::from(r#"echo "hello"world"#)).unwrap(),
        vec!["echo", "helloworld"]
      );
    }

    #[test]
    fn empty_quotes_are_ignored() {
      assert_eq!(
        Parser::parse(String::from(r#"echo hello""world"#)).unwrap(),
        vec!["echo", "helloworld"]
      );
    }

    #[test]
    fn character_within_double_quotes_are_treated_literally() {
      assert_eq!(
        Parser::parse(String::from(r#"echo "shell's test""#)).unwrap(),
        vec!["echo", "shell's test"]
      );
    }

    #[test]
    #[should_panic(expected = "uncompleted single quote")]
    fn uncompleted_single_quote() {
      Parser::parse(String::from("echo hello'world")).unwrap();
    }
  }
}