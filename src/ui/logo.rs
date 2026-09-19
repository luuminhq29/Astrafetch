use crate::system::os::OsInfo;

/// Returns a distro logo suitable for a Fastfetch-style left column.
/// The artwork is intentionally kept as plain Unicode/ASCII so it works
/// without runtime downloads, external assets, or special fonts.
pub fn render(mode: &str, os: &OsInfo) -> Vec<String> {
    match mode.to_lowercase().as_str() {
        "none" => Vec::new(),
        "ubuntu" => ubuntu(),
        "debian" => debian(),
        "arch" => arch(),
        "fedora" => fedora(),
        "opensuse" => opensuse(),
        "alpine" => alpine(),
        "manjaro" => manjaro(),
        "linuxmint" | "mint" => mint(),
        "kali" => kali(),
        "pop" | "popos" => popos(),
        "gentoo" => gentoo(),
        "void" => void_linux(),
        "rocky" => rocky(),
        "almalinux" | "alma" => alma(),
        "centos" => centos(),
        "nixos" => nixos(),
        "linux" => linux(),
        "custom" => custom(),
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
        "opensuse-tumbleweed" | "opensuse-leap" | "opensuse" => opensuse(),
        "alpine" => alpine(),
        "manjaro" => manjaro(),
        "linuxmint" => mint(),
        "kali" => kali(),
        "pop" => popos(),
        "gentoo" => gentoo(),
        "void" => void_linux(),
        "rocky" => rocky(),
        "almalinux" => alma(),
        "centos" => centos(),
        "nixos" => nixos(),
        _ => linux(),
    }
}

