use crate::cli::Cli;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_logo")]
    pub logo: String,
    #[serde(default = "default_animation")]
    pub animation: bool,
    #[serde(default = "default_refresh")]
    pub refresh_rate: u64,
    #[serde(default = "yes")]
    pub show_os: bool,
    #[serde(default = "yes")]
    pub show_kernel: bool,
    #[serde(default = "yes")]
    pub show_cpu: bool,
    #[serde(default = "yes")]
    pub show_gpu: bool,
    #[serde(default = "yes")]
    pub show_memory: bool,
    #[serde(default = "yes")]
    pub show_disk: bool,
    #[serde(default = "yes")]
    pub show_network: bool,
    #[serde(default = "yes")]
    pub show_temperature: bool,
    #[serde(default = "yes")]
    pub show_uptime: bool,
    #[serde(default = "yes")]
    pub author: bool,
}

fn default_theme() -> String {
    "default".into()
}
fn default_logo() -> String {
    "auto".into()
}
fn default_animation() -> bool {
    true
}
fn default_refresh() -> u64 {
    1
}
fn yes() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            logo: default_logo(),
            animation: true,
            refresh_rate: 1,
            show_os: true,
            show_kernel: true,
            show_cpu: true,
            show_gpu: true,
            show_memory: true,
            show_disk: true,
            show_network: true,
            show_temperature: true,
            show_uptime: true,
            author: true,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut cfg = Self::default();
        if let Some(path) = config_path() {
            if path.exists() {
                let text = fs::read_to_string(&path)
                    .with_context(|| format!("read {}", path.display()))?;
                cfg = toml::from_str(&text).with_context(|| format!("parse {}", path.display()))?;
            }
        }
        if cfg.refresh_rate == 0 {
            cfg.refresh_rate = 1;
        }
        Ok(cfg)
    }

    pub fn apply_cli(&mut self, cli: &Cli) {
        if let Some(theme) = &cli.theme {
            self.theme = theme.clone();
        }
        if let Some(logo) = &cli.logo {
            self.logo = logo.clone();
        }
        if cli.no_animation {
            self.animation = false;
        }
        if let Some(rate) = cli.refresh_rate {
            self.refresh_rate = rate.max(1);
        }
        if std::env::var_os("NO_COLOR").is_some()
            || std::env::var("TERM").ok().as_deref() == Some("dumb")
        {
            self.animation = false;
        }
    }
}

pub fn config_path() -> Option<PathBuf> {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
        .map(|p| p.join("astrafetch/config.toml"))
}
