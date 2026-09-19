use crate::{config::Config,system::SystemSnapshot};
use crate::ui::renderer;
use anyhow::Result;
use crossterm::{event::{self,Event,KeyCode,KeyEvent,KeyModifiers},execute,terminal::{disable_raw_mode,enable_raw_mode,EnterAlternateScreen,LeaveAlternateScreen}};
use ratatui::{backend::CrosstermBackend,Terminal};
use std::{io::{self,Write},time::{Duration,Instant}};

pub fn run(c:Config)->Result<()>{enable_raw_mode()?;let mut out=io::stdout();execute!(out,EnterAlternateScreen)?;let backend=CrosstermBackend::new(out);let mut terminal=Terminal::new(backend)?;let result=loop_app(&mut terminal,c);disable_raw_mode()?;execute!(terminal.backend_mut(),LeaveAlternateScreen)?;terminal.show_cursor()?;result}
fn loop_app(terminal:&mut Terminal<CrosstermBackend<io::Stdout>>,c:Config)->Result<()>{let mut paused=false;let mut frame_no=0u64;let mut last=Instant::now()-Duration::from_secs(10);let mut snap=SystemSnapshot::collect(&c)?;loop{if !paused&&last.elapsed()>=Duration::from_secs(c.refresh_rate.max(1)){snap=SystemSnapshot::collect(&c)?;last=Instant::now();}terminal.draw(|f|renderer::draw(f,&snap,&c,frame_no,paused))?;frame_no=frame_no.wrapping_add(1);if event::poll(Duration::from_millis(100))?{if let Event::Key(KeyEvent{code,modifiers,..})=event::read()?{match(code,modifiers){(KeyCode::Char('q'),_)|(KeyCode::Esc,_)=>break,(KeyCode::Char('p'),KeyModifiers::NONE)=>paused=!paused,(KeyCode::Char('r'),KeyModifiers::NONE)=>{snap=SystemSnapshot::collect(&c)?;last=Instant::now();},(KeyCode::Char('h'),KeyModifiers::NONE)=>show_help(terminal)?,(KeyCode::Char('i'),KeyModifiers::NONE)=>show_info(terminal,&snap)?,_=>{}}}}}Ok(())}
fn show_help(terminal:&mut Terminal<CrosstermBackend<io::Stdout>>)->Result<()>{terminal.draw(|f|{let p=ratatui::widgets::Paragraph::new("q / ESC  Exit\np       Pause animation\nr       Refresh\ni       System information\nh       Help\n\nPress any key to return").block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).title(" AstraFetch Help "));f.render_widget(p,f.area());})?;loop{if event::poll(Duration::from_millis(250))?{if matches!(event::read()?,Event::Key(_)){break}}}Ok(())}
#[allow(dead_code)] fn _flush<W:Write>(w:&mut W)->io::Result<()>{w.flush()}

fn show_info(terminal:&mut Terminal<CrosstermBackend<io::Stdout>>, s:&SystemSnapshot)->Result<()> {
    let text = format!("OS: {}\nKernel: {}\nHost: {}\nArch: {}\nCPU: {}\nCores: {} physical / {} logical\nMemory: {} / {}\nGPU: {}\nShell: {}\nTerminal: {}\nDesktop: {}\nDisplay: {}\n\nPress any key to return", s.os.version, s.kernel, s.hostname, s.architecture, s.cpu.model, s.cpu.physical_cores, s.cpu.threads, crate::system::memory::format_bytes(s.memory.used_bytes), crate::system::memory::format_bytes(s.memory.total_bytes), s.gpu.model, s.shell.name, s.terminal.name, s.desktop.desktop, s.desktop.display);
    terminal.draw(|f| {
        let p=ratatui::widgets::Paragraph::new(text).block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).title(" System Information "));
        f.render_widget(p,f.area());
    })?;
    loop { if event::poll(Duration::from_millis(250))? { if matches!(event::read()?,Event::Key(_)){ break; } } }
    Ok(())
}
