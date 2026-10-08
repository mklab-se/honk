//! The honk synthesiser: turns a [`HonkSpec`] into mono `f32` samples.
//!
//! Each honk is one or more oscillators (sawtooth or square) with a pitch
//! sweep, optional vibrato and noise, through a one-pole low-pass filter, an
//! attack/release envelope and soft clipping. Output is deterministic.

use std::f32::consts::TAU;

use crate::spec::{HonkSpec, Mood};
use crate::style::{Voice, Wave};

pub const SAMPLE_RATE: u32 = 44_100;

const SR: f32 = SAMPLE_RATE as f32;
/// The last honk is this much longer when sad.
const SAD_STRETCH: f32 = 1.6;
/// Sad bends the last honk down to this pitch multiplier.
const SAD_END_PITCH: f32 = 0.6;
/// Happy bends up over the last part of the last honk.
const HAPPY_FROM: f32 = 0.6;
const HAPPY_END_PITCH: f32 = 1.3;
/// Soft clip drive; output is normalised so a full-scale input maps to 1.0.
const DRIVE: f32 = 1.5;

fn secs_to_samples(secs: f32) -> usize {
    (secs * SR).round() as usize
}

fn honk_secs(spec: &HonkSpec, voice: &Voice, last: bool) -> f32 {
    let mut secs = voice.honk_secs;
    if spec.long {
        secs *= 2.0;
    }
    if last && spec.mood == Mood::Sad {
        secs *= SAD_STRETCH;
    }
    secs
}

/// Total length of [`render`]'s output in seconds.
pub fn duration_secs(spec: &HonkSpec) -> f32 {
    let voice = spec.style.voice();
    let samples: usize = (0..spec.times)
        .map(|i| secs_to_samples(honk_secs(spec, &voice, i + 1 == spec.times)))
        .sum::<usize>()
        + spec.times.saturating_sub(1) as usize * secs_to_samples(voice.gap_secs);
    samples as f32 / SR
}

/// Mood pitch bend at position `t` (0..=1) of the last honk.
fn mood_bend(mood: Mood, t: f32) -> f32 {
    match mood {
        Mood::Neutral => 1.0,
        Mood::Sad => 1.0 + (SAD_END_PITCH - 1.0) * t,
        Mood::Happy if t > HAPPY_FROM => {
            1.0 + (HAPPY_END_PITCH - 1.0) * ((t - HAPPY_FROM) / (1.0 - HAPPY_FROM))
        }
        Mood::Happy => 1.0,
    }
}

fn oscillator(wave: Wave, phase: f32) -> f32 {
    match wave {
        Wave::Saw => 2.0 * phase - 1.0,
        Wave::Square => {
            if phase < 0.5 {
                1.0
            } else {
                -1.0
            }
        }
    }
}

/// Tiny deterministic noise source (xorshift), so renders are reproducible.
struct Noise(u32);

impl Noise {
    fn next(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        (self.0 as f32 / u32::MAX as f32) * 2.0 - 1.0
    }
}

/// Render the honk to mono samples at [`SAMPLE_RATE`].
pub fn render(spec: &HonkSpec) -> Vec<f32> {
    let voice = spec.style.voice();
    let gap = secs_to_samples(voice.gap_secs);
    let mut out = Vec::new();
    for i in 0..spec.times {
        let last = i + 1 == spec.times;
        if i > 0 {
            out.extend(std::iter::repeat_n(0.0, gap));
        }
        let mood = if last { spec.mood } else { Mood::Neutral };
        let n = secs_to_samples(honk_secs(spec, &voice, last));
        render_honk(&mut out, spec, &voice, n, mood);
    }
    out
}

