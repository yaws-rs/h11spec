//! Builder Sender traits

use crate::H11Header;
use crate::RespComplete;
use core::num::NonZero;

/// Provide the response sizing
pub enum ResponseBodySize {
    /// Fixed size response body to be sent
    Fixed(NonZero<usize>),
    /// Dynamic size (unknown length) streamed response body to be sent
    Dynamic,
}

/// Implement to provide Response to a request
pub trait ResponseSender {
    /// Provide the response status to be sent
    fn resp_on_status<'uri>(&mut self) -> RespComplete<'uri>;
    /// Provide the headers to be sent from the given header array position.
    /// This will be called multiple times until the array is empty to denote completion.
    fn resp_on_headers<'h>(&'h mut self, _at: usize) -> &'h [H11Header<'h>];
    /// What size of body length are going to be sent
    fn resp_body_len(&mut self) -> ResponseBodySize;
    /// Provide some body to be sent from the current position for the given max length
    fn resp_on_body(&mut self, _len: usize) -> &[u8];
    /// Called when the response was completed
    fn resp_on_complete(&mut self) -> ();
}
