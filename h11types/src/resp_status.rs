//! H11Spec Response Status

/// Authenticatiob Challenge
#[derive(Clone, Debug, PartialEq)]
pub enum AuthChallenge {
    /// Basic Auth
    Basic,
    /// Digest Auth
    Digest,
}

/// Final response status line relative to Target / Host etc.
#[derive(Clone, Debug, PartialEq)]
pub enum RespComplete {
    /// 2xx Series
    Resp2xx(Resp2xx),
    /// 3xx Series
    Resp3xx(Resp3xx),
    /// 4xx Series
    Resp4xx(Resp4xx),
    /// 5xx Series
    Resp5xx(Resp5xx),
    /// Other status with a code and a description.
    UncheckedOther(&'static str),
}

impl RespComplete {
    /// Response status line out as bytes excluding the trailing <CR><LF>
    pub fn as_bytes_out(&self) -> &[u8] {
        match self {
            Self::Resp2xx(r) => r.as_bytes_out(),
            Self::Resp3xx(r) => r.as_bytes_out(),
            Self::Resp4xx(r) => r.as_bytes_out(),
            Self::Resp5xx(r) => r.as_bytes_out(),
            Self::UncheckedOther(o) => o.as_bytes(),
        }
    }
}

/// Indicative response relative to Target / Host etc.
#[derive(Clone, Debug, PartialEq)]
pub enum RespIndicative {
    /// Intermediate - Request is welcome without conditions
    GoAhead,
    /// 3xx Series
    R3xx(Resp3xx),
    /// 4xx Series
    R4xx(Resp4xx),
    /// 5xx Series
    R5xx(Resp5xx),
}

/// Status description
#[derive(Clone, Debug, PartialEq)]
pub struct StatusDesc {
    inner: &'static str,
}

impl StatusDesc {
    /// WARNING: This allows <CR> and other illegal characters.
    /// It is up to the implementer to provide legit status description.
    #[inline]
    pub fn new_unchecked(inner: &'static str) -> Self {
        Self { inner }
    }
    /// Status is OK 2xx
    pub fn ok() -> Self {
        Self { inner: "Ok" }
    }
    /// As bytes out
    pub fn as_bytes_out(&self) -> &[u8] {
        self.inner.as_bytes()
    }
}

/// 1xx Rsponses
#[derive(Clone, Debug, PartialEq)]
pub enum Resp1xx {
    /// 100 Continue
    Continue,
    /// 101 Switching Protocols
    SwitchingProtocols,
    /// 102 Processing (WebDAV; RFC 2518)
    Processing,
    /// 103 Early Hints (RFC 8297)
    EarlyHints,
}

/// 2xx Responses
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Resp2xx {
    /// 200 - Ok
    Ok,
    /// 201 Created
    Created,
    /// 202 Accepted
    Accepted,
    /// 203 Non-Authoritative Information (since HTTP/1.1)
    NonAuthoritativeInformation,
    /// 204 No Content
    NoContent,
    /// 205 Reset Content
    ResetContent,
    /// 206 Partial Content
    PartialContent,
    /// 207 Multi-Status (WebDAV; RFC 4918)
    MultiStatus,
    /// 208 Already Reported (WebDAV; RFC 5842)
    AlreadyReported,
    /// 226 IM Used (RFC 3229)
    ImUsed,
}

impl Resp2xx {
    /// Print response status line out as bytes excluding the <CR><LF>
    pub fn as_bytes_out(&self) -> &[u8] {
        match self {
            Self::Ok => "200 OK",
            Self::Created => "201 Created",
            Self::Accepted => "202 Accepted",
            Self::NonAuthoritativeInformation => "203 Non-Authoritative Information",
            Self::NoContent => "204 No Content",
            Self::ResetContent => "205 Reset Content",
            Self::PartialContent => "206 Partial Content",
            Self::MultiStatus => "207 Multi-Status",
            Self::AlreadyReported => "208 Already Reported",
            Self::ImUsed => "226 IM Used",
        }
        .as_bytes()
    }
}

/// 3xx Responses
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Resp3xx {
    /// 301 - Moved Permanently
    MovedPermanently,
    /// 302 - Found
    Found,
    /// 303 - See Other
    SeeOther,
    /// 307 - Temporary Redirect
    TempRedirect,
    /// 308 - Permanent Redirect
    PermRedirect,
}

impl Resp3xx {
    /// Print response status line out as bytes excluding the <CR><LF>
    pub fn as_bytes_out(&self) -> &[u8] {
        match self {
            Self::MovedPermanently => "301 Moved Permanently",
            Self::Found => "302 Found",
            Self::SeeOther => "303 See Other",
            Self::TempRedirect => "307 Temporary Redirect",
            Self::PermRedirect => "308 Permanent Redirect",
        }
        .as_bytes()
    }
}

/// 4xx Response
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
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
    /// 418 - I'm a teapot
    Teapot,
}

