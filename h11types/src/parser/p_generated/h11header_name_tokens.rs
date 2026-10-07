//! (auto generated from iana registry) h11Header Name tokens

use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
#[allow(missing_docs)]
#[logos(utf8 = false)]
pub(crate) enum HeaderKeyToken<'raw> {
    #[regex(r"(?i:Content-Length):\s*", priority = 100)]
    ContentLength,
    #[regex(r"(?i:Set-Cookie):\s*", priority = 100)]
    SetCookie,
    #[regex(r"(?i:Host):\s*", priority = 100)]
    Host,
    #[regex(r"(?:Connection):\s*", priority = 100)]
    Connection,
    #[regex(r"\r\n\r\n", priority = 100)]
    EmptyHeaders,
    #[regex(r"\r\n", priority = 50)]
    Complete,
    #[regex(r"([A-Za-z0-9\-_]+):\s", |lex| lex.slice().strip_suffix(&[58, 32]).unwrap(), priority = 20)]
    Other(&'raw [u8]),
    #[regex(r"([A-Za-z0-9\-_]+)$", |lex| lex.slice(), priority = 10)]
    Incomplete(&'raw [u8]),
    #[regex(r"\r$", priority = 5)]
    CrIncomplete,
}
