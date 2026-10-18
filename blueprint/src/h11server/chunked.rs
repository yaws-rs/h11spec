//! Transfer-Encoding Chunked relay through ychunked::ChunkReceiver

use super::H11Serving;
use super::RequestBodyReceiver;
use h11types::RespIndicative;

pub(super) struct ChunkRelay<'r, App> {
    pub(super) myself: &'r mut H11Serving<App>,
}

use ychunked::ChunkReceiver;

impl<'r, App> ChunkReceiver for ChunkRelay<'r, App>
where
    App: RequestBodyReceiver,
{
    fn chunk_add(&mut self, data: &[u8]) -> bool {
        match self.myself.app.req_body_add(data) {
            RespIndicative::GoAhead => true,
            _ => false,
        }
    }
}
