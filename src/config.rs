//! `.dawon.toml` configuration loader.

use std::path::{Path, PathBuf};

use serde::Deserialize;

pub const CONFIG_FILE: &str = ".dawon.toml";

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub project: ProjectConfig,
    #[serde(default)]
    pub checks: ChecksConfig,
}

#[derive(Debug, Deserialize, Default)]
pub struct ProjectConfig {
    /// E.g. "C00" — shown in the TUI header.
    pub module: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ChecksConfig {
    /// Disable sanitizers (ASAN/UBSAN) for faster runs.
    #[serde(default)]
    pub no_sanitizers: bool,
    /// Disable valgrind check.
    #[serde(default)]
    pub no_valgrind: bool,
    /// Enable symbol export validation.
    #[serde(default)]
    pub check_symbol: bool,
    /// Extra forbidden functions added on top of per-exercise defaults.
    #[serde(default)]
    pub extra_forbidden: Vec<String>,
}

/// Load `.dawon.toml` from *root*, returning defaults if absent.
pub fn load(root: &Path) -> anyhow::Result<Config> {
    let path: PathBuf = root.join(CONFIG_FILE);
    if !path.exists() {
        return Ok(Config::default());
    }
    let raw = std::fs::read_to_string(&path)?;
    let cfg: Config =
        toml::from_str(&raw).map_err(|e| anyhow::anyhow!("invalid {CONFIG_FILE}: {e}"))?;
    Ok(cfg)
}
