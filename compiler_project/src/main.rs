//Temuulen Battulga_CompilerProject_Phase1
//SID:862370287

use std::fs;
use std::env;

#[derive(Debug, Clone)]
enum Token {
  Func, Return, Int, Print, Read, While, If, Else, Break, Continue, LeftParen, RightParen,
  LeftCurly,RightCurly,LeftBracket, RightBracket, Comma, Semicolon, Plus, Subtract, Multiply,
  Divide, Modulus, Assign, Less, LessEqual, Greater, GreaterEqual, Equality, NotEqual,
  Ident(String), Num(i32),
  End,
}

fn create_identifier(code: &str) -> Token {
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
fn lex(mut code: &str) -> Result<Vec<Token>, String> {
  let bytes = code.as_bytes();
  let mut tokens: Vec<Token> = vec![];

  let mut i = 0;
  while i < bytes.len() {
    let c = bytes[i] as char;

    match c {

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
      let string_token = &code[start..end];
      let number_value = string_token.parse::<i32>().unwrap();
      let token = Token::Num(number_value);
      tokens.push(token);
    }

    '+' => {
      tokens.push(Token::Plus);
      i += 1;
    }

    ' ' | '\n' => {
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

    '#' => {
      while i < bytes.len() && bytes[i] as char != '\n' {
          i += 1;
      }
      if i < bytes.len() {
        i += 1;
      }
    }

    '<' => {
      if (i + 1) < bytes.len() && bytes[i+1] as char == '=' {
        tokens.push(Token::LessEqual);
          i += 2;
      } else {
        tokens.push(Token::Less);
        i += 1;
      }
    }

    '>' => {
      if (i + 1) < bytes.len() && bytes[i+1] as char == '=' {
        tokens.push(Token::GreaterEqual);
          i += 2;
      } else {
        tokens.push(Token::Greater);
        i += 1;
      }
    }

    '=' => {
      if (i + 1) < bytes.len() && bytes[i+1] as char == '=' {
        tokens.push(Token::Equality);
          i += 2;
      } else {
        tokens.push(Token::Assign);
        i += 1;
      }
    }

    '!' => {
      if (i + 1) < bytes.len() && bytes[i+1] as char == '=' {
        tokens.push(Token::NotEqual);
        i += 2;
      } else {
        // Missing: what to do if it's just '!' by itself
        return Err(format!("Unrecognized symbol '{}'", c));
      }
    }

    _ => {
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
