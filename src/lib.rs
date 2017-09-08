//! JSON parser which picks up values directly without performing tokenization
extern crate fnv;
extern crate x86intrin;

mod avx;
mod bit;
mod error;
mod index_builder;
mod parser;
mod pikkr;
mod query;
mod result;
mod utf8;

pub use error::{Error, ErrorKind};
pub use pikkr::{CountedParser, Pikkr};
pub use result::Result;
