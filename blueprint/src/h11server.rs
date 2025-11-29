//! h11spec Server blueprints

use blueprint::BluePrint;
use blueprint::Orbit;
use blueprint::{Left, Right, InBuffer};
use crate::H11Error;

/// empty for now
pub struct Position;

/// State machine stages
#[derive(Debug, Default, PartialEq)]
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
    consumed: usize,
}

/// H11Server Serving context
#[derive(Debug, Default)]
pub struct H11Serving {
    stage: H11ServingStage,
    req_meta: H11RequestMeta,
    payload_in_cursor: PayloadCursor,
    payload_out_cursor: PayloadCursor,
    out_sent: usize,
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

        let mut left_in_b = match left_inputs {
            InBuffer::Single(buf) => buf,
            InBuffer::Double(buf1, buf2) => {
                todo!()
            },
        };


        if left_in_len == 0 && self.stage == H11ServingStage::Start {
            return Ok(Position);
        }

        println!("Got data in = {}", core::str::from_utf8(&left_in_b).unwrap());

        let mut out_len = left_out_len;
        
        loop {
        
            match self.stage {
                H11ServingStage::Start => {
                    println!("ServingStatus - Start");
                    let advanced = match self.req_meta.advance_status_with(&left_in_b) {
                        Ok(b) => _ = left_in_b.split_off_mut(..b).unwrap(),
                        Err(e) => panic!("Advance error..{:?}", e),
                    };
                    
                    if self.req_meta.status_complete() {
                        self.stage = H11ServingStage::Headers;
                        continue;
                    }
                },
                H11ServingStage::Headers => {
                    println!("ServingStatus - Headers");
                    let advanced = match self.req_meta.advance_headers_with(&left_in_b) {
                        Ok(b) => _ = left_in_b.split_off_mut(..b).unwrap(),
                        Err(e) => panic!("Headers advance error {:?}", e),
                    };

                    if self.req_meta.headers_complete() {
                        self.stage = H11ServingStage::Passthrough;
                        continue;
                    }
                },
                H11ServingStage::Decoding => {
                },
                H11ServingStage::Encoding => {
                },
                H11ServingStage::Passthrough => {
                    println!("ServingStatus - Passthrough");

                    let rick_roll = core::include_bytes!("/tmp/rick-roll.gif");
                    let rick_len = rick_roll.len();

                    let out_bytes: Option<&[u8]> = if self.payload_out_cursor.consumed == 0 {
                        println!("Initiating rick roll");
                        let resp = "HTTP/1.1 200 OK\r\nContent-Length: 193274\r\n\r\n".as_bytes();
                        left_out_b[0..resp.len()].copy_from_slice(resp);
                        out_len += resp.len();
                        
                        let mut remaining = left_out_b.len() - out_len;

                        if remaining > rick_len {
                            remaining = rick_len;
                        }
                        
                        println!("Rick-Roll 0..{remaining}");
                        self.payload_out_cursor.consumed += remaining;
                        Some(&rick_roll[0..remaining])
                    }
                    else if self.payload_out_cursor.consumed == rick_len {
                        println!("Rick-Roll is complete.");
                        None
                    }
                    else {
                        let consumed = self.payload_out_cursor.consumed;
                        let remaining_capacity = left_out_b.len() - left_out_len;

                        println!("Rick-Roll continue with consumed = {consumed} and remaining_capacity = {remaining_capacity}");
                        
                        if remaining_capacity > 0 {
                            let roll_end = if rick_len < consumed + remaining_capacity {
                                rick_len
                            }
                            else {
                                consumed + remaining_capacity
                            };
                            self.payload_out_cursor.consumed += roll_end - consumed;
                            println!("Rick-Roll continue {consumed} .. {roll_end}");
                            println!("Payload_out_cursor.consumed = {}", self.payload_out_cursor.consumed);
                            Some(&rick_roll[consumed..roll_end])
                        }
                        else {
                            println!("Rick-Roll has no remaining capacity out.");
                            None
                        }
                    };

                    if let Some(out_bytes) = out_bytes {
                        let start = left_out_len + out_len;
                        println!("Start is {} capacity left == {}", left_out_len + out_len, left_out_b.len() - out_len);
                        out_len += out_bytes.len();
                        left_out_b[start..out_len].copy_from_slice(out_bytes);
                    }
                        
                    break;
                },
                H11ServingStage::Shutdown => {
                },
            }
        }
        
//        left_out_b[0..3].copy_from_slice("AA\n".as_bytes());
//        l.left_set_lens(0, 3);

        l.left_set_lens(0, out_len);
        
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
