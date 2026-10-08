//! Tool configuration.
//!
//! A YAML config stored in the platform config directory
//! (`~/.config/honk/config.yaml` on Linux) holding the user's default honk:
//! style, volume and number of honks. Command-line flags override it.

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
    pub fn config_path() -> Result<PathBuf> {
        let dir = dirs::config_dir().ok_or(Error::NoConfigDir)?;
        Ok(dir.join("honk").join("config.yaml"))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Style;

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
