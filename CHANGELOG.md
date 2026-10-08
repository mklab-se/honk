# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Removed

- The `honk ai` command and the Ailloy dependency, inherited from the MKLab tool template but
  unused. The binary shrinks by a quarter (11.2 MB to 8.5 MB on macOS) and builds 67 fewer
  crates. AI features may return later, designed for honk.

## [0.2.1] - 2026-10-08

### Fixed

- `cargo install honk` works on a stock Linux machine again: honk no longer links ALSA there and
  instead plays through the system's own player (`pw-play`, `paplay` or `aplay`, first that
  works). The prebuilt Linux binary no longer needs `libasound2`, and the Homebrew formula drops
  its `alsa-lib` dependency.
- The end of a honk is no longer clipped on macOS and Windows (a short silent tail lets the device
  drain).
- With no audio device, the ASCII car stops at once instead of animating a silent honk.

## [0.2.0] - 2026-10-08

### Added

- Real sound: honks are synthesised on the fly (no audio files) and played on Linux (ALSA),
  macOS (CoreAudio) and Windows (WASAPI).
- Five horn styles with `--style`: `bulb` (default), `awooga`, `car`, `truck` and `clown`.
- Notifier mode: `honk --status <code>` gives a happy honk on 0 and a sad, down-bending honk
  otherwise; `honk -- <command>` runs the command, honks by outcome and exits with its code
  (127 if it cannot be started, 128 + signal if it was killed).
- Shape flags: `--times`, `--long`, `--pitch`, `--volume`, and `--wav FILE` to write the honk to
  a WAV file instead of playing it.
- An ASCII car that honks along on stderr when it is a terminal (`-q` turns it off).
- `config.yaml` defaults for `style`, `volume` and `times`; `HONK_CONFIG_DIR` overrides the
  location. Out-of-range or unreadable config produces a warning, never a failure.
- No audio device (CI, SSH, containers) is a warning, not an error; `HONK_NO_AUDIO` disables
  playback on purpose.
- `honk -- cmd` stays alive on Ctrl-C and lets the command decide how to stop, finds `.cmd` and
  `.bat` shims such as `npm` on Windows, and keeps the command's exit code even if the honk itself
  fails (for example an unwritable `--wav` path).
- On Linux, alsa-lib's own "cannot find card" messages no longer reach stderr; the Homebrew
  formula depends on `alsa-lib`.

### Changed

- Plain `honk` now plays a sound instead of printing "Honk, honk!".
- The background update check can no longer delay exit by more than 300 ms.
- Building on Linux now needs the ALSA development headers (`libasound2-dev`).

## [0.1.0] - 2026-10-08

### Added

- First release of honk: running `honk` prints "Honk, honk!". This release proves the build,
  test and release pipeline on Linux, macOS and Windows; real sound arrives in 0.2.
- `honk ai`, `honk completion` and `honk version` from the MKLab tool template.