fn ubuntu() -> Vec<String> {
    vec![
        "             .-/+oossssoo+/-.",
        "         `:+ssssssssssssssssss+:`",
        "       -+ssssssssssssssssssyyssss+-",
        "     .ossssssssssssssssssdMMMNysssso.",
        "    /ssssssssssshdmmNNmmyNMMMMhssssss/",
        "   +ssssssssshmydMMMMMMMNddddyssssssss+",
        "  /sssssssshNMMMyhhyyyyhmNMMMNhssssssss/",
        " .ssssssssdMMMNhsssssssssshNMMMdssssssss.",
        " +sssshhhyNMMNyssssssssssssyNMMMysssssss+",
        " ossyNMMMNyMMhsssssssssssssshmmmhssssssso",
        " ossyNMMMNyMMhsssssssssssssshmmmhssssssso",
        " +sssshhhyNMMNyssssssssssssyNMMMysssssss+",
        " .ssssssssdMMMNhsssssssssshNMMMdssssssss.",
        "  /sssssssshNMMMyhhyyyyhmNMMMNhssssssss/",
        "   +ssssssssshmydMMMMMMMNddddyssssssss+",
        "    /ssssssssssshdmmNNmmyNMMMMhssssss/",
        "     .ossssssssssssssssssdMMMNysssso.",
        "       -+ssssssssssssssssssyyssss+-",
        "         `:+ssssssssssssssssss+:`",
        "             .-/+oossssoo+/-.      ",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

fn debian() -> Vec<String> {
    lines(&[
        "       _,met$$$$$gg.",
        "    ,g$$$$$$$$$$$$$$$P.",
        "  ,g$$P\"       \"Y$$.",
        " ,$$P'              `$$$.",
        "',$$P       ,ggs.     `$$b:",
        "`d$$'     ,$P\"'   .    $$$",
        " $$P      d$'     ,    $$P",
        " $$:      $$.   -    ,d$$'",
        " $$;      Y$b._   _,d$P'",
        " Y$$.    `.`\"Y$$$$P\"'",
        " `$$b      \"-.__",
        "  `Y$$",
        "   `Y$$.",
        "     `$$b.",
        "       `Y$$b.",
        "          `\"Y$b._",
        "              `\"\"",
    ])
}

fn arch() -> Vec<String> {
    lines(&[
        "                   -`",
        "                  .o+`",
        "                 `ooo/",
        "                `+oooo:",
        "               `+oooooo:",
        "               -+oooooo+:",
        "             `/:-:++oooo+:",
        "            `/++++/++++++:",
        "           `/++++++++++++++:",
        "          `/+++ooooooooooooo/`",
        "         ./ooosssso++osssssso+`",
        "        .oossssso-````/ossssss+`",
        "       -osssssso.      :ssssssso.",
        "      :osssssss/        osssso++.",
        "     /ossssssss/        +ssssooo/-",
        "   `/ossssso+/:-        -:/+osssso+-",
        "  `+sso+:-`                 `.-/+oso:",
        " `++:.                           `-/+/",
        " .`                                 `/",
    ])
}

fn fedora() -> Vec<String> {
    lines(&[
        "        /:-------------:\\",
        "       :-------------------::",
        "      :-----------/shhyo/:--:",
        "    /-----------omMMMMMMMho-:",
        "   :-----------yMMMMMMMMMMs:",
        "  :-----------yMMMMMMMMMMs",
        " /-----------/NMMMMMMMMMM+",
        "/-----------/NMMMMMMMMMM+",
        ":-----------yMMMMMMMMMMs",
        " :---------/yMMMMMMMMMMs",
        "  :--------/yMMMMMMMMMMs",
        "   :-------/yMMMMMMMMMMs",
        "    /------/yMMMMMMMMMMs",
        "     :-----/yMMMMMMMMMMs",
        "      :----/yMMMMMMMMMMs",
        "       :---/yMMMMMMMMMMs",
        "        :--/yMMMMMMMMMMs",
        "         :-/yMMMMMMMMMMs",
        "          :/yMMMMMMMMMMs",
        "           `:yMMMMMMMMMMs",
    ])
}

fn opensuse() -> Vec<String> {
    lines(&[
        "         .;ldkO0000Okdl;.",
        "       .;d00KKKKKKKKKK00d;.",
        "      .d0KKKKKKKKKKKKKKKK0d.",
        "     :0KKKKKKKKKKKKKKKKKKKK0:",
        "    :0KKKKKKKKKKKKKKKKKKKKKK0:",
        "   .0KKKKKKKKKKKKKKKKKKKKKKKK0.",
        "   dKKKKKKKKKKKKKKKKKKKKKKKKKKd",
        "  lKKKKKKKKKKKKKKKKKKKKKKKKKKKKl",
        "  0KKKKKKKKKKKKKKKKKKKKKKKKKKKK0",
        "  0KKKKKKKKKKKKKKKKKKKKKKKKKKKK0",
        "  lKKKKKKKKKKKKKKKKKKKKKKKKKKKKl",
        "   dKKKKKKKKKKKKKKKKKKKKKKKKKKd",
        "    :0KKKKKKKKKKKKKKKKKKKKKK0:",
        "     :0KKKKKKKKKKKKKKKKKKKK0:",
        "      .d0KKKKKKKKKKKKKKKK0d.",
        "       .;d00KKKKKKKKKK00d;.",
        "         .;ldkO0000Okdl;.",
    ])
}

fn alpine() -> Vec<String> {
    lines(&[
        "       /\\      _      ",
        "      /  \\___ / \\     ",
        "     / /\\   / /\\ \\    ",
        "    / /  \\_/ /  \\ \\   ",
        "   /_/    /__/    \\_\\  ",
        "      A L P I N E       ",
    ])
}

fn manjaro() -> Vec<String> {
    lines(&[
        "████████████████████████████",
        "████████████████████████████",
        "███████  ███████  █████████",
        "███████  ███████  █████████",
        "███████  ███████  █████████",
        "███████  ███████  █████████",
        "███████  ███████  █████████",
        "████████████████████████████",
        "████████████████████████████",
    ])
}

fn mint() -> Vec<String> {
    lines(&[
        "             .88888888:.",
        "          888888888888888888.",
        "        .8888888888888888888888.",
        "       .888888888888888888888888.",
        "       88888888888888888888888888",
        "       88888888888888888888888888",
        "       88888888888888888888888888",
        "       88888888888888888888888888",
        "        `8888888888888888888888`",
        "          `888888888888888888`",
        "             `:88888888:`",
    ])
}

fn kali() -> Vec<String> {
    lines(&[
        "            ...............",
        "        .:::::::::::::::::::::. ",
        "      .:::::::::::::::::::::::::.",
        "     :::::::::::::::::::::::::::::",
        "    ::::::::::::-::::::::::::::::",
        "   ::::::::::::-:::::::::::::::::",
        "   :::::::::::::::::::::::::::::::",
        "    :::::::::::::::::::::::::::::",
        "     :::::::::::::::::::::::::::",
        "       .:::::::::::::::::::::.  ",
        "           `:::::::::::::`     ",
        "               `::::`          ",
    ])
}

fn popos() -> Vec<String> {
    lines(&[
        "          .o+`                 ",
        "         `ooo/                 ",
        "        `+oooo:                ",
        "       `+oooooo:               ",
        "       -+oooooo+:              ",
        "     `/:-:++oooo+:             ",
        "    `/++++/+++++++:            ",
        "   `/++++++++++++++:           ",
        "  `/+++ooooooooooooo/`         ",
        " ./ooosssso++osssssso+`        ",
        ".oossssso-````/ossssss+`      ",
        "-osssssso.      :ssssssso.    ",
    ])
}

fn gentoo() -> Vec<String> {
    lines(&[
        "        _-----_        ",
        "       (       \\      ",
        "      (         )      ",
        "      (  GENTOO )      ",
        "       (       /       ",
        "        `-----'        ",
        "      __/   \\__       ",
        "     /           \\     ",
    ])
}

fn void_linux() -> Vec<String> {
    lines(&[
        "        ________        ",
        "       /  _____/        ",
        "      /   \\  ___       ",
        "     /    /   \\  \\     ",
        "    /____/     \\__\\    ",
        "       V O I D          ",
    ])
}

fn rocky() -> Vec<String> {
    lines(&[
        "          /\\            ",
        "         /  \\           ",
        "        / /\\ \\          ",
        "       / /  \\ \\         ",
        "      /_/____\\_\\        ",
        "      ROCKY LINUX         ",
    ])
}

fn alma() -> Vec<String> {
    lines(&[
        "          .--------.      ",
        "        .'          '.    ",
        "       /   .------.   \\   ",
        "      ;   /        \\   ;  ",
        "      |  |  ALMA    |  |  ",
        "      ;   \\        /   ;  ",
        "       \\   '------'   /   ",
        "        '.          .'    ",
        "          '--------'      ",
    ])
}

fn centos() -> Vec<String> {
    lines(&[
        "       .---------.        ",
        "      /  .-----.  \\       ",
        "     /  /       \\  \\      ",
        "    |  |  CENTOS |  |     ",
        "     \\  \\       /  /      ",
        "      \\  '-----'  /       ",
        "       '---------'        ",
    ])
}

fn nixos() -> Vec<String> {
    lines(&[
        "        \\   /\\   /       ",
        "         \\ /  \\ /        ",
        "      ---- NIXOS ----      ",
        "         / \\  / \\        ",
        "        /   \\/   \\       ",
        "       /___________\\      ",
    ])
}

fn linux() -> Vec<String> {
    lines(&[
        "           .-::::::::::-.     ",
        "        .:+ssssssssssssss+:.  ",
        "      -+ssssssssssssssssssss+-",
        "    .osssssssssssssssssssssssso.",
        "   /sssssssssssssssssssssssssss/",
        "  +ssssssssssssssssssssssssssss+",
        " /ssssssssssssssssssssssssssssss/",
        " ossssssssssssssssssssssssssssso",
        " +ssssssssssssssssssssssssssss+",
        "  /ssssssssssssssssssssssssss/",
        "   .osssssssssssssssssssssso.",
        "     -+ssssssssssssssssss+-",
        "        .:+ssssssssss+:.",
        "           .-::::::-.     ",
    ])
}

fn custom() -> Vec<String> {
    let path = std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .map(|h| h.join(".config/astrafetch/logo.txt"));

    path.and_then(|p| std::fs::read_to_string(p).ok())
        .map(|s| s.lines().map(String::from).collect())
        .filter(|lines: &Vec<String>| !lines.is_empty())
        .unwrap_or_else(linux)
}

fn lines(lines: &[&str]) -> Vec<String> {
    lines.iter().map(|line| (*line).to_string()).collect()
}
