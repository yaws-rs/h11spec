//! Unknown header

use crate::parser::HeaderValueToken;
use crate::H11Connection;
use crate::HeaderValidationError;

use logos::{Lexer, Logos};

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
#[allow(missing_docs)]
#[logos(utf8 = false)]
pub(crate) enum TokenH11Connection<'h> {
    #[regex(r"(?i:Close)")]
    Close,
    #[regex(r"(?i:Keep-Alive)")]
    KeepAlive,
    #[regex(r"(?i:Upgrade,\s*HTTP2-Settings)")]
    Http2Upgrade,
    #[regex(r"([A-Za-z0-9\-_]+):\s", |lex| lex.slice(), priority = 1)]
    Other(&'h [u8]),
}

impl<'h> TryFrom<HeaderValueToken<'h>> for H11Connection<'h> {
    type Error = HeaderValidationError;

    fn try_from(tok: HeaderValueToken<'h>) -> Result<Self, Self::Error> {
        let try_val = match tok {
            HeaderValueToken::MaybeValue(o) => o,
            _ => return Err(HeaderValidationError::ExpectedValue),
        };

        let mut lex: Lexer<'h, TokenH11Connection<'h>> = TokenH11Connection::lexer(try_val);

        match lex.next() {
            Some(Ok(TokenH11Connection::Close)) => Ok(Self::Close),
            Some(Ok(TokenH11Connection::KeepAlive)) => Ok(Self::KeepAlive),
            Some(Ok(TokenH11Connection::Http2Upgrade)) => Ok(Self::Http2Upgrade),
            _ => Err(HeaderValidationError::ExpectedConnection),
        }
    }
}
