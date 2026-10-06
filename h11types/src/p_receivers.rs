//! Parsing Receiver traits

use crate::RespIndicative;

use crate::{H11Method, H11Version};

/// Implement to receive URI Target
pub trait TargetReceiver {
    /// Start streaming an URI
    fn req_target_init(&mut self, _target_part: &[u8]) -> ();
    /// Add to started URI
    fn req_target_add(&mut self, _target_part: &[u8]) -> ();
    /// Finish URI
    fn req_target_finish(&mut self) -> ();
}

use crate::H11Header;

/// Implement to reveive Headers
pub trait HeaderReceiver {
    /// Single Header
    fn req_header<'h, 'd>(&mut self, hdr: H11Header<'h>) -> RespIndicative<'d>;
    /// Headers processing has finished
    fn req_headers_finish(&mut self) -> ();
}

/// Implement to receive parsed request
pub trait MetaReceiver {
    /// Provides request method
    fn req_method(&mut self, _mthd: H11Method) -> ();
    /// Provides request version
    fn req_version(&mut self, _version: H11Version) -> ();
    /// Indicates start of headers processing and completion of request line
    fn req_start_headers(&mut self) -> ();
    /// Indicates start of payload processing and completion of headers
    fn req_start_payload(&mut self) -> ();
    /// Indicates completion of entire request
    fn req_complete(&mut self) -> ();
    /// Provide target receiver
    fn impl_target(&mut self) -> impl TargetReceiver;
}

/// Marker to use when no header receiver is used
pub struct NoReceiver;

impl TargetReceiver for &mut NoReceiver {
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

impl HeaderReceiver for NoReceiver {
    fn req_header<'h, 'd>(&mut self, _hdr: H11Header<'h>) -> RespIndicative<'d> {
        RespIndicative::GoAhead
    }
    fn req_headers_finish(&mut self) -> () {
        ()
    }
}

impl MetaReceiver for NoReceiver {
    fn req_method(&mut self, _mthd: H11Method) -> () {
        ()
    }
    fn req_version(&mut self, _version: H11Version) -> () {
        ()
    }
    fn req_start_headers(&mut self) -> () {
        ()
    }
    fn req_start_payload(&mut self) -> () {
        ()
    }
    fn req_complete(&mut self) -> () {
        ()
    }
    fn impl_target(&mut self) -> impl TargetReceiver {
        self
    }
}
