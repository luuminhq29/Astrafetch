use crate::system::os::OsInfo;

pub fn logo(os: &OsInfo) -> Vec<String> {
    let key = os.id.to_lowercase();

    match key.as_str() {
        "ubuntu" => vec![
            "        _        ".to_string(),
            "       / \\       ".to_string(),
            "      /●●\\      ".to_string(),
            "     /●●●●\\     ".to_string(),
            "      \\  /       ".to_string(),
            "    UBUNTU       ".to_string(),
        ],

        "debian" => vec![
            "       .--.      ".to_string(),
            "      /.-. '     ".to_string(),
            "     /   |       ".to_string(),
            "     \\  /        ".to_string(),
            "      `--'       ".to_string(),
            "    DEBIAN       ".to_string(),
        ],

        "arch" => vec![
            "       /\\       ".to_string(),
            "      /  \\      ".to_string(),
            "     / /\\ \\     ".to_string(),
            "    /_/  \\_\\    ".to_string(),
            "      ARCH       ".to_string(),
        ],

        "fedora" => vec![
            "      ______     ".to_string(),
            "     / ____ \\    ".to_string(),
            "    | /    \\ |   ".to_string(),
            "    | \\____/ |   ".to_string(),
            "     \\______/    ".to_string(),
            "     FEDORA      ".to_string(),
        ],

        _ => linux(),
    }
}

fn linux() -> Vec<String> {
    vec![
        "       ██████    ".to_string(),
        "     ██████████  ".to_string(),
        "    ███  ███  ███ ".to_string(),
        "    ███  ███  ███ ".to_string(),
        "    █████████████ ".to_string(),
        "       LINUX      ".to_string(),
    ]
}
