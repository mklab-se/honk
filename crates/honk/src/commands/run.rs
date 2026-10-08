//! `honk -- cmd`: run a command, honk by outcome, pass its exit code through.

use std::ffi::OsStr;
use std::path::PathBuf;
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

/// First `<dir>/<program><ext>` that exists, trying each `PATH` directory in
/// order and each `PATHEXT` extension in order within it.
fn find_in_path(program: &str, path: &OsStr, pathext: &str) -> Option<PathBuf> {
    std::env::split_paths(path).find_map(|dir| {
        pathext
            .split(';')
            .filter(|ext| !ext.is_empty())
            .map(|ext| dir.join(format!("{program}{}", ext.to_ascii_lowercase())))
            .find(|candidate| candidate.is_file())
    })
}

/// The program to spawn. On Windows, `Command` only finds `.exe` files on
/// `PATH`, so `npm`, `yarn` or `gradlew` (`.cmd`/`.bat` shims) would be "not
/// found"; resolve them through `PATHEXT` the way the shell does. Rust runs a
/// resolved `.cmd`/`.bat` path with its own safe argument escaping.
fn resolve_program(program: &str) -> PathBuf {
    if cfg!(windows)
        && !program.contains(['/', '\\'])
        && std::path::Path::new(program).extension().is_none()
    {
        let path = std::env::var_os("PATH").unwrap_or_default();
        let pathext = std::env::var("PATHEXT").unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".into());
        if let Some(found) = find_in_path(program, &path, &pathext) {
            return found;
        }
    }
    PathBuf::from(program)
}

/// Run `args.command`, honk happy or sad, and return the command's exit code.
pub fn run(args: &HonkArgs, quiet: bool) -> Result<i32> {
    let (program, rest) = args
        .command
        .split_first()
        .expect("clap guarantees a command");
    // Ctrl-C goes to the whole foreground process group. Like `time` or a shell,
    // stay alive and let the command decide how to stop; its exit status then
    // picks the honk. A handler (not "ignore") keeps the child's default
    // disposition, since exec resets handled signals.
    let _ = ctrlc::set_handler(|| {});
    let code = match Command::new(resolve_program(program)).args(rest).status() {
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

#[cfg(test)]
mod path_tests {
    use super::*;

    #[test]
    fn finds_a_cmd_shim_through_pathext() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("tool.cmd"), "").unwrap();
        let path = std::env::join_paths([dir.path()]).unwrap();
        assert_eq!(
            find_in_path("tool", &path, ".EXE;.CMD"),
            Some(dir.path().join("tool.cmd"))
        );
    }

    #[test]
    fn earlier_extensions_and_directories_win() {
        let (a, b) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        std::fs::write(a.path().join("tool.cmd"), "").unwrap();
        std::fs::write(b.path().join("tool.exe"), "").unwrap();
        std::fs::write(b.path().join("tool.cmd"), "").unwrap();
        let path = std::env::join_paths([a.path(), b.path()]).unwrap();
        assert_eq!(
            find_in_path("tool", &path, ".EXE;.CMD"),
            Some(a.path().join("tool.cmd"))
        );
    }

    #[test]
    fn missing_program_is_none() {
        let dir = tempfile::tempdir().unwrap();
        let path = std::env::join_paths([dir.path()]).unwrap();
        assert_eq!(find_in_path("tool", &path, ".EXE;.CMD"), None);
    }
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
