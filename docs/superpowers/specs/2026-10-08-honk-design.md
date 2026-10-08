# Honk: design

Date: 2026-10-08
Status: approved 2026-10-08

## Purpose

`honk` makes the computer honk like an old-school car: "Honk, honk!". It is a toy at heart, but
built to be useful as a notifier in scripts and terminals (`cargo build && honk`,
`honk -- long-job`). It must work on Linux, macOS and Windows.

## Decisions

| Topic | Decision |
| --- | --- |
| Name | `honk` (crates `honk` and `honk-core`, repo `mklab-se/honk`, all verified free) |
| Base | Generated from `mklab-se/rusty-tmpl`; MKLab standard applies (MIT, public, crates.io, GitHub Release, Homebrew tap) |
| Sound source | Synthesised in Rust, no bundled audio files |
| Playback | `rodio` 0.22 on macOS (CoreAudio) and Windows (WASAPI). Linux, amended 2026-10-08 after 0.2.0: a WAV piped into the system player (`pw-play`, `paplay` or `aplay`), because linking ALSA broke `cargo install` on stock Linux machines |
| AI | The template's ailloy integration (`honk ai ...`) is kept as is; AI-driven features come later |
| Delivery | Two stages, each ending in a release (see "Stages") |

## Stages

### Stage 1: hello world, v0.1.0

Goal: prove the full CI/CD pipeline before any real feature work.

- Rename the template to `honk` following the template README's "Using this template" recipe
  (crates, media file, `sed`, hand-edited `CLAUDE.md` with the lineage section kept, banner
  regenerated). Set the CLI crate's `description` and `keywords` by hand.
- `honk` with no subcommand prints exactly `Honk, honk!` to stdout and nothing else.
  The template subcommands (`ai`, `completion`, `version`) remain.
- CI: the test job runs on `ubuntu-latest`, `macos-latest` and `windows-latest` (the template
  only runs on Ubuntu). Lint and other jobs stay on Ubuntu.
- Release secrets configured (`setup-release-secrets`, tap `mklab-se/homebrew-tap`).
- Minimal README, INSTALL.md and CHANGELOG describing the hello-world state.
- Release `v0.1.0`: CI green on all three operating systems, 4 archives plus 4 CycloneDX SBOMs on
  the GitHub Release, `honk-core` and `honk` on crates.io, formula in the tap.

Done when: `brew install mklab-se/tap/honk && honk` prints `Honk, honk!`, and
`cargo install honk` does the same.

### Stage 2: the real honk, v0.2.0

Everything in the sections below. Adds the ALSA install step to CI and to the Linux build in
`release.yml`. Released as `v0.2.0`.

## README and artwork (both stages)

The README must sell the tool to anyone landing on the repo: a polished, playful page, not a
template leftover.

- Layout follows the fleet: centred logo `media/honk-horizontal.png` (width 600), the standard
  badge block (CI, crates.io, GitHub Release, Homebrew dynamic badge, licence), a centred
  "What's new" callout, then a punchy one-line pitch and a quick start.
- Artwork is generated with ailloy (`ailloy image`) in the same style as the other tools'
  `media/` images (rigg, cosq, pidge, ailloy): a monochrome graphite and ink sketch with
  detailed cross-hatching on a white background, 1536x1024 landscape, a whimsical visual metaphor
  with a capped developer character, small terminal or code details drawn into the scene, no
  colour and no rendered title text. Honk's metaphor: a vintage open-top car (Model T era) with a
  big brass bulb horn, the developer squeezing the horn next to a laptop showing `honk`, sound
  lines bursting out.
- Files: `media/honk-horizontal.png` (hero, 1536x1024) and `media/honk-vertical.png`
  (1024x1536, same scene composed vertically), replacing the template's placeholder. Several
  candidates are generated and the best one chosen by inspection; the prompt used is recorded in
  `media/README.md` so the art can be regenerated consistently.
- Stage 1 README: hero, badges, pitch, install (Homebrew, cargo, binaries), the hello-world quick
  start, and a short "Coming in v0.2" teaser listing the agreed features. No feature is described
  as working before it ships.
- Stage 2 README: full feature tour (styles table, notifier recipes such as
  `cargo build && honk` and `honk -- make test`, shape knobs, config), the "What's new" callout
  updated for 0.2.

## Command surface (stage 2)

