//! Header parsing

use crate::H11Error;
use crate::H11Header;
use crate::HeaderReceiver;

use crate::parser::{HeaderKeyToken, HeaderValueToken};

use logos::{Lexer, Logos};

/// Indicative completion status of header parsing
#[derive(Debug, PartialEq)]
pub enum HeaderStatus<'h> {
    /// Completed at position relative to input
    Complete(usize),
    /// Incomplete from position relative to input
    Incomplete(usize, &'h [u8]),
}

/// Standalone header parser
#[derive(Debug, Default)]
pub struct HeaderParser {
    /// Location of the error causing header field
    pub fail_location_field: usize,
    /// Location of the error causing header value
    pub fail_location_value: usize,
    /// Headers seen so far
    pub headers_seen: usize,
}

impl HeaderParser {
    /// Parse streamed chunk of data
    #[must_use]
    pub fn parse<'raw, R: HeaderReceiver>(
        &mut self,
        r: &mut R,
        input: &'raw [u8],
    ) -> Result<HeaderStatus<'raw>, H11Error> {
        let mut lexer: Lexer<'raw, HeaderKeyToken<'raw>> = HeaderKeyToken::lexer(input);

        let mut headers_complete = false;

        while let Some(hdr_key_token) = lexer.next() {
            self.fail_location_field = lexer.span().start;
            self.fail_location_value = lexer.span().end + 1;
            let hdr: H11Header<'raw> = match hdr_key_token {
                Err(_e) => {
                    return Err(H11Error::InvalidHeaders(lexer.span().start));
                }
                Ok(HeaderKeyToken::Incomplete(s)) => {
                    return Ok(HeaderStatus::Incomplete(lexer.span().start, s));
                }
                Ok(HeaderKeyToken::EmptyHeaders) if self.headers_seen == 0 => {
                    r.req_headers_finish();
                    headers_complete = true;
                    break;
                }
                Ok(HeaderKeyToken::Complete) if self.headers_seen != 0 => {
                    r.req_headers_finish();
                    headers_complete = true;
                    break;
                }
                Ok(field_token) => {
                    let maybe_incomplete_at = lexer.span().start;
                    let mut v_lexer: Lexer<'raw, HeaderValueToken<'raw>> = lexer.morph();

                    if v_lexer.remainder() == [] {
                        return Ok(HeaderStatus::Incomplete(
                            maybe_incomplete_at,
                            &input[maybe_incomplete_at..],
                        ));
                    }

                    let hdr_v: H11Header<'raw> = match v_lexer.next() {
                        Some(Ok(HeaderValueToken::Incomplete(_s))) => {
                            return Ok(HeaderStatus::Incomplete(
                                maybe_incomplete_at,
                                &input[maybe_incomplete_at..],
                            ));
                        }
                        Some(Ok(value_token)) => {
                            (field_token, value_token).try_into().map_err(|e| {
                                H11Error::InvalidHeaderValue(self.fail_location_value, e)
                            })?
                        }
                        Some(Err(_e)) => {
                            return Err(H11Error::InvalidHeaders(v_lexer.span().start));
                        }
                        None => {
                            return Err(H11Error::InvalidHeaders(v_lexer.span().start));
                        }
                    };

                    self.headers_seen += 1;

                    lexer = v_lexer.morph();
                    hdr_v
                }
            };

            r.req_header(hdr);
        }

        let st = match headers_complete {
            true => HeaderStatus::Complete(lexer.span().end),
            false => HeaderStatus::Incomplete(lexer.span().end, &input[lexer.span().end..]),
        };
        Ok(st)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use insta::assert_debug_snapshot;
    use rstest::rstest;
    use rstest::Context;

    #[derive(Debug, Default, PartialEq)]
    struct TestReceiver {
        hdrs: Vec<String>,
        finished: bool,
    }

    use crate::{H11MaybeValue, H11UnknownField};

    impl HeaderReceiver for TestReceiver {
        fn req_header<'h>(&mut self, hdr: H11Header<'h>) -> () {
            let out = match hdr {
                H11Header::Unknown(H11UnknownField(field), H11MaybeValue::Bytes(value)) => {
                    format!(
                        "Hu[{}={}]",
                        core::str::from_utf8(field).unwrap(),
                        core::str::from_utf8(value).unwrap()
                    )
                }
                _ => format!("Hd{:?}", hdr),
            };
            self.hdrs.push(out);
        }
        fn req_headers_finish(&mut self) -> () {
            self.finished = true;
        }
    }

    pub(crate) fn ctx_insta(ctx: Context) -> String {
        let case_id = ctx.case.unwrap().to_string();
        format!("{}-{}", case_id, ctx.name)
    }

    #[inline]
    fn do_test(
        tc_input: &'static str,
    ) -> (Result<HeaderStatusStr<'static>, H11Error>, TestReceiver) {
        let mut p = HeaderParser::default();
        let mut r = TestReceiver::default();

        let data = tc_input.as_bytes();
        let res = p.parse(&mut r, &data);

        let res_str = match res {
            Err(e) => Err(e),
            Ok(s) => Ok(s.into()),
        };
        (res_str, r)
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct InternalTc<'h> {
        input: &'static str,
        output: TestReceiver,
        res_str: Result<HeaderStatusStr<'h>, H11Error>,
    }

    #[derive(Debug, PartialEq)]
    enum HeaderStatusStr<'h> {
        Complete(usize),
        Incomplete(usize, &'h str),
    }

    impl<'h> From<HeaderStatus<'h>> for HeaderStatusStr<'h> {
        fn from(h: HeaderStatus<'h>) -> Self {
            match h {
                HeaderStatus::Complete(s) => Self::Complete(s),
                HeaderStatus::Incomplete(s, b) => {
                    Self::Incomplete(s, core::str::from_utf8(b).unwrap())
                }
            }
        }
    }

    #[rstest]
    #[case("A: ffffoo.bar\r\nB: foo\r\n\r\n", 0)]
    #[case("\r\n\r\n", 0)]
    #[case("\r\n\r\nfoo", 3)]
    #[case("Host: test.rustcryp.to:8181\r\nUser-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:151.0) Gecko/20100101 Firefox/151.0\r\nAccept: image/avif,image/webp,image/png,image/svg+xml,image/*;q=0.8,*/*;q=0.5\r\nAccept-Language: en-US,en;q=0.9\r\nAccept-Encoding: gzip, deflate, br, zstd\r\nDNT: 1\r\nSec-GPC: 1\r\nConnection: keep-alive\r\nReferer: https://test.rustcryp.to:8181/\r\nSec-Fetch-Dest: image\r\nSec-Fetch-Mode: no-cors\r\nSec-Fetch-Site: same-origin\r\nPriority: u=6\r\nPragma: no-cache\r\nCache-Control: no-cache\r\n\r\n", 0)]
    fn headers_complete(
        #[context] ctx: Context,
        #[case] tc_input: &'static str,
        #[case] offset: usize,
    ) {
        let (res_str, output) = do_test(tc_input);

        assert_eq!(
            res_str,
            Ok(HeaderStatusStr::Complete(tc_input.len() - offset))
        );

        assert_debug_snapshot!(
            ctx_insta(ctx),
            InternalTc {
                input: tc_input,
                output,
                res_str,
            }
        );
    }

    #[rstest]
    #[case("A: ffffoo", "A: ffffoo", 0)]
    #[case("A: foo\r\nB: ", "B: ", 8)]
    #[case("C", "C", 0)]
    fn headers_partial(
        #[context] ctx: Context,
        #[case] tc_input: &'static str,
        #[case] leftover: &'static str,
        #[case] incomplete_at: usize,
    ) {
        let (res_str, output) = do_test(tc_input);

        assert_eq!(
            res_str,
            Ok(HeaderStatusStr::Incomplete(incomplete_at, leftover))
        );

        assert_debug_snapshot!(
            ctx_insta(ctx),
            InternalTc {
                input: tc_input,
                output,
                res_str,
            }
        );
    }
}
