use crate::app::App;
use crate::ui::keyboard::KeyboardWidget;
use crate::ui::theme::Theme;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render_targeted_review(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Diagnostics header & weak key list
            Constraint::Length(10), // Keyboard Heatmap
            Constraint::Length(4),  // Actions
        ])
        .split(frame.area());

    let weakest = app.profile.heatmap.weakest_keys(5, 3);

    let mut lines = vec![
        Line::from(Span::styled(
            "Pro Diagnostics: Weak Key Analysis & Remediation",
            Style::default()
                .fg(Theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    if weakest.is_empty() {
        lines.push(Line::from(Span::styled(
            "No significant error patterns recorded yet! Complete several lessons or speed tests to populate the heatmap.",
            Style::default().fg(Color::Gray),
        )));
    } else {
        lines.push(Line::from(vec![Span::styled(
            "Identified Problem Keys: ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]));

        let mut key_spans = Vec::new();
        for (key, acc, errors) in &weakest {
            let color = if *acc < 80.0 {
                Color::Red
            } else {
                Color::Yellow
            };
            key_spans.push(Span::styled(
                format!(
                    "  [{}] {:.1}% ({} errors)  ",
                    key.to_ascii_uppercase(),
                    acc,
                    errors
                ),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ));
        }
        lines.push(Line::from(key_spans));
    }

    let header_p = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Diagnostic Report "),
        )
        .alignment(Alignment::Center);
    frame.render_widget(header_p, chunks[0]);

    // Keyboard Heatmap widget
    let keyboard = KeyboardWidget::new(None).with_heatmap(&app.profile.heatmap);
    frame.render_widget(keyboard, chunks[1]);

    // Action footer
    let actions = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Press [Enter / p] to Generate Custom Targeted Practice Drill  ",
                Style::default()
                    .fg(Theme::SUCCESS)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("|  [Esc / q] Main Menu", Style::default().fg(Color::Gray)),
        ]),
    ];
    let action_p = Paragraph::new(actions).alignment(Alignment::Center);
    frame.render_widget(action_p, chunks[2]);
}

pub fn render_stats(app: &App, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Typist Lifetime Statistics & Profile ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Theme::ACCENT));

    let content = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("User Profile:         ", Style::default().fg(Color::Gray)),
            Span::styled(
                &app.profile.name,
                Style::default()
                    .fg(Theme::ACCENT)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Lifetime Best Gross:  ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.1} WPM", app.profile.best_gross_wpm),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Lifetime Best Net:    ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.1} WPM", app.profile.best_net_wpm),
                Style::default()
                    .fg(Theme::SUCCESS)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Total Sessions Typed: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.profile.total_sessions),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Completed Exercises:  ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.profile.completed_exercises.len()),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled("Word Drop High Score: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.profile.game_high_score),
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Heatmap Color Coding: Green = 95%+ Accuracy | Yellow = 80 to 95% | Red = <80% (Weak Key)",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press [Esc / q / Enter] to return to Main Menu",
            Style::default().fg(Color::Gray),
        )),
    ];

    let p = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Center);
    frame.render_widget(p, frame.area());
}
