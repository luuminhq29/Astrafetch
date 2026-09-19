use serde::Serialize;
use std::process::Command;
#[derive(Debug, Clone, Serialize)]
pub struct DiskInfo {
    pub mount: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub usage_percent: f64,
}
pub fn collect() -> DiskInfo {
    if let Ok(out) = Command::new("df").args(["-P", "-B1", "/"]).output() {
        if let Ok(s) = String::from_utf8(out.stdout) {
            if let Some(l) = s.lines().nth(1) {
                let p: Vec<&str> = l.split_whitespace().collect();
                if p.len() >= 5 {
                    let total = p[1].parse().unwrap_or(0);
                    let used = p[2].parse().unwrap_or(0);
                    return DiskInfo {
                        mount: "/".into(),
                        total_bytes: total,
                        used_bytes: used,
                        usage_percent: if total > 0 {
                            used as f64 / total as f64 * 100.0
                        } else {
                            0.0
                        },
                    };
                }
            }
        }
    }
    DiskInfo {
        mount: "/".into(),
        total_bytes: 0,
        used_bytes: 0,
        usage_percent: 0.0,
    }
}
