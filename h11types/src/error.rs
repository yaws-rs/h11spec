//! h11 error/s

/// H11 Errors
#[derive(Debug, PartialEq)]
pub enum H11Error {
    /// Expected HTTP Method
    ExpectedMethod,
    /// Invalid HTTP Method
    InvalidMethod,
    /// Expected <SP> after Method
    ExpectedSpAfterMethod,
    /// Invalid data after Method
    InvalidAfterMethod,
    /// Expected HTTP Target (Path, Url etc.)
    ExpectedTarget,
    /// Invalid Target
    InvalidTarget,
    /// Expected <SP> after Target
    ExpectedSpAfterTarget,
    /// Invalid data after Target
    InvalidAfterTarget,
    /// Expected HTTP Version
    ExpectedVersion,
    /// Invalid HTTP Version
    InvalidVersion,
    /// Expected <CR><LF> after Version
    ExpectedCrLfAfterVersion,
    /// Invalid data after Version
    InvalidAfterVersion,
    /// Header key is not a IANA registered header
    MissingHeaderKey,
}
