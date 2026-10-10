//! Validated headers

mod v_connection;
mod v_content_length;
//mod v_host;
mod v_unknown;

use crate::H11Header;

use crate::parser::{HeaderKeyToken, HeaderValueToken};

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
