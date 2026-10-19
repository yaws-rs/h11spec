//! Rick-Roll as a Service (RRaaS)
//!
//! It's only job is to Rick Roll everyone, everywhere and any time regardless of consequences etc.
//! It does not care whether you wanted a favicon.ico or something else, you'll get a Rick!
//!
//! # Usage
//!
//! 1. Place your favorite Rick somewhere in your filesystem you compile this curse in
//! 2. Find the below configuration and set both the rick image location & it's size

//-------------------------------------------
// The default size is this:
// - https://media.tenor.com/Lw4_T2lRPGoAAAAM/rickroll-meme.gif
//
// Set below things for your own copy:
//-------------------------------------------
// CHANGE THE BELOW FOR THE LOCATION OF RICK:
//const RICK_ROLL: &[u8] = core::include_bytes!("/tmp/rick-roll.gif");

// CHANGE THE BELOW FOR THE SIZE OF RICK:
//const RICK_SIZE: &str = "461688";
//-------------------------------------------

use blueprint_h11spec::Reset;
use blueprint_h11spec::Resp2xx;
use blueprint_h11spec::{H11Header, H11MaybeValue};
use blueprint_h11spec::{H11Method, H11Version};
use blueprint_h11spec::{
    RequestBodyReceiver, RequestHeaderReceiver, RequestReceiver, RequestTargetReceiver,
};
use blueprint_h11spec::{RespComplete, RespIndicative};
use blueprint_h11spec::{ResponseBodySize, ResponseSender};

/// sus
pub struct RickRoll {
    payload_out_cursor: PayloadCursor,
    //headers: [H11Header<'static>; 1],
    rick: &'static [u8],
    rick_len_buf: itoa::Buffer,
}

/// Stateful cursor for sending / receiving bytes
#[derive(Debug, Default)]
pub struct PayloadCursor {
    consumed: usize,
}

impl Reset for RickRoll {
    fn reset(&mut self) {
        self.payload_out_cursor = PayloadCursor::default();
    }
}

impl RickRoll {
    /// Initialize a Rick-Roll app with the given payload (GIF)
    pub fn with_rick(rick: &'static [u8]) -> Self {
        Self {
            payload_out_cursor: PayloadCursor::default(),
            rick,
            rick_len_buf: itoa::Buffer::new(),
        }
    }
}

impl Default for RickRoll {
    fn default() -> Self {
        const RICK_ROLL: &[u8] = core::include_bytes!("/tmp/rick-roll.gif");

        Self::with_rick(RICK_ROLL)
    }
}

impl RequestReceiver for RickRoll {
    /// Provides request method but we'll just ignore it given our only purpose is to send a Rick
    fn req_method(&mut self, _mthd: H11Method) {
        //self.method = Some(mthd);
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
        RespIndicative::GoAhead
    }
}

impl RequestTargetReceiver for &mut RickRoll {
    // URI can be long so typically we get the first block of it through this
    #[inline]
    fn req_target_init(&mut self, _target_part: &[u8]) {}
    // Then we might get some more of the URL
    #[inline]
    fn req_target_add(&mut self, _target_part: &[u8]) {}
    // Until we don't get anymore of the URL
    #[inline]
    fn req_target_finish(&mut self) {}
}

impl RequestHeaderReceiver for RickRoll {
    // We don't really care about headers
    fn req_header<'h>(&mut self, _hdr: H11Header<'h>) -> RespIndicative {
        RespIndicative::GoAhead
    }
    // Nor we care about when request headers finished
    fn req_headers_finish(&mut self) -> RespIndicative {
        RespIndicative::GoAhead
    }
}

impl RequestBodyReceiver for RickRoll {
    /// Receive a chunk of data but we'll just ignore it
    fn req_body_add(&mut self, _chunk: &[u8]) -> RespIndicative {
        RespIndicative::GoAhead
    }
}

impl ResponseSender for RickRoll {
    #[inline]
    // We want to tell them that they have succeeded in requesting a Rick whether they asked it for or not
    fn resp_on_status(&mut self) -> RespComplete {
        RespComplete::Resp2xx(Resp2xx::Ok)
    }
    fn resp_on_header_count(&mut self) -> usize {
        1
    }
    // We need to tell them how much of a rick they will be getting through headers
    #[inline]
    fn resp_on_header<'h>(&'h mut self, at: usize) -> H11Header<'h> {
        match at {
            0 => H11Header::ContentLength(H11MaybeValue::Integer(
                self.rick_len_buf.format(self.rick.len()).as_bytes(),
            )),
            _ => unreachable!(),
        }
    }
    // Runtime asks us how much rick of we are going to send
    #[inline]
    fn resp_body_len(&mut self) -> ResponseBodySize {
        ResponseBodySize::Fixed(self.rick.len())
    }
    // Runtime wants more rick so let's send them more rick
    #[inline]
    fn resp_on_body(&mut self, len: usize) -> &[u8] {
        let rick_len = self.rick.len();

        if self.payload_out_cursor.consumed == rick_len {
            return &[];
        }

        let consumed = self.payload_out_cursor.consumed;
        let remaining_capacity = len;

        if remaining_capacity > 0 {
            let roll_end = if rick_len < consumed + remaining_capacity {
                rick_len
            } else {
                consumed + remaining_capacity
            };
            self.payload_out_cursor.consumed += roll_end - consumed;
            &self.rick[consumed..roll_end]
        } else {
            &[]
        }
    }
    // Rick does not care whether this ever completes, they'll get some rick anyways!
    fn resp_on_complete(&mut self) {}
}
