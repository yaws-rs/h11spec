//! Content-Length header

use crate::parser::HeaderValueToken;
use crate::HeaderValidationError;

/*
/// Header containing Content-Length value
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct H11ContentLength(usize);
*/

impl TryFrom<HeaderValueToken<'_>> for usize {
    type Error = HeaderValidationError;

    fn try_from(tok: HeaderValueToken<'_>) -> Result<Self, Self::Error> {
        match tok {
            HeaderValueToken::MaybeInteger(i) => {
                let s = match core::str::from_utf8(i) {
                    Ok(s) => s,
                    Err(_) => return Err(HeaderValidationError::ExpectedInteger),
                };
                use core::str::FromStr;
                match usize::from_str(s) {
                    Ok(s) => Ok(s),
                    Err(_) => Err(HeaderValidationError::ExpectedInteger),
                }
            },
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
