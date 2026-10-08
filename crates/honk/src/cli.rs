//! CLI argument definitions using clap.

use anyhow::Result;
use clap::Parser;

/// Make your computer honk like an old-school car. Honk, honk!
#[derive(Parser)]
#[command(name = "honk")]
#[command(author, version, about)]
#[command(long_about = "Make your computer honk like an old-school car.\n\n\
    Run without a subcommand to honk. Drop it at the end of a long command \
    (`cargo build && honk`) and your computer tells you when it is done.")]
#[command(propagate_version = true)]
pub struct Cli {
    /// Increase output verbosity (-v for debug, -vv for trace)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Suppress non-essential output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Manage AI features (shows status when run without a subcommand)
    Ai {
        #[command(subcommand)]
        command: Option<AiCommands>,
    },

    /// Generate shell completions
    Completion {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Show version information
    Version,
}

#[derive(clap::Subcommand)]
pub enum AiCommands {
    /// Test AI integration by sending a message
    Test {
        /// Message to send (default: "Say hello in one sentence.")
        message: Option<String>,
    },
    /// Enable AI features for honk
    Enable,
    /// Disable AI features for honk
    Disable,
    /// Interactively configure AI provider and model settings
    Config,
    /// Show AI status (same as running `honk ai` without a subcommand)
    Status,
}

/// Shells supported by `honk completion`.
#[derive(Clone, Copy, clap::ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Powershell,
}

impl Cli {
    /// Dispatch to the selected subcommand.
    pub async fn run(self) -> Result<()> {
        match self.command {
            Some(Commands::Ai { command }) => crate::commands::ai::run(command).await,
            Some(Commands::Completion { shell }) => {
                crate::commands::completion::generate_completions(shell);
                Ok(())
            }
            Some(Commands::Version) => {
                crate::banner::print_banner_with_version();
                Ok(())
            }
            // No subcommand: honk. Stage 1 prints the honk; v0.2 plays it.
            None => {
                println!("Honk, honk!");
                Ok(())
            }
        }
    }
}
