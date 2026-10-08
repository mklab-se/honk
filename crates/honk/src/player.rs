//! Linux playback through the system's own audio player.
//!
//! Every desktop Linux ships one of PipeWire's `pw-play`, PulseAudio's `paplay`
//! or ALSA's `aplay`. Piping a WAV into the first one that works means honk
//! needs no audio library to build or run.

use std::io::Write;
use std::process::{Command, Stdio};

/// Players to try, in order, each reading a WAV from stdin.
pub const PLAYERS: &[(&str, &[&str])] = &[("pw-play", &["-"]), ("paplay", &[]), ("aplay", &["-q"])];

/// Whether a player took the honk.
#[derive(Debug)]
pub enum Played {
    Yes,
    /// Nothing could play it; the reason names what was tried.
    No(String),
}

/// Pipe `wav` into the first player that runs and exits successfully. A player
/// that is missing, or fails (no sound server, no card), hands over to the next.
/// Players' own output is discarded; honk reports problems once, itself.
pub fn play_with(players: &[(&str, &[&str])], wav: &[u8]) -> Played {
    for (program, args) in players {
        let Ok(mut child) = Command::new(program)
            .args(*args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        else {
            continue;
        };
        if let Some(mut stdin) = child.stdin.take() {
            // A player that exits early closes the pipe; its exit status decides.
            let _ = stdin.write_all(wav);
        }
        if child.wait().is_ok_and(|status| status.success()) {
            return Played::Yes;
        }
    }
    let tried: Vec<&str> = players.iter().map(|(p, _)| *p).collect();
    Played::No(format!(
        "no working audio player (tried {})",
        tried.join(", ")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    /// A fake player: a shell script that copies stdin to `out` and exits `code`.
    fn fake_player(dir: &std::path::Path, name: &str, out: &std::path::Path, code: i32) -> String {
        let path = dir.join(name);
        std::fs::write(
            &path,
            format!("#!/bin/sh\ncat > '{}'\nexit {code}\n", out.display()),
        )
        .unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        path.to_string_lossy().into_owned()
    }

    #[test]
    fn skips_missing_players_and_feeds_the_wav_to_the_first_that_works() {
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("heard.wav");
        let good = fake_player(dir.path(), "good", &out, 0);
        let players = [
            ("definitely-not-a-player-honk", &[][..]),
            (good.as_str(), &[][..]),
        ];
        let wav = honk_core::wav::encode_wav(&[0.0, 0.5, -0.5]).unwrap();
        assert!(matches!(play_with(&players, &wav), Played::Yes));
        assert_eq!(std::fs::read(&out).unwrap(), wav);
    }

    #[test]
    fn a_failing_player_falls_through_to_the_next() {
        let dir = tempfile::tempdir().unwrap();
        let (bad_out, good_out) = (dir.path().join("bad.wav"), dir.path().join("good.wav"));
        let bad = fake_player(dir.path(), "bad", &bad_out, 1);
        let good = fake_player(dir.path(), "good", &good_out, 0);
        let players = [(bad.as_str(), &[][..]), (good.as_str(), &[][..])];
        assert!(matches!(play_with(&players, b"RIFF"), Played::Yes));
        assert_eq!(std::fs::read(&good_out).unwrap(), b"RIFF");
    }

    #[test]
    fn no_working_player_says_which_were_tried() {
        let players = [("nope-one-honk", &[][..]), ("nope-two-honk", &[][..])];
        match play_with(&players, b"RIFF") {
            Played::No(why) => assert!(
                why.contains("nope-one-honk") && why.contains("nope-two-honk"),
                "{why}"
            ),
            Played::Yes => panic!("nothing should have played"),
        }
    }
}
