//! (auto generated from iana registry) h11Header Name tokens

use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
#[allow(missing_docs)]
#[logos(utf8 = false)]
pub(crate) enum HeaderKeyToken<'raw> {
    #[regex(r"(?i:Content-Length):\s*")]
    ContentLength,
    #[regex(r"(?i:Set-Cookie):\s*")]
    SetCookie,
    #[regex(r"(?i:Host):\s*")]
    Host,
    #[regex(r"(?:Connection):\s*")]
    Connection,
    #[regex(r"\r\n\r\n")]
    EmptyHeaders,
    #[regex(r"\r\n")]
    Complete,
    #[regex(r"([A-Za-z0-9\-_]+):\s", |lex| lex.slice().strip_suffix(&[58, 32]).unwrap(), priority = 1)]
    Other(&'raw [u8]),
}