```
honk                              # "Honk, honk!": default style bulb, 2 honks
honk --style awooga               # bulb | awooga | car | truck | clown
honk --times 3 --long             # number of honks, longer honks
honk --pitch 1.5 --volume 0.6     # pitch multiplier 0.5 to 2.0, volume 0.0 to 1.0
honk --status $?                  # 0 gives a happy honk, anything else a sad honk
honk -- cargo build --release     # run the command, honk by outcome, exit with its code
honk --wav out.wav                # render to a WAV file instead of playing
honk -q                           # no ASCII car
honk ai | completion | version    # template plumbing, unchanged
```

- The honk is the root command; there is no `honk play` subcommand.
- `--wav` combines with `--status` and with `-- cmd`.
- `--status` and `-- cmd` are mutually exclusive.
- The ASCII car and "HONK HONK!" text go to stderr, and only when stderr is a TTY and `-q` is not
  set. stdout is never written by the honk itself in stage 2.
- Config (`config.yaml` in the platform config dir) gains optional `style`, `volume` and `times`.
  Precedence: built-in defaults, then config, then flags.
- The template's background update check stays, but must never delay exit beyond the end of the
  sound: it is given a short timeout and abandoned if it has not finished.

## Architecture (stage 2)

### `honk-core` (pure, deterministic; no clap, tokio or audio device code)

- `spec.rs`: `HonkSpec { style, times, long, pitch, volume, mood }`, `Mood = Neutral | Happy | Sad`.
  Built from defaults, config and overrides; validates ranges.
- `style.rs`: `Style` enum and a `Voice` preset per style:
  - bulb: nasal square-ish wave, fast attack, a little noise for rasp.
  - awooga: sawtooth with a pitch sweep up then down.
  - car: two detuned tones about a major third apart (dual-tone car horn).
  - truck: low, loud air horn with slight beating.
  - clown: short, high, wobbly squeak.
- `synth.rs`: renders a `HonkSpec` to `Vec<f32>`, 44.1 kHz mono. Built from small parts:
  oscillators, ADSR envelope, pitch envelope, one-pole low-pass filter, soft clipping. Happy adds a
  small upward bend at the end; Sad bends the last honk down and slows it.
- `wav.rs`: encodes samples to WAV with `hound`.
- `config.rs`: the template config plus `style`, `volume`, `times`.

### `honk` (CLI)

- `commands/honk.rs`: build spec, render, play or write WAV.
- `commands/run.rs`: `honk -- cmd`. Spawns with `std::process::Command` (no shell), inherits
  stdio, waits, chooses the mood from the outcome, honks, exits with the command's code.
- `audio.rs`: playback via `rodio`. No output device gives a warning and `Ok`.
- `car.rs`: the ASCII car animation on stderr while the sound plays.

## Errors and exit codes

| Situation | Behaviour | Exit code |
| --- | --- | --- |
| No audio device | One-line warning, no sound | 0, or the wrapped command's code |
| Invalid flag value | clap error | 2 |
| `--wav` path not writable | Error | 1 |
| Broken config file | Warning, defaults used, honk proceeds | as normal |
| `-- cmd` exits with code N | Sad honk if N != 0, happy if 0 | N |
| `-- cmd` killed by a signal (Unix) | Sad honk | 128 + signal |
| `-- cmd` not found | Error, sad honk | 127 |

## Testing

Stage 1: an integration test asserts that `honk` prints exactly `Honk, honk!\n` and exits 0; the
template's tests keep passing on all three operating systems.

Stage 2, test-driven, mostly in `honk-core`:

- Synth: every style renders the expected length; peak never exceeds `volume`; no NaN and no hard
  clipping; `times` gives that many bursts (detected as silence gaps in the envelope); Sad bends
  down and Happy bends up (zero-crossing pitch estimate at start versus end of the last burst).
- Spec: precedence defaults, config, flags; out-of-range values are rejected.
- WAV: round-trips through `hound`.
- CLI integration with `assert_cmd`, using `--wav` so no speaker is needed: default honk writes a
  valid WAV; `honk --wav x.wav -- <command exiting 3>` exits 3; stdout stays empty.
- CI runs all of this on Ubuntu (with ALSA headers), macOS and Windows.

## Out of scope

- Bundled real horn recordings (a possible later `--style real`).
- AI features (planned later, via ailloy).
- Playback on systems without any audio stack beyond the warning above.
