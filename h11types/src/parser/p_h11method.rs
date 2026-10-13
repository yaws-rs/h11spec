//! h11 Method Parser

use logos::{Lexer, Logos};

use crate::H11Error;
use crate::H11Method;

#[derive(Debug, Logos)]
#[logos(utf8 = false)]
pub(crate) enum MethodToken<'raw> {
    #[token("CONNECT ", priority = 100)]
    Connect,
    #[token("DELETE ", priority = 100)]
    Delete,
    #[token("GET ", priority = 100)]
    Get,
    #[token("HEAD ", priority = 100)]
    Head,
    #[token("OPTIONS ", priority = 100)]
    Options,
    #[token("PATCH ", priority = 100)]
    Patch,
    #[token("POST ", priority = 100)]
    Post,
    #[token("PUT ", priority = 100)]
    Put,
    #[token("QUERY ", priority = 100)]
    Query,
    #[token("TRACE ", priority = 100)]
    Trace,
    #[regex("[A-Z0-9]{1, 12}$", priority = 1)]
    Incomplete,
    #[allow(dead_code)]
    Phantom(&'raw [u8]),
}

use super::ParseIndication;

pub(crate) fn parse_h11method<'raw>(
    lexer: &mut Lexer<'raw, MethodToken<'raw>>,
) -> Result<ParseIndication<H11Method>, H11Error> {
    if lexer.remainder().len() == 0 {
        return Ok(ParseIndication::Incomplete);
    }
    let mut ret: Option<H11Method> = None;
    if let Some(token) = lexer.next() {
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
            Ok(MethodToken::Incomplete) => {
                return Ok(ParseIndication::Incomplete);
            }
            _ => return Err(H11Error::InvalidMethod),
        };
        ret = Some(maybe_method);
    }
    match ret {
        None => Ok(ParseIndication::Incomplete),
        Some(m) => Ok(ParseIndication::Completed(m)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ParseIndication;
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
        assert_eq!(r, ParseIndication::Completed(expected));
    }

    #[test]
    fn parse_ok_expect_incomplete_wo_sp() {
        let input = "CONNECT";
        let mut l = MethodToken::lexer(input.as_bytes());
        let r = parse_h11method(&mut l);
        assert_eq!(r, Ok(ParseIndication::Incomplete));
        assert_eq!(l.span().start, 0);
    }

    #[test]
    fn parse_err_invalid_method() {
        let input = "cOnNeCt ";
        let mut l = MethodToken::lexer(input.as_bytes());
        let r = parse_h11method(&mut l);
        assert_eq!(r, Err(H11Error::InvalidMethod));
    }

    #[test]
    fn parse_ok_expect_incomplete_on_empty() {
        let input = "";
        let mut l = MethodToken::lexer(input.as_bytes());
        let r = parse_h11method(&mut l);
        assert_eq!(r, Ok(ParseIndication::Incomplete));
        assert_eq!(l.span().end, 0);
    }

    #[test]
    fn parse_err_expect_invalid_after_method() {
        let input = "CONNECT%";
        let mut l = MethodToken::lexer(input.as_bytes());
        let r = parse_h11method(&mut l);
        assert_eq!(r, Err(H11Error::InvalidMethod));
    }
}
