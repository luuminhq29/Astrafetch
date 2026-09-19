use crate::system::{uptime::format_uptime, SystemSnapshot};
pub fn format(s: &SystemSnapshot) -> String {
    format!(
        "{} | {} | {} | RAM {} | Uptime {}",
        s.os.version,
        s.cpu.model,
        s.gpu.model,
        crate::system::memory::format_bytes(s.memory.total_bytes),
        format_uptime(std::time::Duration::from_secs(s.uptime_seconds))
    )
}
