use anyhow::{Result, bail};
use crate::builtin::redirection::{
  FileDescriptor,
  RedirectKind,
  Redirection,
};
use super::Token;

pub struct Parser;

impl Parser {
  pub fn parse(args: String) -> Result<Vec<Token>> {
    let mut parsed_args: Vec<Token> = Vec::new();
    let mut arg: String = String::new();
    let mut in_single_quote: bool = false;
    let mut in_double_quote: bool = false;
    let mut is_escaped_char: bool = false;

    // use peekable iterator to parse '>' operator
    let mut chars = args.chars().peekable();
    while let Some(c) = chars.next() {
      match c {
        c if is_escaped_char => {
          // unhandled special characters (example: echo "\n" -> \n)
          if in_double_quote && matches!(c, '$' | 'n' | '`') { arg.push('\\') }
          arg.push(c);
          is_escaped_char = !is_escaped_char;
        },
        c if c.is_whitespace() && !in_single_quote && !in_double_quote => {
          if arg.len() > 0 {
            parsed_args.push(Token::Word(arg));
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
        '>' if !in_single_quote && !in_double_quote && arg.len() <= 1 => {
          let mut fd: FileDescriptor = FileDescriptor::Stdout;
          let mut redirect_kind: RedirectKind = RedirectKind::Write;

          match arg.chars().nth(0) {
            // cases where there is fd before '>'
            Some(idx_0) => {
              if matches!(idx_0, '0' | '1' | '2') { arg = String::new() }

              if idx_0 == '0' { fd = FileDescriptor::Stdin }
              else if idx_0 == '1' { fd = FileDescriptor::Stdout }
              else if idx_0 == '2' { fd = FileDescriptor::Stderr }
            },
            // cases for no fd (default fd to Stdout)
            None => {
              if chars.peek() == Some(&'>') {
                redirect_kind = RedirectKind::Append;
                chars.next();
              }
            }
          }

          parsed_args.push(Token::Redirection(Redirection::new(fd, redirect_kind)));
        },
        c => { arg.push(c) },
      }
    }

    if in_single_quote { bail!("unterminated single quote") }
    if in_double_quote { bail!("unterminated double quote") }

    if is_escaped_char { arg.push('\\') } // trailing backslash get printed
    if arg.len() > 0 { parsed_args.push(Token::Word(arg)) }
    
    Ok(parsed_args)
  }
}

#[cfg(test)]
mod tests {
  mod single_quotes {
    use super::super::{Parser, Token};

    #[test]
    fn spaces_are_preserved_within_quotes() {
      assert_eq!(
        Parser::parse(String::from("echo 'hello    world'")).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("hello    world".to_string())]
      );
    }

    #[test]
    fn consecutive_spaces_are_collapsed_unless_quoted() {
      assert_eq!(
        Parser::parse(String::from("echo hello    world")).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("hello".to_string()), Token::Word("world".to_string())]
      );
    }

    #[test]
    fn adjacent_quoted_strings_with_space() {
      assert_eq!(
        Parser::parse(String::from("echo 'hello' 'world'")).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("hello".to_string()), Token::Word("world".to_string())]
      );
    }

