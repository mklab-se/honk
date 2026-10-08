//! Horn styles and the synthesis preset ("voice") behind each one.

use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// A horn style the user can pick with `--style`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Style {
    Bulb,
    Awooga,
    #[default]
    Car,
    Truck,
    Clown,
}

/// Oscillator waveform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wave {
    Square,
    Saw,
}

/// Synthesis parameters for one style.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Voice {
    pub wave: Wave,
    /// Base frequencies in Hz, sounded together (two tones = dual-tone horn).
    pub freqs: &'static [f32],
    pub honk_secs: f32,
    pub gap_secs: f32,
    pub attack_secs: f32,
    pub release_secs: f32,
    /// Pitch multiplier over one honk: (position 0..=1, multiplier), three points.
    pub sweep: [(f32, f32); 3],
    pub vibrato_hz: f32,
    pub vibrato_depth: f32,
    /// Noise mix 0..=1, the rasp of a rubber bulb.
    pub noise: f32,
    /// One-pole low-pass cutoff in Hz.
    pub cutoff_hz: f32,
}

impl Voice {
    /// Pitch multiplier at position `t` (0..=1) through a honk, linear between sweep points.
    pub fn pitch_at(&self, t: f32) -> f32 {
        let [(t0, p0), (t1, p1), (t2, p2)] = self.sweep;
        let t = t.clamp(0.0, 1.0);
        if t <= t1 {
            p0 + (p1 - p0) * ((t - t0) / (t1 - t0))
        } else {
            p1 + (p2 - p1) * ((t - t1) / (t2 - t1))
        }
    }
}

const FLAT: [(f32, f32); 3] = [(0.0, 1.0), (0.5, 1.0), (1.0, 1.0)];

impl Style {
    pub const ALL: [Style; 5] = [
        Style::Bulb,
        Style::Awooga,
        Style::Car,
        Style::Truck,
        Style::Clown,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Style::Bulb => "bulb",
            Style::Awooga => "awooga",
            Style::Car => "car",
            Style::Truck => "truck",
            Style::Clown => "clown",
        }
    }

    pub fn voice(self) -> Voice {
        match self {
            Style::Bulb => Voice {
                wave: Wave::Square,
                freqs: &[370.0],
                honk_secs: 0.22,
                gap_secs: 0.12,
                attack_secs: 0.01,
                release_secs: 0.05,
                sweep: FLAT,
                vibrato_hz: 0.0,
                vibrato_depth: 0.0,
                noise: 0.08,
                cutoff_hz: 2500.0,
            },
            Style::Awooga => Voice {
                wave: Wave::Saw,
                freqs: &[220.0],
                honk_secs: 0.6,
                gap_secs: 0.2,
                attack_secs: 0.03,
                release_secs: 0.1,
                sweep: [(0.0, 0.75), (0.35, 1.25), (1.0, 0.95)],
                vibrato_hz: 0.0,
                vibrato_depth: 0.0,
                noise: 0.02,
                cutoff_hz: 1800.0,
            },
            Style::Car => Voice {
                wave: Wave::Saw,
                // G#4 and C5: a major third, like a real dual-tone car horn.
                freqs: &[415.0, 523.0],
                honk_secs: 0.35,
                gap_secs: 0.1,
                attack_secs: 0.01,
                release_secs: 0.04,
                sweep: FLAT,
                vibrato_hz: 0.0,
                vibrato_depth: 0.0,
                noise: 0.0,
                cutoff_hz: 3000.0,
            },
            Style::Truck => Voice {
                wave: Wave::Saw,
                // Two nearly equal tones beat slowly; the third adds body.
                freqs: &[140.0, 141.5, 175.0],
                honk_secs: 0.7,
                gap_secs: 0.18,
                attack_secs: 0.04,
                release_secs: 0.12,
                sweep: FLAT,
                vibrato_hz: 0.0,
                vibrato_depth: 0.0,
                noise: 0.01,
                cutoff_hz: 1500.0,
            },
            Style::Clown => Voice {
                wave: Wave::Square,
                freqs: &[880.0],
                honk_secs: 0.15,
                gap_secs: 0.08,
                attack_secs: 0.005,
                release_secs: 0.03,
                sweep: [(0.0, 1.0), (0.5, 1.15), (1.0, 0.9)],
                vibrato_hz: 12.0,
                vibrato_depth: 0.04,
                noise: 0.03,
                cutoff_hz: 4000.0,
            },
        }
    }
}

impl FromStr for Style {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_ascii_lowercase();
        Style::ALL
            .into_iter()
            .find(|st| st.name() == lower)
            .ok_or_else(|| {
                format!("unknown style '{s}' (choose one of: bulb, awooga, car, truck, clown)")
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_every_style_by_name() {
        for s in Style::ALL {
            assert_eq!(s.name().parse::<Style>().unwrap(), s);
        }
        assert_eq!("AWOOGA".parse::<Style>().unwrap(), Style::Awooga);
    }

    #[test]
    fn unknown_style_lists_the_options() {
        let err = "kazoo".parse::<Style>().unwrap_err();
        assert!(err.contains("bulb, awooga, car, truck, clown"), "{err}");
    }

    #[test]
    fn default_is_car() {
        assert_eq!(Style::default(), Style::Car);
    }

    #[test]
    fn every_voice_is_sane() {
        for s in Style::ALL {
            let v = s.voice();
            assert!(!v.freqs.is_empty(), "{s:?}");
            assert!(v.honk_secs > v.attack_secs + v.release_secs, "{s:?}");
            assert!(v.gap_secs > 0.0 && v.cutoff_hz > 100.0, "{s:?}");
            assert_eq!(v.sweep[0].0, 0.0);
            assert_eq!(v.sweep[2].0, 1.0);
        }
    }

    #[test]
    fn sweep_interpolates_between_points() {
        let v = Style::Awooga.voice();
        assert!((v.pitch_at(0.0) - v.sweep[0].1).abs() < 1e-6);
        assert!((v.pitch_at(v.sweep[1].0) - v.sweep[1].1).abs() < 1e-6);
        assert!((v.pitch_at(1.0) - v.sweep[2].1).abs() < 1e-6);
    }
}
