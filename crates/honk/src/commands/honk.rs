//! The honk itself: resolve the spec, render, then play or write a WAV.

use anyhow::Result;
use colored::Colorize;
use honk_core::config::Config;
use honk_core::{HonkSpec, Mood, Overrides, synth, wav};

use crate::audio::{self, Playback};
use crate::cli::HonkArgs;

fn warn(msg: &str) {
    eprintln!("{} {msg}", "warning:".yellow().bold());
}

/// Honk once with the given mood, honouring config and flags.
pub fn honk(args: &HonkArgs, mood: Mood, quiet: bool) -> Result<()> {
    let config = Config::load().unwrap_or_else(|e| {
        warn(&format!("could not read config, using defaults: {e}"));
        Config::default()
    });
    let overrides = Overrides {
        style: args.style,
        times: args.times,
        long: args.long,
        pitch: args.pitch,
        volume: args.volume,
        mood,
    };
    let (spec, warnings) = HonkSpec::resolve(&config, &overrides)?;
    for w in &warnings {
        warn(w);
    }
    let samples = synth::render(&spec);

    if let Some(path) = &args.wav {
        wav::write_wav(path, &samples)?;
        return Ok(());
    }

    let _ = quiet; // used by the car in Task 14
    if let Playback::NoDevice(why) = audio::play(samples) {
        warn(&format!(
            "no audio output available ({why}); honk silently skipped"
        ));
    }
    Ok(())
}
