use super::read_os_release;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct OsInfo {
    pub name: String,
    pub version: String,
    pub id: String,
}

pub fn detect() -> OsInfo {
    let r = read_os_release();
    let id = r.get("ID").cloned().unwrap_or_else(|| "linux".into());
    let pretty = r
        .get("PRETTY_NAME")
        .cloned()
        .unwrap_or_else(|| "Linux".into());
    let name = match id.as_str() {
        "ubuntu" => "Ubuntu",
        "debian" => "Debian",
        "arch" => "Arch Linux",
        "fedora" => "Fedora",
        "opensuse-tumbleweed" | "opensuse-leap" | "opensuse" => "openSUSE",
        "alpine" => "Alpine",
        "manjaro" => "Manjaro",
        "linuxmint" => "Linux Mint",
        "kali" => "Kali Linux",
        "pop" => "Pop!_OS",
        "gentoo" => "Gentoo",
        "void" => "Void Linux",
        "rocky" => "Rocky Linux",
        "almalinux" => "AlmaLinux",
        "centos" => "CentOS",
        "nixos" => "NixOS",
        _ => "Linux",
    }
    .to_string();
    OsInfo {
        name,
        version: pretty,
        id,
    }
}
