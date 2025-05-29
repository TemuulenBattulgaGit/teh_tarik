// tests/integration_tests.rs
use compiler_project::*;

#[test]
fn test_add_tt_file() {
    let code = std::fs::read_to_string("phase1/src/examples/add.tt").unwrap();
    let tokens = lex(&code).unwrap();
    
    assert!(tokens.len() > 15);
    assert!(matches!(tokens.last(), Some(Token::End)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Func)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Ident(ref s) if s == "main")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Print)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Int)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Num(100))));
    assert!(tokens.iter().any(|t| matches!(t, Token::Num(50))));
}

#[test]
fn test_array_tt_file() {
    let code = std::fs::read_to_string("phase1/src/examples/array.tt").unwrap();
    let tokens = lex(&code).unwrap();
    
    assert!(tokens.len() > 30);
    assert!(matches!(tokens.last(), Some(Token::End)));
    assert!(tokens.iter().any(|t| matches!(t, Token::LeftBracket)));
    assert!(tokens.iter().any(|t| matches!(t, Token::RightBracket)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Num(4))));
    assert!(tokens.iter().any(|t| matches!(t, Token::Ident(ref s) if s == "array")));
}

#[test]
fn test_break_tt_file() {
    let code = std::fs::read_to_string("phase1/src/examples/break.tt").unwrap();
    let tokens = lex(&code).unwrap();
    
    assert!(tokens.len() > 20);
    assert!(matches!(tokens.last(), Some(Token::End)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Break)));
    assert!(tokens.iter().any(|t| matches!(t, Token::While)));
    assert!(tokens.iter().any(|t| matches!(t, Token::If)));
    assert!(tokens.iter().any(|t| matches!(t, Token::GreaterEqual)));
}

#[test]
fn test_function_tt_file() {
    let code = std::fs::read_to_string("phase1/src/examples/function.tt").unwrap();
    let tokens = lex(&code).unwrap();
    
    assert!(tokens.len() > 35);
    assert!(matches!(tokens.last(), Some(Token::End)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Return)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Ident(ref s) if s == "add")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Ident(ref s) if s == "mul")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Comma)));
}

#[test]
fn test_if_tt_file() {
    let code = std::fs::read_to_string("phase1/src/examples/if.tt").unwrap();
    let tokens = lex(&code).unwrap();
    
    assert!(tokens.len() > 40);
    assert!(matches!(tokens.last(), Some(Token::End)));
    assert!(tokens.iter().any(|t| matches!(t, Token::If)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Else)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Less)));
    assert!(tokens.iter().any(|t| matches!(t, Token::GreaterEqual)));
}

#[test]
fn test_loop_tt_file() {
    let code = std::fs::read_to_string("phase1/src/examples/loop.tt").unwrap();
    let tokens = lex(&code).unwrap();
    
    assert!(tokens.len() > 15);
    assert!(matches!(tokens.last(), Some(Token::End)));
    assert!(tokens.iter().any(|t| matches!(t, Token::While)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Less)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Num(10))));
    assert!(tokens.iter().any(|t| matches!(t, Token::Num(1))));
}

#[test]
fn test_math_tt_file() {
    let code = std::fs::read_to_string("phase1/src/examples/math.tt").unwrap();
    let tokens = lex(&code).unwrap();
    
    assert!(tokens.len() > 50);
    assert!(matches!(tokens.last(), Some(Token::End)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Plus)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Subtract)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Multiply)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Divide)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Modulus)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Num(100))));
    assert!(tokens.iter().any(|t| matches!(t, Token::Num(50))));
}

#[test]
fn test_nested_loop_tt_file() {
    let code = std::fs::read_to_string("phase1/src/examples/nested_loop.tt").unwrap();
    let tokens = lex(&code).unwrap();
    
    assert!(tokens.len() > 25);
    assert!(matches!(tokens.last(), Some(Token::End)));
    assert!(tokens.iter().any(|t| matches!(t, Token::Ident(ref s) if s == "i")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Ident(ref s) if s == "j")));
    assert!(tokens.iter().any(|t| matches!(t, Token::Num(2))));
    assert!(tokens.iter().any(|t| matches!(t, Token::Num(3))));
}

#[test]
fn test_numbers_tt_file() {
    let code = std::fs::read_to_string("phase1/src/examples/numbers.tt").unwrap();
    let tokens = lex(&code).unwrap();
    
    assert_eq!(tokens.len(), 6); // 100 + 150 + 10 + End = 6 tokens
    assert!(matches!(tokens[0], Token::Num(100)));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(tokens[2], Token::Num(150)));
    assert!(matches!(tokens[3], Token::Plus));
    assert!(matches!(tokens[4], Token::Num(10)));
    assert!(matches!(tokens[5], Token::End));
}

#[test]
fn test_all_example_files_comprehensive() {
    let files = [
        "phase1/src/examples/add.tt",
        "phase1/src/examples/array.tt", 
        "phase1/src/examples/break.tt",
        "phase1/src/examples/function.tt",
        "phase1/src/examples/if.tt",
        "phase1/src/examples/loop.tt",
        "phase1/src/examples/math.tt",
        "phase1/src/examples/nested_loop.tt",
        "phase1/src/examples/numbers.tt",
    ];
    
    for file in files {
        let code = std::fs::read_to_string(file).unwrap();
        let result = lex(&code);
        assert!(result.is_ok(), "Failed to lex file: {}", file);
        
        let tokens = result.unwrap();
        assert!(matches!(tokens.last(), Some(Token::End)), "Missing End token in: {}", file);
        assert!(tokens.len() > 0, "No tokens found in: {}", file);
        
        // Verify no comments made it through
        assert!(!tokens.iter().any(|t| format!("{:?}", t).contains("#")), "Comment found in tokens for: {}", file);
    }
}

#[test]
fn test_lexical_errors() {
    // Test invalid characters
    let invalid_chars = ["@", "^", "~", "`", "$"];
    for invalid in invalid_chars {
        let result = lex(invalid);
        assert!(result.is_err(), "Should fail on invalid character: {}", invalid);
    }
    
    // Test standalone exclamation mark
    let result = lex("!");
    assert!(result.is_err(), "Should fail on standalone '!'");
}