//! Content-Length header

use crate::HeaderValidationError;
use crate::parser::{
    HeaderValueToken
};

/*
/// Header containing Content-Length value
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct H11ContentLength(usize);
*/

impl TryFrom<HeaderValueToken<'_>> for usize {
    type Error = HeaderValidationError;

    fn try_from(tok: HeaderValueToken<'_>) -> Result<Self, Self::Error> {
        match tok {
            HeaderValueToken::Integer(i) => Ok(i),
            _ => Err(HeaderValidationError::ExpectedInteger),
        }
    }
}

/*
impl From<H11ContentLength> for usize {
    fn from(s: H11ContentLength) -> usize {
        s.0
    }
}
*/
