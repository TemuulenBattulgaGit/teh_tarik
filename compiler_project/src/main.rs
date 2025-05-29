//Temuulen Battulga_CompilerProject_Phase1
//SID:862370287

use std::fs;
use std::env;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Func, Return, Int, Print, Read, While, If, Else, Break, Continue, LeftParen, RightParen,
  LeftCurly,RightCurly,LeftBracket, RightBracket, Comma, Semicolon, Plus, Subtract, Multiply,
  Divide, Modulus, Assign, Less, LessEqual, Greater, GreaterEqual, Equality, NotEqual,
  Ident(String), Num(i32),
  End,
}

pub fn create_identifier(code: &str) -> Token {
  match code {
    "func" => Token::Func,
    "return" => Token::Return,
    "int" => Token::Int,
    "print" => Token::Print,
    "read" => Token::Read,
    "while" => Token::While,
    "if" => Token::If,
    "else" => Token::Else,
    "break" => Token::Break,
    "continue" => Token::Continue,
    _ => Token::Ident(String::from(code)),
  }
}

// This is a lexer that parses numbers and math operations
pub fn lex(code: &str) -> Result<Vec<Token>, String> {
  let bytes = code.as_bytes();
  let mut tokens: Vec<Token> = vec![];

  let mut i = 0;
  while i < bytes.len() {
    let c = bytes[i] as char;

    match c {

      // CASE: Numbers
      '0'..='9' => {

        let start = i;
        i += 1;
        while i < bytes.len() {

          let digit = bytes[i] as char;
          if digit >= '0' && digit <= '9' {
            i += 1;
          } else {
            break;
          }
        }
        let end = i;

        if i < bytes.len() {

          let next_char = bytes[i] as char;
          if next_char.is_alphabetic() || next_char == '-' {

            let error_start = start;
            while i < bytes.len() {
              let ch = bytes[i] as char;
              if ch.is_alphabetic() || ch == '_' {
                i += 1;
              } else {
                break;
              }
            }

            let invalid_token = &code[error_start..i];
            return Err(format!("Invalid identifier '{}' - identifiers cannot start with a digit", invalid_token))
          }
        }

        let string_token = &code[start..end];
        let number_value = string_token.parse::<i32>().unwrap();
        let token = Token::Num(number_value);
        tokens.push(token);
      }

      // Case: Keywords or Identifiers
      'a'..='z' | 'A'..='Z' => {

        let start = i;
        i+=1;

        while i < bytes.len() {
          let character = bytes[i] as char;
          if character.is_alphabetic() || character.is_numeric() || character == '_' {
            i += 1;
          } else {
            break;
          }
        }

        let end = i;
        let string_token = &code[start..end];
        let token = create_identifier(string_token);
        tokens.push(token);

      }

      // CASE: No extra stuff chars EASY!!!!
      '+' => {
        tokens.push(Token::Plus);
        i += 1;
      }

      ' ' | '\n' | '\t' | '\r' => {
        i += 1;
      }

      '(' => {
        tokens.push(Token::LeftParen);
        i += 1;
      }

      ')' => {
        tokens.push(Token::RightParen);
        i += 1;
      }

      '{' => {
        tokens.push(Token::LeftCurly);
        i += 1;
      }

      '}' => {
        tokens.push(Token::RightCurly);
        i += 1;
      }

      '[' => {
        tokens.push(Token::LeftBracket);
        i += 1;
      }

      ']' => {
        tokens.push(Token::RightBracket);
        i += 1;
      }

      ',' => {
        tokens.push(Token::Comma);
        i += 1;
      }

      ';' => {
        tokens.push(Token::Semicolon);
        i += 1;
      }

      '-' => {
        tokens.push(Token::Subtract);
        i += 1;
      }

      '*' => {
        tokens.push(Token::Multiply);
        i += 1;
      }

      '/' => {
        tokens.push(Token::Divide);
        i += 1;
      }

      '%' => {
        tokens.push(Token::Modulus);
        i += 1;
      }

      '#' => { // CASE: Ignoring Comments

        while i < bytes.len() && bytes[i] as char != '\n' {
            i += 1;
        }
        if i < bytes.len() {
          i += 1;
        }
      }

      '<' => { // CASE: Less or LessEqual

        if (i + 1) < bytes.len() && bytes[i+1] as char == '=' {
          tokens.push(Token::LessEqual);
            i += 2;
        } else {
          tokens.push(Token::Less);
          i += 1;
        }
      }

      '>' => { // CASE: Greater or GreaterEqual
        if (i + 1) < bytes.len() && bytes[i+1] as char == '=' {
          tokens.push(Token::GreaterEqual);
            i += 2;
        } else {
          tokens.push(Token::Greater);
          i += 1;
        }
      }

      '=' => { // CASE: Assign or Equality

        if (i + 1) < bytes.len() && bytes[i+1] as char == '=' {
          tokens.push(Token::Equality);
            i += 2;
        } else {
          tokens.push(Token::Assign);
          i += 1;
        }
      }

      '!' => { // CASE: NotEqual

        if (i + 1) < bytes.len() && bytes[i+1] as char == '=' {
          tokens.push(Token::NotEqual);
          i += 2;
        } else {
          // Missing: what to do if it's just '!' by itself
          return Err(format!("Unrecognized symbol '{}'", c));
        }
      }

      _ => {  // CASE: DEFAULT
        
        return Err(format!("Unrecognized symbol '{}'", c));
      }

    }
  }

  tokens.push(Token::End);
  return Ok(tokens);
}


