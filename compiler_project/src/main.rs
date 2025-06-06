//Temuulen Battulga_CompilerProject_Phase1
//SID:862370287

use std::fs;
use std::env;

// PHASE 2: PARSER STARTS HERE ----------------------------------------------------------

// parse programs with multiple functions
// loop over everything, outputting generated code.
fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  assert!(tokens.len() >= 1 && matches!(tokens[tokens.len() - 1], Token::End));
  while !at_end(tokens, *index) {
    parse_function(tokens, index)?;
  }

  return Ok(());
}

// Provided function that checks if the parse program
// has reached the end of the vector of tokens.
fn at_end(tokens: &Vec<Token>, index: usize) -> bool {

  match tokens[index] {
    Token::End => { true }
    _ => { false }
  }
}

// parse function such as:
// func main(int a, int b) {
//    # ... statements here...
//    # ...
// }
// a loop is done to handle statements.
fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match tokens[*index] {  // func
    Token::Func => { *index += 1; }
    _ => { return Err(String::from("functions must begin with func")); }
  }

  match tokens[*index] {  // ident
    Token::Ident(_) => { *index += 1; }
    _  => { return Err(String::from("functions must have a function identifier"));}
  }

  match tokens[*index] {  // '('
    Token::LeftParen => { *index += 1; }
    _ => { return Err(String::from("expected '('"));}
  }

  // if not match ')' check for parameter if any
  if !matches!(tokens[*index], Token::RightParen) {
    parse_function_parameter(tokens, index)?;

    while matches!(tokens[*index], Token::Comma) {  // while there is ',' after each parameter
      *index += 1; // parse comma
      parse_function_parameter(tokens, index)?;
    }
  }

  // ')'
  match tokens[*index] {
    Token::RightParen => { *index += 1; }
    _ => { return Err(String::from("expected ')'"));}
  }

  match tokens[*index] {
    Token::LeftCurly => { *index += 1; }
    _ => { return Err(String::from("expected '{'"));}
  }

  while !matches!(tokens[*index], Token::RightCurly) {
    parse_statement(tokens, index)?;
  }

  match tokens[*index] {
    Token::RightCurly => { *index += 1; }
    _ => { return Err(String::from("expected '}'"));}
  }

  return Ok(());
}

fn parse_function_parameter(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match tokens[*index] {
    Token::Int => {*index += 1;}
    _ => {return Err(String::from("Declaration statements must begin with 'int' keyword"));}
  }

  if matches!(tokens[*index], Token::LeftBracket) {
    *index += 1;

    match tokens[*index] {
      Token::Num(_) => { *index += 1; }
      _ => { return Err(String::from("Expected array index (number) after '['"));  }
    }

    match tokens[*index] {
      Token::RightBracket => { *index += 1; }
      _ => { return Err(String::from("Expected ']' after array size")); }
    }
  }

  match tokens[*index] {
    Token::Ident(_) => {*index += 1;}
    _ => {return Err(String::from("Declarations must have an identifier"));}
  }

  return Ok(());
}

// parsing a statement such as:
// int a;
// a = a + b;
// a = a % b;
// print(a)
// read(a)
// returns epsilon if '}'
fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match tokens[*index] {
    Token::Int => parse_declaration_statement(tokens, index)?,
    Token::Ident(_) => parse_assignment_statement(tokens, index)?,
    Token::Return => parse_return_statement(tokens, index)?,
    Token::Print => parse_print_statement(tokens, index)?,
    Token::Read => parse_read_statement(tokens, index)?,
    Token::While => parse_while_loop(tokens, index)?,
    Token::If => parse_if_statement(tokens, index)?,
    Token::Break => parse_break_statement(tokens, index)?,
    Token::Continue => parse_continue_statement(tokens, index)?,
    _ => Err(String::from("invalid statement"))
  }
  return Ok(());
}

fn parse_declaration_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match tokens[*index] {
    Token::Int => {*index += 1;}
    _ => {return Err(String::from("Declaration statements must begin with 'int' keyword"));}
  }

  // look ahead for an Array declaration
  // the case of "Int [Num] Ident"
  if matches!(tokens[*index], Token::LeftBracket) {
    *index += 1;

    match tokens[*index] {
      Token::Num(_) => { *index += 1; }
      _ => { return Err(String::from("Expected array size (number) after '['"));  }
    }

    match tokens[*index] {
      Token::RightBracket => { *index += 1; }
      _ => { return Err(String::from("Expected ']' after array size")); }
    }
  }

  match tokens[*index] {
    Token::Ident(_) => {*index += 1;}
    _ => {return Err(String::from("Declarations must have an identifier"));}
  }

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statements must end with a semicolon"));}
  }

  return Ok(());
}

