//! End-to-end tests for the `honk` binary.

use assert_cmd::Command;
use predicates::prelude::*;

/// The honk binary with update checks and audio disabled, so tests never touch
/// the network or the speakers.
fn honk() -> Command {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("honk");
    cmd.env("HONK_NO_UPDATE_CHECK", "1");
    cmd.env("HONK_NO_AUDIO", "1");
    cmd
}

fn wav_len(path: &std::path::Path) -> u32 {
    let reader = hound::WavReader::open(path).unwrap();
    assert_eq!(reader.spec().sample_rate, 44_100);
    reader.duration()
}

#[test]
fn bare_honk_prints_nothing_on_stdout() {
    honk().assert().success().stdout("");
}

#[test]
fn no_audio_device_warns_and_succeeds() {
    honk()
        .assert()
        .success()
        .stderr(predicate::str::contains("no audio output"));
}

#[test]
fn wav_writes_a_file_instead_of_playing() {
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("h.wav");
    honk().arg("--wav").arg(&out).assert().success().stdout("");
    assert_eq!(wav_len(&out), 2 * 9702 + 5292);
}

#[test]
fn flags_shape_the_honk() {
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("h.wav");
    honk()
        .args(["--times", "1", "--long"])
        .arg("--wav")
        .arg(&out)
        .assert()
        .success();
    assert_eq!(wav_len(&out), 19404);
    honk()
        .args(["--pitch", "1.5", "--volume", "0.5", "--wav"])
        .arg(&out)
        .assert()
        .success();
}

#[test]
fn every_style_is_accepted() {
    let dir = tempfile::tempdir().unwrap();
    for style in ["bulb", "awooga", "car", "truck", "clown"] {
        let out = dir.path().join(format!("{style}.wav"));
        honk()
            .args(["--style", style])
            .arg("--wav")
            .arg(&out)
            .assert()
            .success();
        assert!(wav_len(&out) > 0);
    }
}

#[test]
fn status_nonzero_renders_the_longer_sad_honk() {
    let dir = tempfile::tempdir().unwrap();
    let (ok, bad) = (dir.path().join("ok.wav"), dir.path().join("bad.wav"));
    honk()
        .args(["--status", "0", "--wav"])
        .arg(&ok)
        .assert()
        .success();
    honk()
        .args(["--status", "1", "--wav"])
        .arg(&bad)
        .assert()
        .success();
    assert!(wav_len(&bad) > wav_len(&ok));
}

#[test]
fn bad_values_are_usage_errors() {
    honk().args(["--pitch", "9"]).assert().code(2);
    honk().args(["--volume", "1.5"]).assert().code(2);
    honk().args(["--times", "0"]).assert().code(2);
    honk().args(["--style", "kazoo"]).assert().code(2);
    honk()
        .args(["--status", "1", "--", "anything"])
        .assert()
        .code(2);
}

#[test]
fn unwritable_wav_path_fails_with_exit_1() {
    let dir = tempfile::tempdir().unwrap();
    honk()
        .arg("--wav")
        .arg(dir.path().join("missing/h.wav"))
        .assert()
        .code(1)
        .stderr(predicate::str::contains("h.wav"));
}

#[test]
fn config_defaults_apply_and_broken_config_is_survivable() {
    let dir = tempfile::tempdir().unwrap();
    let cfg_dir = dir.path().join("honk");
    std::fs::create_dir_all(&cfg_dir).unwrap();
    let out = dir.path().join("h.wav");

    std::fs::write(cfg_dir.join("config.yaml"), "times: 1\n").unwrap();
    honk()
        .env("HONK_CONFIG_DIR", &cfg_dir)
        .arg("--wav")
        .arg(&out)
        .assert()
        .success();
    assert_eq!(wav_len(&out), 9702);

    std::fs::write(cfg_dir.join("config.yaml"), "volume: 7\n").unwrap();
    honk()
        .env("HONK_CONFIG_DIR", &cfg_dir)
        .arg("--wav")
        .arg(&out)
        .assert()
        .success()
        .stderr(predicate::str::contains("ignoring config volume"));

    std::fs::write(cfg_dir.join("config.yaml"), "{{ not yaml").unwrap();
    honk()
        .env("HONK_CONFIG_DIR", &cfg_dir)
        .arg("--wav")
        .arg(&out)
        .assert()
        .success()
        .stderr(predicate::str::contains("config"));
}

/// A shell command that exits with `code`, on every platform.
fn exits_with(code: i32) -> Vec<String> {
    if cfg!(windows) {
        vec!["cmd".into(), "/C".into(), format!("exit {code}")]
    } else {
        vec!["sh".into(), "-c".into(), format!("exit {code}")]
    }
}

#[test]
fn wrapped_command_exit_code_passes_through() {
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("h.wav");
    honk()
        .arg("--wav")
        .arg(&out)
        .arg("--")
        .args(exits_with(3))
        .assert()
        .code(3);
    let sad = wav_len(&out);
    honk()
        .arg("--wav")
        .arg(&out)
        .arg("--")
        .args(exits_with(0))
        .assert()
        .code(0);
    assert!(
        sad > wav_len(&out),
        "failure should render the longer sad honk"
    );
}

#[test]
fn wrapped_command_output_is_not_captured() {
    let echo = if cfg!(windows) {
        vec!["cmd", "/C", "echo hello"]
    } else {
        vec!["sh", "-c", "echo hello"]
    };
    honk()
        .arg("--")
        .args(echo)
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn missing_command_is_exit_127() {
    honk()
        .args(["--", "definitely-not-a-real-command-honk"])
        .assert()
        .code(127)
        .stderr(predicate::str::contains(
            "definitely-not-a-real-command-honk",
        ));
}

#[test]
fn no_car_art_when_stderr_is_not_a_terminal() {
    honk()
        .assert()
        .success()
        .stderr(predicate::str::contains("HONK!").not())
        .stderr(predicate::str::contains("(_)").not());
}

#[test]
fn wrapped_exit_code_survives_a_failed_wav_write() {
    let dir = tempfile::tempdir().unwrap();
    honk()
        .arg("--wav")
        .arg(dir.path().join("missing/h.wav"))
        .arg("--")
        .args(exits_with(3))
        .assert()
        .code(3)
        .stderr(predicate::str::contains("h.wav"));
}
