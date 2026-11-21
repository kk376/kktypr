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
                println!(
                    "A blazingly fast touch typing academy and developer syntax tutor for your terminal\n"
                );
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

    // Initialize terminal in alternate screen and raw mode
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| ui::render(&mut app, frame))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key_event(key);
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    // Save profile state on exit
    app.sync_and_save_profile();

    // Cleanly restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

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
