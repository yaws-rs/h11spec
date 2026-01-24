//! h11 Version Parser

use logos::{Lexer, Logos};

use crate::H11Error;
use crate::H11Version;

#[derive(Debug, Logos)]
#[logos(utf8 = false)]
//#[logos(source = [u8])]
pub(crate) enum VersionToken<'raw> {
    #[token("HTTP/1.1")]
    Http11,
    #[token("\r\n")]
    StatusEnd,
    #[allow(dead_code)]
    Phantom(&'raw [u8]),
}

pub(crate) fn parse_h11version<'raw>(
    lexer: &mut Lexer<'raw, VersionToken<'raw>>,
) -> Result<H11Version, H11Error> {
    let mut ret: Option<H11Version> = None;
    let mut got_crlf = false;
    while let Some(token) = lexer.next() {
        if ret.is_none() {
            let version = match token {
                Ok(VersionToken::Http11) => H11Version::Http11,
                _ => return Err(H11Error::InvalidVersion),
            };
            ret = Some(version);
        } else {
            match token {
                Ok(VersionToken::StatusEnd) => {
                    got_crlf = true;
                    break;
                }
                _ => return Err(H11Error::InvalidAfterVersion),
            }
        }
    }
    match ret {
        None => Err(H11Error::ExpectedVersion),
        Some(t) => match got_crlf {
            true => Ok(t),
            false => Err(H11Error::ExpectedCrLfAfterVersion),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("HTTP/1.1\r\n", 10)]
    fn parse_ok(#[case] input: &str, #[case] expected: usize) {
        let mut l = VersionToken::lexer(input.as_bytes());
        let r = parse_h11version(&mut l).unwrap();
        assert_eq!(r, H11Version::Http11);
        assert_eq!(l.span().end, expected);
    }

    #[test]
    fn parse_err_expect_crlf() {
        let input = "HTTP/1.1";
        let mut l = VersionToken::lexer(input.as_bytes());
        let r = parse_h11version(&mut l);
        assert_eq!(r, Err(H11Error::ExpectedCrLfAfterVersion));
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
        assert_eq!(r, Err(H11Error::ExpectedVersion));
    }

    #[test]
    fn parse_err_expect_invalid_after_version() {
        let input = "HTTP/1.1\r\r\n";
        let mut l = VersionToken::lexer(input.as_bytes());
        let r = parse_h11version(&mut l);
        assert_eq!(r, Err(H11Error::InvalidAfterVersion));
    }
}
