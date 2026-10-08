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
  <strong>honk 0.2</strong> is here: real, synthesised honks in five styles, plus notifier mode.
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
honk                      # Honk, honk!
honk --style awooga       # a-WOO-ga
cargo build && honk       # tell me when it's done
```

```text
                       ⣼⠉⠉⠉⠙⠛⠛⠒⠒⠒⠒⢲⡄
                       ⣸⣆⣀⣀⣀⣀⣀⡀ ⠐  ⣧⣠⡀
     ⣴⣶⣲⣶⣒⡒⠒⠒⣒⣒⠒⣒⣒⠶⣦⣄⣀⡀⠸⣇   ⠉⡩⠉⠉⠉⠉⠉⣿⣿⠧⣶⢦ ) )   HONK!
     ⢨⣿⠮⣽⣿⣿⣿⣍⣍⠩⠙⡉⠻⡙⡍⠉⠉⠩⠹⣿⣶⠶⠶⠛⠒⠚⣶⡶⣶⣶⣯⣿⣟⣳⣊⣀⣀⣀⣀ ⢠⣤⡀
      ⠈⢙⣛⡿⠿⠿⠿⢿⠲⣦⣠⣀⣹⣘⣌⣀⣆⡷⣧⡇   ⡴⠋           ⠉⡩⠛⡩⠚⣗⠢⡄
   ⢀⢠⠖⠋⠉⠁     ⢀⣤⡀     ⠐⡇ ⠈⠉⠁⢸⠓⠒⠒⠒⠒⠒⠒⠒⠒ ⠠⠤⠤⢴⠃⢸⣷⣶⣿⣿⣿ ⣀⣀
   ⢻⣧⣒⣀⣀⠒⠦⡀    ⢫       ⡇    ⢸ ⠠    ⠄⣀⣀⣀⣀⣀ ⢸⢶⣿⠿⣿⣿⣿⣿⣾⣿⠋⣷⠐ ⠤⣀
 ⢿⣶⣿⣿⡿⠿⣿⣷⡀⠈⢦   ⢸       ⡇    ⢸ ⢐ ⢀⣤⢞⣫⣥⣤⣶⣦⢄⡉⠚⠿⣿⢀⣼⣿⣿⣿⣻⣿⡤⠇⣠⣤⣤⣄⠑⡄
 ⢰⣿⡟⣡⣶⣶⣦⡙⣷⡀⠈⢧⡀  ⠣⢄⣀⣀⣀⣀⣠⠇    ⣸⣀⡀⣠⣿⣿⣿⠿⠛⠛⠛⠿⣿⣮⣗⣂⣸⣿⣿⣿⣿⣿⡝⠻ ⣸⣿⣿⡟⢷⣄
⣿⣿⣿⢰⣿⣿⣿⣿⣷⡸⡇ ⢸⡉⠉⠉⠉ ⠒⠒⠒⠒⠒⠒⠒⠦⠤⠤⠿⢤⣼⣿⣿⡟⣡⣶⣿⣿⣿⣶⣌⢻⣿⣿⣿⣿⣿⣿⣿⣿⣁⡀⢀⣇⣇⡼⠙⣆⢻⡆
⠉⠉⢉⢸⣿⣿⣮⢹⣿⡇⣷⣀⣛⣇⣀⣀⣀⡀     ⣤    ⣠⣾⣿⣿⡟⣰⣿⣿⣿⣿⣿⣿⣿⣆⢹⣿⣿⡿⣟⢻⡟⣹⣧⢏⣙⠻⣏⣷⣋⣹⠈⣿
  ⠸⢸⣯⣹⣭⣾⣿⡇⣿⡟⠿⠿⠿⠿⠿⠿⢿⣿⣿⣿⣶⣿⣶⣶⣦⣾⣿⣿⣿⣿⡇⣟⣿⣿⣿⣭⠻⣿⣿⣿⡄⣿⣿⣷⣦⣩⣿⡿⣿⣛⣻⣿⣿⡿⢥⣸ ⣿
   ⢃⢻⣧⣏⣹⡜⣸⡿       ⠈⠻⠿⣿⡿⠟⠉⠉⠁  ⠉⠉⠙⡇⢿⡏⢛⣷⡾⢋⣿⣿⣿⡇⣿⣿⡿⠿⠿⠿⢻⣿⣯⣨⠏⡏⠹⣤⠟⣸⡏
    ⠑⢬⣉⣩⡼⠟⠁                     ⢱⠸⣿⡏⢸⠛⡿⢿⡋⣹⢀⣿⠇     ⠙⢿⣿⡷⠿⠞⣋⡼⠏
                                 ⠳⡘⢿⣿⣄⣧⣤⠿⢁⣾⠟        ⠉⠙⠛⠋⠉
                                  ⠈⠲⢬⣉⣉⣡⡴⠟⠁