fn parse_assignment_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match tokens[*index] {
    Token::Ident(_) => {*index += 1;}
    _ => {return Err(String::from("Assignment statements must begin with an identifier"));}
  }

  // look ahead for an Array assignment
  // the case of "ident [expression] = expression"
  if matches!(tokens[*index], Token::LeftBracket) {
    *index += 1;

    parse_expression(tokens, index)?;

    match tokens[*index] {
      Token::RightBracket => { *index += 1; }
      _ => { return Err(String::from("Expected ']' after array index index")); }
    }
  }
  
  //  '=' 
  match tokens[*index] {
    Token::Assign => {*index += 1;}
    _ => {return Err(String::from("Statement is missing the '=' operator"));}
  }

  parse_expression(tokens, index)?;

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statements must end with a semicolon"));}
  }

  return Ok(());
}

fn parse_return_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match tokens[*index] {
    Token::Return => {*index += 1;}
    _ => {return Err(String::from("Return statement must begin with a 'return' keyword"));}
  }

  parse_expression(tokens, index)?;

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement must end with a semicolon"));}
  }

  return Ok(());
}

fn parse_print_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match tokens[*index] {
    Token::Print => {*index += 1;}
    _ => {return Err(String::from("Print statement must begin with a 'print' keyword"));}
  }

  match tokens[*index] {
    Token::LeftParen => {*index += 1;}
    _ => {return Err(String::from("Expected '('"));}
  }

  parse_term(tokens, index)?;

  match tokens[*index] {
    Token::RightParen => {*index += 1;}
    _ => {return Err(String::from("Expected ')'"));}
  }

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement must end with a semicolon"));}
  }

  return Ok(());
}

fn parse_read_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match tokens[*index] {
    Token::Read => {*index += 1;}
    _ => {return Err(String::from("Read statement must begin with a 'read' keyword"));}
  }

  match tokens[*index] {
    Token::LeftParen => {*index += 1;}
    _ => {return Err(String::from("Expected '('"));}
  }

  parse_term(tokens, index)?;

  match tokens[*index] {
    Token::RightParen => {*index += 1;}
    _ => {return Err(String::from("Expected ')'"));}
  }

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement must end with a semicolon"));}
  }

  return Ok(());
}

fn parse_while_loop(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match tokens[*index] {
    Token::While => {*index += 1;}
    _ => {return Err(String::from("While statement must begin with a 'while' keyword"));}
  }

  parse_bool_expression(tokens, index)?;

  match tokens[*index] {
    Token::LeftCurly => {*index += 1;}
    _ => {return Err(String::from("Expected '{' after bool expression"));}
  }

  while !matches!(tokens[*index], Token::RightCurly) {
    parse_statement(tokens, index)?;
  }

  match tokens[*index] {
    Token::RightCurly => {*index += 1;}
    _ => {return Err(String::from("Expected '}'"));}
  }

  return Ok(());
}

fn parse_if_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match tokens[*index] {
    Token::If => {*index += 1;}
    _ => {return Err(String::from("If statement must begin with a 'if' keyword"));}
  }

  parse_bool_expression(tokens, index)?;

  match tokens[*index] {
    Token::LeftCurly => {*index += 1;}
    _ => {return Err(String::from("Expected '{' after bool expression"));}
  }

  while !matches!(tokens[*index], Token::RightCurly) {
    parse_statement(tokens, index)?;
  }

  match tokens[*index] {
    Token::RightCurly => {*index += 1;}
    _ => {return Err(String::from("Expected '}'"));}
  }

  if matches!(tokens[*index], Token::Else) {
    *index += 1;

    match tokens[*index] {
      Token::LeftCurly => {*index += 1;}
      _ => {return Err(String::from("Expected '{' after 'else' keyword"));}
    }

    parse_statement(tokens, index)?;

    match tokens[*index] {
      Token::RightCurly => {*index += 1;}
      _ => {return Err(String::from("Expected '}'"));}
    }
  }

  return Ok(());
}

