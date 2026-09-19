use anyhow::{Context, Result};
use std::{fs, time::Duration};

pub fn uptime() -> Result<Duration> {
    let raw = fs::read_to_string("/proc/uptime").context("read /proc/uptime")?;
    let seconds = raw
        .split_whitespace()
        .next()
        .context("parse /proc/uptime")?
        .parse::<f64>()
        .context("parse uptime seconds")?;
    Ok(Duration::from_secs_f64(seconds.max(0.0)))
}

pub fn format_uptime(duration: Duration) -> String {
    let mut s = duration.as_secs();
    let days = s / 86_400;
    s %= 86_400;
    let hours = s / 3_600;
    s %= 3_600;
    let minutes = s / 60;
    s %= 60;
    match (days, hours, minutes) {
        (d, h, m) if d > 0 => format!("{d:02}d {h:02}h {m:02}m {s:02}s"),
        (0, h, m) if h > 0 => format!("{h}h {m:02}m {s:02}s"),
        (0, 0, m) if m > 0 => format!("{m}m {s:02}s"),
        _ => format!("{s}s"),
    }
}
