use crate::analytics::targeted::TargetedCourse;
use crate::app::App;
use crate::ui::keyboard::KeyboardWidget;
use crate::ui::theme::Theme;
use crate::ui::views::menu_view::render_mode_selector_bar;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render_targeted_review(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Course selector (Basics vs Developer)
            Constraint::Length(5), // Diagnostics header & weak key list
            Constraint::Min(8),    // Course-specific Keyboard Heatmap
            Constraint::Length(3), // Difficulty Mode bar
            Constraint::Length(2), // Actions
        ])
        .split(frame.area());

    let course_items: Vec<ListItem> = TargetedCourse::ALL
        .iter()
        .enumerate()
        .map(|(idx, course)| {
            let selected = idx == app.selected_targeted_course_idx;
            let (prefix, style) = if selected {
                (
                    "  ▶  ",
                    Style::default()
                        .fg(Color::Black)
                        .bg(Theme::ACCENT)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                ("     ", Style::default().fg(Color::White))
            };
            ListItem::new(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(format!("[{}] {:<50}", idx + 1, course.title()), style),
            ]))
        })
        .collect();

    let course_list = List::new(course_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Weak Key Diagnostics Scope ([↑/↓] or [1/2] Course) ")
            .title_alignment(Alignment::Center)
            .border_style(Style::default().fg(Theme::ACCENT)),
    );
    frame.render_widget(course_list, chunks[0]);

    let active_course = app.targeted_course();
    let course_heatmap = app.profile.heatmap_for(active_course);
    let weakest = course_heatmap.weakest_keys(5, 3);

    let mut lines = vec![Line::from(Span::styled(
        format!("Diagnostics: {}", active_course.title()),
        Style::default()
            .fg(Theme::ACCENT)
            .add_modifier(Modifier::BOLD),
    ))];

    if weakest.is_empty() {
        lines.push(Line::from(Span::styled(
            "No weak keys recorded for this course yet! Practice lessons here to populate diagnostics.",
            Style::default().fg(Color::Gray),
        )));
    } else {
        let mut key_spans = vec![Span::styled(
            "Problem Keys: ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )];
        for (key, acc, errors) in &weakest {
            let color = if *acc < 80.0 {
                Color::Red
            } else {
                Color::Yellow
            };
            key_spans.push(Span::styled(
                format!(
                    "  [{}] {:.1}% ({} err)  ",
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
    frame.render_widget(header_p, chunks[1]);

    // Isolated Keyboard Heatmap widget for the selected course
    let keyboard = KeyboardWidget::new(None).with_heatmap(course_heatmap);
    frame.render_widget(keyboard, chunks[2]);

    render_mode_selector_bar(app.difficulty_mode(), frame, chunks[3]);

    // Action footer
    let actions = vec![Line::from(vec![
        Span::styled(
            "[Enter / p] Start Targeted Drill  ",
            Style::default()
                .fg(Theme::SUCCESS)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "|  [↑/↓ / 1-2] Course  |  [d/←/→] Mode  |  [Esc / q] Main Menu",
            Style::default().fg(Color::Gray),
        ),
    ])];
    let action_p = Paragraph::new(actions).alignment(Alignment::Center);
    frame.render_widget(action_p, chunks[4]);
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
            Span::styled("Average Gross Speed:  ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.1} WPM", app.profile.avg_gross_wpm()),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Average Net Speed:    ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.1} WPM", app.profile.avg_net_wpm()),
                Style::default()
                    .fg(Theme::SUCCESS)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Accuracy Mode:        ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!(
                    "{} ({:.0}% Min)",
                    app.profile.difficulty_mode.label(),
                    app.profile.difficulty_mode.min_accuracy()
                ),
                Style::default()
                    .fg(Theme::ACCENT)
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
