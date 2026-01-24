//! (auto generated from iana registry) h11Header Name tokens

use logos::{Lexer, Logos, Skip};

#[derive(Debug, Logos)]
#[allow(missing_docs)]
#[logos(utf8 = false)]
//#[logos(source = [u8])]
pub(crate) enum HeaderKeyValueToken<'raw> {
    #[regex(
        r"(?i:Content-Length):\s*([^\r\n]*)\r\n",
        allow_greedy = true,
        callback = crate::generated::util::header_value_usize
    )]
    ContentLength(usize),
    #[regex(
        r"(?i:Set-Cookie):\s*([^\r\n]*)\r\n",
        allow_greedy = true,
        callback = crate::generated::util::header_value_u8
    )]
    SetCookie(&'raw [u8]),
    #[regex(r"[A-Za-z0-9\-\.]+:\s*[^\r\n]*\r\n", |_| Skip,
            allow_greedy = true,
            priority = 1)]
    Ignored,
    #[token("\r\n")]
    CrLf,
}
