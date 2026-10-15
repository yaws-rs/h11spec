//! H11RequestMeta Headers processing

use super::*;
use crate::H11TransferEncoding;

impl H11RequestMeta {
    #[inline]
    pub(crate) fn in_transfer_encoding(
        &mut self,
        enc: H11TransferEncoding,
    ) -> Result<(), H11Error> {
        if self.body_length.is_some() {
            return Err(H11Error::BadRequest(
                "Ambigious Content-Length mixed with Transfer-Encoding.",
            ));
        }
        self.transfer_encoding = enc;
        Ok(())
    }
    #[inline]
    pub(crate) fn in_content_length(&mut self, len: usize) -> Result<(), H11Error> {
        if self.transfer_encoding != H11TransferEncoding::None {
            return Err(H11Error::BadRequest(
                "Ambigious Content-Length mixed with Transfer-Encoding.",
            ));
        }
        match self.body_length {
            Some(_) => {
                self.body_length = None;
                Err(H11Error::DuplicateHeader(0))
            }
            None => {
                self.body_length = Some(len);
                Ok(())
            }
        }
    }
    #[inline]
    pub(crate) fn in_header<'h>(&mut self, hdr: H11Header<'h>) -> Result<(), H11Error> {
        #[allow(clippy::single_match)]
        match hdr {
            H11Header::ContentLength(cl) => self.in_content_length(cl.try_into()?)?,
            H11Header::TransferEncoding(te) => self.in_transfer_encoding(te)?,
            _ => {}
        }
        Ok(())
    }
}
