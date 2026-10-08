# CLAUDE.md

Guidance for Claude Code (and other agents) working in this repository.

## What this is

`honk` makes the computer honk like an old-school car: "Honk, honk!". It is a toy at heart, but
built to be useful as a notifier in scripts and terminals (`cargo build && honk`,
`honk -- long-job`). It must work on Linux, macOS and Windows.

Design docs live in `docs/superpowers/`: the spec in `specs/`, implementation plans in `plans/`.

## Template lineage

> **Note for agents working in a repository generated from this template** (i.e. the tool was renamed
> away from `rusty-tmpl`): the rest of this file describes the original scaffold. The references below
> intentionally still point at the upstream template, not at this tool.

- **Upstream template:** https://github.com/mklab-se/rusty-tmpl
- **Likely local clone:** `../rusty-tmpl/` (sibling directory under the same parent)

This repository was scaffolded from that template, which carries the shared MKLab CLI conventions
(workspace layout, clap CLI, Ailloy `ai` command, update checker, CI/release pipeline, `/release`
skill). Use the lineage in both directions:

- **Pulling improvements in:** when the template gains a fix or new convention, compare against
  `../rusty-tmpl/` and port the relevant change here, adapting the `rusty-tmpl` name to this tool.
- **Pushing improvements back:** if you discover a fix or better pattern here that is *generic*
  (not specific to this tool's domain), consider contributing it upstream to `mklab-se/rusty-tmpl`
  so every future tool benefits. Generalize it (strip tool-specific names/logic) before doing so.

When working from the local clone, prefer reading `../rusty-tmpl/` directly to diff conventions; fall
back to the GitHub URL if the sibling directory isn't present.

## Architecture

A two-crate Cargo workspace:

- `crates/honk/`: the CLI binary.
  - `main.rs`: `#[tokio::main]`; logging, dynamic completions, `--no-color`, a background update
    check (abandoned after 300 ms so it never delays exit), then `Cli::run`, whose `i32` becomes
    the process exit code.
  - `cli.rs`: clap-derive `Cli` with flattened `HonkArgs` (the honk flags on the root command),
    `Commands`, `AiCommands`, `Shell`. The no-subcommand arm is the honk itself;
    `args_conflicts_with_subcommands` keeps `honk --times 3 version` from being ambiguous.
  - `commands/honk.rs`: load config (warn and default on errors), resolve the spec, render, then
    play (with the car unless `-q`) or write `--wav`.
  - `commands/run.rs`: `honk -- cmd`; spawns without a shell, inherits stdio, maps the exit
    status (signals to 128 + n, spawn failure to 127), honks happy or sad.
  - `commands/ai.rs`, `commands/completion.rs`: template plumbing.
  - `audio.rs`: rodio playback on the default device; no device (or `HONK_NO_AUDIO`) is
    `Playback::NoDevice`, never an error.
  - `car.rs`: ASCII car frames (pure, tested) and the stderr animation, only on a TTY.
  - `banner.rs`, `update.rs`: from the template.
- `crates/honk-core/`: pure and deterministic, no clap, tokio or audio device code.
  - `style.rs`: `Style` (bulb, awooga, car, truck, clown) and its `Voice` synthesis preset.
  - `spec.rs`: `HonkSpec`, `Mood`, `Overrides`; `HonkSpec::resolve` applies defaults, then
    config (invalid values skipped with warnings), then flags (invalid values are errors).
  - `synth.rs`: `render(&HonkSpec) -> Vec<f32>` at 44.1 kHz mono, `duration_secs`.
  - `wav.rs`: 16-bit mono WAV via `hound`.
  - `config.rs`: YAML `Config` (`style`, `volume`, `times`); `HONK_CONFIG_DIR` overrides the dir.
  - `error.rs`: `thiserror` `Error` enum + `Result` alias.

## Testing

- Core logic is unit-tested inline (`cargo test -p honk-core`): lengths, peaks within volume,
  burst counts, mood pitch bends via zero-crossing counts, config precedence.
- `crates/honk/tests/cli.rs` drives the binary with `HONK_NO_AUDIO=1` and
  `HONK_NO_UPDATE_CHECK=1`, and asserts on `--wav` output, so no speaker or network is needed.
  Wrapped-command tests use `sh -c` on Unix and `cmd /C` on Windows.
- When tuning a style, change only its `Voice` numbers in `style.rs`; the core tests must stay green.

## Adding a command

1. Add a variant to `Commands` in `cli.rs` (with a doc comment; it becomes the help text).
2. Add a `pub mod <name>;` in `commands/mod.rs` and implement `pub async fn run(...) -> anyhow::Result<()>`.
3. Add the dispatch arm in `Cli::run`.

## AI integration

`commands/ai.rs` wraps [Ailloy](https://crates.io/crates/ailloy) via its `config_tui` helpers and the
shared global config (`~/.config/ailloy/config.yaml`). To call a model from a command, use
`ailloy::Client`. The capability list is the `CAPABILITIES` const in `ai.rs` (`["chat"]`).

## Conventions

- Edition 2024, MSRV 1.88 (`[workspace.package]`; set by Ailloy, 1.88 since 2.x and still in 3.0).
- All deps are declared in the root `[workspace.dependencies]` and inherited with `.workspace = true`.
  Current majors: clap 4.6, tokio 1.53, colored 3, dirs 7, thiserror 2, ailloy 3.0, reqwest 0.13.
  YAML is `serde_norway` 0.9, the maintained drop-in fork of the deprecated `serde_yaml`
  (chosen fleet-wide 2026-10-07).
- Building on Linux needs the ALSA headers (`libasound2-dev`): `rodio` plays through `cpal`/ALSA.
  Every Linux CI and release job that compiles installs them.
- Building from source on Windows needs NASM and CMake on `PATH`, because `aws-lc-rs`
  (reqwest's TLS crypto backend) compiles optimized assembly routines at build time. macOS and Linux need nothing extra.
  The release workflow's Windows leg installs NASM via `ilammy/setup-nasm@v1`; CMake and MSVC are
  already on the `windows-latest` image.
- CI gates: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`. The test job runs on Ubuntu, macOS and Windows: cross-platform is a hard
  requirement. CI runs the latest stable toolchain, so run the gates on an up-to-date
  local toolchain (new clippy lints otherwise surface only in CI).
- `README.md` opens with the logo, the badges and then a centred "What's new" callout
  (`<strong>tool X.Y</strong> is here: ...` plus a `What's new` link to `CHANGELOG.md`). Refresh the
  callout on every minor/major release; generated tools inherit it.
- Releases go through the `/release` skill (`.claude/skills/release/`) → tag push → `release.yml`. The
  skill updates the toolchain and dependencies first, then watches the workflow. `release.yml` builds
  binaries with `cargo auditable` and attaches a per-target CycloneDX SBOM (`.cdx.json`) to the
  GitHub Release alongside the archives.

## Writing style

- **No em-dashes (U+2014), anywhere:** docs, CHANGELOG, code comments, doc comments, CLI help text,
  error messages, test strings, commit messages and PR descriptions. Use a comma where it works;
  otherwise a colon, parentheses, or a new sentence. If Rust code genuinely needs the character at
  runtime, write the escape `\u{2014}`, never the literal. CI enforces this (the "No em-dashes"
  step in `ci.yml`); check locally with `git grep -nIP '\x{2014}'`.

## Dependency Policy

We keep this tool's dependencies at their latest compatible versions, not just the versions that
happen to still compile. Staying current is the default, not something we get to eventually:
letting dependencies drift is how technical debt accumulates unnoticed until a security advisory or
a forced breaking upgrade makes it urgent. When a newer major is available and there's no concrete,
documented reason not to take it (see any `# Stays on ...` comments in `Cargo.toml` for the current
exceptions and why), take it during the next maintenance round rather than deferring it. This
applies to every tool scaffolded from this template too; the cross-repo `maintaining-rust-tools`
skill drives it for the whole fleet (ailloy + cosq + deemer + honk + mdeck + pidge + rigg + rusty-tmpl).
