use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::App;
use crate::ui::theme::Theme;

pub fn render_main_menu(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Crisp header box
            Constraint::Min(8),    // Clean menu items
            Constraint::Length(4), // Footer stats
        ])
        .split(frame.area());

    // Clean, modern header box (no font-glitching double-width blocks)
    let header_lines = vec![
        Line::from(vec![
            Span::styled("K K T Y P R", Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Terminal Touch-Typing Academy  •  Rust 2024 Edition", Style::default().fg(Color::Gray)),
        ]),
    ];
    let header = Paragraph::new(header_lines)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Theme::ACCENT)),
        );
    frame.render_widget(header, chunks[0]);

    // Clean menu items (Zero annoying descriptions)
    let menu_items = [
        "1. Touch Typing Academy (12 Lessons)",
        "2. Speed Tests (1m, 2m, 5m)",
        "3. Weak-Key Diagnostics & Targeted Review",
        "4. Word Drop Arcade Game",
        "5. Typist Profile & Heatmap",
        "6. Typist Profile Manager (Switch, Add, Rename)",
        "7. Reset All Statistics & Progress",
        "8. Exit",
    ];

    let mut items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(idx, title)| {
            let is_selected = idx == app.selected_menu_idx;
            let (prefix, style) = if is_selected {
                let bg_color = if idx == 6 { Color::Red } else { Theme::ACCENT };
                (
                    "  ▶  ",
                    Style::default()
                        .fg(Color::Black)
                        .bg(bg_color)
                        .add_modifier(Modifier::BOLD),
                )
            } else if idx == 6 {
                ("     ", Style::default().fg(Color::LightRed))
            } else {
                ("     ", Style::default().fg(Color::White))
            };

            let line = Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(format!("{:<48}", title), style),
            ]);
            ListItem::new(line)
        })
        .collect();

    if app.confirm_reset_stats {
        items.push(ListItem::new(Line::from("")));
        items.push(ListItem::new(Line::from(vec![
            Span::styled(
                "  ⚠️  WARNING: Press [y] to confirm wiping all progress & stats, or [n/Esc] to cancel.  ",
                Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ])));
    }

    let menu_title = if app.confirm_reset_stats {
        " ⚠️  RESET STATS CONFIRMATION: Press [y] to Wipe, [n/Esc] to Cancel  ⚠️ ".to_string()
    } else if let Some(ref note) = app.status_notification {
        format!(" ✓ {} ", note)
    } else {
        " Navigation (↑/k: Up, ↓/j: Down, Enter: Select, r: Reset Stats, q: Quit) ".to_string()
    };

    let border_color = if app.confirm_reset_stats {
        Color::Red
    } else if app.status_notification.is_some() {
        Theme::SUCCESS
    } else {
        Color::DarkGray
    };

    let menu_block = Block::default()
        .borders(Borders::ALL)
        .title(menu_title)
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(border_color));

    let menu_list = List::new(items).block(menu_block);
    frame.render_widget(menu_list, chunks[1]);

    // Footer stats
    let footer_text = vec![
        Line::from(vec![
            Span::styled("Typist: ", Style::default().fg(Color::Gray)),
            Span::styled(&app.profile.name, Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD)),
            Span::raw("   |   "),
            Span::styled("Best Gross: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.1} WPM", app.profile.best_gross_wpm), Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD)),
            Span::raw("   |   "),
            Span::styled("Best Net: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.1} WPM", app.profile.best_net_wpm), Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD)),
            Span::raw("   |   "),
            Span::styled("Completed: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{} drills", app.profile.completed_exercises.len()), Style::default().fg(Color::Yellow)),
            Span::raw("   |   "),
            Span::styled("Arcade Record: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", app.profile.game_high_score), Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Focus on accuracy first. Speed emerges naturally from effortless rhythm.", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP).border_style(Style::default().fg(Color::DarkGray)));
    frame.render_widget(footer, chunks[2]);
}

pub fn render_course_select(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(3)])
        .split(frame.area());

    let items: Vec<ListItem> = app
        .courses
        .iter()
        .enumerate()
        .map(|(idx, course)| {
            let is_selected = idx == app.selected_course_idx;
            let (prefix, style) = if is_selected {
                ("  ▶  ", Style::default().fg(Color::Black).bg(Theme::ACCENT).add_modifier(Modifier::BOLD))
            } else {
                ("     ", Style::default().fg(Color::White))
            };

            let line = Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(format!("{:<46}", course.title), style),
            ]);
            ListItem::new(line)
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Select Course (Enter: Open, Esc: Back) ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Theme::ACCENT));

    frame.render_widget(List::new(items).block(block), chunks[0]);

    let help = Paragraph::new("Press Enter to view course curriculum  |  Esc to return to Main Menu")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, chunks[1]);
}

pub fn render_lesson_select(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(3)])
        .split(frame.area());

    let course = app.current_course();
    let max_title_len = course
        .lessons
        .iter()
        .map(|l| l.title.chars().count())
        .max()
        .unwrap_or(44)
        .max(46);

    let items: Vec<ListItem> = course
        .lessons
        .iter()
        .enumerate()
        .map(|(idx, lesson)| {
            let is_selected = idx == app.selected_lesson_idx;
            let (prefix, style) = if is_selected {
                ("  ▶  ", Style::default().fg(Color::Black).bg(Theme::ACCENT).add_modifier(Modifier::BOLD))
            } else {
                ("     ", Style::default().fg(Color::White))
            };

            let mut exercise_tags = Vec::new();
            for (ex_idx, _ex) in lesson.exercises.iter().enumerate() {
                let completed = app.profile.is_exercise_completed(&course.id, lesson.number, ex_idx);
                if completed {
                    exercise_tags.push(Span::styled("[✓] ", Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD)));
                } else {
                    exercise_tags.push(Span::styled("[ ] ", Style::default().fg(Color::DarkGray)));
                }
            }

            let formatted_title = format!("Lesson {:2}: {:<width$}  ", lesson.number, lesson.title, width = max_title_len);
            let mut line_spans = vec![
                Span::styled(prefix, style),
                Span::styled(formatted_title, style),
                Span::styled("Steps: ", Style::default().fg(Color::DarkGray)),
            ];
            line_spans.extend(exercise_tags);

            ListItem::new(Line::from(line_spans))
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} (Enter: Start Lesson, Esc: Back) ", course.title))
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Theme::ACCENT));

    frame.render_widget(List::new(items).block(block), chunks[0]);

    let help = Paragraph::new("Steps: Warm-Up → Word Drill → Sentence Drill → Mastery Test  |  Pass with 94%+ to certify")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, chunks[1]);
}

pub fn render_speed_test_select(_app: &App, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Speed Test Selection ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Theme::ACCENT));

    let content = vec![
        Line::from(""),
        Line::from(Span::styled("Select Test Duration:", Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [1] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("1-Minute Sprint", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [2] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("2-Minute Standard Benchmark", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [5] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("5-Minute Endurance Assessment", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(Span::styled("Press [1], [2], or [5] to begin  |  Esc to return to Main Menu", Style::default().fg(Color::DarkGray))),
    ];

    let p = Paragraph::new(content).block(block).alignment(Alignment::Center);
    frame.render_widget(p, frame.area());
}
