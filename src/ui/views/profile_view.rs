use crate::app::App;
use crate::ui::theme::Theme;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render_profile_manager(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Header
            Constraint::Min(8),    // Profile list
            Constraint::Length(4), // Action bar / shortcuts
        ])
        .split(frame.area());

    // 1. Header
    let header_lines = vec![
        Line::from(vec![Span::styled(
            "TYPIST PROFILE MANAGER",
            Style::default()
                .fg(Theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "Switch between typists, create new training profiles, or rename existing accounts.",
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

    // 2. Profile List
    let items: Vec<ListItem> = app
        .profile_store
        .profiles
        .iter()
        .enumerate()
        .map(|(idx, profile)| {
            let is_cursor_selected = idx == app.selected_profile_idx;
            let is_active = profile.name.eq_ignore_ascii_case(&app.profile_store.active_name);

            let status_badge = if is_active {
                Span::styled(" [ACTIVE] ", Style::default().fg(Color::Black).bg(Theme::SUCCESS).add_modifier(Modifier::BOLD))
            } else {
                Span::styled(" [      ] ", Style::default().fg(Color::DarkGray))
            };

            let (prefix, row_style) = if is_cursor_selected {
                (
                    "  ▶ ",
                    Style::default()
                        .fg(Color::Black)
                        .bg(Theme::ACCENT)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                ("    ", Style::default().fg(Color::White))
            };

            let name_str = format!("{:<20}", profile.name);
            let stats_str = format!(
                "Avg Net: {:>5.1} WPM  •  Avg Gross: {:>5.1} WPM  •  Completed: {:>2} drills  •  Arcade: {:>4}",
                profile.avg_net_wpm(),
                profile.avg_gross_wpm(),
                profile.completed_exercises.len(),
                profile.game_high_score,
            );

            let line = Line::from(vec![
                Span::styled(prefix, row_style),
                status_badge,
                Span::styled(format!("  {}  ", name_str), row_style),
                Span::styled(stats_str, if is_cursor_selected { row_style } else { Style::default().fg(Color::Gray) }),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list_title = if let Some(ref note) = app.status_notification {
        format!(" ✓ {} ", note)
    } else {
        format!(
            " Profiles ({} Registered) ",
            app.profile_store.profiles.len()
        )
    };

    let list_block = Block::default()
        .borders(Borders::ALL)
        .title(list_title)
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::DarkGray));

    let list_widget = List::new(items).block(list_block);
    frame.render_widget(list_widget, chunks[1]);

    // 3. Actions / Footer
    let actions_lines = vec![Line::from(vec![
        Span::styled(
            "[Enter] ",
            Style::default()
                .fg(Theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Switch Profile   |   "),
        Span::styled(
            "[a] ",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Add New   |   "),
        Span::styled(
            "[r] ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Rename   |   "),
        Span::styled(
            "[d] ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::raw("Delete   |   "),
        Span::styled("[Esc/q] ", Style::default().fg(Color::Cyan)),
        Span::raw("Main Menu"),
    ])];
    let footer = Paragraph::new(actions_lines)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    frame.render_widget(footer, chunks[2]);
}
