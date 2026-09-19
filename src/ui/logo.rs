use crate::system::os::OsInfo;

pub fn render(mode: &str, os: &OsInfo) -> Vec<String> {
    match mode.to_lowercase().as_str() {
        "none" => Vec::new(),
        "ubuntu" => ubuntu(),
        "debian" => debian(),
        "arch" => arch(),
        "fedora" => fedora(),
        "linux" => linux(),
        "auto" => render_auto(os),
        _ => render_auto(os),
    }
}

fn render_auto(os: &OsInfo) -> Vec<String> {
    match os.id.to_lowercase().as_str() {
        "ubuntu" => ubuntu(),
        "debian" => debian(),
        "arch" | "archlinux" => arch(),
        "fedora" => fedora(),
        _ => linux(),
    }
}

fn ubuntu() -> Vec<String> {
    vec![
        "        _        ".into(),
        "       / \\       ".into(),
        "      /● ●\\      ".into(),
        "     /● ● ● ●\\   ".into(),
        "      \\  /       ".into(),
        "    UBUNTU       ".into(),
    ]
}

fn debian() -> Vec<String> {
    vec![
        "       .--.      ".into(),
        "      /.-. '     ".into(),
        "     /   |       ".into(),
        "     \\  /        ".into(),
        "      `--'       ".into(),
        "     DEBIAN      ".into(),
    ]
}

fn arch() -> Vec<String> {
    vec![
        "       /\\        ".into(),
        "      /  \\       ".into(),
        "     / /\\ \\      ".into(),
        "    /_/  \\_\\     ".into(),
        "      ARCH       ".into(),
    ]
}

fn fedora() -> Vec<String> {
    vec![
        "      ______     ".into(),
        "     / ____ \\    ".into(),
        "    | /    \\ |   ".into(),
        "    | \\____/ |   ".into(),
        "     \\______/    ".into(),
        "     FEDORA      ".into(),
    ]
}

fn linux() -> Vec<String> {
    vec![
        "       ██████       ".into(),
        "     ██████████     ".into(),
        "    ███  ███  ███   ".into(),
        "    ███  ███  ███   ".into(),
        "    █████████████    ".into(),
        "       LINUX        ".into(),
    ]
}
