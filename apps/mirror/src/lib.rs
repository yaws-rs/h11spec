#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

// We need alloc for this one. It would be great to provide a storage / an arena API in the future.
#[macro_use]
extern crate alloc;

//***********************************************
// Re-Exports
//***********************************************

//-----------------------------------------------
// App
//-----------------------------------------------
mod mirror;
#[doc(inline)]
pub use mirror::*;
