#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    EOF,

    // Identifiers + literals
    Identifier,
    Int,
    Float,

    // Operators
    Assign,
    Plus,
    Subtract,
    Divide,
    Multiply,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Not,
    EqualEqual,
    NotEqual,

    // Delimeters
    Comma,
    SemiColon,

    // Brackets
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub typ: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(typ: TokenType, lit: &str) -> Self {
        Self {
            typ,
            literal: lit.into(),
        }
    }

    pub fn from_keyword(s: &str) -> Option<Self> {
        match s {
            "let" => Some(Token::new(TokenType::Let, s)),
            "fn" => Some(Token::new(TokenType::Function, s)),
            "if" => Some(Token::new(TokenType::If, s)),
            "else" => Some(Token::new(TokenType::Else, s)),
            "return" => Some(Token::new(TokenType::Return, s)),
            "true" => Some(Token::new(TokenType::True, s)),
            "false" => Some(Token::new(TokenType::False, s)),
            _ => None,
        }
    }
}
