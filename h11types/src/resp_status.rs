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

/// Indicative response relative to Target / Host etc.
#[derive(Clone, Debug, PartialEq)]
pub enum RespIndicative<'uri> {
    /// Intermediate - Request is welcome without conditions
    GoAhead,
    /// 3xx Series
    R3xx(Intermediate3xx<'uri>),
    /// 4xx Series
    R4xx(Intermediate4xx),
}

/// Intermediate 3xx Responses
#[derive(Clone, Debug, PartialEq)]
pub enum Intermediate3xx<'uri> {
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

/// Intermediate 4xx Response
#[derive(Clone, Debug, PartialEq)]
pub enum Intermediate4xx {
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

