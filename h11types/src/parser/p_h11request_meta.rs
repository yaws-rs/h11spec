//! # h11request Meta Parser
//!
//! THis parser is genereally used by the server implementation for the in flight requests and
//! will not allocate.

mod meta_headers;

use logos::{Lexer, Logos};

use crate::H11Error;
use crate::H11Header;
use crate::H11Method;
use crate::H11RequestMeta;
use crate::H11Version;

use crate::parser::{
    parse_h11method, parse_h11target, parse_h11version, MethodToken, TargetToken, VersionToken,
};

use crate::HeaderReceiver;
use crate::RequestHeaderReceiver;
use crate::RequestReceiver;

use crate::p_receivers::RequestTargetReceiver;

/// Indicative completion status of start line parsing
#[derive(Debug, PartialEq)]
pub enum StartLineStatus<'h> {
    /// Completed at position relative to input
    Complete(usize),
    /// Incomplete from position relative to input
    Incomplete(usize, &'h [u8]),
}

use crate::ParseIndication;

impl H11RequestMeta {
    /// Advance parsing the start line with the given input buffer
    #[inline]
    pub fn advance_status_with<'raw, R: RequestReceiver>(
        &mut self,
        r: &mut R,
        input: &'raw [u8],
    ) -> Result<StartLineStatus<'raw>, H11Error> {
        let mut lexer: Lexer<'raw, MethodToken<'raw>> = MethodToken::lexer(input);

        let mut try_method = H11Method::Unknown;
        let mut try_target: Option<(usize, usize)> = None;
        let mut try_version = H11Version::Unknown;

        if self.method == H11Method::Unknown {
            try_method = match parse_h11method(&mut lexer) {
                Ok(ParseIndication::Incomplete) => {
                    return Ok(StartLineStatus::Incomplete(0, input));
                }
                Ok(ParseIndication::Completed(mthd)) => mthd,
                Err(e) => return Err(e),
            };
        }

        if self.target_loc.is_none() {
            let start = lexer.span().start;
            let mut target_lexer: Lexer<'raw, TargetToken<'raw>> = lexer.morph();
            let target = match parse_h11target(&mut target_lexer) {
                Ok(ParseIndication::Incomplete) => {
                    return Ok(StartLineStatus::Incomplete(0, input));
                }
                Ok(ParseIndication::Completed(target)) => target,
                Err(e) => return Err(e),
            };

            let mut i_target = r.impl_target();
            i_target.req_target_init(target);
            i_target.req_target_finish();

            try_target = Some((start, target_lexer.span().start));
            lexer = target_lexer.morph();
        }

        if self.version == H11Version::Unknown {
            let mut version_lexer: Lexer<'_, VersionToken> = lexer.morph();
            try_version = match parse_h11version(&mut version_lexer) {
                Ok(ParseIndication::Incomplete) => {
                    return Ok(StartLineStatus::Incomplete(0, input));
                }
                Ok(ParseIndication::Completed(version)) => version,
                Err(e) => return Err(e),
            };
            lexer = version_lexer.morph();
        }

        self.version = try_version;
        self.target_loc = try_target;
        self.method = try_method;

        Ok(StartLineStatus::Complete(lexer.span().end))
    }
}

use crate::HeaderParser;

struct HeaderRelay<'r, R> {
    myself: &'r mut H11RequestMeta,
    relay_receiver: &'r mut R,

    in_header_err: Option<H11Error>,
}

impl<'r, R> HeaderReceiver for HeaderRelay<'r, R>
where
    R: RequestHeaderReceiver,
{
    fn req_header<'h>(&mut self, hdr: H11Header<'h>) {
        match self.myself.in_header(hdr) {
            Err(e) => {
                self.in_header_err = Some(e);
            }
            _ => {
                self.relay_receiver.req_header(hdr);
            }
        }
    }
    fn req_headers_finish(&mut self) {
        self.myself.headers_end = Some(0);
        self.relay_receiver.req_headers_finish();
    }
}

use crate::HeaderStatus;

impl H11RequestMeta {
    /// Advance parsing the headers with the given input buffer.
    ///
    /// ## Minimum Input
    ///
    /// Minimum input is always a single complete header
    #[inline]
    pub fn advance_headers_with<'raw, R: RequestHeaderReceiver>(
        &mut self,
        r: &mut R,
        input: &'raw [u8],
    ) -> Result<HeaderStatus<'raw>, H11Error> {
        let mut relay = HeaderRelay {
            myself: self,
            relay_receiver: r,
            in_header_err: None,
        };
        let mut p = HeaderParser::default();
        let h_status = p.parse(&mut relay, input)?;

        if let Some(err) = relay.in_header_err {
            return Err(err);
        }

        Ok(h_status)
    }
}

#[cfg(test)]
mod rfc_9110;

#[cfg(test)]
pub(super) use test::do_header_test;

