use serde::Serialize;
use std::env;
#[derive(Debug,Clone,Serialize)] pub struct TerminalInfo{pub name:String}
pub fn collect()->TerminalInfo{let vars=[("kitty","KITTY_PID"),("Alacritty","ALACRITTY_LOG"),("tmux","TMUX"),("screen","STY")];for (name,key) in vars{if env::var_os(key).is_some(){return TerminalInfo{name:name.into()}}}let term=env::var("TERM").unwrap_or_default();let name=if term.contains("xterm"){"xterm"}else if term.contains("linux"){"TTY"}else if term.is_empty(){"Unknown"}else{term.as_str()};TerminalInfo{name:name.into()}}
