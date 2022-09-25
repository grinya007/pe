#![deny(rust_2018_idioms)]
#![deny(clippy::correctness)]
#![deny(clippy::perf)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

#[macro_use]
extern crate derive_error;

pub mod client;
pub mod input;
pub mod processor;
pub mod transaction;
pub mod write_csv;
