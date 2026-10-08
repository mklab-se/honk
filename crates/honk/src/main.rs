//! honk: make your computer honk like an old-school car.

use anyhow::Result;
use clap::{CommandFactory, Parser};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod audio;
mod banner;
mod car;
mod cli;
mod commands;
#[cfg(any(target_os = "linux", all(test, unix)))]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
mod player;
mod update;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    // Handle dynamic shell completions (when invoked via COMPLETE=<shell> honk)
    clap_complete::CompleteEnv::with_factory(Cli::command).complete();

    let cli = Cli::parse();

    // Initialize logging based on -v / -q.
    let filter = if cli.verbose > 0 {
        match cli.verbose {
            1 => "honk=debug",
            _ => "honk=trace",
        }
    } else if cli.quiet {
        "error"
    } else {
        "honk=info"
    };

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false).without_time())
        .with(EnvFilter::new(filter))
        .init();

    if cli.no_color {
        colored::control::set_override(false);
    }

    // Spawn a background update check (skipped in quiet mode or if disabled via env).
    let update_handle = if !cli.quiet && std::env::var("HONK_NO_UPDATE_CHECK").is_err() {
        Some(tokio::spawn(update::check_for_updates()))
    } else {
        None
    };

    let result = cli.run().await;

    // Never hold up exit for the update check: give it a moment, then abandon it.
    if let Some(handle) = update_handle {
        let _ = tokio::time::timeout(std::time::Duration::from_millis(300), handle).await;
    }

    match result {
        Ok(0) => Ok(()),
        Ok(code) => std::process::exit(code),
        Err(e) => Err(e),
    }
}
