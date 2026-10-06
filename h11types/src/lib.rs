#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
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

mod resp_status;
#[doc(inline)]
pub use resp_status::*;

//-----------------------------------------------
// Parser impls
//-----------------------------------------------
mod parser;
#[doc(inline)]
pub use parser::*;

//-----------------------------------------------
// Receiver (Parsing) types
//-----------------------------------------------
mod p_receivers;
#[doc(inline)]
pub use p_receivers::*;

//-----------------------------------------------
// Validated types
//-----------------------------------------------
mod validated;
#[doc(inline)]
pub use validated::*;
