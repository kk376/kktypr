use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::App;
use crate::game::word_drop::GameState;
use crate::ui::theme::Theme;

pub fn render_word_drop(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Game HUD
            Constraint::Min(10),   // Falling word playfield
            Constraint::Length(2), // Baseline floor
        ])
        .split(frame.area());

    let game = &app.game;

    // 1. Game HUD
    let mut hearts = String::new();
    for _ in 0..game.lives {
        hearts.push_str("♥ ");
    }

    let multiplier = 1 + game.streak / 5;
    let hud_line = Line::from(vec![
        Span::styled(" Score: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{} ", game.score), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled(" Streak: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}x (x{}) ", game.streak, multiplier), Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled(" Lives: ", Style::default().fg(Color::Gray)),
        Span::styled(hearts, Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled(" High Score: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{} ", app.profile.game_high_score), Style::default().fg(Color::Magenta)),
        Span::raw(" | "),
        Span::styled(" Esc: Exit ", Style::default().fg(Color::DarkGray)),
    ]);

    let hud = Paragraph::new(hud_line)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Word Drop Arcade (Typing Bubbles TUI) ")
                .title_alignment(Alignment::Center)
                .border_style(Style::default().fg(Color::Magenta)),
        );
    frame.render_widget(hud, chunks[0]);

    // 2. Playfield
    let playfield_area = chunks[1];
    let playfield_block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT)
        .border_style(Style::default().fg(Color::DarkGray));
    frame.render_widget(playfield_block, playfield_area);

    if game.state == GameState::GameOver {
        let game_over_text = vec![
            Line::from(""),
            Line::from(Span::styled("  ██████╗  █████╗ ███╗   ███╗███████╗   ██████╗ ██╗   ██╗███████╗██████╗  ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
            Line::from(Span::styled(" ██╔════╝ ██╔══██╗████╗ ████║██╔════╝  ██╔═══██╗██║   ██║██╔════╝██╔══██╗ ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
            Line::from(Span::styled(" ██║  ███╗███████║██╔████╔██║█████╗    ██║   ██║██║   ██║█████╗  ██████╔╝ ", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD))),
            Line::from(Span::styled(" ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝    ██║   ██║╚██╗ ██╔╝██╔══╝  ██╔══██╗ ", Style::default().fg(Color::LightRed))),
            Line::from(Span::styled(" ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗  ╚██████╔╝ ╚████╔╝ ███████╗██║  ██║ ", Style::default().fg(Color::DarkGray))),
            Line::from(""),
            Line::from(Span::styled(format!("Final Score: {}", game.score), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(vec![
                Span::styled("Press [r] to Play Again  ", Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD)),
                Span::styled("|  [q / Esc] Quit to Main Menu", Style::default().fg(Color::Gray)),
            ]),
        ];
        let p = Paragraph::new(game_over_text).alignment(Alignment::Center);
        frame.render_widget(p, playfield_area);
    } else {
        // Render falling words at their respective (x, y) coordinates with high-contrast pills
        let buf = frame.buffer_mut();
        for (idx, word) in game.words.iter().enumerate() {
            let row = playfield_area.top() + (word.y as u16);
            if row < playfield_area.bottom() {
                let start_col = playfield_area.left() + word.x;
                let is_targeted = game.targeted_word_idx == Some(idx);
                let bubble_bg = if is_targeted { Color::Yellow } else { Color::Cyan };

                // Left capsule padding
                if start_col > playfield_area.left()
                    && let Some(cell) = buf.cell_mut((start_col - 1, row))
                {
                    cell.set_char(' ').set_style(Style::default().bg(bubble_bg));
                }

                for (char_idx, c) in word.text.chars().enumerate() {
                    let col = start_col + char_idx as u16;
                    if col < playfield_area.right() {
                        let style = if char_idx < word.typed_len {
                            Style::default().fg(Color::White).bg(Theme::SUCCESS).add_modifier(Modifier::BOLD)
                        } else if is_targeted {
                            Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
                        };

                        if let Some(cell) = buf.cell_mut((col, row)) {
                            cell.set_char(c).set_style(style);
                        }
                    }
                }

                // Right capsule padding
                let end_col = start_col + word.text.chars().count() as u16;
                if end_col < playfield_area.right()
                    && let Some(cell) = buf.cell_mut((end_col, row))
                {
                    cell.set_char(' ').set_style(Style::default().bg(bubble_bg));
                }
            }
        }
    }

    // 3. Floor Danger Zone
    let floor_line = Line::from(Span::styled(
        "════════════════════════════════ DANGER BASELINE ════════════════════════════════",
        Style::default().fg(Color::Red),
    ));
    let floor_p = Paragraph::new(floor_line).alignment(Alignment::Center);
    frame.render_widget(floor_p, chunks[2]);
}
