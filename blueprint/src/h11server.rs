//! h11spec Server blueprints

mod chunked;
use chunked::ChunkRelay;
use ychunked::{ChunkDecoder, DecodingStage};

use crate::H11Error;
use crate::{borrow_remaining_left_b, try_fill_left_with, FillAttempt};
use blueprint::BluePrint;
use blueprint::Orbit;
use blueprint::{InBuffer, Left, Right};
use h11types::{HeaderStatus, RequestBodySize, StartLineStatus};

/// empty for now
pub struct Position;

#[doc(inline)]
pub use h11types::{H11Header, H11MaybeValue, H11Method, H11TransferEncoding, H11Version, Uri};
#[doc(inline)]
pub use h11types::{NoRequestReceiver, NoRequestTargetReceiver};
#[doc(inline)]
pub use h11types::{
    RequestBodyReceiver, RequestHeaderReceiver, RequestReceiver, RequestTargetReceiver,
};
#[doc(inline)]
pub use h11types::{
    Resp2xx, RespComplete, RespIndicative, ResponseBodySize, ResponseSender, StatusDesc,
};

/// Reset App
pub trait Reset {
    /// Implement to reset for the next request
    fn reset(&mut self);
}

/// State machine stages
#[derive(Debug, Default, PartialEq)]
pub enum H11ServingStage {
    /// Expect Request Start line
    #[default]
    Start,
    /// Expect Request Headers or <CRLF><CRLF>
    Headers,
    /// Expect Request Body
    Body,
    /// Expect Response
    Response,
    /// Expect Shutdown
    Shutdown,
    /// Expect Recycle
    Recycle,
    /// Bad Client Request
    BadRequest,
}

use h11types::H11RequestMeta;

#[derive(Debug, Default, PartialEq)]
enum StatusOut {
    #[default]
    Initialized,
    HeadersSent(usize),
    HeadersCompleted,
    SendBody,
    Complete,
}

/// H11Server Serving context
#[derive(Debug, Default)]
pub struct H11Serving<App> {
    stage: H11ServingStage,
    req_meta: H11RequestMeta,
    req_relayed: usize,
    status_out: StatusOut,
    expected_response_len: ResponseBodySize,
    expected_filled: usize,
    app: App,
}

impl<App: Reset> H11Serving<App> {
    fn reset_app(&mut self) {
        self.stage = H11ServingStage::default();
        self.status_out = StatusOut::default();
        self.expected_response_len = ResponseBodySize::default();
        self.expected_filled = 0;
        self.app.reset();
    }
}

const CRLF: &[u8] = "\r\n".as_bytes();

