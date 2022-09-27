#![deny(rust_2018_idioms)]
#![deny(clippy::correctness)]
#![deny(clippy::perf)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! Key-Value store engine with two interchangeable implementations

pub mod store;
pub mod store_db;
pub mod store_mem;
