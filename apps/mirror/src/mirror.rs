//! Mirror as a Service

use blueprint_h11spec::Resp2xx;
use blueprint_h11spec::{H11Header, H11MaybeValue};
use blueprint_h11spec::{H11Method, H11Version};
use blueprint_h11spec::{
    RequestBodyReceiver, RequestHeaderReceiver, RequestReceiver, RequestTargetReceiver,
};
use blueprint_h11spec::{RespComplete, RespIndicative};
use blueprint_h11spec::{ResponseBodySize, ResponseSender};

use alloc::string::String;
use alloc::vec::Vec;
//use crate::alloc::string::ToString;

/// Mirror YAWS App
pub struct Mirror {
    payload_out_cursor: PayloadCursor,
    headers_in: Vec<String>,
    test_data: Vec<u8>,
    body: Vec<u8>,
    body_expected_len: Option<usize>,
    method: H11Method,
    target: Vec<u8>,
    target_completed: bool,
    req_completed: bool,
    bad_request: Option<&'static str>,
    resp_completed: bool,
}

/// Stateful cursor for sending / receiving bytes
#[derive(Debug, Default)]
pub struct PayloadCursor {
    consumed: usize,
}

impl Default for Mirror {
    fn default() -> Self {
        Self {
            payload_out_cursor: PayloadCursor::default(),
            headers_in: Vec::with_capacity(20),
            body: Vec::with_capacity(8192),
            body_expected_len: None,
            method: H11Method::default(),
            target: Vec::with_capacity(255),
            target_completed: false,
            req_completed: false,
            bad_request: None,
            resp_completed: true,
            test_data: Vec::with_capacity(12),
        }
    }
}

impl RequestReceiver for Mirror {
    /// Provides request method
    fn req_method(&mut self, mthd: H11Method) {
        self.method = mthd;
    }
    /// Provide target receiver
    fn impl_target(&mut self) -> impl RequestTargetReceiver {
        self
    }
    /// Provides request version
    fn req_version(&mut self, _version: H11Version) {}
    /// Indicates start of headers processing and completion of request line
    fn req_start_headers(&mut self) -> RespIndicative {
        RespIndicative::GoAhead
    }
    /// Indicates start of payload processing and completion of headers
    fn req_start_payload(&mut self) -> RespIndicative {
        RespIndicative::GoAhead
    }
    /// Indicates completion of entire request
    fn req_complete(&mut self) -> RespIndicative {
        self.req_completed = true;
        RespIndicative::GoAhead
    }
}

impl RequestTargetReceiver for &mut Mirror {
    // URI can be long so typically we get the first block of it through this
    #[inline]
    fn req_target_init(&mut self, target_part: &[u8]) {
        self.target.extend(target_part);
    }
    // Then we might get some more of the URL
    #[inline]
    fn req_target_add(&mut self, target_part: &[u8]) {
        self.target.extend(target_part);
    }
    // Until we don't get anymore of the URL
    #[inline]
    fn req_target_finish(&mut self) {
        self.target_completed = true;
    }
}

impl RequestHeaderReceiver for Mirror {
    // We don't really care about headers
    fn req_header<'h>(&mut self, hdr: H11Header<'h>) -> RespIndicative {
        match hdr {
            H11Header::ContentLength(H11MaybeValue::Integer(l)) => {
                let lstr = match core::str::from_utf8(l) {
                    Ok(s) => s,
                    Err(_) => {
                        self.bad_request = Some("Bad Non-UTF8 Content-Length integer");
                        return RespIndicative::GoAhead;
                    }
                };
                match self.body_expected_len {
                    None => {
                        self.body_expected_len = match usize::from_str_radix(lstr, 10) {
                            Ok(u) => {
                                self.test_data.extend(l);
                                Some(u)
                            }
                            Err(_) => {
                                self.bad_request = Some("Bad content-length.");
                                None
                            }
                        };
                    }
                    // This should never happen
                    Some(_) => {
                        self.bad_request = Some("Multiple Content-Length headers.");
                    }
                }
            }
            _ => {}
        }
        self.headers_in.push(format!("{:?}", hdr));
        RespIndicative::GoAhead
    }
    // Nor we care about when request headers finished
    fn req_headers_finish(&mut self) -> RespIndicative {
        RespIndicative::GoAhead
    }
}

impl RequestBodyReceiver for Mirror {
    fn req_body_add(&mut self, chunk: &[u8]) -> RespIndicative {
        self.body.extend(chunk);
        RespIndicative::GoAhead
    }
}

impl ResponseSender for Mirror {
    #[inline]
    fn resp_on_status(&mut self) -> RespComplete {
        RespComplete::Resp2xx(Resp2xx::Ok)
    }
    #[inline]
    fn resp_on_header_count(&mut self) -> usize {
        1
    }
    #[inline]
    fn resp_on_header<'h>(&'h mut self, at: usize) -> H11Header<'h> {
        if at == 0 {
            let size_out = self.body.len();
            self.test_data = format!("{size_out}").into_bytes();
            H11Header::ContentLength(H11MaybeValue::Integer(&self.test_data))
        } else {
            panic!("Was supposed to only request one header")
        }
    }
    #[inline]
    fn resp_body_len(&mut self) -> ResponseBodySize {
        ResponseBodySize::Fixed(self.body.len())
    }
    #[inline]
    fn resp_on_body(&mut self, len: usize) -> &[u8] {
        let mirror_len = self.body.len();

        if self.payload_out_cursor.consumed == mirror_len {
            return &[];
        }

        let consumed = self.payload_out_cursor.consumed;
        let remaining_capacity = len;

        if remaining_capacity > 0 {
            let roll_end = if mirror_len < consumed + remaining_capacity {
                mirror_len
            } else {
                consumed + remaining_capacity
            };
            self.payload_out_cursor.consumed += roll_end - consumed;
            &self.body[consumed..roll_end]
        } else {
            &[]
        }
    }
    fn resp_on_complete(&mut self) {
        self.resp_completed = true;
    }
}
