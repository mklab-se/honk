# Installing honk

## Homebrew (macOS / Linux)

```sh
brew install mklab-se/tap/honk
```

Or add the tap once, then install:

```sh
brew tap mklab-se/tap
brew install honk
```

Upgrade with `brew upgrade honk`.

## Sound on Linux

On Linux, honk plays through the audio player your system already has: PipeWire's `pw-play`,
PulseAudio's `paplay` or ALSA's `aplay`, whichever works first. Every desktop distribution ships
at least one, so there is nothing to install and no audio library to build against. If none is
present (minimal containers), honk prints one warning and carries on.

## Cargo (from crates.io)

```sh
cargo install honk
```

On Windows, building from source needs [NASM](https://www.nasm.us/) and [CMake](https://cmake.org/)
on `PATH` (plus the Visual Studio Build Tools most Rust installs already have). They're needed to
compile [`aws-lc-rs`](https://github.com/aws/aws-lc-rs), the TLS crypto backend. macOS and Linux need
nothing extra. If you'd rather skip the build tools entirely, use `cargo binstall` or Homebrew below:
both fetch a pre-built binary.

## cargo-binstall (prebuilt binaries, no compilation)

```sh
cargo binstall honk
```

## Prebuilt binaries (GitHub Releases)

Download the archive for your platform from the
[latest release](https://github.com/mklab-se/honk/releases/latest), extract it, and put the
`honk` binary somewhere on your `PATH`:

| Platform | Archive |
| --- | --- |
| Linux (x86-64) | `honk-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz` |
| macOS (Apple Silicon) | `honk-vX.Y.Z-aarch64-apple-darwin.tar.gz` |
| macOS (Intel) | `honk-vX.Y.Z-x86_64-apple-darwin.tar.gz` |
| Windows (x86-64) | `honk-vX.Y.Z-x86_64-pc-windows-msvc.zip` |

## Software bill of materials (SBOM)

Every release asset above has a matching CycloneDX 1.5 SBOM listing the exact crate versions
compiled into that platform's binary:

```
honk-vX.Y.Z-<target>.cdx.json
```

The binaries are also built with [`cargo auditable`](https://github.com/rust-secure-code/cargo-auditable),
so the dependency list travels inside the executable itself. Check a downloaded binary against the
RustSec advisory database with:

```sh
cargo install cargo-audit --features=fix
cargo audit bin ./honk
```

`syft` and `trivy` also understand this format.

## From source

```sh
git clone https://github.com/mklab-se/honk
cd honk
cargo install --path crates/honk
```

## Shell completions

```sh
# Static script (write it where your shell loads completions)
honk completion zsh > ~/.zfunc/_honk

# Or dynamic completions (re-evaluated on each tab)
source <(COMPLETE=zsh honk)
```

## Environment variables

| Variable | Effect |
| --- | --- |
| `HONK_NO_AUDIO` | Any value: never open an audio device (honk warns and exits 0). Handy in CI. |
| `HONK_CONFIG_DIR` | Directory holding `config.yaml`, instead of the platform config directory. |
| `HONK_NO_UPDATE_CHECK` | Any value: skip the background check for a newer release. |
