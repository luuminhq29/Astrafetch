use crate::ui::logo;
use crate::{
    config::Config,
    system::{uptime::format_uptime, SystemSnapshot},
};
use anyhow::Result;

pub fn run(c: Config) -> Result<()> {
    let s = SystemSnapshot::collect(&c)?;

    if crate::ui::colors::no_color() {
        println!("{}", plain(&s));
        return Ok(());
    }

    let ls = logo::render(&c.logo, &s.os);
    let info = info_lines(&s);
    let logo_width = ls
        .iter()
        .map(|line| display_width(line.as_str()))
        .max()
        .unwrap_or(0)
        .max(28)
        + 2;

    let n = ls.len().max(info.len());
    for i in 0..n {
        let left = ls.get(i).map(String::as_str).unwrap_or("");
        let right = info.get(i).map(String::as_str).unwrap_or("");
        let padded = format!("{left:<width$}", width = logo_width);
        println!("\x1b[1;36m{padded}\x1b[0m  {right}");
    }

    if c.author {
        println!();
        println!("\x1b[90mAstraFetch  •  by Lưu Minh Quang - Astra\x1b[0m");
    }

    Ok(())
}

fn info_lines(s: &SystemSnapshot) -> Vec<String> {
    vec![
        format!("\x1b[1;36m{}@{}\x1b[0m", username(), s.hostname),
        "\x1b[90m----------------\x1b[0m".to_string(),
        format!("\x1b[1;36mOS:\x1b[0m {}", s.os.version),
        format!("\x1b[1;36mHost:\x1b[0m {}", s.hostname),
        format!("\x1b[1;36mKernel:\x1b[0m {}", s.kernel),
        format!(
            "\x1b[1;36mUptime:\x1b[0m {}",
            format_uptime(std::time::Duration::from_secs(s.uptime_seconds))
        ),
        format!("\x1b[1;36mCPU:\x1b[0m {}", s.cpu.model),
        format!("\x1b[1;36mGPU:\x1b[0m {}", s.gpu.model),
        format!(
            "\x1b[1;36mMemory:\x1b[0m {} / {} ({:.0}%)",
            crate::system::memory::format_bytes(s.memory.used_bytes),
            crate::system::memory::format_bytes(s.memory.total_bytes),
            s.memory.usage_percent
        ),
        format!(
            "\x1b[1;36mDisk (/):\x1b[0m {} / {} ({:.0}%)",
            crate::system::memory::format_bytes(s.disk.used_bytes),
            crate::system::memory::format_bytes(s.disk.total_bytes),
            s.disk.usage_percent
        ),
        format!("\x1b[1;36mShell:\x1b[0m {}", s.shell.name),
        format!("\x1b[1;36mTerminal:\x1b[0m {}", s.terminal.name),
        format!("\x1b[1;36mDE:\x1b[0m {}", s.desktop.desktop),
        format!("\x1b[1;36mDisplay:\x1b[0m {}", s.desktop.display),
    ]
}

fn display_width(s: &str) -> usize {
    s.chars().count()
}

fn username() -> String {
    std::env::var("USER").unwrap_or_else(|_| "user".to_string())
}

fn plain(s: &SystemSnapshot) -> String {
    format!(
        "AstraFetch\nOS: {}\nHost: {}\nKernel: {}\nUptime: {}\nCPU: {}\nGPU: {}\nMemory: {} / {} ({:.0}%)\nDisk: {} / {} ({:.0}%)\nShell: {}\nTerminal: {}\nDE: {}\n",
        s.os.version,
        s.hostname,
        s.kernel,
        format_uptime(std::time::Duration::from_secs(s.uptime_seconds)),
        s.cpu.model,
        s.gpu.model,
        crate::system::memory::format_bytes(s.memory.used_bytes),
        crate::system::memory::format_bytes(s.memory.total_bytes),
        s.memory.usage_percent,
        crate::system::memory::format_bytes(s.disk.used_bytes),
        crate::system::memory::format_bytes(s.disk.total_bytes),
        s.disk.usage_percent,
        s.shell.name,
        s.terminal.name,
        s.desktop.desktop,
    )
}
