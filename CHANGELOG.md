# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

### Changed

- Plain `honk` now plays a sound instead of printing "Honk, honk!".
- The background update check can no longer delay exit by more than 300 ms.
- Building on Linux now needs the ALSA development headers (`libasound2-dev`).

## [0.1.0] - 2026-10-08

### Added

- First release of honk: running `honk` prints "Honk, honk!". This release proves the build,
  test and release pipeline on Linux, macOS and Windows; real sound arrives in 0.2.
- `honk ai`, `honk completion` and `honk version` from the MKLab tool template.