impl<App> Orbit for H11Serving<App>
where
    App: ResponseSender + RequestReceiver + RequestHeaderReceiver + RequestBodyReceiver + Reset,
{
    type Position = Position;
    type Error = H11Error;
    fn advance_with<B, L: Left, R: Right>(
        &mut self,
        _u: &mut B,
        l: &mut L,
        _r: &mut R,
    ) -> Result<Self::Position, Self::Error> {
        let (left_in_len, left_out_len) = l.left_lens();
        let (left_inputs, left_out_b) = l.left_bufs_mut();

        let mut left_in_b = match left_inputs {
            InBuffer::Single(buf) => buf,
            InBuffer::Double(_buf1, _buf2) => {
                todo!()
            }
        };

        if left_in_len == 0 && self.stage == H11ServingStage::Start {
            return Ok(Position);
        }

        let mut out_len = left_out_len;
        let mut discard = 0;

        loop {
            match self.stage {
                H11ServingStage::Start => {
                    let advanced = match self.req_meta.advance_status_with(&mut self.app, left_in_b)
                    {
                        Ok(StartLineStatus::Complete(advanced)) => advanced,
                        Ok(StartLineStatus::Incomplete(_, _)) => {
                            break;
                        }
                        Err(e) => return Err(e),
                    };

                    let _ = left_in_b.split_off_mut(..advanced);
                    discard += advanced;

                    if self.req_meta.status_complete() {
                        self.app.req_start_headers();
                        self.stage = H11ServingStage::Headers;
                        continue;
                    }
                }
                H11ServingStage::Headers => {
                    match self.req_meta.advance_headers_with(&mut self.app, left_in_b) {
                        Ok(HeaderStatus::Incomplete(b, _)) | Ok(HeaderStatus::Complete(b)) => {
                            discard += b;
                            _ = left_in_b.split_off_mut(..b);
                        }
                        Err(e) => panic!("Headers advance error {:?}", e),
                    };

                    if self.req_meta.headers_complete() {
                        if self.req_meta.body_size().is_empty() {
                            self.app.req_complete();
                            self.stage = H11ServingStage::Response;
                        } else {
                            self.app.req_start_payload();
                            self.stage = H11ServingStage::Body;
                        }

                        continue;
                    }
                }
                H11ServingStage::Body => {
                    if let RequestBodySize::Fixed(expected_len) = self.req_meta.body_size() {
                        let remaining = expected_len.get() - self.req_relayed;
                        let relay_size = if left_in_b.len() >= remaining {
                            self.stage = H11ServingStage::Response;
                            remaining
                        } else {
                            left_in_b.len()
                        };
                        match left_in_b.split_off_mut(..relay_size) {
                            Some(relay_b) => {
                                discard += relay_b.len();
                                self.app.req_body_add(&relay_b);
                            }
                            None => {}
                        }
                    } else {
                        let mut chunk_relay = ChunkRelay { myself: self };
                        let mut decoder = ChunkDecoder::new();
                        let res = decoder.decode_detached(&mut chunk_relay, left_in_b);

                        let advanced = match res {
                            Ok(DecodingStage::Complete(discard_add)) => {
                                self.stage = H11ServingStage::Response;
                                discard_add
                            }
                            Ok(DecodingStage::WaitingSize(discard_add)) => discard_add,
                            Ok(DecodingStage::Decoding(discard_add, _pending)) => discard_add,
                            Ok(DecodingStage::WaitingCrLf(discard_add)) => discard_add,
                            Err(_) => {
                                self.stage = H11ServingStage::BadRequest;
                                continue;
                            }
                        };
                        discard += advanced;
                        let _ = left_in_b.split_off_mut(..advanced);
                    }
                }
                H11ServingStage::Response => {
                    let mut out_pos = 0;

                    if self.status_out == StatusOut::Initialized {
                        let resp_status = self.app.resp_on_status();
                        let out_status_b = resp_status.as_bytes_out();

                        let remaining = borrow_remaining_left_b(left_out_b, out_pos, 22);

                        let hvers = "HTTP/1.1 ".as_bytes();

                        let filled =
                            match try_fill_left_with(remaining, &[hvers, out_status_b, CRLF]) {
                                FillAttempt::Complete(filled) => filled,
                                FillAttempt::Overflow => break,
                            };
                        out_pos += filled;
                        out_len += filled;

                        self.expected_response_len = self.app.resp_body_len();
                        self.status_out = StatusOut::HeadersSent(0);
                    }

                    if let StatusOut::HeadersSent(ref mut sent) = self.status_out {
                        let hdr_count = self.app.resp_on_header_count();

                        if *sent >= hdr_count {
                            self.status_out = StatusOut::HeadersCompleted;
                            continue;
                        }

                        for x in *sent..hdr_count {
                            let resp_header = self.app.resp_on_header(x);

                            let (out_header_field_b, out_header_value_b) =
                                resp_header.as_bytes_out();

                            let remaining = borrow_remaining_left_b(left_out_b, out_pos, 22);

                            let hdr_line_combined = &[
                                out_header_field_b,
                                ": ".as_bytes(),
                                out_header_value_b,
                                CRLF,
                            ];

                            let filled = match try_fill_left_with(remaining, hdr_line_combined) {
                                FillAttempt::Complete(filled) => filled,
                                FillAttempt::Overflow => break,
                            };
                            out_pos += filled;
                            out_len += filled;
                            *sent += 1;
                        }

                        self.status_out = StatusOut::HeadersCompleted;
                    };

                    if self.status_out == StatusOut::HeadersCompleted {
                        let remaining = borrow_remaining_left_b(left_out_b, out_pos, 22);
                        let filled = match try_fill_left_with(remaining, &[CRLF]) {
                            FillAttempt::Complete(filled) => filled,
                            FillAttempt::Overflow => break,
                        };
                        out_pos += filled;
                        out_len += filled;

                        match self.expected_response_len {
                            ResponseBodySize::Fixed(_) | ResponseBodySize::Dynamic => {
                                self.status_out = StatusOut::SendBody;
                            }
                            _ => {
                                self.status_out = StatusOut::Complete;
                            }
                        }
                    }

                    if self.status_out == StatusOut::SendBody {
                        let buf_remaining = borrow_remaining_left_b(left_out_b, out_pos, 22);

                        let remaining_len = match self.expected_response_len {
                            ResponseBodySize::Fixed(l) => {
                                let expected_remaining = l - self.expected_filled;
                                let feed_rate = if buf_remaining.len() >= expected_remaining {
                                    self.status_out = StatusOut::Complete;
                                    expected_remaining
                                } else {
                                    buf_remaining.len()
                                };
                                self.expected_filled += feed_rate;
                                feed_rate
                            }
                            ResponseBodySize::Dynamic => buf_remaining.len(),
                            ResponseBodySize::Nothing => {
                                self.status_out = StatusOut::Complete;
                                0
                            }
                        };

                        if remaining_len > 0 {
                            let resp_body = self.app.resp_on_body(remaining_len);
                            if !resp_body.is_empty() {
                                let filled = match try_fill_left_with(buf_remaining, &[resp_body]) {
                                    FillAttempt::Complete(filled) => filled,
                                    FillAttempt::Overflow => break,
                                };
                                //out_pos += filled;
                                out_len += filled;
                            } else {
                                self.status_out = StatusOut::Complete;
                            };
                        }
                    }

                    if self.status_out == StatusOut::Complete {
                        self.stage = H11ServingStage::Recycle;
                    }

                    break;
                }
                H11ServingStage::Shutdown => {
                    break;
                }
                H11ServingStage::Recycle => {
                    break;
                }
                H11ServingStage::BadRequest => {
                    let remaining = borrow_remaining_left_b(left_out_b, 0, 22);

                    let bad_request =
                        "HTTP/1.1 400 Bad Request\r\nConnection: close\r\n\r\n".as_bytes();

                    let filled = match try_fill_left_with(remaining, &[bad_request, CRLF]) {
                        FillAttempt::Complete(filled) => filled,
                        FillAttempt::Overflow => break,
                    };
                    out_len += filled;
                    self.stage = H11ServingStage::Shutdown;
                    l.shutdown();
                    break;
                }
            }
        }

        let new_in_len = left_in_len - discard;

        l.left_set_lens(new_in_len, out_len);

        if self.stage == H11ServingStage::Recycle {
            self.reset_app();
        }

        Ok(Position)
    }
}

