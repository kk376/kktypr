use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::App;
use crate::engine::session::CharStatus;
use crate::ui::keyboard::KeyboardWidget;
use crate::ui::theme::Theme;

pub fn render_typing_active(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Top HUD metrics
            Constraint::Min(8),    // Target text typing box
            Constraint::Length(10), // Visual Keyboard Widget
        ])
        .split(frame.area());

    let session = match app.active_session.as_ref() {
        Some(s) => s,
        None => return,
    };

    let metrics = session.current_metrics();

    // 1. Top HUD
    let time_display = if let Some(remaining) = session.remaining_time() {
        let secs = remaining.as_secs();
        format!("Time Left: {:02}:{:02}", secs / 60, secs % 60)
    } else {
        let elapsed = session.elapsed().as_secs();
        format!("Elapsed: {:02}:{:02}", elapsed / 60, elapsed % 60)
    };

    let title_context = if let Some(ex) = app.current_exercise() {
        format!(" Lesson: {}  |  {} ", ex.title, ex.exercise_type.display_name())
    } else {
        " Speed Test Mode ".to_string()
    };

    let hud_line = Line::from(vec![
        Span::styled(" Gross: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{:.1} WPM ", metrics.gross_wpm), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled(" Net: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{:.1} WPM ", metrics.net_wpm), Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled(" Accuracy: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{:.1}% ", metrics.accuracy_pct), Style::default().fg(if metrics.accuracy_pct >= 94.0 { Theme::SUCCESS } else { Theme::WARNING }).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled(" Errors: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{} ", metrics.error_count), Style::default().fg(if metrics.error_count > 0 { Theme::ERROR } else { Color::Green })),
        Span::raw(" | "),
        Span::styled(time_display, Style::default().fg(Color::Yellow)),
    ]);

    let hud = Paragraph::new(hud_line)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title_context)
                .title_alignment(Alignment::Center)
                .border_style(Style::default().fg(Theme::ACCENT)),
        );
    frame.render_widget(hud, chunks[0]);

    // 2. Typing Text Area
    let cursor_idx = session.cursor();
    let statuses = session.char_statuses();
    let mut spans: Vec<Span> = Vec::new();

    for (idx, c) in session.target_text.chars().enumerate() {
        let display_char = if c == '\n' { '↵' } else { c };

        if idx < cursor_idx {
            match statuses[idx] {
                CharStatus::Correct => spans.push(Span::styled(display_char.to_string(), Theme::correct_char())),
                CharStatus::Incorrect => spans.push(Span::styled(display_char.to_string(), Theme::incorrect_char())),
                CharStatus::Pending => spans.push(Span::styled(display_char.to_string(), Theme::pending_char())),
            }
        } else if idx == cursor_idx {
            spans.push(Span::styled(display_char.to_string(), Theme::active_cursor_char()));
        } else {
            spans.push(Span::styled(display_char.to_string(), Theme::pending_char()));
        }
    }

    let prompt_desc = if let Some(ex) = app.current_exercise() {
        ex.prompt.as_str()
    } else {
        "Type continuously. Punctuation and spaces count toward net WPM accuracy."
    };

    let text_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Prompt: {} (Esc to Exit) ", prompt_desc))
        .title_alignment(Alignment::Left)
        .border_style(Style::default().fg(Color::White));

    let text_p = Paragraph::new(Line::from(spans))
        .block(text_block)
        .wrap(ratatui::widgets::Wrap { trim: false });
    frame.render_widget(text_p, chunks[1]);

    // 3. Visual Keyboard Widget
    let active_expected_char = session.current_expected_char();
    let keyboard = KeyboardWidget::new(active_expected_char);
    frame.render_widget(keyboard, chunks[2]);
}

pub fn render_results(app: &App, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Session Scorecard & Evaluation ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Theme::ACCENT));

    let metrics = match app.last_metrics {
        Some(m) => m,
        None => return,
    };

    let min_required = app
        .current_exercise()
        .map(|e| e.minimum_accuracy)
        .unwrap_or(90.0);

    let passed = metrics.accuracy_pct >= min_required;

    let verdict_line = if passed {
        Line::from(vec![
            Span::styled("  ★ EXERCISE PASSED - EXCELLENT CADENCE ★  ", Style::default().fg(Color::Black).bg(Theme::SUCCESS).add_modifier(Modifier::BOLD)),
        ])
    } else {
        Line::from(vec![
            Span::styled(format!("  NEEDS REFINEMENT - REQUIRED ACCURACY: {:.0}%  ", min_required), Style::default().fg(Color::White).bg(Theme::ERROR).add_modifier(Modifier::BOLD)),
        ])
    };

    let content = vec![
        Line::from(""),
        verdict_line,
        Line::from(""),
        Line::from(vec![
            Span::styled("Gross Typing Speed: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.1} WPM", metrics.gross_wpm), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Net Typing Speed:   ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.1} WPM", metrics.net_wpm), Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Accuracy Score:     ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.1}%", metrics.accuracy_pct), Style::default().fg(if passed { Theme::SUCCESS } else { Theme::WARNING }).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Total Keystrokes:   ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", metrics.total_keystrokes), Style::default().fg(Color::White)),
            Span::styled(format!("  (Correct: {}, Errors: {})", metrics.correct_keystrokes, metrics.error_count), Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("Elapsed Time:       ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.1} seconds", metrics.elapsed.as_secs_f64()), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Actions: ", Style::default().fg(Color::Gray)),
            Span::styled("[r] Retry Exercise  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("[n/Enter] Next Step  ", Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD)),
            Span::styled("[m/Esc] Main Menu", Style::default().fg(Color::Cyan)),
        ]),
    ];

    let p = Paragraph::new(content).block(block).alignment(Alignment::Center);
    frame.render_widget(p, frame.area());
}
