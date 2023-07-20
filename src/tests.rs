use super::*;
use pretty_assertions::assert_eq;

const INPUT: &str = "(add 10 (subtract 10 6))";

fn token_test_inst() -> Vec<Token> {
    vec![
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
    ]
}

fn ast_test_inst() -> Node {
    Node {
        kind: NodeKind::Program,
        name: None,
        value: None,
        body: vec![Node {
            kind: NodeKind::CallExpression,
            name: Some("add".to_string()),
            value: None,
            body: vec![],
            params: vec![
                Node {
                    kind: NodeKind::NumberLiteral,
                    name: None,
                    value: Some("10".to_string()),
                    body: vec![],
                    params: vec![],
                },
                Node {
                    kind: NodeKind::CallExpression,
                    name: Some("subtract".to_string()),
                    value: None,
                    body: vec![],
                    params: vec![
                        Node {
                            kind: NodeKind::NumberLiteral,
                            name: None,
                            value: Some("10".to_string()),
                            body: vec![],
                            params: vec![],
                        },
                        Node {
                            kind: NodeKind::NumberLiteral,
                            name: None,
                            value: Some("6".to_string()),
                            body: vec![],
                            params: vec![],
                        },
                    ],
                },
            ],
        }],
        params: vec![],
    }
}

#[test]
fn tokenizer_test() {
    let tokens = tokenizer(INPUT.to_string());

    assert_eq!(token_test_inst(), tokens);
}

#[test]
fn test_parser() {
    let ast = parser(token_test_inst());

    assert_eq!(ast_test_inst(), ast);
}
