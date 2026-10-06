//! H11RequestMeta Headers processing

use super::*;

impl H11RequestMeta {
    #[inline]
    pub(crate) const fn in_content_length(&mut self, len: usize) -> Result<(), H11Error> {
        match self.body_length {
            Some(_) => {
                self.body_length = None;
                Err(H11Error::DuplicateHeader(0))
            },
            None => {
                self.body_length = Some(len);
                Ok(())
            },
        }
    }
    #[inline]
    pub(crate) fn in_header<'h>(&mut self, hdr: H11Header<'h>) -> Result<(), H11Error> {
        let ret = match hdr {
            H11Header::ContentLength(cl) => self.in_content_length(cl.into())?,
            _ => {},
        };
        Ok(ret)
    }        
}
