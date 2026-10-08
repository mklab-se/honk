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
  <strong>honk 0.3</strong> is here: a detailed Braille roadster, the car horn as the new default, and a binary a quarter smaller.
  <a href="CHANGELOG.md">What's new</a>
</p>

# honk

**Make your computer honk like an old-school car.** Honk, honk!

honk is a toy with a job. Put it after a long build, a test run or a deploy, and your computer
tells you how it went with a sound you can't miss: a cheerful honk when it worked, a sad, sagging
one when it failed. You don't have to keep the terminal in view, or look at a screen at all.

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

## Hear it, don't watch it

Most "it's done" signals are visual: a pop-up, a badge, a red line in a terminal you have to be
looking at. honk tells you through your ears instead.

- **Know the outcome without looking.** Success and failure sound different: the happy honk
  bends up at the end, the sad one is longer and sags down. You hear whether it worked, not just
  that it finished.
- **Tell jobs apart by ear.** Give each long job its own horn, and you know which one finished
  from across the room: `honk --style truck -- ./deploy.sh`, `honk --style clown -- cargo test`.
- **Step away from the screen.** Make coffee, read on paper, rest your eyes, or work in another
  window. The honk comes to you.
- **Works with screen readers.** honk never writes to stdout, the wrapped command's output passes
  through untouched, and any problem is one plain-text line on stderr. The car is decoration only:
  a screen reader would read its Braille dots out as dot patterns, so add `-q` (or
  `alias honk='honk -q'`) to switch it off.

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
to turn the sound off on purpose (honk still prints that one warning line).

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
