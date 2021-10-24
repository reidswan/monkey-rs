use crate::lexer::tokens::TokenLoc;

use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub loc: Option<TokenLoc>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = if let Some(loc) = self.loc {
            format!("At line={}, col={}: ", loc.line, loc.col)
        } else {
            String::new()
        };
        write!(f, "{}{}", prefix, self.message)
    }
}
