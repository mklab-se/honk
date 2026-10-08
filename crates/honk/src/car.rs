//! The car that honks along on stderr while the sound plays: a detailed Braille
//! drawing when the terminal has room, a small ASCII car otherwise.

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

/// A 1920s roadster in Braille dots, 60 columns by 16 rows, every row padded to
/// full width. Generated from `media/car-lineart.png` by
/// `scripts/car_to_braille.py`; regenerate rather than hand-edit.
const BRAILLE_CAR: &str = include_str!("../assets/car-braille.txt");

/// Row of the Braille car that holds the bell horn; the blast comes out there.
const BRAILLE_HORN_ROW: usize = 2;

const BLAST: [&str; 3] = ["", " )", " ) )"];

/// Which car to draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Art {
    Small,
    Braille,
}

impl Art {
    /// The Braille car (60 columns plus the blast, 16 rows) needs an 80x18
    /// terminal; anything smaller, or unknown, gets the small car.
    pub fn for_terminal(size: Option<(u16, u16)>) -> Art {
        match size {
            Some((cols, rows)) if cols >= 80 && rows >= 18 => Art::Braille,
            _ => Art::Small,
        }
    }

    fn rows(self) -> Vec<&'static str> {
        match self {
            Art::Small => CAR.to_vec(),
            Art::Braille => BRAILLE_CAR.lines().collect(),
        }
    }

    fn horn_row(self) -> usize {
        match self {
            Art::Small => 1,
            Art::Braille => BRAILLE_HORN_ROW,
        }
    }
}

/// Animation frames: the car with growing sound waves, then HONK! on each honk.
pub fn frames(honks: u32, art: Art) -> Vec<String> {
    let (rows, horn) = (art.rows(), art.horn_row());
    let mut out = Vec::new();
    for _ in 0..honks.max(1) {
        for (i, blast) in BLAST.iter().enumerate() {
            let shout = if i == BLAST.len() - 1 {
                "  HONK!"
            } else {
                "       "
            };
            let lines: Vec<String> = rows
                .iter()
                .enumerate()
                .map(|(row, line)| {
                    // Every row is the car's width plus room for the blast,
                    // which starts right at the horn.
                    let width = line.chars().count() + 12;
                    let text = if row == horn {
                        format!("{}{blast:<5}{shout}", line.trim_end())
                    } else {
                        (*line).to_string()
                    };
                    format!("{text:<width$}")
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
    let size = terminal_size::terminal_size_of(&stderr).map(|(w, h)| (w.0, h.0));
    let art = Art::for_terminal(size);
    let frames = frames(spec.times, art);
    let total = Duration::from_secs_f32(synth::duration_secs(spec));
    let per_frame = total / frames.len() as u32;
    let player = std::thread::spawn(move || audio::play(samples));
    let height = art.rows().len();
    let start = Instant::now();
    for (i, frame) in frames.iter().enumerate() {
        // Playback ended early (no device): stop instead of honking silently.
        if player.is_finished() {
            break;
        }
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

    fn widths(frame: &str) -> Vec<usize> {
        frame.lines().map(|l| l.chars().count()).collect()
    }

    #[test]
    fn small_frames_are_plain_ascii_of_equal_height() {
        let f = frames(2, Art::Small);
        assert!(f.len() >= 4);
        let h = f[0].lines().count();
        for frame in &f {
            assert!(frame.is_ascii(), "{frame}");
            assert_eq!(frame.lines().count(), h);
        }
    }

    #[test]
    fn braille_frames_are_16_rows_that_redraw_cleanly() {
        for frame in frames(2, Art::Braille) {
            assert_eq!(frame.lines().count(), 16);
            // Same width on every row of every frame, so a redraw overwrites
            // the previous frame completely.
            let w = widths(&frame);
            assert!(w.iter().all(|&x| x == w[0]), "{w:?}");
            assert!(w[0] <= 76, "too wide: {}", w[0]);
        }
    }

    #[test]
    fn the_braille_car_is_drawn_in_braille_and_spaces() {
        let car: String = frames(1, Art::Braille)[0].lines().collect();
        assert!(
            car.chars()
                .all(|c| c == ' ' || ('\u{2800}'..='\u{28FF}').contains(&c))
        );
    }

    #[test]
    fn both_cars_say_honk() {
        for art in [Art::Small, Art::Braille] {
            assert!(
                frames(1, art).iter().any(|f| f.contains("HONK!")),
                "{art:?}"
            );
        }
    }

    #[test]
    fn the_big_car_needs_room() {
        assert_eq!(Art::for_terminal(Some((80, 24))), Art::Braille);
        assert_eq!(Art::for_terminal(Some((79, 24))), Art::Small);
        assert_eq!(Art::for_terminal(Some((120, 17))), Art::Small);
        assert_eq!(Art::for_terminal(None), Art::Small);
    }
}
