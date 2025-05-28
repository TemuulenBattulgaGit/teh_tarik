//Temuulen Battulga_CompilerProject_Phase1
//SID:862370287

use std::fs;
use std::env;

#[derive(Debug, Clone)]
enum Token {
    Func, Return, Int, Print, Read, While, If, Else, Break, Continue, LeftParen, RightParen,
    LeftCurly,RightCurly,LeftBraket, RightBraket, Comma, Semicolon, Plus, Substract, Multiply,
    Devide, Modulus, Assign, Less, LessEqual, Greater, GreaterEqual, Equality, NotEqual,
    Ident(String),Num(i32),
    End,
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

    let filename = match args.get(1) {

        Some(name) => name,  // Extract the String reference
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

}
