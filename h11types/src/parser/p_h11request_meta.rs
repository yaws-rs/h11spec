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

impl H11RequestMeta {
    /// Advance parsing the status line with the given input buffer
    #[inline]
    pub fn advance_status_with<'raw, R: RequestReceiver>(
        &mut self,
        r: &mut R,
        input: &'raw [u8],
    ) -> Result<usize, H11Error> {
        let mut lexer: Lexer<'raw, MethodToken<'raw>> = MethodToken::lexer(input);

        let mut try_method = H11Method::Unknown;
        let mut try_target: Option<(usize, usize)> = None;
        let mut try_version = H11Version::Unknown;

        if self.method == H11Method::Unknown {
            try_method = parse_h11method(&mut lexer)?;
        }

        let mut loc_parsed = false;

        if self.target_loc.is_none() {
            let start = lexer.span().start;
            let mut target_lexer: Lexer<'raw, TargetToken<'raw>> = lexer.morph();
            parse_h11target(&mut target_lexer)?;
            try_target = Some((start, target_lexer.span().start));
            lexer = target_lexer.morph();
            loc_parsed = true;
        }

        if self.version == H11Version::Unknown {
            let mut version_lexer: Lexer<'raw, VersionToken<'raw>> = lexer.morph();
            try_version = parse_h11version(&mut version_lexer)?;
            lexer = version_lexer.morph();
        }

        self.version = try_version;
        self.target_loc = try_target;
        self.method = try_method;

        // TODO: stream parsing target. We should not expect it to be fully in
        if loc_parsed {
            if let Some((loc_start, loc_end)) = self.target_loc {
                let mut i_target = r.impl_target();
                let target_loc_data = &input[loc_start..loc_end];
                let _ = i_target.req_target_init(target_loc_data);
                let _ = i_target.req_target_finish();
            }
        }

        Ok(lexer.span().end)
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
                //RespIndicative::R4xx(Resp4xx::BadRequest)
            }
            _ => {
                self.relay_receiver.req_header(hdr);
            }
        }
    }
    fn req_headers_finish(&mut self) -> () {
        self.myself.headers_end = Some(0);
        self.relay_receiver.req_headers_finish();
        //RespIndicative::GoAhead
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
        fn req_header<'h, 'd>(&mut self, header: H11Header<'h>) -> RespIndicative<'d> {
            self.seen.push(format!("{:?}", header));
            RespIndicative::GoAhead
        }
        fn req_headers_finish<'d>(&mut self) -> RespIndicative<'d> {
            RespIndicative::GoAhead
        }
    }

    #[rstest]
    #[case("GET / HTTP/1.1\r\n", 16)]
    #[case("GET /foo=bar?ding=dong&ping=baa+baa#anchor HTTP/1.1\r\n", 53)]
    fn try_advance_status_ok(#[case] raw_in: &str, #[case] expected_advanced: usize) {
        let mut meta = H11RequestMeta::default();

        let advanced = meta
            .advance_status_with(&mut NoRequestReceiver, raw_in.as_bytes())
            .unwrap();
        assert_eq!(advanced, expected_advanced);
        assert_eq!(meta.method, H11Method::Get);
        assert_eq!(meta.status_complete(), true);
    }

    #[rstest]
    #[case("GET / HTTP/1.1", H11Error::ExpectedCrLfAfterVersion)]
    // TODO: fix
    //#[case("GET / HTTP/1.1\r", H11Error::ExpectedCrLfAfterVersion)]
    #[case("GET / HTTP/1.1\r", H11Error::InvalidAfterVersion)]
    #[case("GET / ", H11Error::ExpectedVersion)]
    #[case(
        "GET /foo=bar?ding=dong&ping=baa+baa#anchor",
        H11Error::ExpectedSpAfterTarget
    )]
    fn try_advance_status_incomplete(#[case] raw_in: &str, #[case] expected_err: H11Error) {
        let mut meta = H11RequestMeta::default();

        let res = meta.advance_status_with(&mut NoRequestReceiver, raw_in.as_bytes());
        assert_eq!(res, Err(expected_err));
        assert_eq!(meta.method, H11Method::Unknown);
        assert_eq!(meta.status_complete(), false);
    }

    #[rstest]
    #[case("GET / HTTP/1.1\n", H11Error::InvalidAfterVersion)]
    #[case("GET /\r\n", H11Error::InvalidAfterTarget)]
    #[case("GET\r\n", H11Error::InvalidAfterMethod)]
    fn try_advance_status_err(#[case] raw_in: &str, #[case] expected_err: H11Error) {
        let mut meta = H11RequestMeta::default();

        let res = meta.advance_status_with(&mut NoRequestReceiver, raw_in.as_bytes());
        assert_eq!(res, Err(expected_err));
        assert_eq!(meta.method, H11Method::Unknown);
        assert_eq!(meta.status_complete(), false);
    }
}