fn parse_break_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match tokens[*index] {
    Token::Break => {*index += 1;}
    _ => {return Err(String::from("Break statement must start with a 'break' keyword"));}
  }

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement must end with a semicolon"));}
  }

  return Ok(());
}

fn parse_continue_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match tokens[*index] {
    Token::Continue => {*index += 1;}
    _ => {return Err(String::from("Continue statement must start with a 'continue' keyword"));}
  }

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statement must end with a semicolon"));}
  }

  return Ok(());
}

// parsing complex expressions such as: "a + b - (c * d) / (f + g - 8);
fn parse_bool_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  parse_expression(tokens, index)?;

  match tokens[*index] {
    Token::Less => {*index += 1;}
    Token::LessEqual => {*index += 1;}
    Token::Equality=> {*index += 1;}
    Token::NotEqual => {*index += 1;}
    Token::GreaterEqual => {*index += 1;}
    Token::Greater => {*index += 1;}
    _ => return {Err(String::from("Expected a boolean operator"));}
  }
  
  parse_expression(tokens, index)?;
  return Ok(());
}

// parsing complex expressions such as: "a + b - (c * d) / (f + g - 8);
fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match parse_multiply_expression(tokens, index) {
    Ok(()) => {},
    Err(e) => {return Err(e);}
  }
  loop {
    match tokens[*index] {

      Token::Plus => {
        *index += 1;
        parse_multiply_expression(tokens, index)?;
      }

      Token::Subtract => {
        *index += 1;
        parse_multiply_expression(tokens, index)?;
      }

      _ => { 
        break;
      }

    };
  }

  return Ok(());
}

fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match parse_term(tokens, index) {
    Ok(()) => {},
    Err(e) => {return Err(e);}
  }
  loop {
    match tokens[*index] {

      Token::Multiply => {
        *index += 1;
        parse_term(tokens, index)?;
      }

      Token::Divide => {
        *index += 1;
        parse_term(tokens, index)?;
      }

      Token::Modulus => {
        *index += 1;
        parse_term(tokens, index)?;
      }

      _ => {
        break;
      }
    };
  }

  return Ok(());
}

// a term is either a Number or an Identifier.
fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {

  match tokens[*index] {

    Token::Ident(_) => {
      *index += 1;

      // if after 'Ident' the token is '('
      if matches!(tokens[*index], Token::LeftParen) {
        *index += 1;
        parse_expression(tokens, index)?;
          
        while matches!(tokens[*index], Token::Comma) {  // while there is ',' after each parameter
          *index += 1; // parse comma
          parse_expression(tokens, index)?;
        }
      }

      match tokens[*index] {
          Token::RightParen => {*index += 1;}
          _ => return {Err(String::from("Expected ')'" ));}
        }

      // if after 'Ident' the token is '['
      if matches!(tokens[*index], Token::LeftBracket) {
        *index += 1;
        parse_expression(tokens, index)?;

        match tokens[*index] {
          Token::RightBracket => {*index += 1;}
          _ => {return Err(String::from("Expecter ']'"));}
        }
      }

      return Ok(());
    }

    Token::Num(_) => {
      *index += 1;
      return Ok(());
    }

    Token::LeftParen => {
      *index += 1;

      parse_expression(tokens, index)?;

      match tokens[*index] {
        Token::RightParen => {*index += 1;}
        _ => { return Err(String::from("missing right parenthesis ')'")); }
      }
      return Ok(());
    }
    
    _ => {
      return Err(String::from("missing expression term."));
    }

  }
}
// PHASE 2: END---------------------------------------------------------------------------------

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
      println!("**Error**");
      println!("----------------------");
      println!("{}", error);
      println!("----------------------");
      return;
    }
    Ok(tokens) => tokens,
  };

  // Parsing the tokens
  let mut index: usize = 0;
  match parse_program(&tokens, &mut index) {

    Ok(()) => {
      println!("Program Parsed Successfully.");
    }

    Err(message) => {

      println!("**Error**");
      println!("----------------------");

      if tokens.len() == 0 {
        println!("No code has been provided.");
      } else {
        println!("Error: {}", message);
        println!("----------------------");
      }
    }
  }

}

// PHASE 1: LEXICAL SCANNER STARTS HERE --------------------------------------------------------------
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

// PHASE 1: END ------------------------------------------------------------

