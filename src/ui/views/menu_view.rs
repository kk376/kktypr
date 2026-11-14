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
            Constraint::Length(7), // Header / Banner
            Constraint::Min(10),   // Menu options
            Constraint::Length(4), // Footer stats
        ])
        .split(frame.area());

    // Banner
    let banner_text = vec![
        Line::from(vec![
            Span::styled("  ██████╗ ███████╗██████╗ ████████╗██████╗ ", Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  ██╔══██╗╚══███╔╝██╔══██╗╚══██╔══╝██╔══██╗", Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  ██████╔╝  ███╔╝ ██████╔╝   ██║   ██████╔╝", Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  ██╔═══╝  ███╔╝  ██╔═══╝    ██║   ██╔═══╝ ", Style::default().fg(Color::LightCyan)),
        ]),
        Line::from(vec![
            Span::styled("  ██║     ███████╗██║        ██║   ██║     ", Style::default().fg(Color::LightBlue)),
            Span::styled("   Terminal Touch-Typing Master (Clean-Room Rust)", Style::default().fg(Color::Gray)),
        ]),
    ];
    let banner = Paragraph::new(banner_text).alignment(Alignment::Center);
    frame.render_widget(banner, chunks[0]);

    // Menu list
    let menu_items = [
        ("1. Touch Typing Academy", "Master QWERTY from Home Row to Numbers (12 Progressive Lessons)"),
        ("2. Speed Tests", "Timed 1-minute, 2-minute, and 5-minute typing benchmarks"),
        ("3. Weak-Key Diagnostics & Targeted Review", "Heatmap of error-prone keys with dynamic drill synthesizer"),
        ("4. Word Drop Arcade Game", "TUI falling-words typing game with multiplier combos and streaks"),
        ("5. User Profile & Historical Statistics", "View lifetime accuracy, speed curves, and lesson completion status"),
        ("6. Exit", "Quit kktypr"),
    ];

    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(idx, (title, desc))| {
            let is_selected = idx == app.selected_menu_idx;
            let (prefix, style) = if is_selected {
                (
                    " ▶ ",
                    Style::default()
                        .fg(Color::Black)
                        .bg(Theme::ACCENT)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                ("   ", Style::default().fg(Color::White))
            };

            let line = Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(format!("{:<38} ", title), style),
                Span::styled(format!("— {}", desc), Style::default().fg(Color::DarkGray)),
            ]);
            ListItem::new(line)
        })
        .collect();

    let menu_block = Block::default()
        .borders(Borders::ALL)
        .title(" Main Navigation (↑/k: Up, ↓/j: Down, Enter: Select, q: Quit) ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Theme::ACCENT));

    let menu_list = List::new(items).block(menu_block);
    frame.render_widget(menu_list, chunks[1]);

    // Footer stats
    let footer_text = vec![
        Line::from(vec![
            Span::styled("Typist: ", Style::default().fg(Color::Gray)),
            Span::styled(&app.profile.name, Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD)),
            Span::raw("  |  "),
            Span::styled("Best Gross: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.1} WPM", app.profile.best_gross_wpm), Style::default().fg(Theme::SUCCESS)),
            Span::raw("  |  "),
            Span::styled("Best Net: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.1} WPM", app.profile.best_net_wpm), Style::default().fg(Theme::SUCCESS)),
            Span::raw("  |  "),
            Span::styled("Completed Exercises: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", app.profile.completed_exercises.len()), Style::default().fg(Color::Yellow)),
            Span::raw("  |  "),
            Span::styled("Arcade High Score: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", app.profile.game_high_score), Style::default().fg(Color::Magenta)),
        ]),
        Line::from(vec![
            Span::styled("Tip: Touch typing relies on muscle memory. Keep your eyes on the screen, not the keys.", Style::default().fg(Color::DarkGray)),
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
                (" ▶ ", Style::default().fg(Color::Black).bg(Theme::ACCENT).add_modifier(Modifier::BOLD))
            } else {
                ("   ", Style::default().fg(Color::White))
            };

            let line = Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(format!("{:<42} ", course.title), style),
                Span::styled(format!("— {}", course.description), Style::default().fg(Color::Gray)),
            ]);
            ListItem::new(line)
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Select a Course (Enter: Open, Esc: Back) ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Theme::ACCENT));

    frame.render_widget(List::new(items).block(block), chunks[0]);

    let help = Paragraph::new("Press Enter to view lessons in the selected course | Esc to return to main menu")
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
    let items: Vec<ListItem> = course
        .lessons
        .iter()
        .enumerate()
        .map(|(idx, lesson)| {
            let is_selected = idx == app.selected_lesson_idx;
            let (prefix, style) = if is_selected {
                (" ▶ ", Style::default().fg(Color::Black).bg(Theme::ACCENT).add_modifier(Modifier::BOLD))
            } else {
                ("   ", Style::default().fg(Color::White))
            };

            // Exercise indicators: [✓] or [·]
            let mut exercise_tags = Vec::new();
            for (ex_idx, _ex) in lesson.exercises.iter().enumerate() {
                let completed = app.profile.is_exercise_completed(&course.id, lesson.number, ex_idx);
                if completed {
                    exercise_tags.push(Span::styled(" [✓] ", Style::default().fg(Theme::SUCCESS)));
                } else {
                    exercise_tags.push(Span::styled(" [ ] ", Style::default().fg(Color::DarkGray)));
                }
            }

            let mut line_spans = vec![
                Span::styled(prefix, style),
                Span::styled(format!("Lesson {:2}: {:<32} ", lesson.number, lesson.title), style),
                Span::raw("Progress: "),
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

    let help = Paragraph::new("Each lesson contains 4 steps: Warm-Up → Word Drill → Sentence Drill → Mastery Test. Pass with 94%+ accuracy to unlock.")
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
        Line::from(Span::styled("Choose Speed Test Duration:", Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [1] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("1-Minute Sprint       ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled("— Fast evaluation of maximum bursting cadence", Style::default().fg(Color::Gray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [2] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("2-Minute Standard Test", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled("— Balanced benchmark for sustained touch typing speed", Style::default().fg(Color::Gray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [5] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("5-Minute Endurance    ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled("— Comprehensive test measuring rhythm consistency and stamina", Style::default().fg(Color::Gray)),
        ]),
        Line::from(""),
        Line::from(Span::styled("Press [1], [2], or [5] to start test  |  Esc to return to Main Menu", Style::default().fg(Color::DarkGray))),
    ];

    let p = Paragraph::new(content).block(block).alignment(Alignment::Center);
    frame.render_widget(p, frame.area());
}
