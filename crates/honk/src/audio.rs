//! Playing samples on the default output device.
//!
//! macOS and Windows play through `rodio` (CoreAudio, WASAPI). Linux pipes a WAV
//! into the system's own player (see `player.rs`), so building and running honk
//! there needs no audio library.

use honk_core::synth::SAMPLE_RATE;

/// Outcome of trying to play a honk.
#[derive(Debug)]
pub enum Playback {
    Played,
    /// No usable output (headless CI, SSH, containers). Not an error.
    NoDevice(String),
}

/// Trailing silence so the device has drained the honk's release before the
/// stream closes; without it the last buffer can be cut off.
const TAIL_SAMPLES: usize = SAMPLE_RATE as usize / 10;

fn with_tail(mut samples: Vec<f32>) -> Vec<f32> {
    samples.extend(std::iter::repeat_n(0.0, TAIL_SAMPLES));
    samples
}

/// Play mono samples at [`SAMPLE_RATE`], blocking until the sound has finished.
pub fn play(samples: Vec<f32>) -> Playback {
    if std::env::var_os("HONK_NO_AUDIO").is_some() {
        return Playback::NoDevice("disabled by HONK_NO_AUDIO".into());
    }
    play_on_device(with_tail(samples))
}

#[cfg(target_os = "linux")]
fn play_on_device(samples: Vec<f32>) -> Playback {
    use crate::player::{PLAYERS, Played, play_with};
    let wav = match honk_core::wav::encode_wav(&samples) {
        Ok(wav) => wav,
        Err(e) => return Playback::NoDevice(e.to_string()),
    };
    match play_with(PLAYERS, &wav) {
        Played::Yes => Playback::Played,
        Played::No(why) => Playback::NoDevice(why),
    }
}

#[cfg(not(target_os = "linux"))]
fn play_on_device(samples: Vec<f32>) -> Playback {
    use std::num::NonZero;

    let mut handle = match rodio::DeviceSinkBuilder::open_default_sink() {
        Ok(h) => h,
        Err(e) => return Playback::NoDevice(e.to_string()),
    };
    handle.log_on_drop(false);
    let player = rodio::Player::connect_new(handle.mixer());
    player.append(rodio::buffer::SamplesBuffer::new(
        NonZero::new(1).expect("1 is non-zero"),
        NonZero::new(SAMPLE_RATE).expect("sample rate is non-zero"),
        samples,
    ));
    player.sleep_until_end();
    Playback::Played
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_silent_tail_protects_the_end_of_the_honk() {
        let padded = with_tail(vec![0.5; 10]);
        assert_eq!(padded.len(), 10 + TAIL_SAMPLES);
        assert!(padded[..10].iter().all(|&s| s == 0.5));
        assert!(padded[10..].iter().all(|&s| s == 0.0));
    }

    #[test]
    fn honk_no_audio_disables_playback() {
        // SAFETY: tests in this module are the only readers of HONK_NO_AUDIO.
        unsafe { std::env::set_var("HONK_NO_AUDIO", "1") };
        assert!(matches!(play(vec![0.0; 10]), Playback::NoDevice(_)));
    }
}
