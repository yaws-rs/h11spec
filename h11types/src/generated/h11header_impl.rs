//! (auto generated from iana registry) h11header Token -> Name impl

impl TryFrom<HeaderKeyToken<'_>> for HeaderKey {
    type Error = crate::H11Error;
    fn try_from(i: HeaderKeyToken<'_>) -> Result<Self, Self::Error> {
        match i {
            HeaderKeyToken::ContentLength => Ok(Self::ContentLength),
            HeaderKeyToken::SetCookie => Ok(Self::SetCookie),
             _ => Err(H11Error::MissingHeaderKey),
         }
    }
}
