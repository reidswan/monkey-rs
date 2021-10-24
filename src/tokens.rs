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
pub struct TokenLoc {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub typ: TokenType,
    pub literal: String,
    pub loc: TokenLoc,
}

impl Token {
    pub fn new(typ: TokenType, lit: &str, line: usize, col: usize) -> Self {
        Self {
            typ,
            literal: lit.into(),
            loc: TokenLoc { line, col },
        }
    }

    pub fn from_keyword(s: &str, line: usize, col: usize) -> Option<Self> {
        let typ = match s {
            "let" => TokenType::Let,
            "fn" => TokenType::Function,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ => return None,
        };

        Some(Token::new(typ, s, line, col))
    }
}
