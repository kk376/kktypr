mod analytics;
mod app;
mod engine;
mod game;
mod ui;

use crate::app::App;
use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, stdout};
use std::time::{Duration, Instant};

fn handle_cli_args<T: AsRef<str>>(args: &[T]) -> Option<bool> {
    if args.len() > 1 {
        match args[1].as_ref() {
            "-v" | "--version" | "-V" => {
                println!("kktypr {}", env!("CARGO_PKG_VERSION"));
                Some(true)
            }
            "-h" | "--help" => {
                println!("kktypr {}", env!("CARGO_PKG_VERSION"));
                println!("Terminal touch typing academy and developer syntax trainer\n");
                println!("USAGE:\n    kktypr [OPTIONS]\n");
                println!("OPTIONS:\n    -h, --help          Print help information");
                println!("    -v, -V, --version   Print version information");
                Some(true)
            }
            _ => None,
        }
    } else {
        None
    }
}

struct TerminalGuard {
    restored: bool,
}

impl TerminalGuard {
    fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut out = stdout();
        if let Err(e) = execute!(out, EnterAlternateScreen) {
            let _ = disable_raw_mode();
            return Err(e);
        }
        Ok(Self { restored: false })
    }

    fn restore(&mut self) -> io::Result<()> {
        if !self.restored {
            self.restored = true;
            disable_raw_mode()?;
            execute!(stdout(), LeaveAlternateScreen)?;
        }
        Ok(())
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

fn main() -> io::Result<()> {
    // Quick non-interactive CLI argument handling
    let args: Vec<String> = std::env::args().collect();
    if handle_cli_args(&args).is_some() {
        return Ok(());
    }

    // Setup clean terminal panic hook
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
        default_panic(info);
    }));

    // Initialize terminal in alternate screen and raw mode with RAII guard
    let mut guard = TerminalGuard::enter()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let initial_size = terminal.size()?;
    app.update_playfield_size(initial_size.width, initial_size.height);

    let tick_rate = Duration::from_millis(16); // ~60 FPS when active
    let idle_tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    let mut needs_redraw = true;

    loop {
        if needs_redraw {
            terminal.draw(|frame| ui::render(&mut app, frame))?;
            needs_redraw = false;
        }

        let is_live_screen = matches!(
            app.screen,
            crate::app::AppScreen::WordDropGame | crate::app::AppScreen::TypingActive
        );
        let poll_budget = if is_live_screen {
            tick_rate.saturating_sub(last_tick.elapsed())
        } else {
            idle_tick_rate
        };

        if event::poll(poll_budget)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    app.handle_key_event(key);
                    needs_redraw = true;
                }
                Event::Resize(w, h) => {
                    app.update_playfield_size(w, h);
                    needs_redraw = true;
                }
                _ => {}
            }
        }

        if is_live_screen && last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
            needs_redraw = true;
        }

        if app.should_quit {
            break;
        }
    }

    // Save profile state on exit
    app.sync_and_save_profile();

    // Cleanly restore terminal
    terminal.show_cursor()?;
    guard.restore()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_cli_args_version() {
        assert_eq!(handle_cli_args(&["kktypr", "--version"]), Some(true));
        assert_eq!(handle_cli_args(&["kktypr", "-v"]), Some(true));
        assert_eq!(handle_cli_args(&["kktypr", "-V"]), Some(true));
    }

    #[test]
    fn test_handle_cli_args_help() {
        assert_eq!(handle_cli_args(&["kktypr", "--help"]), Some(true));
        assert_eq!(handle_cli_args(&["kktypr", "-h"]), Some(true));
    }

    #[test]
    fn test_handle_cli_args_passthrough() {
        assert_eq!(handle_cli_args(&["kktypr"]), None);
        assert_eq!(handle_cli_args(&["kktypr", "other"]), None);
    }
}
