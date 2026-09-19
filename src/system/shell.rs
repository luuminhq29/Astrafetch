use serde::Serialize;
use std::{env, process::Command};
#[derive(Debug, Clone, Serialize)]
pub struct ShellInfo {
    pub name: String,
    pub version: Option<String>,
}
pub fn collect() -> ShellInfo {
    let raw = env::var("SHELL").unwrap_or_default();
    let name = raw
        .rsplit('/')
        .next()
        .filter(|x| !x.is_empty())
        .unwrap_or("Unknown")
        .to_string();
    let version = if matches!(name.as_str(), "bash" | "zsh" | "fish") {
        Command::new(&name)
            .arg("--version")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .and_then(|s| s.lines().next().map(str::to_owned))
    } else {
        None
    };
    ShellInfo { name, version }
}
