use serde::Serialize;
use std::fs;

#[derive(Debug, Clone, Serialize)]
pub struct MemoryInfo { pub total_bytes:u64, pub used_bytes:u64, pub available_bytes:u64, pub usage_percent:f64, pub swap_total_bytes:u64, pub swap_used_bytes:u64 }

pub fn collect()->MemoryInfo { let mut total=0; let mut avail=0; let mut swap_total=0; let mut swap_free=0; if let Ok(t)=fs::read_to_string("/proc/meminfo") { for l in t.lines(){ if let Some((k,v))=l.split_once(':'){let n=v.split_whitespace().next().and_then(|x|x.parse::<u64>().ok()).unwrap_or(0)*1024; match k {"MemTotal"=>total=n,"MemAvailable"=>avail=n,"SwapTotal"=>swap_total=n,"SwapFree"=>swap_free=n,_=>{}}}}} let used=total.saturating_sub(avail); MemoryInfo{total_bytes:total,used_bytes:used,available_bytes:avail,usage_percent:if total>0{used as f64/total as f64*100.0}else{0.0},swap_total_bytes:swap_total,swap_used_bytes:swap_total.saturating_sub(swap_free)} }

pub fn format_bytes(v:u64)->String { const G:u64=1024*1024*1024; const M:u64=1024*1024; if v>=G {format!("{:.1} GiB",v as f64/G as f64)} else {format!("{:.0} MiB",v as f64/M as f64)} }
