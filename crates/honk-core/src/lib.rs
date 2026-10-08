//! Core types for `honk`.
//!
//! This crate holds the framework-agnostic pieces of the tool (configuration,
//! error types and, from v0.2, the honk synthesiser) so they can be reused and
//! tested without `clap`, `tokio` or any audio device code.

pub mod config;
pub mod error;

pub use error::{Error, Result};
