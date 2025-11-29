//! (auto generated from iana registry) h11Header Name tokens

use logos::{Lexer, Logos, Skip};

#[derive(Debug, Logos)]
#[allow(missing_docs)]
#[logos(source = [u8])]
pub(crate) enum HeaderKeyToken<'raw> {
    #[regex(r"(?i:Content-Length):\s*")]
    ContentLength,
    #[regex(r"(?i:Set-Cookie):\s*")]
    SetCookie,
    //#[regex(r"[A-Za-z0-9\-\.]+:\s([^\r\n]*)\r\n", |_| Skip, priority = 1)]
    //Ignored,
    Nothing(&'raw str),
}
