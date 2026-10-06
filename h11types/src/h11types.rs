//! h11spec types
///
/// Reflects RFC 9112 (June 2022)
///
/// # Transfer coding
/// Registry at https://www.iana.org/assignments/http-parameters
///
mod method;
pub use method::*;

///
#[derive(Debug, Default, PartialEq)]
pub enum H11TransferEncoding {
    ///
    #[default]
    None,
    /// Chunked encoding
    Chunked,
}

///
#[derive(Debug, Default, PartialEq)]
pub enum H11Version {
    /// Unknown version
    #[default]
    Unknown,
    /// HTTP/1.1
    Http11,
}

/// HTTP Connection type - RFC 9110 7.6.1
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum H11Connection {
    /// Close
    #[default]
    Close,
    /// Keep-Alive
    KeepAlive,
}

/// HTTP Compression scheme
#[derive(Debug, Default, PartialEq)]
pub enum H11TransferCompression {
    ///
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

/// Used by the server
#[derive(Debug, Default, PartialEq)]
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
}


/// Header field is unknown
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct H11UnknownField<'h>(pub &'h [u8]);

/// Non-validated Header value that may be valid
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum H11MaybeValue<'h> {
    /// That looks like Integer value
    Integer(usize),
    /// That looks like Bytes value
    Bytes(&'h [u8]),
}
