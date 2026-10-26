//! h11spec types
///
/// Reflects RFC 9112 (June 2022)
///
/// # Transfer coding
/// Registry at https://www.iana.org/assignments/http-parameters
///
mod method;
use core::num::NonZero;
pub use method::*;

/// HTTP Transfer-Encoding
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum H11TransferEncoding {
    /// None
    #[default]
    None,
    /// Chunked encoding
    Chunked,
}

impl H11TransferEncoding {
    /// Header value as bytes
    pub fn as_bytes_out(&self) -> &[u8] {
        match self {
            Self::None => &[],
            Self::Chunked => "chunked".as_bytes(),
        }
    }
}

/// HTTP Version
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum H11Version {
    /// Unknown version
    #[default]
    Unknown,
    /// HTTP/1.1
    Http11,
}

/// HTTP Connection type - RFC 9110 7.6.1
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum H11Connection<'h> {
    /// Close
    #[default]
    Close,
    /// Keep-Alive
    KeepAlive,
    /// HTTP2 Upgrade
    Http2Upgrade,
    /// Unknown Other
    Other(&'h [u8]),
}

impl<'h> H11Connection<'h> {
    /// Conection header as bytes
    pub fn as_bytes_out(&self) -> &[u8] {
        match self {
            Self::Close => "Close",
            Self::KeepAlive => "Keep-Alive",
            Self::Http2Upgrade => "Upgrade, HTTP2-Settings",
            Self::Other(o) => return o,
        }
        .as_bytes()
    }
}

/// HTTP Compression scheme
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum H11TransferCompression {
    /// None
    #[default]
    None,
    /// Compress
    Compress,
    /// Deflate
    Deflate,
    /// Gzip
    Gzip,
    /// Brotli
    Br,
    /// Zstd
    Zstd,
}

/// Request sizing                                                                                                       
#[derive(Debug, PartialEq)]
pub enum RequestBodySize {
    /// Fixed size request body to be streamed
    Fixed(NonZero<usize>),
    /// Chunked size (unknown length) request body to be streamed
    Chunked,
    /// Dynamic size (unknown length) request body to be streamed
    Dynamic,
    /// No request body
    Empty,
}

impl RequestBodySize {
    /// Is the request body empty?
    #[inline]
    pub const fn is_empty(&self) -> bool {
        matches!(self, RequestBodySize::Empty)
    }
}

/// Used by the server
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct H11RequestMeta {
    pub(crate) method: H11Method,
    pub(crate) target_loc: Option<(usize, usize)>,
    pub(crate) version: H11Version,
    pub(crate) transfer_encoding: H11TransferEncoding,
    pub(crate) transfer_compression: H11TransferCompression,
    pub(crate) body_length: Option<usize>,
    pub(crate) headers_end: Option<usize>,
}

impl H11RequestMeta {
    /// Is the status line parsing complete?
    #[inline]
    pub fn status_complete(&self) -> bool {
        self.method != H11Method::Unknown
            && self.target_loc.is_some()
            && self.version != H11Version::Unknown
    }
    /// Are the headers complete?
    #[inline]
    pub fn headers_complete(&self) -> bool {
        self.headers_end.is_some()
    }
    /// Request body size
    pub fn body_size(&self) -> RequestBodySize {
        let l = match self.body_length {
            Some(s) => match s {
                0 => RequestBodySize::Empty,
                _ => RequestBodySize::Fixed(NonZero::new(s).expect("Bug")),
            },
            None => RequestBodySize::Empty,
        };
        if let H11TransferEncoding::Chunked = self.transfer_encoding {
            return RequestBodySize::Chunked;
        }
        return l;
    }
    /// Request method
    pub fn method(&self) -> H11Method {
        self.method
    }
}

