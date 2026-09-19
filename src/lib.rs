pub fn format_uptime_for_test(seconds: u64) -> String {
    let mut s = seconds;
    let days = s / 86_400; s %= 86_400;
    let hours = s / 3_600; s %= 3_600;
    let minutes = s / 60; s %= 60;
    match (days, hours, minutes) {
        (d, h, m) if d > 0 => format!("{d:02}d {h:02}h {m:02}m {s:02}s"),
        (0, h, m) if h > 0 => format!("{h}h {m:02}m {s:02}s"),
        (0, 0, m) if m > 0 => format!("{m}m {s:02}s"),
        _ => format!("{s}s"),
    }
}

pub fn format_bytes_for_test(v: u64) -> String {
    const G: u64 = 1024 * 1024 * 1024;
    const M: u64 = 1024 * 1024;
    if v >= G { format!("{:.1} GiB", v as f64 / G as f64) } else { format!("{:.0} MiB", v as f64 / M as f64) }
}