    #[test]
    fn adjacent_quoted_strings_are_concatenated() {
      assert_eq!(
        Parser::parse(String::from("echo 'hello''world'")).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("helloworld".to_string())]
      );
    }

    #[test]
    fn adjacent_quoted_and_unquoted_strings_are_concatenated() {
      assert_eq!(
        Parser::parse(String::from("echo 'hello'world")).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("helloworld".to_string())]
      );
    }

    #[test]
    fn empty_quotes_are_ignored() {
      assert_eq!(
        Parser::parse(String::from("echo hello''world")).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("helloworld".to_string())]
      );
    }

    #[test]
    #[should_panic(expected = "unterminated single quote")]
    fn unterminated_single_quote() {
      Parser::parse(String::from("echo hello'world")).unwrap();
    }
  }

  
  mod double_quotes {
    use super::super::{Parser, Token};

    #[test]
    fn spaces_are_preserved_within_quotes() {
      assert_eq!(
        Parser::parse(String::from(r#"echo "hello    world""#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("hello    world".to_string())]
      );
    }

    #[test]
    fn consecutive_spaces_are_collapsed_unless_quoted() {
      assert_eq!(
        Parser::parse(String::from("echo hello    world")).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("hello".to_string()), Token::Word("world".to_string())]
      );
    }

    #[test]
    fn adjacent_quoted_strings_with_space() {
      assert_eq!(
        Parser::parse(String::from(r#"echo "hello" "world""#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("hello".to_string()), Token::Word("world".to_string())]
      );
    }

    #[test]
    fn adjacent_quoted_strings_are_concatenated() {
      assert_eq!(
        Parser::parse(String::from(r#"echo "hello""world""#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("helloworld".to_string())]
      );
    }

    #[test]
    fn adjacent_quoted_and_unquoted_strings_are_concatenated() {
      assert_eq!(
        Parser::parse(String::from(r#"echo "hello"world"#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("helloworld".to_string())]
      );
    }

    #[test]
    fn empty_quotes_are_ignored() {
      assert_eq!(
        Parser::parse(String::from(r#"echo hello""world"#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("helloworld".to_string())]
      );
    }

    #[test]
    fn character_within_double_quotes_are_treated_literally() {
      assert_eq!(
        Parser::parse(String::from(r#"echo "shell's test""#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("shell's test".to_string())]
      );
    }

    #[test]
    #[should_panic(expected = "unterminated double quote")]
    fn unterminated_double_quote() {
      Parser::parse(String::from(r#"echo hello"world"#)).unwrap();
    }
  }

  
  mod backslash_outside_quotes {
    use super::super::{Parser, Token};

    #[test]
    fn each_backslash_creates_literal_space_as_part_of_one_argument() {
      assert_eq!(
        Parser::parse(String::from(r#"echo three\ \ \ spaces"#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("three   spaces".to_string())]
      );
    }

    #[test]
    fn backslash_preserves_the_first_space_but_parser_collapses_the_subsequent_unescaped_spaces() {
      assert_eq!(
        Parser::parse(String::from(r#"echo before\     after"#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("before ".to_string()), Token::Word("after".to_string())]
      );
    }

    #[test]
    fn backslash_n_just_become_n() {
      assert_eq!(
        Parser::parse(String::from(r#"echo test\nexample"#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("testnexample".to_string())]
      );
    }

    #[test]
    fn backslash_convert_any_special_character_into_literal() {
      assert_eq!(
        Parser::parse(String::from(r#"echo \'hello\"\$\*\?\\"#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word(r#"'hello"$*?\"#.to_string())]
      );
    }

    #[test]
    fn trailing_backslash() {
      assert_eq!(
        Parser::parse(String::from(r#"echo \"#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word(r#"\"#.to_string())]
      );
    }
  }


  mod backslash_within_single_quotes {
    use super::super::{Parser, Token};

    #[test]
    fn backslash_in_single_quote() {
      assert_eq!(
        Parser::parse(String::from(r#"echo 'shell\\"\nscript'"#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word(r#"shell\\"\nscript"#.to_string())]
      );
    }
  }


  mod backslash_within_double_quotes {
    use super::super::{Parser, Token};

    #[test]
    fn handled_special_characters() {
      assert_eq!(
        Parser::parse(String::from(r#"echo "shell\\\"script""#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word(r#"shell\"script"#.to_string())]
      );
    }
    
    #[test]
    fn unhandled_special_characters() {
      assert_eq!(
        Parser::parse(String::from(r#"echo "shell\$\n\`script""#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word(r#"shell\$\n\`script"#.to_string())]
      );
    }
  }

  mod redirection_operator {
    use super::super::{Parser, Token};
    use crate::builtin::redirection::{
      FileDescriptor,
      RedirectKind,
      Redirection
    };

    #[test]
    fn f() {
      assert_eq!(
        Parser::parse(String::from(r#"echo shell> tes.txt"#)).unwrap(),
        vec![Token::Word("echo".to_string()), Token::Word("shell>".to_string()), Token::Word("tes.txt".to_string())]
      );
    }

    #[test]
    fn should_be_able_to_read_redirection_op() {
      assert_eq!(
        Parser::parse(String::from(r#"echo shell > tes.txt"#)).unwrap(),
        vec![
          Token::Word("echo".to_string()),
          Token::Word("shell".to_string()),
          Token::Redirection(Redirection::new(FileDescriptor::Stdout, RedirectKind::Write)),
          Token::Word("tes.txt".to_string())
        ],
      );

      assert_eq!(
        Parser::parse(String::from(r#"echo shell >tes.txt"#)).unwrap(),
        vec![
          Token::Word("echo".to_string()),
          Token::Word("shell".to_string()),
          Token::Redirection(Redirection::new(FileDescriptor::Stdout, RedirectKind::Write)),
          Token::Word("tes.txt".to_string())
        ],
      );

      assert_eq!(
        Parser::parse(String::from(r#"echo shell >> tes.txt"#)).unwrap(),
        vec![
          Token::Word("echo".to_string()),
          Token::Word("shell".to_string()),
          Token::Redirection(Redirection::new(FileDescriptor::Stdout, RedirectKind::Append)),
          Token::Word("tes.txt".to_string())
        ],
      );

      assert_eq!(
        Parser::parse(String::from(r#"echo shell >>tes.txt"#)).unwrap(),
        vec![
          Token::Word("echo".to_string()),
          Token::Word("shell".to_string()),
          Token::Redirection(Redirection::new(FileDescriptor::Stdout, RedirectKind::Append)),
          Token::Word("tes.txt".to_string())
        ],
      );
    }

    #[test]
    fn should_be_able_to_read_file_descriptor_on_redirection_op() {
      assert_eq!(
        Parser::parse(String::from(r#"echo shell 1> tes.txt"#)).unwrap(),
        vec![
          Token::Word("echo".to_string()),
          Token::Word("shell".to_string()),
          Token::Redirection(Redirection::new(FileDescriptor::Stdout, RedirectKind::Write)),
          Token::Word("tes.txt".to_string())
        ],
      );

      assert_eq!(
        Parser::parse(String::from(r#"echo shell 2> tes.txt"#)).unwrap(),
        vec![
          Token::Word("echo".to_string()),
          Token::Word("shell".to_string()),
          Token::Redirection(Redirection::new(FileDescriptor::Stderr, RedirectKind::Write)),
          Token::Word("tes.txt".to_string())
        ],
      );
    }
  }
}