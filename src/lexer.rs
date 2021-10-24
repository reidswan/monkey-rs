use std::{fmt::Write, iter::Peekable, str::Chars};

use crate::tokens::{self, Token, TokenType};

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
    curr: char,
    complete: bool,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut l = Lexer {
            iter: src.chars().peekable(),
            curr: '\0',
            complete: false,
            line: 1,
            col: 0,
        };

        l.read_char();

        l
    }

    fn skip_whitespace(&mut self) {
        while self.curr.is_ascii_whitespace() {
            self.read_char()
        }
    }

    fn skip_to_next_line(&mut self) {
        while self.curr != '\n' && self.curr != '\0' {
            self.read_char()
        }
    }

    fn next_token(&mut self) -> tokens::Token {
        use tokens::TokenType::*;

        self.skip_whitespace();

        if self.curr == '\0' {
            self.complete = true;
            return self.new_token(EOF, "");
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
            '/' => {
                if self.peek_char() == '/' {
                    // line comment
                    self.skip_to_next_line();
                    return self.next_token();
                } else {
                    Divide
                }
            }
            ',' => Comma,
            ';' => SemiColon,
            '+' => Plus,
            '-' => Subtract,
            '*' => Multiply,
            '{' => LBrace,
            '}' => RBrace,
            '(' => LParen,
            ')' => RParen,
            c if c.is_ascii_digit() || c == '.' => return self.read_number(),
            _ => return self.read_identifier(),
        };

        let tok = self.new_token(typ, &literal);

        self.read_char();

        tok
    }

    fn read_number(&mut self) -> tokens::Token {
        let mut has_point = false;
        let mut literal = String::new();

        while self.curr.is_ascii_digit() || self.curr == '.' {
            if self.curr == '.' {
                if has_point {
                    return self.new_token(TokenType::Illegal, &(literal + "."));
                }

                has_point = true
            }

            literal
                .write_char(self.curr)
                .expect("failed appending to literal string");

            self.read_char()
        }

        if literal.is_empty() {
            self.new_token(TokenType::Illegal, &self.curr.to_string())
        } else {
            let typ = if has_point {
                TokenType::Float
            } else {
                TokenType::Int
            };
            let mut t = self.new_token(typ, &literal);

            // subtract 1 from loc for the non-numeric character that terminated the number
            t.loc.col -= 1;

            t
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
            return self.new_token(TokenType::Illegal, &self.curr.to_string());
        }

        // check if literal is a keyword
        let mut tok =
            if let Some(t) = Token::from_keyword(&literal, self.line, self.col - literal.len()) {
                t
            } else {
                self.new_token(TokenType::Identifier, &literal)
            };

        // subtract 1 from loc for the non-identifier character that terminated the identifier
        tok.loc.col -= 1;
        tok
    }

    fn read_char(&mut self) {
        self.curr = self.iter.next().unwrap_or('\0');
        if self.curr == '\n' {
            self.col = 0;
            self.line += 1;
        } else {
            self.col += 1;
        }
    }

    fn peek_char(&mut self) -> char {
        *self.iter.peek().unwrap_or(&'\0')
    }

    fn new_token(&self, typ: tokens::TokenType, literal: &str) -> tokens::Token {
        tokens::Token::new(typ, literal, self.line, self.col - literal.len())
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
        // this is a comment!
        // this should all be ignored";

        let mut l = Lexer::new(input);

        let expected = vec![
            // let five = 5;
            Token::new(Let, "let", 1, 0),
            Token::new(Identifier, "five", 1, 4),
            Token::new(Assign, "=", 1, 9),
            Token::new(Int, "5", 1, 11),
            Token::new(SemiColon, ";", 1, 12),
            // let tenish = 2*10.0/2;
            Token::new(Let, "let", 3, 8),
            Token::new(Identifier, "tenish", 3, 12),
            Token::new(Assign, "=", 3, 19),
            Token::new(Int, "2", 3, 21),
            Token::new(Multiply, "*", 3, 22),
            Token::new(Float, "10.0", 3, 23),
            Token::new(Divide, "/", 3, 27),
            Token::new(Int, "2", 3, 28),
            Token::new(SemiColon, ";", 3, 29),
            // // let add = fn(x, y) { x + y; };
            Token::new(Let, "let", 5, 8),
            Token::new(Identifier, "add", 5, 12),
            Token::new(Assign, "=", 5, 16),
            Token::new(Function, "fn", 5, 18),
            Token::new(LParen, "(", 5, 20),
            Token::new(Identifier, "x", 5, 21),
            Token::new(Comma, ",", 5, 22),
            Token::new(Identifier, "y", 5, 24),
            Token::new(RParen, ")", 5, 25),
            Token::new(LBrace, "{", 5, 27),
            Token::new(Identifier, "x", 6, 12),
            Token::new(Plus, "+", 6, 14),
            Token::new(Identifier, "y", 6, 16),
            Token::new(SemiColon, ";", 6, 17),
            Token::new(RBrace, "}", 7, 8),
            Token::new(SemiColon, ";", 7, 9),
            // let result = add(five, ten);
            Token::new(Let, "let", 9, 8),
            Token::new(Identifier, "result", 9, 12),
            Token::new(Assign, "=", 9, 19),
            Token::new(Identifier, "add", 9, 21),
            Token::new(LParen, "(", 9, 24),
            Token::new(Identifier, "five", 9, 25),
            Token::new(Comma, ",", 9, 29),
            Token::new(Identifier, "tenish", 9, 31),
            Token::new(RParen, ")", 9, 37),
            Token::new(SemiColon, ";", 9, 38),
            // // if((!true == false) != true){ return false; } else { return true; }
            Token::new(If, "if", 11, 8),
            Token::new(LParen, "(", 11, 10),
            Token::new(LParen, "(", 11, 11),
            Token::new(Not, "!", 11, 12),
            Token::new(True, "true", 11, 13),
            Token::new(EqualEqual, "==", 11, 18),
            Token::new(False, "false", 11, 21),
            Token::new(RParen, ")", 11, 26),
            Token::new(NotEqual, "!=", 11, 28),
            Token::new(True, "true", 11, 31),
            Token::new(RParen, ")", 11, 35),
            Token::new(LBrace, "{", 11, 37),
            Token::new(Return, "return", 12, 12),
            Token::new(False, "false", 12, 19),
            Token::new(SemiColon, ";", 12, 24),
            Token::new(RBrace, "}", 13, 8),
            Token::new(Else, "else", 13, 10),
            Token::new(LBrace, "{", 13, 15),
            Token::new(Return, "return", 14, 12),
            Token::new(True, "true", 14, 19),
            Token::new(SemiColon, ";", 14, 23),
            Token::new(RBrace, "}", 15, 8),
            // // 5 < 10 > -5 <= 10 >= 50;
            Token::new(Int, "5", 17, 8),
            Token::new(Less, "<", 17, 10),
            Token::new(Int, "10", 17, 12),
            Token::new(Greater, ">", 17, 15),
            Token::new(Subtract, "-", 17, 17),
            Token::new(Int, "5", 17, 18),
            Token::new(LessEqual, "<=", 17, 20),
            Token::new(Int, "10", 17, 23),
            Token::new(GreaterEqual, ">=", 17, 26),
            Token::new(Int, "50", 17, 29),
            Token::new(SemiColon, ";", 17, 31),
            //
            Token::new(EOF, "", 19, 38),
        ];

        for i in expected.into_iter() {
            assert_eq!(i, l.next_token())
        }

        assert!(l.complete, "expected no more tokens")
    }
}
