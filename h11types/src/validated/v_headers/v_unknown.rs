//! Unknown header

use crate::parser::HeaderValueToken;
use crate::HeaderValidationError;

use crate::H11MaybeValue;
use crate::H11UnknownField;

impl<'h> From<&'h [u8]> for H11UnknownField<'h> {
    fn from(f: &'h [u8]) -> H11UnknownField<'h> {
        Self(f)
    }
}

impl<'h> TryFrom<HeaderValueToken<'h>> for H11MaybeValue<'h> {
    type Error = HeaderValidationError;

    fn try_from(tok: HeaderValueToken<'h>) -> Result<Self, Self::Error> {
        match tok {
            HeaderValueToken::Incomplete(_i) => Err(HeaderValidationError::Internal),
            HeaderValueToken::Integer(i) => Ok(H11MaybeValue::Integer(i)),
            HeaderValueToken::MaybeValue(o) => Ok(H11MaybeValue::Bytes(o)),
            HeaderValueToken::MaybeQuotedValue(qv) => {
                let qv_start = match qv.strip_prefix(&[34]) {
                    Some(qv) => qv,
                    None => return Err(HeaderValidationError::ExpectedQuotedValue),
                };

                let qv_final = match qv_start.strip_suffix(&[34]) {
                    Some(qv) => qv,
                    None => return Err(HeaderValidationError::ExpectedQuotedValue),
                };

                /* TODO: Needs SourceMut per maciejhirsz/logos/issues/568
                let qv_final = &mut qv[1..qv.len()-1];

                let qv_final = match quoted_values::BackslashRemoval::in_place(qv_final) {
                    Ok(ref qv) => qv,
                    Err(e) => return Err(HeaderValidationError::InvalidQuotedValue(e)),
                }; */

                Ok(H11MaybeValue::Bytes(qv_final))
            }
        }
    }
}
