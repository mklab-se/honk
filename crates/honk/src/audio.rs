//! Playing samples on the default output device.

use std::num::NonZero;

use honk_core::synth::SAMPLE_RATE;

/// Outcome of trying to play a honk.
#[derive(Debug)]
pub enum Playback {
    Played,
    /// No usable output device (headless CI, SSH, containers). Not an error.
    NoDevice(String),
}

/// Play mono samples at [`SAMPLE_RATE`], blocking until the sound has finished.
pub fn play(samples: Vec<f32>) -> Playback {
    if std::env::var_os("HONK_NO_AUDIO").is_some() {
        return Playback::NoDevice("disabled by HONK_NO_AUDIO".into());
    }
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
    fn honk_no_audio_disables_playback() {
        // SAFETY: tests in this module are the only readers of HONK_NO_AUDIO.
        unsafe { std::env::set_var("HONK_NO_AUDIO", "1") };
        assert!(matches!(play(vec![0.0; 10]), Playback::NoDevice(_)));
    }
}
