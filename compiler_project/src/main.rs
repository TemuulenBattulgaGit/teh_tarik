fn main() {

    //First setting up my enum Token
    #[derive(Debug, Clone)]
    enum Token {
    Func,
    Return,
    Int,
    Print,
    Read,
    While,
    If,
    Else,
    Break,
    Continue,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftBraket,
    RightBraket,
    Comma,
    Semicolon,
    Plus,
    Substract,
    Multiply,
    Devide,
    Modulus,
    Assign,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equality,
    NotEqual,
    Ident(String),
    Num(i32),
    End,
    }

    

}
