pub mod cpu;
pub mod desktop;
pub mod disk;
pub mod gpu;
pub mod memory;
pub mod network;
pub mod os;
pub mod shell;
pub mod temperature;
pub mod terminal;
pub mod uptime;

use crate::config::Config;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SystemSnapshot {
    pub name: String,
    pub version: String,
    pub author: String,
    pub os: os::OsInfo,
    pub kernel: String,
    pub hostname: String,
    pub architecture: String,
    pub uptime_seconds: u64,
    pub cpu: cpu::CpuInfo,
    pub memory: memory::MemoryInfo,
    pub gpu: gpu::GpuInfo,
    pub disk: disk::DiskInfo,
    pub network: network::NetworkInfo,
    pub temperature: temperature::TemperatureInfo,
    pub shell: shell::ShellInfo,
    pub terminal: terminal::TerminalInfo,
    pub desktop: desktop::DesktopInfo,
}

impl SystemSnapshot {
    pub fn collect(_config: &Config) -> Result<Self> {
        Ok(Self {
            name: "AstraFetch".into(),
            version: "1.0.0".into(),
            author: "Lưu Minh Quang - Astra".into(),
            os: os::detect(),
            kernel: read_first("/proc/sys/kernel/osrelease").unwrap_or_else(|| "Unknown".into()),
            hostname: hostname(),
            architecture: std::env::consts::ARCH.to_string(),
            uptime_seconds: uptime::uptime()?.as_secs(),
            cpu: cpu::collect(),
            memory: memory::collect(),
            gpu: gpu::collect(),
            disk: disk::collect(),
            network: network::collect(),
            temperature: temperature::collect(),
            shell: shell::collect(),
            terminal: terminal::collect(),
            desktop: desktop::collect(),
        })
    }
}

pub(crate) fn read_first(path: &str) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| s.lines().next().map(|x| x.trim().to_string()))
        .filter(|s| !s.is_empty())
}

fn hostname() -> String {
    read_first("/etc/hostname")
        .unwrap_or_else(|| std::env::var("HOSTNAME").unwrap_or_else(|_| "Unknown".into()))
}

pub fn read_os_release() -> std::collections::HashMap<String, String> {
    let mut out = std::collections::HashMap::new();
    if let Ok(text) = std::fs::read_to_string("/etc/os-release") {
        for line in text.lines() {
            if let Some((k, v)) = line.split_once('=') {
                out.insert(k.to_string(), v.trim_matches('"').to_string());
            }
        }
    }
    out
}
