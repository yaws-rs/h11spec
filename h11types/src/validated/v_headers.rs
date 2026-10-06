//! Validated headers

use crate::*;

mod v_content_length;
//use v_content_length::*;
mod v_connection;
//mod v_host;
//pub use v_host::*;
mod v_unknown;

/// HTTP/1.1 RFC 9110 Mandated Headers
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum H11Header<'h> {
    /// RFC 9110 s. 7.6.1
    Connection(H11Connection),
    /// RFC 9110 s. 8.6
    ContentLength(usize),
    /// draft-ietf-httpbis-rfc6265bis-22 s. 5.8.1 & 4.1
    SetCookie(H11MaybeValue<'h>),
    /// RFC 9110 s. 7.2
    Host(H11MaybeValue<'h>),
    /// Unknown header field containing maybe a header value
    Unknown(H11UnknownField<'h>, H11MaybeValue<'h>),
}

use crate::parser::{
    HeaderKeyToken,
    HeaderValueToken
};

use crate::HeaderValidationError;

impl<'h> TryFrom<(HeaderKeyToken<'h>, HeaderValueToken<'h>)> for H11Header<'h> {
    type Error = HeaderValidationError;

    fn try_from(tokens: (HeaderKeyToken<'h>, HeaderValueToken<'h>)) -> Result<Self, Self::Error> {
        match tokens.0 {
            HeaderKeyToken::ContentLength => Ok(H11Header::ContentLength(tokens.1.try_into()?)),
            HeaderKeyToken::Other(o) => Ok(H11Header::Unknown(o.into(), tokens.1.try_into()?)),
            HeaderKeyToken::SetCookie => Ok(H11Header::SetCookie(tokens.1.try_into()?)),
            HeaderKeyToken::Host => Ok(H11Header::Host(tokens.1.try_into()?)),
            HeaderKeyToken::Connection => Ok(H11Header::Connection(tokens.1.try_into()?)),
            #[allow(unreachable_patterns)]
            _ => todo!(),            
        }
    }
}
