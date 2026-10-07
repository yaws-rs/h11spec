//! Parser components

mod p_h11header;
mod p_h11method;
mod p_h11request_meta;
mod p_h11target;
mod p_h11version;

mod p_generated;
pub(crate) use p_generated::*;
mod h11header_value_tokens;
pub(crate) use h11header_value_tokens::*;
mod p_generated_util;

use p_h11method::*;
use p_h11target::*;
use p_h11version::*;

//****************************************
// Re-export public parser interface
//****************************************
pub use p_h11header::{HeaderParser, HeaderStatus};
