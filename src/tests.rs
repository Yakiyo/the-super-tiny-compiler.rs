use super::*;
use pretty_assertions::assert_eq;

const INPUT: &str = "(add 10 (subtract 10 6))";

#[test]
fn tokenizer_test() {
    let test_tokens: Vec<Token> = vec![
        Token {
            kind: TokenKind::Paren,
            value: "(".to_string(),
        },
        Token {
            kind: TokenKind::Name,
            value: "add".to_string(),
        },
        Token {
            kind: TokenKind::Number,
            value: "10".to_string(),
        },
        Token {
            kind: TokenKind::Paren,
            value: "(".to_string(),
        },
        Token {
            kind: TokenKind::Name,
            value: "subtract".to_string(),
        },
        Token {
            kind: TokenKind::Number,
            value: "10".to_string(),
        },
        Token {
            kind: TokenKind::Number,
            value: "6".to_string(),
        },
        Token {
            kind: TokenKind::Paren,
            value: ")".to_string(),
        },
    ];
    let tokens = tokenizer(INPUT.to_string());

    assert_eq!(test_tokens, tokens);
}
