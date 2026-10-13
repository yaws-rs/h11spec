//! h11 Version Parser

use logos::{Lexer, Logos};

use crate::H11Error;
use crate::H11Version;
use super::ParseIndication;

#[derive(Debug, Logos)]
#[logos(utf8 = false)]
pub(crate) enum VersionToken {
    #[token("HTTP/1.1\r\n", priority = 100)]
    Http11,
    #[regex(r"H(T|TT|TTP|TTP/|TTP/1|TTP/1\.|TTP/1\.1|TTP/1\.1\r)$", priority = 50)]
    Incomplete,
}

pub(crate) fn parse_h11version<'raw>(
    lexer: &mut Lexer<'raw, VersionToken>,
) -> Result<ParseIndication<H11Version>, H11Error> {
    let mut ret: Option<H11Version> = None;
    if let Some(token) = lexer.next() {
        let maybe_version = match token {
            Ok(VersionToken::Http11) => H11Version::Http11,
            Ok(VersionToken::Incomplete) => {
                return Ok(ParseIndication::Incomplete);
            },
            Err(_) => {
                return Err(H11Error::InvalidVersion);
            },
        };
        ret = Some(maybe_version);
    }
    match ret {
        None => Ok(ParseIndication::Incomplete),
        Some(t) => Ok(ParseIndication::Completed(t)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use super::ParseIndication;
    
    #[rstest]
    #[case("HTTP/1.1\r\n", 10)]
    fn parse_ok(#[case] input: &str, #[case] expected: usize) {
        let mut l = VersionToken::lexer(input.as_bytes());
        let r = parse_h11version(&mut l).unwrap();
        assert_eq!(r, ParseIndication::Completed(H11Version::Http11));
        assert_eq!(l.span().end, expected);
    }

    #[test]
    fn parse_err_expect_crlf() {
        let input = "HTTP/1.1";
        let mut l = VersionToken::lexer(input.as_bytes());
        let r = parse_h11version(&mut l);
        assert_eq!(r, Ok(ParseIndication::Incomplete));
    }

    #[test]
    fn parse_err_invalid_version() {
        let input = "HTTP/1.4\r\n";
        let mut l = VersionToken::lexer(input.as_bytes());
        let r = parse_h11version(&mut l);
        assert_eq!(r, Err(H11Error::InvalidVersion));
    }

    #[test]
    fn parse_err_expect_version() {
        let input = "";
        let mut l = VersionToken::lexer(input.as_bytes());
        let r = parse_h11version(&mut l);
        assert_eq!(r, Ok(ParseIndication::Incomplete));
    }

    #[test]
    fn parse_err_expect_invalid_after_version() {
        let input = "HTTP/1.1\r\r\n";
        let mut l = VersionToken::lexer(input.as_bytes());
        let r = parse_h11version(&mut l);
        assert_eq!(r, Err(H11Error::InvalidVersion));
    }
}
