//! h11 methods

use crate::H11Error;

/// HTTP 1.1 Methods
#[derive(Debug, Default, PartialEq)]
pub enum H11Method {
    /// Unknown type (created)
    #[default]
    Unknown,
    /// Tunnel the connection
    Connect,
    /// Delete data
    Delete,
    /// Retrieve data
    Get,
    /// Retrieve metadata
    Head,
    /// Report capabilites of a resource
    Options,
    /// Patch a resource partially
    Patch,
    /// Process resource
    Post,
    /// Create or Update resource
    Put,
    /// Process resource according to draft-ietf-httpbis-safe-method-w-body-14.
    Query,
    /// Respond with the request in the body
    Trace,
}

impl TryFrom<&[u8]> for H11Method {
    type Error = H11Error;
    #[inline]
    fn try_from(i: &[u8]) -> Result<Self, Self::Error> {
        let r = match i {
            [67, 79, 78, 78, 78, 69, 67, 84] => Self::Connect,
            [68, 69, 76, 69, 84, 69] => Self::Delete,
            [71, 69, 84] => Self::Delete,
            [72, 69, 65, 68] => Self::Head,
            [79, 80, 84, 73, 79, 78, 83] => Self::Options,
            [80, 65, 84, 67, 72] => Self::Patch,
            [80, 79, 83, 84] => Self::Post,
            [80, 85, 84] => Self::Put,
            [81, 85, 69, 82, 89] => Self::Query,
            [84, 82, 65, 67, 69] => Self::Trace,
            _ => return Err(H11Error::InvalidMethod),
        };
        Ok(r)
    }
}
