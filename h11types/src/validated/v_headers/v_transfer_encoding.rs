//! Transfer-Encoding header

use crate::parser::HeaderValueToken;
use crate::H11TransferEncoding;
use crate::HeaderValidationError;

use logos::{Lexer, Logos};

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
#[allow(missing_docs)]
#[logos(utf8 = false)]
pub(crate) enum TokenH11TransferEncoding {
    #[regex(r"(?i:Chunked)")]
    Chunked,
}

impl TryFrom<HeaderValueToken<'_>> for H11TransferEncoding {
    type Error = HeaderValidationError;

    fn try_from(tok: HeaderValueToken<'_>) -> Result<Self, Self::Error> {
        let try_val = match tok {
            HeaderValueToken::MaybeValue(o) => o,
            _ => return Err(HeaderValidationError::ExpectedValue),
        };

        let mut lex: Lexer<'_, TokenH11TransferEncoding> = TokenH11TransferEncoding::lexer(try_val);

        match lex.next() {
            Some(Ok(TokenH11TransferEncoding::Chunked)) => Ok(Self::Chunked),
            _ => Err(HeaderValidationError::ExpectedTransferEncoding),
        }
    }
}