impl Resp4xx {
    /// Print response status line out as bytes excluding the <CR><LF>
    pub fn as_bytes_out(&self) -> &[u8] {
        match self {
            Self::BadRequest => "400 Bad Request",
            Self::AuthRequired(_) => "401 Authentication is required",
            Self::PaymentRequired => "402 Payment required",
            Self::Forbidden => "403 Forbidden",
            Self::NotFound => "404 Not Found",
            Self::MethodNotAllowed => "405 Method Not Allowed",
            Self::NotAcceptable => "406 Not Acceptable",
            Self::UnsupportedMediaType => "415 Unsupported Media Type",
            Self::Teapot => "418 I'm a teapot",
        }
        .as_bytes()
    }
}

/// 5xx status indicates that the server is aware that it has encountered an error or
/// is otherwise incapable of performing the request. Except when responding to a HEAD
/// request, the server should include an entity containing an explanation of the error
/// situation, and indicate whether it is a temporary or permanent condition. Likewise,
/// user agents should display any included entity to the user. These response codes are
/// applicable to any request method.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Resp5xx {
    /// ### 500 Internal Server Error
    ///
    /// A generic error message, given when an unexpected condition was encountered and no more specific message is suitable.
    InternalServerError,
    /// ### 501 Not Implemented
    ///
    /// The server either does not recognize the request method, or it lacks the ability to fulfil the request. Usually this implies future availability (e.g., a new feature of a web-service API).
    NotImplemented,
    /// ### 502 Bad Gateway
    ///
    /// The server was acting as a gateway or proxy and received an invalid response from the upstream server.
    BadGateway,
    /// ### 503 Service Unavailable
    ///
    /// The server cannot handle the request (because it is overloaded or down for maintenance). Generally, this is a temporary state.
    ServiceUnavailable,
    /// ### 504 Gateway Timeout
    ///
    /// The server was acting as a gateway or proxy and did not receive a timely response from the upstream server.
    GatewayTimeout,
    /// ### 505 HTTP Version Not Supported
    ///
    /// The server does not support the HTTP version used in the request.
    HttpVersionNotSupported,
    /// ### 506 Variant Also Negotiates (RFC 2295)
    ///
    /// Transparent content negotiation for the request results in a circular reference.
    VariantAlsoNegotiates,
    /// ### 507 Insufficient Storage (WebDAV; RFC 4918)
    ///
    /// The server is unable to store the representation needed to complete the request.
    InsufficientStorage,
    /// ### 508 Loop Detected (WebDAV; RFC 5842)
    ///
    /// The server detected an infinite loop while processing the request.
    LoopDetected,
    /// ### 509 Bandwidth Limit Exceeded
    BandwidthLimitExceeded,
    /// ### 510 Not Extended (RFC 2774)
    ///
    /// Further extensions to the request are required for the server to fulfil it.
    NotExtended,
    /// ### 511 Network Authentication Required (RFC 6585)
    ///
    /// The client needs to authenticate to gain network access.
    NetworkAuthenticationRequired,
}

impl Resp5xx {
    /// Print response status line out as bytes excluding the <CR><LF>
    pub fn as_bytes_out(&self) -> &[u8] {
        match self {
            Self::InternalServerError => "500 Internal Server Error",
            Self::NotImplemented => "501 Not Implemented",
            Self::BadGateway => "502 Bad Gateway",
            Self::ServiceUnavailable => "503 Service Unavailable",
            Self::GatewayTimeout => "504 Gateway Timeout",
            Self::HttpVersionNotSupported => "505 HTTP Version Not Supported",
            Self::VariantAlsoNegotiates => "506 Variant Also Negotiates",
            Self::InsufficientStorage => "507 Insufficient Storage",
            Self::LoopDetected => "508 Loop Detected",
            Self::BandwidthLimitExceeded => "509 Bandwidth Limit Exceeded",
            Self::NotExtended => "510 Not Extended",
            Self::NetworkAuthenticationRequired => "511 Network Authentication Required",
        }
        .as_bytes()
    }
}
