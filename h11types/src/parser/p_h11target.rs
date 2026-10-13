//! h11 Target Parser

use logos::{Lexer, Logos};

use crate::H11Error;
use super::ParseIndication;

#[derive(Debug, Logos)]
#[logos(utf8 = false)]
pub(crate) enum TargetToken<'raw> {
    #[regex(r"[^\n\s\r]+ ", |lex| lex.slice().trim_ascii_end().trim_ascii_start(), priority = 200)]
    MaybeTarget(&'raw [u8]),
    #[regex(r"[^\n\s\r]+$", |lex| lex.slice(), priority = 100)]
    #[allow(dead_code)]
    MaybeIncomplete(&'raw [u8]),
}

pub(crate) fn parse_h11target<'raw>(
    lexer: &mut Lexer<'raw, TargetToken<'raw>>,
) -> Result<ParseIndication<&'raw [u8]>, H11Error> {
    let mut ret: Option<&[u8]> = None;
    if let Some(token) = lexer.next() {
        let maybe_target = match token {
            Ok(TargetToken::MaybeTarget(t)) => t,
            Ok(TargetToken::MaybeIncomplete(_)) => return Ok(ParseIndication::Incomplete),
            Err(_) => return Err(H11Error::InvalidTarget),
        };
        ret = Some(maybe_target);
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
    use crate::ParseIndication;
    
    #[rstest]
    #[case("/ ", 1)]
    #[case("/foo?bar=baz&nn=z#anchors ", 25)]
    fn parse_ok(#[case] input: &str, #[case] expected: usize) {
        let mut l = TargetToken::lexer(input.as_bytes());
        let r = match parse_h11target(&mut l).unwrap() {
            ParseIndication::Completed(r) => r,
            ParseIndication::Incomplete => {
                panic!("Expected completion");
            }
        };

        assert_eq!(r.len(), expected);
    }

    #[test]
    fn parse_err_expect_sp() {
        let input = "/";
        let mut l = TargetToken::lexer(input.as_bytes());
        let r = parse_h11target(&mut l);
        assert_eq!(r, Ok(ParseIndication::Incomplete));
        assert_eq!(l.span().start, 0);
    }

    #[test]
    fn parse_err_invalid_target() {
        let input = "  ";
        let mut l = TargetToken::lexer(input.as_bytes());
        let r = parse_h11target(&mut l);
        assert_eq!(r, Err(H11Error::InvalidTarget));
    }

    #[test]
    fn parse_ok_incomplete_expect_target() {
        let input = "";
        let mut l = TargetToken::lexer(input.as_bytes());
        let r = parse_h11target(&mut l);
        assert_eq!(r, Ok(ParseIndication::Incomplete));
    }

    #[test]
    fn parse_err_expect_invalid_after_target() {
        let input = "/\r";
        let mut l = TargetToken::lexer(input.as_bytes());
        let r = parse_h11target(&mut l);
        assert_eq!(r, Err(H11Error::InvalidTarget));
    }
}
