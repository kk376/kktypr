mod analytics;
mod app;
mod engine;
mod game;
mod ui;

use std::io::{self, stdout};
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use crate::app::App;

fn main() -> io::Result<()> {
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
        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press {
                    app.handle_key_event(key);
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
