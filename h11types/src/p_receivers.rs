//! Parsing Receiver traits

use crate::RespIndicative;
//use crate::RespComplete;
//use crate::{Resp2xx, StatusDesc};

use crate::{H11Method, H11Version};

/// Implement to receive parsed request
pub trait RequestReceiver {
    /// Provides request method
    fn req_method(&mut self, _mthd: H11Method) -> ();
    /// Provide target receiver
    fn impl_target(&mut self) -> impl RequestTargetReceiver;
    /// Provides request version
    fn req_version(&mut self, _version: H11Version) -> ();
    /// Indicates start of headers processing and completion of request line
    fn req_start_headers<'d>(&mut self) -> RespIndicative<'d>;
    /// Indicates start of payload processing and completion of headers
    fn req_start_payload<'d>(&mut self) -> RespIndicative<'d>;
    /// Indicates completion of entire request
    fn req_complete<'d>(&mut self) -> RespIndicative<'d>;
}

/// Implement to receive URI Target
pub trait RequestTargetReceiver {
    /// Start streaming an URI
    fn req_target_init(&mut self, _target_part: &[u8]) -> ();
    /// Add to started URI
    fn req_target_add(&mut self, _target_part: &[u8]) -> ();
    /// Finish URI
    fn req_target_finish(&mut self) -> ();
}

/// Use to ignore request Target
pub struct NoRequestTargetReceiver;

impl RequestTargetReceiver for NoRequestTargetReceiver {
    #[inline]
    fn req_target_init(&mut self, _target_part: &[u8]) -> () {
        ()
    }
    #[inline]
    fn req_target_add(&mut self, _target_part: &[u8]) -> () {
        ()
    }
    #[inline]
    fn req_target_finish(&mut self) -> () {
        ()
    }
}

use crate::H11Header;

/// Implement to reveive Request Headers
pub trait RequestHeaderReceiver {
    /// Single Header
    fn req_header<'h, 'd>(&mut self, hdr: H11Header<'h>) -> RespIndicative<'d>;
    /// Headers processing has finished
    fn req_headers_finish<'d>(&mut self) -> RespIndicative<'d>;
}

/// Implement to reveive Parsed Headers
pub trait HeaderReceiver {
    /// Single Header
    fn req_header<'h>(&mut self, hdr: H11Header<'h>) -> ();
    /// Headers processing has finished
    fn req_headers_finish(&mut self) -> ();
}

/// Implement to receive body
pub trait RequestBodyReceiver {
    /// Receive a chunk of data
    fn payload_chunk<'d>(&mut self, _chunk: &[u8]) -> RespIndicative<'d>;
}

/// Marker to use when no header receiver is used
pub struct NoRequestReceiver;

impl RequestTargetReceiver for &mut NoRequestReceiver {
    fn req_target_init(&mut self, _target_part: &[u8]) -> () {
        ()
    }
    fn req_target_add(&mut self, _target_part: &[u8]) -> () {
        ()
    }
    fn req_target_finish(&mut self) -> () {
        ()
    }
}

impl RequestHeaderReceiver for NoRequestReceiver {
    fn req_header<'h, 'd>(&mut self, _hdr: H11Header<'h>) -> RespIndicative<'d> {
        RespIndicative::GoAhead
    }
    fn req_headers_finish<'d>(&mut self) -> RespIndicative<'d> {
        RespIndicative::GoAhead
    }
}

impl RequestReceiver for NoRequestReceiver {
    fn req_method(&mut self, _mthd: H11Method) -> () {
        ()
    }
    fn req_version(&mut self, _version: H11Version) -> () {
        ()
    }
    fn req_start_headers<'d>(&mut self) -> RespIndicative<'d> {
        RespIndicative::GoAhead
    }
    fn req_start_payload<'d>(&mut self) -> RespIndicative<'d> {
        RespIndicative::GoAhead
    }
    fn req_complete<'d>(&mut self) -> RespIndicative<'d> {
        RespIndicative::GoAhead
        //RespComplete::Resp2xx(Resp2xx::Ok(StatusDesc::ok()))
    }
    fn impl_target(&mut self) -> impl RequestTargetReceiver {
        self
    }
}
