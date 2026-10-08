<p align="center">
  <img src="https://raw.githubusercontent.com/mklab-se/honk/main/media/honk-horizontal.png" alt="honk" width="600">
</p>

<p align="center">
  <a href="https://github.com/mklab-se/honk/actions/workflows/ci.yml"><img src="https://github.com/mklab-se/honk/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://crates.io/crates/honk"><img src="https://img.shields.io/crates/v/honk.svg" alt="crates.io"></a>
  <a href="https://github.com/mklab-se/honk/releases/latest"><img src="https://img.shields.io/github/v/release/mklab-se/honk" alt="GitHub Release"></a>
  <a href="https://github.com/mklab-se/homebrew-tap/blob/main/Formula/honk.rb"><img src="https://img.shields.io/badge/dynamic/regex?url=https%3A%2F%2Fraw.githubusercontent.com%2Fmklab-se%2Fhomebrew-tap%2Fmain%2FFormula%2Fhonk.rb&search=%5Cd%2B%5C.%5Cd%2B%5C.%5Cd%2B&label=homebrew&prefix=v&color=orange" alt="Homebrew"></a>
  <a href="https://github.com/mklab-se/honk/blob/main/LICENSE"><img src="https://img.shields.io/crates/l/honk.svg" alt="License"></a>
</p>

<p align="center">
  <strong>honk 0.1</strong> is here: the very first honk, in text form. Real sound arrives in 0.2.
  <a href="CHANGELOG.md">What's new</a>
</p>

# honk

**Make your computer honk like an old-school car.** Honk, honk!

Long build? Slow test suite? A deploy that takes forever? Put `honk` at the end and go get a
coffee. Your computer tells you when it is done, the way a 1920s roadster would.

## Quick start

```bash
brew install mklab-se/tap/honk
```

```bash
honk
```

```text
Honk, honk!
```

## Install

| Method | Command |
| --- | --- |
| Homebrew (macOS, Linux) | `brew install mklab-se/tap/honk` |
| Cargo | `cargo install honk` |
| cargo-binstall (prebuilt, no compiling) | `cargo binstall honk` |
| Prebuilt binaries | Linux, macOS (Intel and Apple Silicon) and Windows on the [Releases page](https://github.com/mklab-se/honk/releases/latest) |

Every release ships a CycloneDX SBOM per platform. Shell completions, building from source and
more are in [INSTALL.md](INSTALL.md).

## Coming in 0.2: the real honk

0.1 proves the pipeline end to end on Linux, macOS and Windows. 0.2 makes the noise:

- **Real sound, synthesised on the fly.** No audio files; works on Linux, macOS and Windows.
- **Five horns:** `bulb`, `awooga`, `car`, `truck` and `clown`.
- **Notifier mode:** `honk --status $?` and `honk -- cargo build` give a happy honk on success
  and a sad, down-bending honk on failure, and pass the exit code straight through.
- **Shape it:** `--times`, `--long`, `--pitch`, `--volume`, or `--wav out.wav` to keep the honk.
- **An ASCII car** that honks along in your terminal.

## Part of the MKLab toolbox

honk is one of a family of small, sharp command-line tools from [MKLab](https://mklab.se):
[ailloy](https://github.com/mklab-se/ailloy) (vendor-flexible AI for Rust tools),
[cosq](https://github.com/mklab-se/cosq) (query Azure Cosmos DB),
[deemer](https://github.com/mklab-se/deemer) (AI-judged integration tests),
[mdeck](https://github.com/mklab-se/mdeck) (Markdown presentations),
[pidge](https://github.com/mklab-se/pidge) (e-mail and calendar for your AI agent) and
[rigg](https://github.com/mklab-se/rigg) (configuration as code for Azure AI Search and Microsoft
Foundry).

## License

[MIT](LICENSE)
