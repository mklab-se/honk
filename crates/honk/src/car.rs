//! The ASCII car that honks along on stderr while the sound plays.

use std::io::{IsTerminal, Write};
use std::time::{Duration, Instant};

use honk_core::{HonkSpec, synth};

use crate::audio::{self, Playback};

const CAR: [&str; 4] = [
    r"     ______        ",
    r"  __/  |   \___  o<",
    r" |_  _ |  _  __|   ",
    r"   (_)    (_)      ",
];

const BLAST: [&str; 3] = ["", " )", " ) )"];

/// Animation frames: the car with growing sound waves, then HONK! on each honk.
pub fn frames(honks: u32) -> Vec<String> {
    let mut out = Vec::new();
    for _ in 0..honks.max(1) {
        for (i, blast) in BLAST.iter().enumerate() {
            let shout = if i == BLAST.len() - 1 {
                "  HONK!"
            } else {
                "       "
            };
            let lines: Vec<String> = CAR
                .iter()
                .enumerate()
                .map(|(row, line)| {
                    if row == 1 {
                        format!("{line}{blast:<5}{shout}")
                    } else {
                        format!("{line}{:12}", "")
                    }
                })
                .collect();
            out.push(lines.join("\n"));
        }
    }
    out
}

/// Whether the terminal understands the cursor-up escape the animation redraws
/// with. Windows consoles need virtual terminal processing switched on first;
/// colored does that on the console behind stdout, which a terminal session
/// shares with stderr. If it cannot (old conhost, redirected stdout), skip the
/// car rather than print raw escapes.
fn ansi_ok() -> bool {
    #[cfg(windows)]
    {
        colored::control::set_virtual_terminal(true).is_ok()
    }
    #[cfg(not(windows))]
    {
        true
    }
}

/// Play the honk; animate the car on stderr if it is a terminal.
pub fn play_with_car(samples: Vec<f32>, spec: &HonkSpec) -> Playback {
    let mut stderr = std::io::stderr();
    if !stderr.is_terminal() || !ansi_ok() {
        return audio::play(samples);
    }
    let frames = frames(spec.times);
    let total = Duration::from_secs_f32(synth::duration_secs(spec));
    let per_frame = total / frames.len() as u32;
    let player = std::thread::spawn(move || audio::play(samples));
    let height = CAR.len();
    let start = Instant::now();
    for (i, frame) in frames.iter().enumerate() {
        if i > 0 {
            let _ = write!(stderr, "\x1b[{height}A");
        }
        let _ = writeln!(stderr, "{frame}");
        let _ = stderr.flush();
        let next = per_frame * (i as u32 + 1);
        if let Some(wait) = next.checked_sub(start.elapsed()) {
            std::thread::sleep(wait);
        }
    }
    player
        .join()
        .unwrap_or(Playback::NoDevice("playback thread panicked".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_are_plain_ascii_of_equal_height() {
        let f = frames(2);
        assert!(f.len() >= 4);
        let h = f[0].lines().count();
        for frame in &f {
            assert!(frame.is_ascii(), "{frame}");
            assert_eq!(frame.lines().count(), h);
        }
    }

    #[test]
    fn frames_say_honk() {
        assert!(frames(1).iter().any(|f| f.contains("HONK")));
    }
}
