//! # h11request Meta Parser
//!
//! THis parser is genereally used by the server implementation for the in flight requests and
//! will not allocate.

use logos::{Lexer, Logos};

use crate::H11Error;
use crate::H11Method;
use crate::H11Version;
use crate::H11RequestMeta;

use crate::parser::{MethodToken, parse_h11method};
use crate::parser::{TargetToken, parse_h11target};
use crate::parser::{VersionToken, parse_h11version};

impl H11RequestMeta {
    /// Advance parsing the status line with the given input buffer
    pub fn advance_status_with<'raw>(&mut self, input: &'raw [u8]) -> Result<usize, H11Error> {
        let mut lexer: Lexer<'raw, MethodToken<'raw>> = MethodToken::lexer(input);

        if self.method == H11Method::Unknown {
            self.method = parse_h11method(&mut lexer)?;
        }

        if self.target_loc.is_none() {
            let start = lexer.span().start;
            let mut target_lexer: Lexer<'raw, TargetToken<'raw>> = lexer.morph();
            parse_h11target(&mut target_lexer)?;
            self.target_loc = Some((start, target_lexer.span().start));
            lexer = target_lexer.morph();
        }

        if self.version == H11Version::Unknown {
            let mut version_lexer: Lexer<'raw, VersionToken<'raw>> = lexer.morph();
            self.version = parse_h11version(&mut version_lexer)?;
            lexer = version_lexer.morph();
        }

        self.status_end = Some(lexer.span().end);

        Ok(lexer.span().end)
    }
}

impl H11RequestMeta {
    /// Advance parsing the headers with the given input buffer.
    /// # Minimum Input
    /// Minimum input is always a single complete header
    pub fn advance_headers_with<'raw>(&mut self, input: &'raw [u8]) -> Result<usize, H11Error> {
        let mut lexer: Lexer<'raw, MethodToken<'raw>> = HeaderToken::lexer(input);
        
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("GET / HTTP/1.1\r\n", 16)]
    #[case("GET /foo=bar?ding=dong&ping=baa+baa#anchor HTTP/1.1\r\n", 53)]
    fn try_advance_ok(#[case] raw_in: &str, #[case] expected_advanced: usize) {
        let mut meta = H11RequestMeta::default();

        let advanced = meta.advance_status_with(raw_in.as_bytes()).unwrap();
        assert_eq!(advanced, expected_advanced);
        assert_eq!(meta.method, H11Method::Get);
        assert_eq!(meta.status_complete(), true);
    }
}
