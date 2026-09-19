use serde::Serialize;
use std::{
    fs,
    sync::{Mutex, OnceLock},
    time::Instant,
};

#[derive(Debug, Clone, Serialize)]
pub struct CpuInfo {
    pub model: String,
    pub physical_cores: usize,
    pub threads: usize,
    pub frequency_mhz: u64,
    pub usage_percent: f64,
}

#[derive(Default)]
struct CpuSample {
    total: u64,
    idle: u64,
    at: Option<Instant>,
}
static SAMPLE: OnceLock<Mutex<CpuSample>> = OnceLock::new();

pub fn collect() -> CpuInfo {
    let text = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let mut model = "Unknown".into();
    let mut threads = 0;
    let mut freq = 0;
    let mut physical = std::collections::HashSet::new();
    for block in text.split("\n\n") {
        let mut phys = None;
        let mut core = None;
        for line in block.lines() {
            if let Some((k, v)) = line.split_once(':') {
                let k = k.trim();
                let v = v.trim();
                if k == "model name" || k == "Hardware" {
                    if model == "Unknown" {
                        model = v.into();
                    }
                }
                if k == "cpu MHz" && freq == 0 {
                    freq = v.parse::<f64>().unwrap_or(0.0) as u64;
                }
                if k == "processor" {
                    threads += 1;
                }
                if k == "physical id" {
                    phys = Some(v.to_string());
                }
                if k == "core id" {
                    core = Some(v.to_string());
                }
            }
        }
        if let (Some(p), Some(c)) = (phys, core) {
            physical.insert((p, c));
        }
    }
    if threads == 0 {
        threads = std::thread::available_parallelism()
            .map(|x| x.get())
            .unwrap_or(1);
    }
    let physical_cores = if physical.is_empty() {
        threads
    } else {
        physical.len()
    };
    CpuInfo {
        model,
        physical_cores,
        threads,
        frequency_mhz: freq,
        usage_percent: usage(),
    }
}

fn usage() -> f64 {
    let line = fs::read_to_string("/proc/stat")
        .ok()
        .and_then(|s| s.lines().find(|l| l.starts_with("cpu ")).map(str::to_owned));
    let Some(line) = line else {
        return 0.0;
    };
    let vals: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .filter_map(|v| v.parse().ok())
        .collect();
    if vals.len() < 4 {
        return 0.0;
    }
    let idle = vals[3] + vals.get(4).copied().unwrap_or(0);
    let total: u64 = vals.iter().sum();
    let state = SAMPLE.get_or_init(|| Mutex::new(CpuSample::default()));
    let mut s = state.lock().unwrap();
    let usage = if s.total > 0 && total > s.total {
        ((total - s.total).saturating_sub(idle.saturating_sub(s.idle)) as f64
            / (total - s.total) as f64)
            * 100.0
    } else {
        0.0
    };
    s.total = total;
    s.idle = idle;
    s.at = Some(Instant::now());
    usage.clamp(0.0, 100.0)
}
