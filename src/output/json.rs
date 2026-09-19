use crate::system::SystemSnapshot;
use anyhow::Result;
pub fn print_json(s:&SystemSnapshot)->Result<()>{println!("{}",serde_json::to_string_pretty(s)?);Ok(())}
