#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

//***********************************************
// Re-Exports
//***********************************************

/// HTTP URI
pub use yuri::Uri;

//-----------------------------------------------
// All Errors
//-----------------------------------------------
mod error;
#[doc(inline)]
pub use error::*;

//-----------------------------------------------
// h11spec Types
//-----------------------------------------------
mod h11types;
#[doc(inline)]
pub use h11types::*;

//-----------------------------------------------
// Parser impls
//-----------------------------------------------
mod parser;

//-----------------------------------------------
// Auto-generated types
//-----------------------------------------------
pub(crate) mod generated {
    pub(crate) mod h11header_name;
    pub(crate) mod h11header_name_tokens;
    pub(crate) mod h11header_name_value_tokens;
    pub(crate) mod h11header_value_tokens;
    pub(crate) mod util;
}
#[doc(inline)]
pub use generated::h11header_name::*;
