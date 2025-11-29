//! h11 Method Parser

use logos::{Lexer, Logos};

use crate::H11Error;
use crate::H11Method;

#[derive(Debug, Logos)]
#[logos(source = [u8])]
pub(crate) enum MethodToken<'raw> {
    #[token("CONNECT")]
    Connect,
    #[token("DELETE")]
    Delete,
    #[token("GET")]
    Get,
    #[token("HEAD")]
    Head,
    #[token("OPTIONS")]
    Options,
    #[token("PATCH")]
    Patch,
    #[token("POST")]
    Post,
    #[token("PUT")]
    Put,
    #[token("QUERY")]
    Query,
    #[token("TRACE")]
    Trace,
    #[token(r" ")]
    MethodSep,
    #[allow(dead_code)]
    Phantom(&'raw [u8]),
}

pub(crate) fn parse_h11method<'raw>(
    lexer: &mut Lexer<'raw, MethodToken<'raw>>,
) -> Result<H11Method, H11Error> {
    let mut ret: Option<H11Method> = None;
    let mut got_space = false;
    while let Some(token) = lexer.next() {
        if ret.is_none() {
            let maybe_method = match token {
                Ok(MethodToken::Connect) => H11Method::Connect,
                Ok(MethodToken::Delete) => H11Method::Delete,
                Ok(MethodToken::Get) => H11Method::Get,
                Ok(MethodToken::Head) => H11Method::Head,
                Ok(MethodToken::Options) => H11Method::Options,
                Ok(MethodToken::Patch) => H11Method::Patch,
                Ok(MethodToken::Post) => H11Method::Post,
                Ok(MethodToken::Put) => H11Method::Put,
                Ok(MethodToken::Query) => H11Method::Query,
                Ok(MethodToken::Trace) => H11Method::Trace,
                _ => return Err(H11Error::InvalidMethod),
            };
            ret = Some(maybe_method);
        } else {
            match token {
                Ok(MethodToken::MethodSep) => {
                    got_space = true;
                    break;
                }
                _ => return Err(H11Error::InvalidAfterMethod),
            }
        }
    }
    match ret {
        None => Err(H11Error::ExpectedMethod),
        Some(m) => match got_space {
            true => Ok(m),
            false => Err(H11Error::ExpectedSpAfterMethod),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("CONNECT ", H11Method::Connect)]
    #[case("DELETE ", H11Method::Delete)]
    #[case("GET ", H11Method::Get)]
    #[case("HEAD ", H11Method::Head)]
    #[case("OPTIONS ", H11Method::Options)]
    #[case("PATCH ", H11Method::Patch)]
    #[case("POST ", H11Method::Post)]
    #[case("PUT ", H11Method::Put)]
    #[case("QUERY ", H11Method::Query)]
    #[case("TRACE ", H11Method::Trace)]
    fn parse_ok(#[case] input: &str, #[case] expected: H11Method) {
        let mut l = MethodToken::lexer(input.as_bytes());
        let r = parse_h11method(&mut l).unwrap();
        assert_eq!(r, expected);
    }

    #[test]
    fn parse_err_expect_sp() {
        let input = "CONNECT";
        let mut l = MethodToken::lexer(input.as_bytes());
        let r = parse_h11method(&mut l);
        assert_eq!(r, Err(H11Error::ExpectedSpAfterMethod));
    }

    #[test]
    fn parse_err_invalid_method() {
        let input = "cOnNeCt ";
        let mut l = MethodToken::lexer(input.as_bytes());
        let r = parse_h11method(&mut l);
        assert_eq!(r, Err(H11Error::InvalidMethod));
    }

    #[test]
    fn parse_err_expect_method() {
        let input = "";
        let mut l = MethodToken::lexer(input.as_bytes());
        let r = parse_h11method(&mut l);
        assert_eq!(r, Err(H11Error::ExpectedMethod));
    }

    #[test]
    fn parse_err_expect_invalid_after_method() {
        let input = "CONNECT%";
        let mut l = MethodToken::lexer(input.as_bytes());
        let r = parse_h11method(&mut l);
        assert_eq!(r, Err(H11Error::InvalidAfterMethod));
    }
}
