//! CLI command implementations.
//!
//! One module per command. Add a `pub mod <name>;` here and a matching arm in
//! [`crate::cli::Cli::run`] when you introduce a new command.

pub mod completion;
pub mod honk;
pub mod run;
