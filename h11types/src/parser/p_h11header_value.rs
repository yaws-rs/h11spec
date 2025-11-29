//! h11 Headers Parser

use logos::{Lexer, Logos};

use crate::generated::h11header_value_tokens::HeaderValueToken;
use crate::H11Error;

#[inline]
pub(crate) fn parse_h11header_value<'raw>(
    lexer: &mut Lexer<'raw, HeaderValueToken<'raw>>,
) -> Result<usize, H11Error> {
    while let Some(token) = lexer.next() {
        match token {
            Ok(_) => {}
            Err(e) => panic!("err = {:?}", e),
        }
    }
    Ok(lexer.span().end)
}
