//! CLI argument definitions using clap.

use anyhow::Result;
use clap::Parser;
use clap::builder::TypedValueParser;

/// Make your computer honk like an old-school car. Honk, honk!
#[derive(Parser)]
#[command(name = "honk")]
#[command(author, version, about)]
#[command(long_about = "Make your computer honk like an old-school car.\n\n\
    Run without a subcommand to honk. Drop it at the end of a long command \
    (`cargo build && honk`) and your computer tells you when it is done.")]
#[command(propagate_version = true)]
#[command(args_conflicts_with_subcommands = true)]
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

    #[command(flatten)]
    pub honk: HonkArgs,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Flags for the honk itself (the root command).
#[derive(clap::Args, Debug, Default)]
pub struct HonkArgs {
    /// Horn style
    #[arg(long, value_parser = clap::builder::PossibleValuesParser::new(["bulb", "awooga", "car", "truck", "clown"])
        .map(|s| s.parse::<honk_core::Style>().expect("possible values are valid styles")))]
    pub style: Option<honk_core::Style>,

    /// Number of honks (1-10)
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..=10))]
    pub times: Option<u32>,

    /// Longer honks
    #[arg(long)]
    pub long: bool,

    /// Pitch multiplier (0.5-2.0)
    #[arg(long, value_parser = parse_pitch)]
    pub pitch: Option<f32>,

    /// Volume (0.0-1.0)
    #[arg(long, value_parser = parse_volume)]
    pub volume: Option<f32>,

    /// Honk for an exit status: 0 is a happy honk, anything else a sad one
    #[arg(long, allow_negative_numbers = true, conflicts_with = "command")]
    pub status: Option<i32>,

    /// Write the honk to a WAV file instead of playing it
    #[arg(long, value_name = "FILE")]
    pub wav: Option<std::path::PathBuf>,

    /// Command to run; honk when it finishes and exit with its code (after `--`)
    #[arg(last = true, value_name = "COMMAND")]
    pub command: Vec<String>,
}

fn parse_in(s: &str, range: std::ops::RangeInclusive<f32>, what: &str) -> Result<f32, String> {
    let v: f32 = s.parse().map_err(|_| format!("'{s}' is not a number"))?;
    if range.contains(&v) {
        Ok(v)
    } else {
        Err(format!(
            "{what} must be between {} and {}",
            range.start(),
            range.end()
        ))
    }
}

fn parse_pitch(s: &str) -> Result<f32, String> {
    parse_in(s, honk_core::spec::PITCH_RANGE, "pitch")
}

fn parse_volume(s: &str) -> Result<f32, String> {
    parse_in(s, honk_core::spec::VOLUME_RANGE, "volume")
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Generate shell completions
    Completion {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Show version information
    Version,
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
    /// Dispatch to the selected subcommand. Returns the process exit code.
    pub async fn run(self) -> Result<i32> {
        match self.command {
            Some(Commands::Completion { shell }) => {
                crate::commands::completion::generate_completions(shell);
                Ok(0)
            }
            Some(Commands::Version) => {
                crate::banner::print_banner_with_version();
                Ok(0)
            }
            // No subcommand: the honk itself.
            None => {
                let quiet = self.quiet;
                if !self.honk.command.is_empty() {
                    return crate::commands::run::run(&self.honk, quiet);
                }
                let mood = self
                    .honk
                    .status
                    .map(honk_core::Mood::from_exit_code)
                    .unwrap_or_default();
                crate::commands::honk::honk(&self.honk, mood, quiet)?;
                Ok(0)
            }
        }
    }
}
