//! (auto generated from iana registry) h11Header Name tokens

use logos::{Lexer, Logos, Skip};

#[derive(Debug, Logos)]
#[allow(missing_docs)]
#[logos(source = [u8])]
pub(crate) enum HeaderValueToken<'raw> {
    #[regex(r"\s*([^\r\n]+)", |lex| lex.slice(), priority = 1)]
    MaybeValue(&'raw [u8]),
    #[token("\r\n")]
    CrLf,
}
