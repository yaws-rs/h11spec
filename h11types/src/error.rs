//! h11 error/s

/// H11 Errors
#[derive(Debug, PartialEq)]
pub enum H11Error {
    /// Expected HTTP Method
    ExpectedMethod,
    /// Invalid HTTP Method
    InvalidMethod,
    /// Expected HTTP Target (Path, Url etc.)
    ExpectedTarget,
    /// Invalid Target
    InvalidTarget,
    /// Invalid HTTP Version
    InvalidVersion,
    /// Header key is not a IANA registered header
    MissingHeaderKey,
    /// Encountered invalid header at position
    InvalidHeaders(usize),
    // Encuntered invalid header value/s at position
    //InvalidHeaderValue(usize),
    /// Encountered duplicate same-name headers
    DuplicateHeader(usize),
    /// Encuntered invalid header value/s at position (parser phase)
    InvalidHeaderValue(usize, HeaderValidationError),
    /// Invalid header value (validation phase)
    InvalidHeaderValueNoPos(HeaderValidationError),
}

// TODO: maciejhirsz/logos/issues/568
//use quoted_values::DecodeError;

/// Header validation errors
#[derive(Debug, PartialEq)]
pub enum HeaderValidationError {
    /// Expected integer
    ExpectedInteger,
    /// Expected Host / Address value
    ExpectedHostAddr,
    /// Expected non-Quoted non-Integer Value
    ExpectedValue,
    /// Expected Quoted value
    ExpectedQuotedValue,
    /// Expected a valid Connection header value
    ExpectedConnection,
    /// Internal validation error
    Internal,
}
