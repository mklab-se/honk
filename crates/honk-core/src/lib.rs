//! Core types for `honk`.
//!
//! This crate holds the framework-agnostic pieces of the tool (configuration,
//! error types and, from v0.2, the honk synthesiser) so they can be reused and
//! tested without `clap`, `tokio` or any audio device code.

pub mod config;
pub mod error;
pub mod paths;
pub mod spec;
pub mod style;
pub mod synth;
pub mod wav;

pub use error::{Error, Result};
pub use spec::{HonkSpec, Mood, Overrides};
pub use style::Style;
