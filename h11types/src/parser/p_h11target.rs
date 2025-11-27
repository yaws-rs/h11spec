//! h11 Target Parser

use logos::{Lexer, Logos};

use crate::H11Error;

#[derive(Debug, Logos)]
#[logos(source = [u8])]
pub(crate) enum TargetToken<'raw> {
    #[regex(r"[^\s\r]+", |lex| lex.slice(), priority = 200)]
    MaybeTarget(&'raw [u8]),
    #[token(r" ")]
    TargetSep,
//    #[allow(dead_code)]
//    Phantom(&'raw [u8]),
}

pub(crate) fn parse_h11target<'raw>(lexer: &mut Lexer<'raw, TargetToken<'raw>>) -> Result<&'raw [u8], H11Error> {
    let mut ret: Option<&[u8]> = None;
    let mut got_space = false;
    while let Some(token) = lexer.next() {
        if ret.is_none() {
            let maybe_target = match token {
                Ok(TargetToken::MaybeTarget(t)) => t,
                _ => return Err(H11Error::InvalidTarget),
            };
            ret = Some(maybe_target);
        }
        else {
            match token {
                Ok(TargetToken::TargetSep) => {
                    got_space = true;
                    break;
                },
                _ => return Err(H11Error::InvalidAfterTarget),
            }
        }
    }
    match ret {
        None => Err(H11Error::ExpectedTarget),
        Some(t) => match got_space {
            true => Ok(t),
            false => Err(H11Error::ExpectedSpAfterTarget),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("/ ", 1)]
    #[case("/foo?bar=baz&nn=z#anchors ", 25)]    
    fn parse_ok(#[case] input: &str, #[case] expected: usize) {   
        let mut l = TargetToken::lexer(input.as_bytes());
        let r = parse_h11target(&mut l).unwrap();
        assert_eq!(r.len(), expected);
    }

    #[test]
    fn parse_err_expect_sp() {
        let input = "/";
        let mut l = TargetToken::lexer(input.as_bytes());
        let r = parse_h11target(&mut l);
        assert_eq!(r, Err(H11Error::ExpectedSpAfterTarget));
    }

    #[test]
    fn parse_err_invalid_target() {
        let input = "  ";
        let mut l = TargetToken::lexer(input.as_bytes());
        let r = parse_h11target(&mut l);
        assert_eq!(r, Err(H11Error::InvalidTarget));
    }    
    
    #[test]
    fn parse_err_expect_target() {
        let input = "";
        let mut l = TargetToken::lexer(input.as_bytes());
        let r = parse_h11target(&mut l);
        assert_eq!(r, Err(H11Error::ExpectedTarget));
    }

    #[test]
    fn parse_err_expect_invalid_after_target() {
        let input = "/\r";
        let mut l = TargetToken::lexer(input.as_bytes());
        let r = parse_h11target(&mut l);
        assert_eq!(r, Err(H11Error::InvalidAfterTarget));
    }
}
