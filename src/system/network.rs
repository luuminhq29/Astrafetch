use serde::Serialize;
use std::{
    fs,
    net::IpAddr,
    process::Command,
    sync::{Mutex, OnceLock},
    time::Instant,
};
#[derive(Debug, Clone, Serialize)]
pub struct NetworkInfo {
    pub interface: String,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    pub download_bps: f64,
    pub upload_bps: f64,
}
#[derive(Default)]
struct Sample {
    rx: u64,
    tx: u64,
    at: Option<Instant>,
}
static SAMPLE: OnceLock<Mutex<Sample>> = OnceLock::new();
pub fn collect() -> NetworkInfo {
    let mut iface = "N/A".into();
    let mut rx = 0;
    let mut tx = 0;
    if let Ok(es) = fs::read_dir("/sys/class/net") {
        for e in es.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if n != "lo"
                && fs::read_to_string(e.path().join("operstate"))
                    .map(|x| x.trim() == "up")
                    .unwrap_or(false)
            {
                iface = n;
                break;
            }
        }
    }
    if iface != "N/A" {
        rx = read_counter(&iface, "rx_bytes");
        tx = read_counter(&iface, "tx_bytes");
    }
    let (down, up) = rates(rx, tx);
    let (v4, v6) = addresses(&iface);
    NetworkInfo {
        interface: iface,
        ipv4: v4,
        ipv6: v6,
        download_bps: down,
        upload_bps: up,
    }
}
fn read_counter(iface: &str, name: &str) -> u64 {
    fs::read_to_string(format!("/sys/class/net/{iface}/statistics/{name}"))
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}
fn rates(rx: u64, tx: u64) -> (f64, f64) {
    let st = SAMPLE.get_or_init(|| Mutex::new(Sample::default()));
    let mut s = st.lock().unwrap();
    let now = Instant::now();
    let out = if let Some(at) = s.at {
        let dt = now.duration_since(at).as_secs_f64();
        if dt > 0.0 {
            (
                rx.saturating_sub(s.rx) as f64 / dt,
                tx.saturating_sub(s.tx) as f64 / dt,
            )
        } else {
            (0.0, 0.0)
        }
    } else {
        (0.0, 0.0)
    };
    s.rx = rx;
    s.tx = tx;
    s.at = Some(now);
    out
}
fn addresses(iface: &str) -> (Option<String>, Option<String>) {
    if iface == "N/A" {
        return (None, None);
    }
    let out = Command::new("ip")
        .args(["-o", "addr", "show", "dev", iface])
        .output();
    let Ok(out) = out else { return (None, None) };
    let mut v4 = None;
    let mut v6 = None;
    if let Ok(s) = String::from_utf8(out.stdout) {
        for l in s.lines() {
            let p: Vec<&str> = l.split_whitespace().collect();
            if let Some(i) = p.iter().position(|x| *x == "inet") {
                if let Some(a) = p.get(i + 1) {
                    v4 = Some(a.split('/').next().unwrap_or(a).to_string())
                }
            }
            if let Some(i) = p.iter().position(|x| *x == "inet6") {
                if let Some(a) = p.get(i + 1) {
                    v6 = Some(a.split('/').next().unwrap_or(a).to_string())
                }
            }
        }
    }
    (v4, v6)
}
#[allow(dead_code)]
fn _ip_is_valid(s: &str) -> bool {
    s.parse::<IpAddr>().is_ok()
}