/// HTTP/1.1 RFC 9110 Mandated Headers
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum H11Header<'h> {
    /// RFC 9110 s.
    Accept(H11MaybeValue<'h>),
    /// RFC 9110 s. 7.6.1
    Connection(H11Connection<'h>),
    /// RFC 9110 s. 8.6
    ContentLength(H11MaybeValue<'h>),
    /// RFC 9110 s. 7.2
    Host(H11MaybeValue<'h>),
    /// HTTP/2 Upgrade
    Http2Settings(H11MaybeValue<'h>),
    /// draft-ietf-httpbis-rfc6265bis-22 s. 5.8.1 & 4.1
    SetCookie(H11MaybeValue<'h>),
    /// RFC 9110 s. 6.1
    TransferEncoding(H11TransferEncoding),
    /// RFC 9110
    Upgrade(H11MaybeValue<'h>),
    /// RFC 9110
    UserAgent(H11MaybeValue<'h>),
    /// Unknown header field containing maybe a header value
    Unknown(H11UnknownField<'h>, H11MaybeValue<'h>),
}

/// Header field is unknown
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct H11UnknownField<'h>(pub &'h [u8]);

impl<'h> H11UnknownField<'h> {
    /// Header field as bytes
    pub fn as_bytes_out(&self) -> &[u8] {
        self.0
    }
}

/// Non-validated Header value that may be valid
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum H11MaybeValue<'h> {
    /// That looks like Integer value
    Integer(&'h [u8]),
    /// That looks like Bytes value
    Bytes(&'h [u8]),
}

impl<'h> H11MaybeValue<'h> {
    /// Header value as bytes
    pub fn as_bytes_out(&self) -> &[u8] {
        match self {
            Self::Integer(i) => i,
            Self::Bytes(b) => b,
        }
    }
}

use crate::{H11Error, HeaderValidationError};

impl<'h> TryFrom<H11MaybeValue<'h>> for usize {
    type Error = H11Error;
    fn try_from(from: H11MaybeValue<'h>) -> Result<usize, Self::Error> {
        match from {
            H11MaybeValue::Integer(i) => {
                let s = match core::str::from_utf8(i) {
                    Ok(s) => s,
                    Err(_) => {
                        return Err(H11Error::InvalidHeaderValueNoPos(
                            HeaderValidationError::ExpectedInteger,
                        ))
                    }
                };
                use core::str::FromStr;
                match usize::from_str(s) {
                    Ok(s) => Ok(s),
                    Err(_) => Err(H11Error::InvalidHeaderValueNoPos(
                        HeaderValidationError::ExpectedInteger,
                    )),
                }
            }
            _ => Err(H11Error::InvalidHeaderValueNoPos(
                HeaderValidationError::ExpectedInteger,
            )),
        }
    }
}

impl<'h> H11Header<'h> {
    /// Header field & value as bytes excluding the trailing <CR><LF>.
    /// Caller is rquired to concatenate due to non-allocating nature of the library.
    pub fn as_bytes_out(&self) -> (&[u8], &[u8]) {
        let (field, value) = match self {
            Self::Accept(a) => ("Accept".as_bytes(), a.as_bytes_out()),
            Self::Connection(c) => ("Connection".as_bytes(), c.as_bytes_out()),
            Self::ContentLength(l) => ("Content-Length".as_bytes(), l.as_bytes_out()),
            Self::Host(h) => ("Host".as_bytes(), h.as_bytes_out()),
            Self::Http2Settings(s) => ("HTTP2-Settings".as_bytes(), s.as_bytes_out()),
            Self::SetCookie(c) => ("Set-Cookie".as_bytes(), c.as_bytes_out()),
            Self::TransferEncoding(te) => ("Transfer-Encoding".as_bytes(), te.as_bytes_out()),
            Self::Upgrade(u) => ("Upgrade".as_bytes(), u.as_bytes_out()),
            Self::UserAgent(ua) => ("User-Agent".as_bytes(), ua.as_bytes_out()),
            Self::Unknown(field, value) => (field.as_bytes_out(), value.as_bytes_out()),
        };
        (field, value)
    }
}
