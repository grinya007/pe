#![deny(rust_2018_idioms)]
#![deny(clippy::correctness)]
#![deny(clippy::perf)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! The core implementation of the Payment Engine
//!
//! All exceptional situations (when the input data is considered invalid)
//! are covered in three error enums:
//! * [Client-level errors](client/enum.Error.html)
//! * [Transaction-level errors](transaction/enum.Error.html)
//! * [Global processing errors](processor/enum.Error.html)

#[macro_use]
extern crate derive_error;

/// Implements the mutation and a serde-serializable representation
/// of the client's account
pub mod client;

/// Implements the serde-deserializable struct for a single row in the input
pub mod input;

/// Implements the core validation and processing of transactions
pub mod processor;

/// Implements the mutation of a transaction
pub mod transaction;

/// The helper function that writes any serde-serializable structure into CSV format
pub mod write_csv;
