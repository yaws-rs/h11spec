//! (auto generated from iana registry) h11Header Name tokens

use logos::{Lexer, Logos};

#[derive(Debug, Logos)]
#[logos(source = [u8])]
pub(crate) enum HeaderKeyToken<'raw> {
    #[regex(r"(?i:Content-Length):\s*([^\r\n]*)\r\n", crate::generated::util::header_value_u8)]
    ContentLength(&'raw [u8]),
    #[allow(dead_code)]
    Phantom(&'raw [u8]),
}
