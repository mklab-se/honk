//! `honk -- cmd`: run a command, honk by outcome, pass its exit code through.

use std::process::{Command, ExitStatus};

use anyhow::Result;
use colored::Colorize;
use honk_core::Mood;

use crate::cli::HonkArgs;

/// Exit code for a finished child, following the shell convention for signals.
pub fn exit_code(status: ExitStatus) -> i32 {
    if let Some(code) = status.code() {
        return code;
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(sig) = status.signal() {
            return 128 + sig;
        }
    }
    1
}

/// Run `args.command`, honk happy or sad, and return the command's exit code.
pub fn run(args: &HonkArgs, quiet: bool) -> Result<i32> {
    let (program, rest) = args
        .command
        .split_first()
        .expect("clap guarantees a command");
    let code = match Command::new(program).args(rest).status() {
        Ok(status) => exit_code(status),
        Err(e) => {
            eprintln!("{} could not run '{program}': {e}", "error:".red().bold());
            127
        }
    };
    // The command's exit code is what callers depend on; a failed honk must not replace it.
    if let Err(e) = crate::commands::honk::honk(args, Mood::from_exit_code(code), quiet) {
        eprintln!("{} {e:#}", "error:".red().bold());
    }
    Ok(code)
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::process::ExitStatusExt;

    #[test]
    fn signal_maps_to_128_plus_signal() {
        // Raw wait status 9 = killed by SIGKILL.
        assert_eq!(exit_code(ExitStatus::from_raw(9)), 137);
    }

    #[test]
    fn normal_exit_code_is_kept() {
        assert_eq!(exit_code(ExitStatus::from_raw(3 << 8)), 3);
    }
}
