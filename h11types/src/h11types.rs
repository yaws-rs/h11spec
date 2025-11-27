//! h11spec types
///
/// Reflects RFC 9112 (June 2022)
///
/// # Transfer coding
/// Registry at https://www.iana.org/assignments/http-parameters
///

use core::mem::MaybeUninit;

mod method;
pub use method::*;

///
#[derive(Debug, Default)]
pub enum H11TransferEncoding {
    ///
    #[default]
    None,
    ///
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

///
#[derive(Debug, Default)]
pub enum H11Connection {
    ///
    #[default]
    Close,
    ///
    KeepAlive,
}

///
#[derive(Debug, Default)]
pub enum H11TransferCompression {
    ///
    #[default]
    None,
    ///
    Compress,
    ///
    Deflate,
    ///
    Gzip,
    ///
    Br,
    ///
    Zstd,
}

/// Used by the server
#[derive(Debug, Default)]
pub struct H11RequestMeta {
    pub(crate) method: H11Method,
    pub(crate) target_loc: Option<(usize, usize)>,
    pub(crate) version: H11Version,
    pub(crate) transfer_encoding: H11TransferEncoding,
    pub(crate) transfer_compression: H11TransferCompression,
    pub(crate) body_length: Option<usize>,
    pub(crate) status_end: Option<usize>,
    pub(crate) headers_end: Option<usize>,
}

impl H11RequestMeta {
    /// Is the status line parsing complete?
    #[inline]
    pub fn status_complete(&self) -> bool {
        self.method != H11Method::Unknown &&
            self.target_loc.is_some() &&
            self.version != H11Version::Unknown
    }
}

///
#[derive(Debug)]
pub enum H11Path<const S: usize> {
    /// Heap Allocated variant
    #[cfg(any(feature = "std", feature = "alloc"))]
    Heap(H11PathHeap),
    /// Static const generic variant
    #[cfg(feature = "static")]
    Static(H11PathStatic<{ S }>),
    /// Ptr ref variant
    #[cfg(feature = "ptr")]
    Ptr(*const u8, usize),
}

///
#[derive(Debug)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub struct H11PathHeap(Vec<u8>);

///
#[derive(Debug)]
#[cfg(all(not(feature = "std"), not(feature = "alloc")))]
pub struct H11PathStatic<const S: usize>(MaybeUninit<[u8; S]>);

/*
#[derive(Debug, Default)]
pub struct H11Serving {
    stage: H11ServingStage,
    req_meta: Option<H11RequestMeta>,
    payload_in_cursor: Option<PayloadCursor>,
    payload_out_cursor: Option<PayloadCursor>,
} */
