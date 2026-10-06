//! Host header

use super::H11Header;
use crate::HeaderValidationError;
use crate::generated::{
    h11header_value_tokens::HeaderValueToken
};

use core::net::SocketAddr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum H11Host<'h> {
    Host(&'h str),
    Addr(SocketAddr),
}

impl TryFrom<HeaderValueToken<'_>> for H11Host {
    type Error = HeaderValidationError;

    fn try_from(tok: HeaderValueToken<'_>) -> Result<Self, Self::Error> {
        match tok {
            HeaderValueToken::Integer(i) => Ok(Self(i)),
            _ => Err(HeaderValidationError::ExpectedInteger),
        }
    }
}

impl From<H11ContentLength> for usize {
    fn from(s: H11ContentLength) -> usize {
        s.0
    }
}