fn render_honk(out: &mut Vec<f32>, spec: &HonkSpec, voice: &Voice, n: usize, mood: Mood) {
    let attack = secs_to_samples(voice.attack_secs).max(1);
    let release = secs_to_samples(voice.release_secs).max(1);
    let alpha = 1.0 - (-TAU * voice.cutoff_hz / SR).exp();
    let norm = DRIVE.tanh();
    let mut phases = vec![0.0f32; voice.freqs.len()];
    let mut noise = Noise(0x9E37_79B9);
    let mut lp = 0.0f32;

    for j in 0..n {
        let t = j as f32 / n as f32;
        let vibrato = 1.0 + voice.vibrato_depth * (TAU * voice.vibrato_hz * j as f32 / SR).sin();
        let pitch = spec.pitch * voice.pitch_at(t) * mood_bend(mood, t) * vibrato;

        let mut x = 0.0;
        for (phase, &f) in phases.iter_mut().zip(voice.freqs) {
            x += oscillator(voice.wave, *phase);
            *phase = (*phase + f * pitch / SR).fract();
        }
        x /= voice.freqs.len() as f32;
        x = x * (1.0 - voice.noise) + noise.next() * voice.noise;

        lp += alpha * (x - lp);

        let env = if j < attack {
            j as f32 / attack as f32
        } else if j + release >= n {
            (n - 1 - j) as f32 / release as f32
        } else {
            1.0
        };

        out.push((lp * DRIVE).tanh() / norm * env * spec.volume);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::{HonkSpec, Mood};
    use crate::style::Style;

    fn spec(style: Style) -> HonkSpec {
        HonkSpec {
            style,
            ..HonkSpec::default()
        }
    }

    /// Split into bursts: runs of sound separated by at least 100 exact-zero samples.
    fn bursts(samples: &[f32]) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        let (mut start, mut zeros) = (None, 0usize);
        for (i, &s) in samples.iter().enumerate() {
            if s == 0.0 {
                zeros += 1;
                if zeros == 100
                    && let Some(st) = start.take()
                {
                    out.push((st, i + 1 - 100));
                }
            } else {
                zeros = 0;
                start.get_or_insert(i);
            }
        }
        if let Some(st) = start {
            out.push((st, samples.len()));
        }
        out
    }

    /// Positive-going zero crossings in samples[a..b], a pitch proxy.
    fn crossings(samples: &[f32], a: usize, b: usize) -> usize {
        samples[a..b]
            .windows(2)
            .filter(|w| w[0] < 0.0 && w[1] >= 0.0)
            .count()
    }

    #[test]
    fn bulb_has_the_expected_length() {
        // 2 honks of 0.22 s with one 0.12 s gap.
        assert_eq!(render(&spec(Style::Bulb)).len(), 2 * 9702 + 5292);
    }

    #[test]
    fn long_doubles_each_honk() {
        let s = HonkSpec {
            long: true,
            ..spec(Style::Bulb)
        };
        assert_eq!(render(&s).len(), 2 * 19404 + 5292);
    }

    #[test]
    fn duration_matches_render() {
        for style in Style::ALL {
            let s = HonkSpec {
                times: 3,
                mood: Mood::Sad,
                ..spec(style)
            };
            let secs = render(&s).len() as f32 / SAMPLE_RATE as f32;
            assert!((secs - duration_secs(&s)).abs() < 0.001, "{style:?}");
        }
    }

    #[test]
    fn every_style_stays_within_volume_and_is_finite() {
        for style in Style::ALL {
            for volume in [0.0, 0.3, 1.0] {
                let s = HonkSpec {
                    volume,
                    times: 2,
                    ..spec(style)
                };
                let samples = render(&s);
                assert!(!samples.is_empty());
                assert!(samples.iter().all(|x| x.is_finite()), "{style:?}");
                let peak = samples.iter().fold(0.0f32, |m, x| m.max(x.abs()));
                assert!(peak <= volume + 1e-6, "{style:?} peak {peak} > {volume}");
                if volume > 0.0 {
                    assert!(peak > volume * 0.3, "{style:?} is too quiet: {peak}");
                }
            }
        }
    }

    #[test]
    fn times_gives_that_many_bursts() {
        for style in Style::ALL {
            for times in [1, 3, 5] {
                let s = HonkSpec {
                    times,
                    ..spec(style)
                };
                assert_eq!(
                    bursts(&render(&s)).len(),
                    times as usize,
                    "{style:?} x{times}"
                );
            }
        }
    }

    #[test]
    fn rendering_is_deterministic() {
        let s = spec(Style::Clown);
        assert_eq!(render(&s), render(&s));
    }

    #[test]
    fn sad_bends_down_and_is_longer() {
        let neutral = render(&HonkSpec {
            times: 1,
            ..spec(Style::Bulb)
        });
        let sad = render(&HonkSpec {
            times: 1,
            mood: Mood::Sad,
            ..spec(Style::Bulb)
        });
        assert!(sad.len() > neutral.len());
        let n = sad.len();
        let start = crossings(&sad, n / 10, n * 3 / 10);
        let end = crossings(&sad, n * 7 / 10, n * 9 / 10);
        assert!(
            (end as f32) < start as f32 * 0.85,
            "start {start}, end {end}"
        );
    }

    #[test]
    fn happy_bends_up_at_the_end() {
        let happy = render(&HonkSpec {
            times: 1,
            mood: Mood::Happy,
            ..spec(Style::Bulb)
        });
        let n = happy.len();
        let start = crossings(&happy, n / 10, n * 3 / 10);
        let end = crossings(&happy, n * 75 / 100, n * 95 / 100);
        assert!(end as f32 > start as f32 * 1.08, "start {start}, end {end}");
    }

    #[test]
    fn only_the_last_honk_carries_the_mood() {
        let neutral = render(&HonkSpec {
            times: 2,
            ..spec(Style::Car)
        });
        let sad = render(&HonkSpec {
            times: 2,
            mood: Mood::Sad,
            ..spec(Style::Car)
        });
        let first = bursts(&neutral)[0];
        assert_eq!(&neutral[first.0..first.1], &sad[first.0..first.1]);
    }

    #[test]
    fn higher_pitch_means_more_crossings() {
        let low = render(&HonkSpec {
            times: 1,
            pitch: 0.5,
            ..spec(Style::Bulb)
        });
        let high = render(&HonkSpec {
            times: 1,
            pitch: 2.0,
            ..spec(Style::Bulb)
        });
        assert!(crossings(&high, 0, high.len()) > 3 * crossings(&low, 0, low.len()));
    }
}
