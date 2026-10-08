//! What to honk: the fully resolved request handed to the synthesiser.

use std::ops::RangeInclusive;

use crate::config::Config;
use crate::error::{Error, Result};
use crate::style::Style;

pub const TIMES_RANGE: RangeInclusive<u32> = 1..=10;
pub const PITCH_RANGE: RangeInclusive<f32> = 0.5..=2.0;
pub const VOLUME_RANGE: RangeInclusive<f32> = 0.0..=1.0;

/// How the honk should feel: neutral, or the outcome of a command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mood {
    #[default]
    Neutral,
    Happy,
    Sad,
}

impl Mood {
    /// 0 is success (happy); anything else is failure (sad).
    pub fn from_exit_code(code: i32) -> Mood {
        if code == 0 { Mood::Happy } else { Mood::Sad }
    }
}

/// A fully resolved honk.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HonkSpec {
    pub style: Style,
    pub times: u32,
    pub long: bool,
    pub pitch: f32,
    pub volume: f32,
    pub mood: Mood,
}

impl Default for HonkSpec {
    fn default() -> Self {
        HonkSpec {
            style: Style::Bulb,
            times: 2,
            long: false,
            pitch: 1.0,
            volume: 0.8,
            mood: Mood::Neutral,
        }
    }
}

/// Values given on the command line; `None` means "not given".
#[derive(Debug, Clone, Copy, Default)]
pub struct Overrides {
    pub style: Option<Style>,
    pub times: Option<u32>,
    pub long: bool,
    pub pitch: Option<f32>,
    pub volume: Option<f32>,
    pub mood: Mood,
}

impl HonkSpec {
    /// Resolve defaults, then config, then overrides. Invalid config values are
    /// skipped and reported as warnings; invalid overrides are errors.
    pub fn resolve(config: &Config, o: &Overrides) -> Result<(HonkSpec, Vec<String>)> {
        let mut spec = HonkSpec::default();
        let mut warnings = Vec::new();

        if let Some(style) = config.style {
            spec.style = style;
        }
        match config.volume {
            Some(v) if VOLUME_RANGE.contains(&v) => spec.volume = v,
            Some(v) => warnings.push(format!(
                "ignoring config volume {v}: must be between 0.0 and 1.0"
            )),
            None => {}
        }
        match config.times {
            Some(t) if TIMES_RANGE.contains(&t) => spec.times = t,
            Some(t) => warnings.push(format!(
                "ignoring config times {t}: must be between 1 and 10"
            )),
            None => {}
        }

        if let Some(style) = o.style {
            spec.style = style;
        }
        if let Some(t) = o.times {
            check(
                TIMES_RANGE.contains(&t),
                format!("--times {t} must be between 1 and 10"),
            )?;
            spec.times = t;
        }
        if let Some(p) = o.pitch {
            check(
                PITCH_RANGE.contains(&p),
                format!("--pitch {p} must be between 0.5 and 2.0"),
            )?;
            spec.pitch = p;
        }
        if let Some(v) = o.volume {
            check(
                VOLUME_RANGE.contains(&v),
                format!("--volume {v} must be between 0.0 and 1.0"),
            )?;
            spec.volume = v;
        }
        spec.long = o.long;
        spec.mood = o.mood;
        Ok((spec, warnings))
    }
}

fn check(ok: bool, msg: String) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(Error::InvalidSpec(msg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn defaults_without_config_or_flags() {
        let (spec, warnings) =
            HonkSpec::resolve(&Config::default(), &Overrides::default()).unwrap();
        assert_eq!(spec, HonkSpec::default());
        assert!(warnings.is_empty());
    }

    #[test]
    fn config_beats_defaults_and_flags_beat_config() {
        let cfg = Config {
            style: Some(Style::Truck),
            volume: Some(0.5),
            times: Some(3),
        };
        let (spec, _) = HonkSpec::resolve(&cfg, &Overrides::default()).unwrap();
        assert_eq!(
            (spec.style, spec.volume, spec.times),
            (Style::Truck, 0.5, 3)
        );

        let o = Overrides {
            style: Some(Style::Clown),
            times: Some(1),
            ..Overrides::default()
        };
        let (spec, _) = HonkSpec::resolve(&cfg, &o).unwrap();
        assert_eq!(
            (spec.style, spec.volume, spec.times),
            (Style::Clown, 0.5, 1)
        );
    }

    #[test]
    fn out_of_range_config_values_are_skipped_with_a_warning() {
        let cfg = Config {
            style: None,
            volume: Some(7.0),
            times: Some(0),
        };
        let (spec, warnings) = HonkSpec::resolve(&cfg, &Overrides::default()).unwrap();
        assert_eq!(spec.volume, HonkSpec::default().volume);
        assert_eq!(spec.times, HonkSpec::default().times);
        assert_eq!(warnings.len(), 2, "{warnings:?}");
        assert!(warnings[0].contains("volume"));
    }

    #[test]
    fn out_of_range_overrides_are_errors() {
        for o in [
            Overrides {
                pitch: Some(9.0),
                ..Overrides::default()
            },
            Overrides {
                volume: Some(-0.1),
                ..Overrides::default()
            },
            Overrides {
                times: Some(11),
                ..Overrides::default()
            },
        ] {
            assert!(HonkSpec::resolve(&Config::default(), &o).is_err());
        }
    }

    #[test]
    fn mood_from_exit_code() {
        assert_eq!(Mood::from_exit_code(0), Mood::Happy);
        assert_eq!(Mood::from_exit_code(1), Mood::Sad);
        assert_eq!(Mood::from_exit_code(-1), Mood::Sad);
    }
}
