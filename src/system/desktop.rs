use serde::Serialize;
use std::env;
#[derive(Debug, Clone, Serialize)]
pub struct DesktopInfo {
    pub desktop: String,
    pub wm: String,
    pub display: String,
}
pub fn collect() -> DesktopInfo {
    let desktop = env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| "None".into());
    let wm = env::var("XDG_SESSION_DESKTOP").unwrap_or_else(|_| "Unknown".into());
    let display = env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "Unknown".into());
    DesktopInfo {
        desktop,
        wm,
        display,
    }
}
