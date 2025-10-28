//! A `no_std` Rust library for parsing, formatting, and displaying Stellar blockchain data structures.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod display;
pub mod formatter;

pub mod parser;
pub mod serialize;

pub use formatter::{
    format_hash_id_preimage_soroban_authorization, format_operation,
    format_transaction_signature_payload, get_operation_intent, DataEntry, FormatConfig,
    FormatError,
};
pub use parser::*;
