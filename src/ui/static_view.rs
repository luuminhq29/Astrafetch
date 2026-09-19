use crate::ui::logo;
use crate::{
    config::Config,
    system::{uptime::format_uptime, SystemSnapshot},
};
use anyhow::Result;

pub fn run(c: Config) -> Result<()> {
    let s = SystemSnapshot::collect(&c)?;

    if crate::ui::colors::no_color() {
        println!("{}", plain(&s, &c));
        return Ok(());
    }

    let ls = logo::render(&c.logo, &s.os);

    let info = vec![
        format!("OS          {}", s.os.version),
        format!("Kernel      {}", s.kernel),
        format!("Host        {}", s.hostname),
        format!("Arch        {}", s.architecture),
        format!(
            "Uptime      {}",
            format_uptime(std::time::Duration::from_secs(s.uptime_seconds))
        ),
        format!("CPU         {} ({:.0}%)", s.cpu.model, s.cpu.usage_percent),
        format!(
            "Memory      {} / {}",
            crate::system::memory::format_bytes(s.memory.used_bytes),
            crate::system::memory::format_bytes(s.memory.total_bytes)
        ),
        format!(
            "Disk        {} / {} ({:.0}%)",
            crate::system::memory::format_bytes(s.disk.used_bytes),
            crate::system::memory::format_bytes(s.disk.total_bytes),
            s.disk.usage_percent
        ),
        format!("GPU         {}", s.gpu.model),
        format!("Shell       {}", s.shell.name),
        format!("Terminal    {}", s.terminal.name),
        format!("DE          {}", s.desktop.desktop),
        format!("Display     {}", s.desktop.display),
    ];

    let width = info.iter().map(String::len).max().unwrap_or(30) + 4;

    println!("\x1b[1;36m╔{}╗\x1b[0m", "═".repeat(width));

    let title = "ASTRΛFETCH";
    println!("║ \x1b[1;36m{title:^width$}\x1b[0m ║");

    let n = ls.len().max(info.len());

    for i in 0..n {
        let left = ls.get(i).cloned().unwrap_or_default();
        let right = info.get(i).cloned().unwrap_or_default();

        println!("║ \x1b[36m{left:<24}\x1b[0m ║ {right:<width$}║");
    }

    println!("╚{}╝", "═".repeat(width + 28));

    if c.author {
        println!("\x1b[90mAstraFetch\nby Lưu Minh Quang - Astra\x1b[0m");
    }

    Ok(())
}

fn plain(s: &SystemSnapshot, _: &Config) -> String {
    format!(
        "AstraFetch\n\
         OS: {}\n\
         Kernel: {}\n\
         Host: {}\n\
         Arch: {}\n\
         Uptime: {}\n\
         CPU: {}\n\
         Memory: {} / {}\n\
         GPU: {}\n\
         Shell: {}\n\
         Terminal: {}\n\
         DE: {}\n",
        s.os.version,
        s.kernel,
        s.hostname,
        s.architecture,
        format_uptime(std::time::Duration::from_secs(s.uptime_seconds)),
        s.cpu.model,
        crate::system::memory::format_bytes(s.memory.used_bytes),
        crate::system::memory::format_bytes(s.memory.total_bytes),
        s.gpu.model,
        s.shell.name,
        s.terminal.name,
        s.desktop.desktop,
    )
}
