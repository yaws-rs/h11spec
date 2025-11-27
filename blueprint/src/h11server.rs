//! h11spec Server blueprints

use blueprint::BluePrint;
use blueprint::Orbit;
use blueprint::{Left, Right, InBuffer};
use crate::H11Error;

/// empty for now
pub struct Position;

/// State machine stages
#[derive(Debug, Default)]
pub enum H11ServingStage {
    /// Expect Start line
    #[default]
    Start,
    /// Expect Headers or <CRLF><CRLF>
    Headers,
    /// Expect Encoded content to pass through as Decoded
    Decoding,
    /// Expect Decoded content to pass through as Encoded
    Encoding,
    /// Expect plain passthrough.
    Passthrough,
    /// Expect Shutdown
    Shutdown,
}

use h11types::{H11RequestMeta, H11Method, H11Version, H11TransferEncoding, Uri};

#[derive(Debug, Default)]
pub struct PayloadCursor {
    
}

/// H11Server Serving context
#[derive(Debug, Default)]
pub struct H11Serving {
    stage: H11ServingStage,
    req_meta: H11RequestMeta,
    payload_in_cursor: PayloadCursor,
    payload_out_cursor: PayloadCursor,
}

impl Orbit for H11Serving {
    type Position = Position;
    type Error = H11Error;
    fn advance_with<B, L: Left, R: Right>(
        &mut self,
        _u: &mut B,
        l: &mut L,
        _r: &mut R,
    ) -> Result<Self::Position, Self::Error> {
        let (left_in_len, left_out_len) = l.left_lens();
        let (left_inputs, left_out_b) = l.left_bufs_mut();

        let left_in_b = match left_inputs {
            InBuffer::Single(buf) => buf,
            InBuffer::Double(buf1, buf2) => {
                todo!()
            },
        };


        if left_in_len == 0 {
            return Ok(Position);
        }

        println!("Got data in = {}", core::str::from_utf8(&left_in_b).unwrap());
        
        loop {
        
            match self.stage {
                H11ServingStage::Start => {
                    let advanced = self.req_meta.advance_status_with(&left_in_b);
                    
                    if self.req_meta.status_complete() {
                        self.stage = H11ServingStage::Headers;
                        continue;
                    }
                },
                H11ServingStage::Headers => {
                },
                H11ServingStage::Decoding => {
                },
                H11ServingStage::Encoding => {
                },
                H11ServingStage::Passthrough => {
                },
                H11ServingStage::Shutdown => {
                },
            }
        }
        
        left_out_b[0..3].copy_from_slice("AA\n".as_bytes());
        l.left_set_lens(0, 3);

        Ok(Position)
    }
}

pub struct H11SpecServer;
pub struct H11ServerConfig;

impl BluePrint<H11Serving> for H11SpecServer {
    type Config = H11ServerConfig;
    type Error = H11Error;

    fn with_defaults() -> Result<H11Serving, Self::Error> {
        Ok(H11Serving::default())
    }
    fn with_configuration(_: Self::Config) -> Result<H11Serving, Self::Error> {
	    todo!()
    }
}
