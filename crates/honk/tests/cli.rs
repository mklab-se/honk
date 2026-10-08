//! End-to-end tests for the `honk` binary.

use assert_cmd::Command;

/// The honk binary with update checks disabled so tests never touch the network.
fn honk() -> Command {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("honk");
    cmd.env("HONK_NO_UPDATE_CHECK", "1");
    cmd
}

#[test]
fn bare_honk_says_honk_honk() {
    honk().assert().success().stdout("Honk, honk!\n");
}
