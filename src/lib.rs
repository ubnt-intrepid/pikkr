//! JSON parser which picks up values directly without performing tokenization
extern crate fnv;
#[cfg(feature = "avx-accel")]
extern crate x86intrin;

#[cfg(feature = "avx-accel")]
mod avx;
#[cfg(not(feature = "avx-accel"))]
#[path = "emulated.rs"]
mod avx;

mod bit;
mod error;
#[doc(hidden)]
pub mod index_builder;
#[doc(hidden)]
pub mod parser;
mod pikkr;
#[doc(hidden)]
pub mod query;
mod result;
mod utf8;

pub use error::{Error, ErrorKind};
pub use pikkr::Pikkr;
pub use result::Result;