fn main() {

  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
      println!("Please provide an input file through the commandline arguments for the lexer.");
      return;
  }

  if args.len() > 2 {
      println!("Too many commandline arguments.");
      return;
  }

  // Getting the name of the file for lexical scanning
  let filename = match args.get(1) {
    Some(name) => name,  // Extracting the String reference
    None => {
      println!("Please provide a filename as argument");
      return;
    }
  };


  let code = match fs::read_to_string(filename) {

    Err(error) => {
      println!("**Error. File \"{}\": {}", filename, error);
      return;
    }

    Ok(code) => {
      code
    } 

  };

  println!("Code:");
  println!("{}", code);


  // Connecting the lexer to the main function
  // After reading the file successfully:
  let tokens = match lex(&code) {

    Err(error) => {
      println!("Lexer error: {}", error);
      return;
    }
    Ok(tokens) => tokens,
  };

  // Printing out the tokens
  for token in tokens {
    println!("{:?}", token);
  }

}


#[cfg(test)]
mod tests {
    use crate::*; // Imports Token enum and lex function

    #[test]
    fn test_add_tt() {
        let code = r#"func main() {
  int a;
  int b;
  int c;
  a = 100;
  b = 50;
  c = a + b;
  print(c);
}"#;
        
        let expected = vec![
            Token::Func,
            Token::Ident("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurly,
            Token::Int,
            Token::Ident("a".to_string()),
            Token::Semicolon,
            Token::Int,
            Token::Ident("b".to_string()),
            Token::Semicolon,
            Token::Int,
            Token::Ident("c".to_string()),
            Token::Semicolon,
            Token::Ident("a".to_string()),
            Token::Assign,
            Token::Num(100),
            Token::Semicolon,
            Token::Ident("b".to_string()),
            Token::Assign,
            Token::Num(50),
            Token::Semicolon,
            Token::Ident("c".to_string()),
            Token::Assign,
            Token::Ident("a".to_string()),
            Token::Plus,
            Token::Ident("b".to_string()),
            Token::Semicolon,
            Token::Print,
            Token::LeftParen,
            Token::Ident("c".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::RightCurly,
            Token::End,
        ];
        
        let result = lex(code).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_math_tt() {
        let code = r#"# A simple program which shows mathematical operations.

func main() {
  int a;
  int b;
  int c;

  a = 100;
  b = 50;

  # This should output '150'
  c = a + b;
  print(c);

  # This should output '50'
  c = a - b;
  print(c);

  # This should output '5000'
  c = a * b;
  print(c);

  # This should output '2'
  c = a / b;
  print(c);

  # This should output '0'
  c = a % b;
  print(c);

  # Complex Expression. (4 + 2) * 7
  a = 4;
  b = 7;
  c = (a + 2) * b;
  print(c);
}"#;

        let expected = vec![
            Token::Func,
            Token::Ident("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurly,
            Token::Int,
            Token::Ident("a".to_string()),
            Token::Semicolon,
            Token::Int,
            Token::Ident("b".to_string()),
            Token::Semicolon,
            Token::Int,
            Token::Ident("c".to_string()),
            Token::Semicolon,
            Token::Ident("a".to_string()),
            Token::Assign,
            Token::Num(100),
            Token::Semicolon,
            Token::Ident("b".to_string()),
            Token::Assign,
            Token::Num(50),
            Token::Semicolon,
            Token::Ident("c".to_string()),
            Token::Assign,
            Token::Ident("a".to_string()),
            Token::Plus,
            Token::Ident("b".to_string()),
            Token::Semicolon,
            Token::Print,
            Token::LeftParen,
            Token::Ident("c".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::Ident("c".to_string()),
            Token::Assign,
            Token::Ident("a".to_string()),
            Token::Subtract,
            Token::Ident("b".to_string()),
            Token::Semicolon,
            Token::Print,
            Token::LeftParen,
            Token::Ident("c".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::Ident("c".to_string()),
            Token::Assign,
            Token::Ident("a".to_string()),
            Token::Multiply,
            Token::Ident("b".to_string()),
            Token::Semicolon,
            Token::Print,
            Token::LeftParen,
            Token::Ident("c".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::Ident("c".to_string()),
            Token::Assign,
            Token::Ident("a".to_string()),
            Token::Divide,
            Token::Ident("b".to_string()),
            Token::Semicolon,
            Token::Print,
            Token::LeftParen,
            Token::Ident("c".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::Ident("c".to_string()),
            Token::Assign,
            Token::Ident("a".to_string()),
            Token::Modulus,
            Token::Ident("b".to_string()),
            Token::Semicolon,
            Token::Print,
            Token::LeftParen,
            Token::Ident("c".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::Ident("a".to_string()),
            Token::Assign,
            Token::Num(4),
            Token::Semicolon,
            Token::Ident("b".to_string()),
            Token::Assign,
            Token::Num(7),
            Token::Semicolon,
            Token::Ident("c".to_string()),
            Token::Assign,
            Token::LeftParen,
            Token::Ident("a".to_string()),
            Token::Plus,
            Token::Num(2),
            Token::RightParen,
            Token::Multiply,
            Token::Ident("b".to_string()),
            Token::Semicolon,
            Token::Print,
            Token::LeftParen,
            Token::Ident("c".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::RightCurly,
            Token::End,
        ];
        
        let result = lex(code).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_array_tt() {
        let code = r#"func main() {
    int [4] array;

    # Should print out '2'
    array[0] = 2;
    print(array[0]);

    # Should print out '4'
    array[1] = array[0] + array[0];
    print(array[1]);

    # Should print out '8'
    array[2] = array[1] + 2 * 2;
    print(array[2]);

}"#;

        let expected = vec![
            Token::Func,
            Token::Ident("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurly,
            Token::Int,
            Token::LeftBracket,
            Token::Num(4),
            Token::RightBracket,
            Token::Ident("array".to_string()),
            Token::Semicolon,
            Token::Ident("array".to_string()),
            Token::LeftBracket,
            Token::Num(0),
            Token::RightBracket,
            Token::Assign,
            Token::Num(2),
            Token::Semicolon,
            Token::Print,
            Token::LeftParen,
            Token::Ident("array".to_string()),
            Token::LeftBracket,
            Token::Num(0),
            Token::RightBracket,
            Token::RightParen,
            Token::Semicolon,
            Token::Ident("array".to_string()),
            Token::LeftBracket,
            Token::Num(1),
            Token::RightBracket,
            Token::Assign,
            Token::Ident("array".to_string()),
            Token::LeftBracket,
            Token::Num(0),
            Token::RightBracket,
            Token::Plus,
            Token::Ident("array".to_string()),
            Token::LeftBracket,
            Token::Num(0),
            Token::RightBracket,
            Token::Semicolon,
            Token::Print,
            Token::LeftParen,
            Token::Ident("array".to_string()),
            Token::LeftBracket,
            Token::Num(1),
            Token::RightBracket,
            Token::RightParen,
            Token::Semicolon,
            Token::Ident("array".to_string()),
            Token::LeftBracket,
            Token::Num(2),
            Token::RightBracket,
            Token::Assign,
            Token::Ident("array".to_string()),
            Token::LeftBracket,
            Token::Num(1),
            Token::RightBracket,
            Token::Plus,
            Token::Num(2),
            Token::Multiply,
            Token::Num(2),
            Token::Semicolon,
            Token::Print,
            Token::LeftParen,
            Token::Ident("array".to_string()),
            Token::LeftBracket,
            Token::Num(2),
            Token::RightBracket,
            Token::RightParen,
            Token::Semicolon,
            Token::RightCurly,
            Token::End,
        ];
        
        let result = lex(code).unwrap();
        assert_eq!(result, expected);
    }

#[test]
fn test_function_tt() {
    let code = r#"func add(int a, int b) {
    return a + b;
}

func mul(int a, int b) {
     return a * b;
}

func main() {
    int a;
    int b;
    int c;
    a = 10;
    b = 2;
    c = add(a, b);
    print(c);
    c = mul(c, a + b);
    print(c);
}"#;

    let expected = vec![
        Token::Func,
        Token::Ident("add".to_string()),
        Token::LeftParen,
        Token::Int,
        Token::Ident("a".to_string()),
        Token::Comma,
        Token::Int,
        Token::Ident("b".to_string()),
        Token::RightParen,
        Token::LeftCurly,
        Token::Return,
        Token::Ident("a".to_string()),
        Token::Plus,
        Token::Ident("b".to_string()),
        Token::Semicolon,
        Token::RightCurly,
        Token::Func,
        Token::Ident("mul".to_string()),
        Token::LeftParen,
        Token::Int,
        Token::Ident("a".to_string()),
        Token::Comma,
        Token::Int,
        Token::Ident("b".to_string()),
        Token::RightParen,
        Token::LeftCurly,
        Token::Return,
        Token::Ident("a".to_string()),
        Token::Multiply,
        Token::Ident("b".to_string()),
        Token::Semicolon,
        Token::RightCurly,
        Token::Func,
        Token::Ident("main".to_string()),
        Token::LeftParen,
        Token::RightParen,
        Token::LeftCurly,
        Token::Int,
        Token::Ident("a".to_string()),
        Token::Semicolon,
        Token::Int,
        Token::Ident("b".to_string()),
        Token::Semicolon,
        Token::Int,
        Token::Ident("c".to_string()),  // ← This was missing
        Token::Semicolon,
        Token::Ident("a".to_string()),
        Token::Assign,
        Token::Num(10),
        Token::Semicolon,
        Token::Ident("b".to_string()),
        Token::Assign,
        Token::Num(2),
        Token::Semicolon,
        Token::Ident("c".to_string()),  // ← Fixed: was "a"
        Token::Assign,
        Token::Ident("add".to_string()),
        Token::LeftParen,
        Token::Ident("a".to_string()),  // ← Fixed: was Num(10)
        Token::Comma,
        Token::Ident("b".to_string()),  // ← Fixed: was Num(2)
        Token::RightParen,
        Token::Semicolon,
        Token::Print,
        Token::LeftParen,
        Token::Ident("c".to_string()),  // ← Fixed: was "a"
        Token::RightParen,
        Token::Semicolon,
        Token::Ident("c".to_string()),
        Token::Assign,
        Token::Ident("mul".to_string()),
        Token::LeftParen,
        Token::Ident("c".to_string()),
        Token::Comma,
        Token::Ident("a".to_string()),
        Token::Plus,
        Token::Ident("b".to_string()),
        Token::RightParen,
        Token::Semicolon,
        Token::Print,
        Token::LeftParen,
        Token::Ident("c".to_string()),
        Token::RightParen,
        Token::Semicolon,
        Token::RightCurly,
        Token::End,
    ];
    
    let result = lex(code).unwrap();
    assert_eq!(result, expected);
}

    #[test]
    fn test_loop_tt() {
        let code = r#"func main() {
    int i;
    i = 0;
    while i < 10 {
        print(i);
        i = i + 1;
    }
}"#;

        let expected = vec![
            Token::Func,
            Token::Ident("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurly,
            Token::Int,
            Token::Ident("i".to_string()),
            Token::Semicolon,
            Token::Ident("i".to_string()),
            Token::Assign,
            Token::Num(0),
            Token::Semicolon,
            Token::While,
            Token::Ident("i".to_string()),
            Token::Less,
            Token::Num(10),
            Token::LeftCurly,
            Token::Print,
            Token::LeftParen,
            Token::Ident("i".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::Ident("i".to_string()),
            Token::Assign,
            Token::Ident("i".to_string()),
            Token::Plus,
            Token::Num(1),
            Token::Semicolon,
            Token::RightCurly,
            Token::RightCurly,
            Token::End,
        ];
        
        let result = lex(code).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_tt() {
        let code = r#"func main() {
    int a;
    int b;
    int c;

    
    a = 100;
    b = 50;
    if a < b {
        c = 0;
    } else {
        c = 1;
    }

    # Should print out '1'.
    print(c);



    a = 100;
    b = 50;
    if a >= b {
        c = 0;
    } else {
        c = 1;
    }

    # Should print out '0'
    print(c);
}"#;

        let expected = vec![
            Token::Func,
            Token::Ident("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurly,
            Token::Int,
            Token::Ident("a".to_string()),
            Token::Semicolon,
            Token::Int,
            Token::Ident("b".to_string()),
            Token::Semicolon,
            Token::Int,
            Token::Ident("c".to_string()),
            Token::Semicolon,
            Token::Ident("a".to_string()),
            Token::Assign,
            Token::Num(100),
            Token::Semicolon,
            Token::Ident("b".to_string()),
            Token::Assign,
            Token::Num(50),
            Token::Semicolon,
            Token::If,
            Token::Ident("a".to_string()),
            Token::Less,
            Token::Ident("b".to_string()),
            Token::LeftCurly,
            Token::Ident("c".to_string()),
            Token::Assign,
            Token::Num(0),
            Token::Semicolon,
            Token::RightCurly,
            Token::Else,
            Token::LeftCurly,
            Token::Ident("c".to_string()),
            Token::Assign,
            Token::Num(1),
            Token::Semicolon,
            Token::RightCurly,
            Token::Print,
            Token::LeftParen,
            Token::Ident("c".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::Ident("a".to_string()),
            Token::Assign,
            Token::Num(100),
            Token::Semicolon,
            Token::Ident("b".to_string()),
            Token::Assign,
            Token::Num(50),
            Token::Semicolon,
            Token::If,
            Token::Ident("a".to_string()),
            Token::GreaterEqual,
            Token::Ident("b".to_string()),
            Token::LeftCurly,
            Token::Ident("c".to_string()),
            Token::Assign,
            Token::Num(0),
            Token::Semicolon,
            Token::RightCurly,
            Token::Else,
            Token::LeftCurly,
            Token::Ident("c".to_string()),
            Token::Assign,
            Token::Num(1),
            Token::Semicolon,
            Token::RightCurly,
            Token::Print,
            Token::LeftParen,
            Token::Ident("c".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::RightCurly,
            Token::End,
        ];
        
        let result = lex(code).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_loop_tt() {
        let code = r#"func main() {
    int i;
    int j;
    i = 0;
    while i < 2 {
        j = 0;
        while j < 3 {
            print(j);
            j = j + 1;
        }
        i = i + 1;
    }
}"#;

        let expected = vec![
            Token::Func,
            Token::Ident("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurly,
            Token::Int,
            Token::Ident("i".to_string()),
            Token::Semicolon,
            Token::Int,
            Token::Ident("j".to_string()),
            Token::Semicolon,
            Token::Ident("i".to_string()),
            Token::Assign,
            Token::Num(0),
            Token::Semicolon,
            Token::While,
            Token::Ident("i".to_string()),
            Token::Less,
            Token::Num(2),
            Token::LeftCurly,
            Token::Ident("j".to_string()),
            Token::Assign,
            Token::Num(0),
            Token::Semicolon,
            Token::While,
            Token::Ident("j".to_string()),
            Token::Less,
            Token::Num(3),
            Token::LeftCurly,
            Token::Print,
            Token::LeftParen,
            Token::Ident("j".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::Ident("j".to_string()),
            Token::Assign,
            Token::Ident("j".to_string()),
            Token::Plus,
            Token::Num(1),
            Token::Semicolon,
            Token::RightCurly,
            Token::Ident("i".to_string()),
            Token::Assign,
            Token::Ident("i".to_string()),
            Token::Plus,
            Token::Num(1),
            Token::Semicolon,
            Token::RightCurly,
            Token::RightCurly,
            Token::End,
        ];
        
        let result = lex(code).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_break_tt() {
        let code = r#"func main() {
    int i;
    i = 0;
    while i < 10 {
        if i >= 4 {
            break;
        }
        print(i);
        i = i + 1;
    }
}"#;

        let expected = vec![
            Token::Func,
            Token::Ident("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurly,
            Token::Int,
            Token::Ident("i".to_string()),
            Token::Semicolon,
            Token::Ident("i".to_string()),
            Token::Assign,
            Token::Num(0),
            Token::Semicolon,
            Token::While,
            Token::Ident("i".to_string()),
            Token::Less,
            Token::Num(10),
            Token::LeftCurly,
            Token::If,
            Token::Ident("i".to_string()),
            Token::GreaterEqual,
            Token::Num(4),
            Token::LeftCurly,
            Token::Break,
            Token::Semicolon,
            Token::RightCurly,
            Token::Print,
            Token::LeftParen,
            Token::Ident("i".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::Ident("i".to_string()),
            Token::Assign,
            Token::Ident("i".to_string()),
            Token::Plus,
            Token::Num(1),
            Token::Semicolon,
            Token::RightCurly,
            Token::RightCurly,
            Token::End,
        ];
        
        let result = lex(code).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_lexical_errors() {
        // Test invalid identifier starting with number
        assert!(lex("2a").is_err());
        
        // Test unrecognized symbols
        assert!(lex("^^^").is_err());
        assert!(lex("@").is_err());
        assert!(lex("$").is_err());
        
        // Test invalid characters
        assert!(lex("&").is_err());
        assert!(lex("~").is_err());
    }

    #[test]
    fn test_comments_removed() {
        let code = r#"int x; # This is a comment
int y;"#;
        
        let expected = vec![
            Token::Int,
            Token::Ident("x".to_string()),
            Token::Semicolon,
            Token::Int,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::End,
        ];
        
        let result = lex(code).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comparison_operators() {
        let code = "< <= > >= == !=";
        
        let expected = vec![
            Token::Less,
            Token::LessEqual,
            Token::Greater,
            Token::GreaterEqual,
            Token::Equality,
            Token::NotEqual,
            Token::End,
        ];
        
        let result = lex(code).unwrap();
        assert_eq!(result, expected);
    }
}