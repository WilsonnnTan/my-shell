use anyhow::{Result, bail};

pub struct Parser;

impl Parser {
  pub fn parse(args: String) -> Result<Vec<String>> {
    let mut parsed_args: Vec<String> = Vec::new();
    let mut arg: String = String::new();
    let mut in_single_quote: bool = false;
    let mut in_double_quote: bool = false;
    let mut is_escaped_char: bool = false;

    for c in args.chars() {
      match c {
        c if is_escaped_char => {
          // unhandled special characters (example: echo "\n" -> \n)
          if in_double_quote && matches!(c, '$' | 'n' | '`') { arg.push('\\') }
          arg.push(c);
          is_escaped_char = !is_escaped_char;
        },
        c if c.is_whitespace() && !in_single_quote && !in_double_quote => {
          if arg.len() > 0 {
            parsed_args.push(arg);
            arg = String::new();
          }
        },
        '\'' => {
          if in_double_quote { arg.push(c) } 
          else { in_single_quote = !in_single_quote }
        },
        '\"' => {
          if in_single_quote { arg.push(c) }
          else { in_double_quote = !in_double_quote }
        },
        '\\' if !in_single_quote => { is_escaped_char = !is_escaped_char },
        c => { arg.push(c) },
      }
    }

    if in_single_quote { bail!("unterminated single quote") }
    if in_double_quote { bail!("unterminated double quote") }

    if is_escaped_char { arg.push('\\') } // trailing backslash get printed
    if arg.len() > 0 { parsed_args.push(arg) }
    
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
    #[should_panic(expected = "unterminated single quote")]
    fn unterminated_single_quote() {
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
    #[should_panic(expected = "unterminated double quote")]
    fn unterminated_double_quote() {
      Parser::parse(String::from(r#"echo hello"world"#)).unwrap();
    }
  }

  
  mod backslash_outside_quotes {
    use super::super::Parser;

    #[test]
    fn each_backslash_creates_literal_space_as_part_of_one_argument() {
      assert_eq!(
        Parser::parse(String::from(r#"echo three\ \ \ spaces"#)).unwrap(),
        vec!["echo", "three   spaces"]
      );
    }

    #[test]
    fn backslash_preserves_the_first_space_but_parser_collapses_the_subsequent_unescaped_spaces() {
      assert_eq!(
        Parser::parse(String::from(r#"echo before\     after"#)).unwrap(),
        vec!["echo", "before ", "after"]
      );
    }

    #[test]
    fn backslash_n_just_become_n() {
      assert_eq!(
        Parser::parse(String::from(r#"echo test\nexample"#)).unwrap(),
        vec!["echo", "testnexample"]
      );
    }

    #[test]
    fn backslash_convert_any_special_character_into_literal() {
      assert_eq!(
        Parser::parse(String::from(r#"echo \'hello\"\$\*\?\\"#)).unwrap(),
        vec!["echo", r#"'hello"$*?\"#]
      );
    }

    #[test]
    fn trailing_backslash() {
      assert_eq!(
        Parser::parse(String::from(r#"echo \"#)).unwrap(),
        vec!["echo", r#"\"#]
      );
    }
  }


  mod backslash_within_single_quotes {
    use super::super::Parser;

    #[test]
    fn backslash_in_single_quote() {
      assert_eq!(
        Parser::parse(String::from(r#"echo 'shell\\"\nscript'"#)).unwrap(),
        vec!["echo", r#"shell\\"\nscript"#]
      );
    }
  }


  mod backslash_within_double_quotes {
    use super::super::Parser;

    #[test]
    fn handled_special_characters() {
      assert_eq!(
        Parser::parse(String::from(r#"echo "shell\\\"script""#)).unwrap(),
        vec!["echo", r#"shell\"script"#]
      );
    }
    
    #[test]
    fn unhandled_special_characters() {
      assert_eq!(
        Parser::parse(String::from(r#"echo "shell\$\n\`script""#)).unwrap(),
        vec!["echo", r#"shell\$\n\`script"#]
      );
    }
  }
}