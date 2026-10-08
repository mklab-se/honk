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

/// A 1920s roadster in Braille dots at one size. All sizes are generated from
/// `media/car-lineart.png` by `scripts/car_to_braille.py`; regenerate rather
/// than hand-edit. Every row of `art` is padded to `cols` characters.
#[derive(Debug)]
struct BrailleCar {
    cols: usize,
    rows: usize,
    /// Row holding the bell horn; the blast comes out there.
    horn_row: usize,
    art: &'static str,
}

include!("../assets/braille/cars.rs");

/// Columns the blast and `HONK!` need to the right of the car.
const BLAST_COLS: usize = 12;
/// Rows to leave free besides the car: the command line and the next prompt.
const SPARE_ROWS: usize = 3;

const BLAST: [&str; 3] = ["", " )", " ) )"];

/// Which car to draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Art {
    Small,
    /// The Braille car with this many columns.
    Braille(usize),
}

impl Art {
    /// The largest Braille car that fits with room for the blast and the
    /// prompt; the small ASCII car when none fits or the size is unknown.
    /// Never pick a car taller than the window: the redraw moves the cursor up
    /// by the car's height and would smear across the screen.
    pub fn for_terminal(size: Option<(u16, u16)>) -> Art {
        let Some((cols, rows)) = size else {
            return Art::Small;
        };
        let (cols, rows) = (usize::from(cols), usize::from(rows));
        BRAILLE_CARS
            .iter()
            .find(|car| cols >= car.cols + BLAST_COLS && rows >= car.rows + SPARE_ROWS)
            .map_or(Art::Small, |car| Art::Braille(car.cols))
    }

    fn braille(cols: usize) -> &'static BrailleCar {
        BRAILLE_CARS
            .iter()
            .find(|car| car.cols == cols)
            .expect("Art::Braille is only built from BRAILLE_CARS")
    }

    fn rows(self) -> Vec<&'static str> {
        match self {
            Art::Small => CAR.to_vec(),
            Art::Braille(cols) => Art::braille(cols).art.lines().collect(),
        }
    }

    fn horn_row(self) -> usize {
        match self {
            Art::Small => 1,
            Art::Braille(cols) => Art::braille(cols).horn_row,
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
                    let width = line.chars().count() + BLAST_COLS;
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
    tracing::debug!(?size, ?art, "terminal size (columns, rows) and chosen car");
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
    fn every_braille_car_matches_its_table_entry() {
        for car in BRAILLE_CARS {
            let rows: Vec<&str> = car.art.lines().collect();
            assert_eq!(rows.len(), car.rows, "{}", car.cols);
            assert!(
                rows.iter().all(|r| r.chars().count() == car.cols),
                "{}",
                car.cols
            );
            assert!(car.horn_row < car.rows, "{}", car.cols);
            assert!(!rows[car.horn_row].trim().is_empty(), "{}", car.cols);
            assert!(
                car.art
                    .chars()
                    .all(|c| c == ' ' || c == '\n' || ('\u{2800}'..='\u{28FF}').contains(&c)),
                "{}",
                car.cols
            );
        }
    }

    #[test]
    fn braille_frames_redraw_cleanly_and_say_honk() {
        for car in BRAILLE_CARS {
            let art = Art::Braille(car.cols);
            let f = frames(2, art);
            assert!(f.iter().any(|fr| fr.contains("HONK!")), "{}", car.cols);
            for frame in f {
                assert_eq!(frame.lines().count(), car.rows);
                // Same width on every row of every frame, so a redraw
                // overwrites the previous frame completely.
                let w = widths(&frame);
                assert!(w.iter().all(|&x| x == w[0]), "{} {w:?}", car.cols);
                assert!(w[0] <= car.cols + BLAST_COLS, "{}", car.cols);
            }
        }
    }

    #[test]
    fn the_small_car_says_honk() {
        assert!(frames(1, Art::Small).iter().any(|f| f.contains("HONK!")));
    }

    #[test]
    fn the_largest_car_that_fits_wins() {
        assert_eq!(Art::for_terminal(Some((200, 50))), Art::Braille(96));
        assert_eq!(Art::for_terminal(Some((100, 30))), Art::Braille(76));
        assert_eq!(Art::for_terminal(Some((80, 24))), Art::Braille(60));
        assert_eq!(Art::for_terminal(Some((60, 20))), Art::Braille(48));
        assert_eq!(Art::for_terminal(Some((55, 20))), Art::Braille(40));
        // Height matters too: never draw a car taller than the window.
        assert_eq!(Art::for_terminal(Some((200, 22))), Art::Braille(60));
        assert_eq!(Art::for_terminal(Some((100, 12))), Art::Small);
        assert_eq!(Art::for_terminal(Some((50, 40))), Art::Small);
        assert_eq!(Art::for_terminal(None), Art::Small);
    }
}
