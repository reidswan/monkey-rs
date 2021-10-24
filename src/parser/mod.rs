pub mod ast;
pub mod error;

use crate::lexer::{
    self,
    tokens::{Token, TokenType},
};
use std::iter::Peekable;

use self::{
    ast::{Expression, Identifier, Program, Statement},
    error::ParseError,
};

struct Parser<'a> {
    lexer: Peekable<lexer::Lexer<'a>>,
}

impl<'a> Parser<'a> {
    fn new(l: lexer::Lexer<'a>) -> Self {
        Parser {
            lexer: l.peekable(),
        }
    }

    fn parse(&mut self) -> Program {
        let mut program = Program::default();

        while let Some(t) = self.lexer.peek() {
            if t.typ == TokenType::EOF {
                break;
            }
            match self.parse_statement() {
                Ok(s) => program.statements.push(s),
                Err(e) => program.errors.push(e),
            }
        }

        program
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let tok = self.next()?;

        match tok.typ {
            TokenType::Let => self.parse_let_statement(tok),
            _ => Err(ParseError {
                message: format!("unexpected token: {:?}", tok.typ),
                loc: Some(tok.loc),
            }),
        }
    }

    fn parse_let_statement(&mut self, start: Token) -> Result<Statement, ParseError> {
        let id = self.expect_next(TokenType::Identifier)?;

        self.expect_next(TokenType::Assign)?;

        // TODO actually parse an expression
        while self.next()?.typ != TokenType::SemiColon {}

        Ok(Statement::LetStatement {
            token: start,
            identifier: Identifier {
                value: id.literal.clone(),
                token: id,
            },
            value: Expression::Dummy,
        })
    }

    fn expect_peek(&'a mut self, typ: TokenType) -> Result<&'a Token, ParseError> {
        let tok = self.peek()?;
        if tok.typ != typ {
            Err(ParseError {
                message: format!("expected a '{:?}' token but got '{:?}'", typ, tok.typ),
                loc: Some(tok.loc),
            })
        } else {
            Ok(tok)
        }
    }

    fn peek(&'a mut self) -> Result<&'a Token, ParseError> {
        self.lexer.peek().ok_or(ParseError {
            message: "Unexpected end of input".into(),
            loc: None,
        })
    }

    fn expect_next(&mut self, typ: TokenType) -> Result<Token, ParseError> {
        let tok = self.next()?;
        if tok.typ != typ {
            Err(ParseError {
                message: format!("expected a '{:?}' token but got '{:?}'", typ, tok.typ),
                loc: Some(tok.loc),
            })
        } else {
            Ok(tok)
        }
    }

    fn next(&mut self) -> Result<Token, ParseError> {
        self.lexer.next().ok_or(ParseError {
            message: "Unexpected end of input".into(),
            loc: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer, parser::ast::Identifier};

    use super::{ast::Statement, Parser};

    #[test]
    fn test_parse() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;";

        let lex = lexer::Lexer::new(input);
        let mut parser = Parser::new(lex);

        let program = parser.parse();

        assert_eq!(program.statements.len(), 3);
        assert_eq!(program.errors.len(), 0);

        let expected_ids = vec!["x", "y", "foobar"];
        for (s, exp) in program.statements.into_iter().zip(expected_ids.iter()) {
            assert_let_statement(s, exp)
        }
    }

    // TODO also assert expression value
    fn assert_let_statement(s: Statement, id: &str) {
        match s {
            Statement::LetStatement {
                identifier: Identifier { value, .. },
                ..
            } => assert_eq!(id, &value),
            _ => panic!("not a let statement"),
        }
    }
}
