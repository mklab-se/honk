//! Tool configuration.
//!
//! A YAML config in `~/.config/honk/config.yaml` on Linux and macOS (or under
//! `$XDG_CONFIG_HOME`), `%APPDATA%\honk\config.yaml` on Windows, holding the user's default honk:
//! style, volume and number of honks. Command-line flags override it.

use std::ffi::OsString;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Persisted configuration for the tool.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Default horn style.
    pub style: Option<crate::style::Style>,
    /// Default volume, 0.0 to 1.0.
    pub volume: Option<f32>,
    /// Default number of honks, 1 to 10.
    pub times: Option<u32>,
}

impl Config {
    /// Return the path to the config file, creating no files.
    ///
    /// `HONK_CONFIG_DIR` overrides the directory (used by tests and power users).
    /// Otherwise Linux and macOS use `$XDG_CONFIG_HOME/honk` or `~/.config/honk`,
    /// the same rule as Ailloy; Windows uses `%APPDATA%\honk`.
    pub fn config_path() -> Result<PathBuf> {
        if let Some(dir) = std::env::var_os("HONK_CONFIG_DIR") {
            return Ok(PathBuf::from(dir).join("config.yaml"));
        }
        let dir = if cfg!(windows) {
            dirs::config_dir().map(|d| d.join("honk"))
        } else {
            unix_config_dir(std::env::var_os("XDG_CONFIG_HOME"), dirs::home_dir())
        };
        Ok(dir.ok_or(Error::NoConfigDir)?.join("config.yaml"))
    }

    /// Load the config from disk. Returns [`Config::default`] if no file exists yet.
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let yaml = std::fs::read_to_string(&path)?;
        Ok(serde_norway::from_str(&yaml)?)
    }

    /// Save the config to disk, creating the parent directory if needed.
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let yaml = serde_norway::to_string(self)?;
        std::fs::write(&path, yaml)?;
        Ok(())
    }
}

/// `$XDG_CONFIG_HOME/honk` when it is set to an absolute path, else
/// `~/.config/honk`. Used on Linux and macOS.
fn unix_config_dir(xdg: Option<OsString>, home: Option<PathBuf>) -> Option<PathBuf> {
    match xdg.map(PathBuf::from) {
        Some(xdg) if xdg.is_absolute() => Some(xdg.join("honk")),
        _ => home.map(|h| h.join(".config").join("honk")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Style;

    #[test]
    fn xdg_config_home_wins() {
        let dir = unix_config_dir(Some("/xdg".into()), Some("/home/k".into()));
        assert_eq!(dir, Some(PathBuf::from("/xdg/honk")));
    }

    #[test]
    fn otherwise_dot_config_in_home_like_ailloy() {
        let dir = unix_config_dir(None, Some("/Users/k".into()));
        assert_eq!(dir, Some(PathBuf::from("/Users/k/.config/honk")));
    }

    #[test]
    fn empty_or_relative_xdg_is_ignored() {
        // The XDG spec says to ignore a relative or empty XDG_CONFIG_HOME.
        for xdg in ["", "relative/dir"] {
            let dir = unix_config_dir(Some(xdg.into()), Some("/home/k".into()));
            assert_eq!(dir, Some(PathBuf::from("/home/k/.config/honk")), "{xdg:?}");
        }
    }

    #[test]
    fn no_home_and_no_xdg_is_none() {
        assert_eq!(unix_config_dir(None, None), None);
    }

    #[test]
    fn round_trips_through_yaml() {
        let cfg = Config {
            style: Some(Style::Awooga),
            volume: Some(0.5),
            times: Some(3),
        };
        let yaml = serde_norway::to_string(&cfg).unwrap();
        let parsed: Config = serde_norway::from_str(&yaml).unwrap();
        assert_eq!(parsed.style, Some(Style::Awooga));
        assert_eq!(parsed.volume, Some(0.5));
        assert_eq!(parsed.times, Some(3));
    }

    #[test]
    fn serialized_output_is_stable() {
        // Pins the exact bytes `save` writes, so a YAML library change cannot
        // silently alter existing config files.
        let cfg = Config {
            style: Some(Style::Car),
            volume: Some(0.5),
            times: Some(2),
        };
        assert_eq!(
            serde_norway::to_string(&cfg).unwrap(),
            "style: car\nvolume: 0.5\ntimes: 2\n"
        );
        assert_eq!(
            serde_norway::to_string(&Config::default()).unwrap(),
            "style: null\nvolume: null\ntimes: null\n"
        );
    }

    #[test]
    fn empty_yaml_uses_defaults() {
        let parsed: Config = serde_norway::from_str("{}").unwrap();
        assert!(parsed.style.is_none() && parsed.volume.is_none() && parsed.times.is_none());
    }
}
