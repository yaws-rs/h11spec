//! Header parsing

use crate::H11Error;
use crate::H11Header;
use crate::HeaderReceiver;

use crate::parser::{HeaderKeyToken, HeaderValueToken};

use logos::{Logos, Lexer};

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
    /// Parse data
    pub fn parse<'raw, R: HeaderReceiver>(&mut self, r: &mut R, input: &'raw [u8]) -> Result<usize, H11Error> {
        let mut lexer: Lexer<'raw, HeaderKeyToken<'raw>> = HeaderKeyToken::lexer(input);
        
	    while let Some(hdr_key_token) = lexer.next() {
            self.fail_location_field = lexer.span().start;
            self.fail_location_value = lexer.span().end + 1;
            let hdr: H11Header<'raw> = match hdr_key_token {
                Err(_e) => {
                    return Err(H11Error::InvalidHeaders(lexer.span().start));
                }
                Ok(HeaderKeyToken::EmptyHeaders) if self.headers_seen == 0 => {
                    r.req_headers_finish();
                    break;
                },
                Ok(HeaderKeyToken::Complete) if self.headers_seen != 0 => {
                    r.req_headers_finish();
                    break;
                },
                Ok(field_token) => {
                    let mut v_lexer: Lexer<'raw, HeaderValueToken<'raw>> = lexer.morph();
                    
                    let hdr_v: H11Header<'raw> = match v_lexer.next() {
			            Some(Ok(value_token)) => {
                            (field_token, value_token).try_into()
                                .map_err(|e| H11Error::InvalidHeaderValue(self.fail_location_value, e))?
			            },
                        Some(Err(_e)) => {
                            return Err(H11Error::InvalidHeaders(v_lexer.span().start));
                        }
                        None => {
                            return Err(H11Error::InvalidHeaders(v_lexer.span().start));
                        }
                    };
                    
                    self.headers_seen+=1;

                    lexer = v_lexer.morph();
                    hdr_v
                },
            };
            
            r.req_header(hdr);
        }

        Ok(lexer.span().end)
    }
}


#[cfg(test)]
mod test {

    use super::*;
    use crate::RespIndicative;
    use insta::assert_debug_snapshot;
    use rstest::Context;
    use rstest::rstest;    

    #[derive(Debug, Default, PartialEq)]
    struct TestReceiver {
        hdrs: Vec<String>,
        finished: bool,
    }

    use crate::{H11MaybeValue, H11UnknownField};
    
    impl HeaderReceiver for TestReceiver {
        fn req_header<'h, 'd>(&mut self, hdr: H11Header<'h>) -> RespIndicative<'d> {
            let out = match hdr {
                H11Header::Unknown(H11UnknownField(field), H11MaybeValue::Bytes(value)) => {
                    format!("Hu[{}={}]", core::str::from_utf8(field).unwrap(), core::str::from_utf8(value).unwrap())
                },
                _ => format!("Hd{:?}", hdr),
            };
            self.hdrs.push(out);
            RespIndicative::GoAhead
        }
        fn req_headers_finish(&mut self) -> () {
            self.finished = true;
        }
    }

    pub(crate) fn ctx_insta(ctx: Context) -> String {
        let case_id = ctx.case.unwrap().to_string();
        format!("{}-{}", case_id, ctx.name)
    }    
    
    #[derive(Debug)]
    #[allow(dead_code)]
    struct InternalTc {
        input: &'static str,
        output: TestReceiver,
    }

    #[rstest]
    #[case("A: ffffoo.bar\r\nB: foo\r\n\r\n", true, 0)]
    #[case("\r\n\r\n", true, 0)]
    #[case("\r\n\r\nfoo", true, 3)]
    fn headers_complete(#[context] ctx: Context, #[case] tc_input: &'static str, #[case] expected_finish: bool, #[case] offset: usize) {
        let mut p = HeaderParser::default();
        let mut r = TestReceiver::default();

        let data = tc_input.as_bytes();
        let res = p.parse(&mut r, &data);

        assert_eq!(r.finished, expected_finish);
        assert_eq!(res, Ok(tc_input.len()-offset));        

        assert_debug_snapshot!(ctx_insta(ctx), InternalTc { input: tc_input, output: r });

    }
}