#[cfg(test)]
mod test {

    use super::*;
    use crate::NoRequestReceiver;
    use crate::RespIndicative;
    use insta::assert_debug_snapshot;
    use rstest::rstest;
    use rstest::Context;

    #[derive(Debug)]
    #[allow(unused)] // Debug is used through assert and compiler ignores this
    pub(crate) struct HeaderTc<'h> {
        pub(crate) tc_input: &'static str,
        pub(crate) tester: HeaderTest,
        pub(crate) res: Result<HeaderStatus<'h>, H11Error>,
        pub(crate) meta: H11RequestMeta,
    }

    #[inline]
    pub(crate) fn do_header_test(ctx: Context, tc_input: &'static str) {
        let mut meta = H11RequestMeta::default();
        let mut tester = HeaderTest { seen: vec![] };
        let res = meta.advance_headers_with(&mut tester, tc_input.as_bytes());
        assert_debug_snapshot!(
            ctx_insta(ctx),
            HeaderTc {
                tc_input,
                tester,
                res,
                meta
            }
        );
    }

    pub(crate) fn ctx_insta(ctx: Context) -> String {
        let case_id = ctx.case.unwrap().to_string();
        format!("{}-{}", case_id, ctx.name)
    }

    #[derive(Debug, PartialEq)]
    pub(crate) struct HeaderTest {
        pub(crate) seen: Vec<String>,
    }

    impl RequestHeaderReceiver for HeaderTest {
        fn req_header<'h>(&mut self, header: H11Header<'h>) -> RespIndicative {
            self.seen.push(format!("{:?}", header));
            RespIndicative::GoAhead
        }
        fn req_headers_finish(&mut self) -> RespIndicative {
            RespIndicative::GoAhead
        }
    }

    use super::StartLineStatus;

    #[rstest]
    #[case("GET / HTTP/1.1\r\n", 16)]
    #[case("GET /foo=bar?ding=dong&ping=baa+baa#anchor HTTP/1.1\r\n", 53)]
    fn try_advance_status_ok(#[case] raw_in: &str, #[case] expected_advanced: usize) {
        let mut meta = H11RequestMeta::default();

        let advanced = meta
            .advance_status_with(&mut NoRequestReceiver, raw_in.as_bytes())
            .unwrap();
        assert_eq!(advanced, StartLineStatus::Complete(expected_advanced));
        assert_eq!(meta.method, H11Method::Get);
        assert_eq!(meta.status_complete(), true);
    }

    #[rstest]
    #[case("", Ok(StartLineStatus::Incomplete(0, "".as_bytes())))]
    #[case("GE", Ok(StartLineStatus::Incomplete(0, "GE".as_bytes())))]
    #[case("GET", Ok(StartLineStatus::Incomplete(0, "GET".as_bytes())))]
    #[case("GET ", Ok(StartLineStatus::Incomplete(0, "GET ".as_bytes())))]
    #[case("GET /", Ok(StartLineStatus::Incomplete(0, "GET /".as_bytes())))]
    #[case(
        "GET / HTTP/1.1",
        Ok(StartLineStatus::Incomplete(0, b"GET / HTTP/1.1"))
    )]
    #[case(
        "GET / HTTP/1.1\r",
        Ok(StartLineStatus::Incomplete(0, b"GET / HTTP/1.1\r"))
    )]
    #[case("GET / ", Ok(StartLineStatus::Incomplete(0, b"GET / ")))]
    #[case(
        "GET /foo=bar?ding=dong&ping=baa+baa#anchor",
        Ok(StartLineStatus::Incomplete(0, b"GET /foo=bar?ding=dong&ping=baa+baa#anchor"))
    )]
    fn try_advance_status_incomplete<'h>(
        #[case] raw_in: &'h str,
        #[case] expected_res: Result<StartLineStatus<'h>, H11Error>,
    ) {
        let mut meta = H11RequestMeta::default();

        let res = meta.advance_status_with(&mut NoRequestReceiver, raw_in.as_bytes());
        assert_eq!(res, expected_res);
        assert_eq!(meta.method, H11Method::Unknown);
        assert_eq!(meta.status_complete(), false);
    }

    #[rstest]
    #[case("GET / HTTP/1.1\n", H11Error::InvalidVersion)]
    #[case("GET /\r\n", H11Error::InvalidTarget)]
    #[case("GET\r\n", H11Error::InvalidMethod)]
    fn try_advance_status_err(#[case] raw_in: &str, #[case] expected_err: H11Error) {
        let mut meta = H11RequestMeta::default();

        let res = meta.advance_status_with(&mut NoRequestReceiver, raw_in.as_bytes());
        assert_eq!(res, Err(expected_err));
        assert_eq!(meta.method, H11Method::Unknown);
        assert_eq!(meta.status_complete(), false);
    }
}