/// H11Server which implements the [`Orbit`] and [`BluePrint`] traits.
pub struct H11SpecServer;
/// Placeholder for H11ServerConfig
pub struct H11ServerConfig;

impl<App> H11Serving<App>
where
    App: ResponseSender
        + RequestReceiver
        + RequestHeaderReceiver
        + RequestBodyReceiver
        + Reset
        + Default,
{
    /// Initialize H11Serving with the specified pre-initialized App
    pub fn with_app(app: App) -> Self {
        Self {
            stage: H11ServingStage::default(),
            req_meta: H11RequestMeta::default(),
            req_relayed: 0,
            status_out: StatusOut::default(),
            expected_response_len: ResponseBodySize::default(),
            expected_filled: 0,
            app,
        }
    }
}

impl<App> BluePrint<H11Serving<App>> for H11SpecServer
where
    App: ResponseSender
        + RequestReceiver
        + RequestHeaderReceiver
        + RequestBodyReceiver
        + Reset
        + Default,
{
    type Config = H11ServerConfig;
    type Error = H11Error;

    fn with_defaults() -> Result<H11Serving<App>, Self::Error> {
        Ok(H11Serving::default())
    }
    fn with_configuration(_: Self::Config) -> Result<H11Serving<App>, Self::Error> {
        todo!()
    }
}
