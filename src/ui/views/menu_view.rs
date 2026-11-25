use crate::app::App;
use crate::ui::theme::Theme;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

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
        Line::from(vec![Span::styled(
            "K K T Y P R",
            Style::default()
                .fg(Theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "Terminal Touch Typing Academy",
            Style::default().fg(Color::Gray),
        )]),
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
        "3. Weak Key Diagnostics & Targeted Review",
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
            Span::styled(
                &app.profile.name,
                Style::default()
                    .fg(Theme::ACCENT)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("   |   "),
            Span::styled("Best Gross: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.1} WPM", app.profile.best_gross_wpm),
                Style::default()
                    .fg(Theme::SUCCESS)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("   |   "),
            Span::styled("Best Net: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.1} WPM", app.profile.best_net_wpm),
                Style::default()
                    .fg(Theme::SUCCESS)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("   |   "),
            Span::styled("Completed: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{} drills", app.profile.completed_exercises.len()),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw("   |   "),
            Span::styled("Arcade Record: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.profile.game_high_score),
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Accuracy Mode: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!(
                    "{} ({:.0}% Min)",
                    app.difficulty_mode().label(),
                    app.required_accuracy()
                ),
                Style::default()
                    .fg(Theme::ACCENT)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "   [d] Switch Mode   •   Focus on accuracy first; speed follows rhythm.",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
    ];
    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    frame.render_widget(footer, chunks[2]);
}

pub fn render_course_select(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(8),
            Constraint::Length(4),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let items: Vec<ListItem> = app
        .courses
        .iter()
        .enumerate()
        .map(|(idx, course)| {
            let is_selected = idx == app.selected_course_idx;
            let (prefix, style) = if is_selected {
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

    let mut mode_spans = vec![Span::styled(
        "  Accuracy Mode:  ",
        Style::default()
            .fg(Color::Gray)
            .add_modifier(Modifier::BOLD),
    )];
    for mode in crate::engine::curriculum::DifficultyMode::ALL {
        let active = mode == app.difficulty_mode();
        let pill = format!(" {} ({:.0}%) ", mode.label(), mode.min_accuracy());
        let style = if active {
            Style::default()
                .fg(Color::Black)
                .bg(Theme::ACCENT)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        mode_spans.push(Span::styled(pill, style));
        mode_spans.push(Span::raw("  "));
    }

    let mode_box = Paragraph::new(Line::from(mode_spans))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Accuracy Criteria Mode ([d / ← / →] Switch) ")
                .title_alignment(Alignment::Center)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    frame.render_widget(mode_box, chunks[1]);

    let help = Paragraph::new(
        "Press [Enter] to view curriculum  |  [d/←/→] Change Accuracy Mode  |  [Esc] Main Menu",
    )
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, chunks[2]);
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

            let mut exercise_tags = Vec::new();
            for (ex_idx, _ex) in lesson.exercises.iter().enumerate() {
                let completed =
                    app.profile
                        .is_exercise_completed(&course.id, lesson.number, ex_idx);
                let is_step_selected = is_selected && ex_idx == app.selected_exercise_idx;

                if is_step_selected {
                    if completed {
                        exercise_tags.push(Span::styled(
                            "[✓] ",
                            Style::default()
                                .fg(Color::Black)
                                .bg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ));
                    } else {
                        exercise_tags.push(Span::styled(
                            "[▶] ",
                            Style::default()
                                .fg(Color::Black)
                                .bg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ));
                    }
                } else if completed {
                    exercise_tags.push(Span::styled(
                        "[✓] ",
                        Style::default()
                            .fg(Theme::SUCCESS)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    exercise_tags.push(Span::styled("[ ] ", Style::default().fg(Color::DarkGray)));
                }
            }

            let formatted_title = format!(
                "Lesson {:2}: {:<width$}  ",
                lesson.number,
                lesson.title,
                width = max_title_len
            );
            let mut line_spans = vec![
                Span::styled(prefix, style),
                Span::styled(formatted_title, style),
                Span::styled("Steps: ", Style::default().fg(Color::DarkGray)),
            ];
            line_spans.extend(exercise_tags);

            ListItem::new(Line::from(line_spans))
        })
        .collect();

    let step_display_num = app.selected_exercise_idx + 1;
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(
            " {} (Enter: Start Step {}, ←/→ or 1 to 4: Select Step, Esc: Back) ",
            course.title, step_display_num
        ))
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Theme::ACCENT));

    frame.render_widget(List::new(items).block(block), chunks[0]);

    let mode = app.difficulty_mode();
    let active_step_detail = course
        .lessons
        .get(app.selected_lesson_idx)
        .and_then(|l| l.exercises.get(app.selected_exercise_idx))
        .map(|ex| {
            format!(
                "▶ Step {}: {} • {}  (Mode: {} • Min Accuracy: {:.0}%)",
                step_display_num,
                ex.exercise_type.display_name(),
                ex.title,
                mode.label(),
                mode.min_accuracy(),
            )
        })
        .unwrap_or_else(|| {
            "Steps: Warm Up → Word Drill → Sentence Drill → Mastery Test".to_string()
        });

    let help_lines = vec![
        Line::from(vec![Span::styled(
            active_step_detail,
            Style::default()
                .fg(Theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "Navigation: [↑/↓] Lesson  |  [←/→] or [1-4] Step  |  [d/m] Mode  |  [Enter] Start  |  [Esc] Back",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    let help = Paragraph::new(help_lines).alignment(Alignment::Center);
    frame.render_widget(help, chunks[1]);
}

pub fn render_speed_test_select(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(3)])
        .split(frame.area());

    struct SpeedOption {
        key_label: &'static str,
        name: &'static str,
        duration_label: &'static str,
        desc: &'static str,
    }

    let options = [
        SpeedOption {
            key_label: "1",
            name: "1 Minute Sprint",
            duration_label: "60s",
            desc: "Rapid cadence burst • Great for quick daily calibration",
        },
        SpeedOption {
            key_label: "2",
            name: "2 Minute Standard Benchmark",
            duration_label: "120s",
            desc: "Standard typing examination • Balances speed and continuous focus",
        },
        SpeedOption {
            key_label: "5",
            name: "5 Minute Endurance Assessment",
            duration_label: "300s",
            desc: "Full sustained performance audit • High focus cognitive stamina",
        },
    ];

    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(idx, opt)| {
            let is_selected = idx == app.selected_speed_test_idx;
            if is_selected {
                let line = Line::from(vec![
                    Span::styled(
                        "  ▶  ",
                        Style::default()
                            .fg(Color::Black)
                            .bg(Theme::ACCENT)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("[{}] ", opt.key_label),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Theme::ACCENT)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{:<32}", opt.name),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Theme::ACCENT)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("({:<4})  ", opt.duration_label),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Theme::ACCENT)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{} ", opt.desc),
                        Style::default().fg(Color::Black).bg(Theme::ACCENT),
                    ),
                ]);
                ListItem::new(line)
            } else {
                let line = Line::from(vec![
                    Span::raw("     "),
                    Span::styled(
                        format!("[{}] ", opt.key_label),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{:<32}", opt.name),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("({:<4})  ", opt.duration_label),
                        Style::default().fg(Color::Cyan),
                    ),
                    Span::styled(
                        format!("{} ", opt.desc),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]);
                ListItem::new(line)
            }
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Speed Test Selection (Enter: Start, ↑/k / ↓/j: Navigate, Esc/q: Back) ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Theme::ACCENT));

    frame.render_widget(List::new(items).block(block), chunks[0]);

    let help = Paragraph::new("Navigation: [↑/k] Up  |  [↓/j] Down  |  [Enter/l] Start Speed Test  |  [1/2/5] Direct  |  [Esc/q/h] Back")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, chunks[1]);
}
