use std::{fmt::Write, iter::Peekable, str::Chars};

use crate::tokens::{self, Token, TokenType};

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
    curr: char,
    complete: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut l = Lexer {
            iter: src.chars().peekable(),
            curr: '\0',
            complete: false,
        };

        l.read_char();

        l
    }

    fn skip_whitespace(&mut self) {
        while self.curr.is_ascii_whitespace() {
            self.read_char()
        }
    }

    fn next_token(&mut self) -> tokens::Token {
        use tokens::TokenType::*;

        self.skip_whitespace();

        if self.curr == '\0' {
            self.complete = true;
            return Token::new(EOF, "");
        }

        let mut literal: String = self.curr.into();

        let typ = match self.curr {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    literal = "==".into();
                    EqualEqual
                } else {
                    Assign
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    literal = "!=".into();
                    NotEqual
                } else {
                    Not
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    literal = ">=".into();
                    GreaterEqual
                } else {
                    Greater
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    literal = "<=".into();
                    LessEqual
                } else {
                    Less
                }
            }
            ',' => Comma,
            ';' => SemiColon,
            '+' => Plus,
            '-' => Subtract,
            '/' => Divide,
            '*' => Multiply,
            '{' => LBrace,
            '}' => RBrace,
            '(' => LParen,
            ')' => RParen,
            c if c.is_ascii_digit() || c == '.' => return self.read_number(),
            _ => return self.read_identifier(),
        };

        let tok = tokens::Token { typ, literal };

        self.read_char();

        tok
    }

    fn read_number(&mut self) -> tokens::Token {
        let mut has_point = false;
        let mut literal = String::new();

        while self.curr.is_ascii_digit() || self.curr == '.' {
            if self.curr == '.' {
                if has_point {
                    return tokens::Token {
                        typ: TokenType::Illegal,
                        literal: literal + ".",
                    };
                }

                has_point = true
            }

            literal
                .write_char(self.curr)
                .expect("failed appending to literal string");

            self.read_char()
        }

        if literal.is_empty() {
            tokens::Token {
                typ: TokenType::Illegal,
                literal: self.curr.into(),
            }
        } else {
            let typ = if has_point {
                TokenType::Float
            } else {
                TokenType::Int
            };
            tokens::Token { typ, literal }
        }
    }

    fn read_identifier(&mut self) -> tokens::Token {
        let mut literal = String::new();

        while legal_identifier_char(self.curr) {
            literal
                .write_char(self.curr)
                .expect("failed appending to literal string");
            self.read_char()
        }

        if literal.is_empty() {
            return tokens::Token {
                typ: TokenType::Illegal,
                literal: self.curr.into(),
            };
        }

        // check if literal is a keyword
        if let Some(t) = Token::from_keyword(&literal) {
            t
        } else {
            tokens::Token {
                typ: TokenType::Identifier,
                literal,
            }
        }
    }

    fn read_char(&mut self) {
        self.curr = self.iter.next().unwrap_or('\0')
    }

    fn peek_char(&mut self) -> char {
        *self.iter.peek().unwrap_or(&'\0')
    }
}

fn legal_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.complete {
            Some(self.next_token())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::tokens::{Token, TokenType::*};

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
        
        let tenish = 2*10.0/2;

        let add = fn(x, y) { 
            x + y; 
        };

        let result = add(five, tenish);

        if((!true == false) != true) {
            return false;
        } else {
            return true;
        }
        
        5 < 10 > -5 <= 10 >= 50;
        ";

        let mut l = Lexer::new(input);

        let expected = vec![
            // let five = 5;
            Token::new(Let, "let"),
            Token::new(Identifier, "five"),
            Token::new(Assign, "="),
            Token::new(Int, "5"),
            Token::new(SemiColon, ";"),
            // let ten = 10;
            Token::new(Let, "let"),
            Token::new(Identifier, "tenish"),
            Token::new(Assign, "="),
            Token::new(Int, "2"),
            Token::new(Multiply, "*"),
            Token::new(Float, "10.0"),
            Token::new(Divide, "/"),
            Token::new(Int, "2"),
            Token::new(SemiColon, ";"),
            // let add = fn(x, y) { x + y; };
            Token::new(Let, "let"),
            Token::new(Identifier, "add"),
            Token::new(Assign, "="),
            Token::new(Function, "fn"),
            Token::new(LParen, "("),
            Token::new(Identifier, "x"),
            Token::new(Comma, ","),
            Token::new(Identifier, "y"),
            Token::new(RParen, ")"),
            Token::new(LBrace, "{"),
            Token::new(Identifier, "x"),
            Token::new(Plus, "+"),
            Token::new(Identifier, "y"),
            Token::new(SemiColon, ";"),
            Token::new(RBrace, "}"),
            Token::new(SemiColon, ";"),
            // let result = add(five, ten);
            Token::new(Let, "let"),
            Token::new(Identifier, "result"),
            Token::new(Assign, "="),
            Token::new(Identifier, "add"),
            Token::new(LParen, "("),
            Token::new(Identifier, "five"),
            Token::new(Comma, ","),
            Token::new(Identifier, "tenish"),
            Token::new(RParen, ")"),
            Token::new(SemiColon, ";"),
            // if((!true == false) != true){ return false; } else { return true; }
            Token::new(If, "if"),
            Token::new(LParen, "("),
            Token::new(LParen, "("),
            Token::new(Not, "!"),
            Token::new(True, "true"),
            Token::new(EqualEqual, "=="),
            Token::new(False, "false"),
            Token::new(RParen, ")"),
            Token::new(NotEqual, "!="),
            Token::new(True, "true"),
            Token::new(RParen, ")"),
            Token::new(LBrace, "{"),
            Token::new(Return, "return"),
            Token::new(False, "false"),
            Token::new(SemiColon, ";"),
            Token::new(RBrace, "}"),
            Token::new(Else, "else"),
            Token::new(LBrace, "{"),
            Token::new(Return, "return"),
            Token::new(True, "true"),
            Token::new(SemiColon, ";"),
            Token::new(RBrace, "}"),
            // 5 < 10 > -5 <= 10 >= 50;
            Token::new(Int, "5"),
            Token::new(Less, "<"),
            Token::new(Int, "10"),
            Token::new(Greater, ">"),
            Token::new(Subtract, "-"),
            Token::new(Int, "5"),
            Token::new(LessEqual, "<="),
            Token::new(Int, "10"),
            Token::new(GreaterEqual, ">="),
            Token::new(Int, "50"),
            Token::new(SemiColon, ";"),
            // done
            Token::new(EOF, ""),
        ];

        for i in expected.into_iter() {
            assert_eq!(i, l.next_token())
        }
    }
}
