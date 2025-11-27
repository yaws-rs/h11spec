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

//-----------------------------------------------
// All Errors
//-----------------------------------------------
mod error;
#[doc(inline)]
pub use error::*;

//-----------------------------------------------
// h11spec Server impl
//-----------------------------------------------
#[cfg(feature = "h11server")]
mod h11server;
#[cfg(feature = "h11server")]
#[doc(inline)]
pub use h11server::*;