```

## Five horns

| Style | Sounds like | Try it |
| --- | --- | --- |
| `car` (default) | A classic dual-tone car horn, two notes a major third apart | `honk` |
| `bulb` | A squeezed rubber bulb horn on a vintage car | `honk --style bulb` |
| `awooga` | The Model T klaxon: "a-WOO-ga" | `honk --style awooga` |
| `truck` | A low, slowly beating air horn | `honk --style truck` |
| `clown` | A short, high, wobbly squeak | `honk --style clown` |

Every honk is synthesised on the fly: no audio files, nothing to download.

## Notifier mode

honk knows how your command went. Success gets a happy honk that bends up at the end; failure
gets a sad, longer honk that sags down.

```bash
# Honk when it's done, whatever happened
cargo build && honk

# Happy or sad, depending on the exit status
make test; honk --status $?

# Run the command for you, honk by outcome, and pass its exit code through,
# so && chains and CI keep working
honk -- cargo test

# Make a deploy impossible to miss
honk --times 5 --style truck -- ./deploy.sh
```

With `honk -- cmd`, the command's output is untouched and honk exits with the command's own code
(127 if it could not be started).

## Shape it

| Flag | What it does |
| --- | --- |
| `--times N` | Number of honks, 1 to 10 (default 2) |
| `--long` | Longer honks |
| `--pitch X` | Pitch multiplier, 0.5 to 2.0 |
| `--volume X` | Volume, 0.0 to 1.0 (default 0.8) |
| `--wav FILE` | Write the honk to a WAV file instead of playing it |
| `-q`, `--quiet` | No car |

The car only appears when stderr is a terminal, so pipes and logs stay clean, and the honk
itself never writes to stdout.

## Make it yours

Put your favourite honk in `config.yaml` and plain `honk` uses it. Flags still win.

```yaml
style: awooga
volume: 0.5
times: 3
```

| Platform | Location |
| --- | --- |
| Linux | `~/.config/honk/config.yaml` |
| macOS | `~/Library/Application Support/honk/config.yaml` |
| Windows | `%APPDATA%\honk\config.yaml` |

Set `HONK_CONFIG_DIR` to use a different directory.

## Works everywhere

- **Linux** through the player your system already has (`pw-play`, `paplay` or `aplay`), so it
  works with PipeWire, PulseAudio and plain ALSA, with nothing extra to install.
- **macOS** through CoreAudio.
- **Windows** through WASAPI.

No speaker? No problem: on a headless server, in CI or over SSH, honk prints one warning and
exits 0 (or with the wrapped command's code), so it never breaks a build. Set `HONK_NO_AUDIO=1`
to silence it on purpose.

## Install

| Method | Command |
| --- | --- |
| Homebrew (macOS, Linux) | `brew install mklab-se/tap/honk` |
| Cargo | `cargo install honk` |
| cargo-binstall (prebuilt, no compiling) | `cargo binstall honk` |
| Prebuilt binaries | Linux, macOS (Intel and Apple Silicon) and Windows on the [Releases page](https://github.com/mklab-se/honk/releases/latest) |

Every release ships a CycloneDX SBOM per platform. Shell completions, building from source and
more are in [INSTALL.md](INSTALL.md).

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
