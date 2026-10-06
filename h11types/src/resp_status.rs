//! H11Spec Response Status

use crate::Uri;

/// Authenticatiob Challenge
#[derive(Clone, Debug, PartialEq)]
pub enum AuthChallenge {
    /// Basic Auth
    Basic,
    /// Digest Auth
    Digest,
}

/// Final response relative to Target / Host etc.
pub enum RespComplete<'uri, 'desc> {
    /// 2xx Series
    Resp2xx(Resp2xx<'desc>),
    /// 3xx Series
    Resp3xx(Resp3xx<'uri>),
    /// 4xx Series
    Resp4xx(Resp4xx),
}

/// Indicative response relative to Target / Host etc.
#[derive(Clone, Debug, PartialEq)]
pub enum RespIndicative<'uri> {
    /// Intermediate - Request is welcome without conditions
    GoAhead,
    /// 3xx Series
    R3xx(Resp3xx<'uri>),
    /// 4xx Series
    R4xx(Resp4xx),
}

/// Status description
#[derive(Clone, Debug, PartialEq)]
pub struct StatusDesc<'desc> {
    inner: &'desc str,
}

impl<'desc> StatusDesc<'desc> {
    /// WARNING: This allows <CR> and other illegal characters.
    /// It is up to the implementer to provide legit status description.
    #[inline]
    pub fn new_unchecked(inner: &'desc str) -> Self {
        Self { inner }
    }
    /// Status is OK 2xx
    pub fn ok() -> Self {
        Self { inner: "Ok" }
    }
}

/// 2xx Responses
#[derive(Clone, Debug, PartialEq)]
pub enum Resp2xx<'desc> {
    /// 200 - Ok
    Ok(StatusDesc<'desc>),
}

/// 3xx Responses
#[derive(Clone, Debug, PartialEq)]
pub enum Resp3xx<'uri> {
    /// 301 - Moved Permanently
    MovedPermanently(Uri<'uri>),
    /// 302 - Found
    Found(Uri<'uri>),
    /// 303 - See Other
    SeeOther(Uri<'uri>),
    /// 307 - Temporary Redirect
    TempRedirecrt(Uri<'uri>),
    /// 308 - Permanent Redirect
    PermRedirect(Uri<'uri>),
    /// Other 3xx Response where the parameter is the optional Location:
    Other(u8, Option<Uri<'uri>>),
}

/// 4xx Response
#[derive(Clone, Debug, PartialEq)]
pub enum Resp4xx {
    /// 400 - Bad Request
    BadRequest,
    /// 401 - Authentication is required
    AuthRequired(AuthChallenge),
    /// 402 - Payment required
    PaymentRequired,
    /// 403 - Forbidden regardless of anything
    Forbidden,
    /// 404 - Not Found
    NotFound,
    /// 405 - Method Not Allowed
    MethodNotAllowed,
    /// 406 - Not Acceptable
    NotAcceptable,
    /// 415 - Unsupported Media Type
    UnsupportedMediaType,
    /// 518 - I'm a teapot
    Teapot,
    /// Other 4xx Response where the parameter is the xx part
    Other(u8),
}
