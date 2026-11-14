use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use crate::app::App;
use crate::engine::session::CharStatus;
use crate::ui::keyboard::{get_finger_for_char, KeyboardWidget};
use crate::ui::theme::Theme;

#[derive(Debug, Clone)]
struct TargetWord<'a> {
    text: &'a str,
    char_start: usize,
    char_end: usize, // exclusive
    has_trailing_space: bool,
}

fn extract_words(text: &str) -> Vec<TargetWord<'_>> {
    let mut words = Vec::new();
    let mut current_start = None;
    let mut char_idx = 0;

    for (byte_idx, c) in text.char_indices() {
        if c.is_whitespace() {
            if let Some(start) = current_start {
                let word_str = &text[start..byte_idx];
                let char_len = word_str.chars().count();
                words.push(TargetWord {
                    text: word_str,
                    char_start: char_idx - char_len,
                    char_end: char_idx,
                    has_trailing_space: true,
                });
                current_start = None;
            }
        } else if current_start.is_none() {
            current_start = Some(byte_idx);
        }
        char_idx += 1;
    }

    if let Some(start) = current_start {
        let word_str = &text[start..];
        let char_len = word_str.chars().count();
        words.push(TargetWord {
            text: word_str,
            char_start: char_idx - char_len,
            char_end: char_idx,
            has_trailing_space: false,
        });
    }

    words
}

pub fn render_typing_active(app: &App, frame: &mut Frame) {
    if app.active_session.is_none() {
        return;
    }

    let total_area = frame.area();
    let use_two_columns = total_area.width >= 78;

    if use_two_columns {
        let main_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(74), // Left: Typing Stage & Keyboard
                Constraint::Percentage(26), // Right: TypingMaster-style Progress Sidebar
            ])
            .split(total_area);

        render_main_typing_stage(app, frame, main_columns[0]);
        render_progress_sidebar(app, frame, main_columns[1]);
    } else {
        // Fallback for compact terminal windows
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(12),
                Constraint::Length(10),
            ])
            .split(total_area);
        render_main_typing_stage(app, frame, rows[0]);
    }
}

fn render_main_typing_stage(app: &App, frame: &mut Frame, area: Rect) {
    let session = match app.active_session.as_ref() {
        Some(s) => s,
        None => return,
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Instruction header
            Constraint::Length(9), // Large, structured Typing Stage (TypingMaster style)
            Constraint::Length(10), // Centered interactive keyboard
            Constraint::Min(3),    // Finger Guide Pill
        ])
        .split(area);

    // 1. Instruction prompt banner
    let prompt_desc = if let Some(ex) = app.current_exercise() {
        format!("▶ {}  |  {}", ex.title, ex.prompt)
    } else {
        "▶ Type each highlighted word and press space. Maintain steady rhythm.".to_string()
    };

    let instruction_p = Paragraph::new(Line::from(vec![
        Span::styled("● ", Style::default().fg(Theme::SUCCESS)),
        Span::styled(prompt_desc, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
    ]))
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    frame.render_widget(instruction_p, chunks[0]);

    // 2. The Structured Typing Stage
    let words = extract_words(&session.target_text);
    let cursor = session.cursor();
    let statuses = session.char_statuses();

    // Determine which word is currently active
    let mut active_word_idx = 0;
    for (idx, w) in words.iter().enumerate() {
        let span_end = if w.has_trailing_space { w.char_end + 1 } else { w.char_end };
        if cursor >= w.char_start && cursor < span_end {
            active_word_idx = idx;
            break;
        } else if cursor >= span_end {
            active_word_idx = idx + 1;
        }
    }
    active_word_idx = active_word_idx.min(words.len().saturating_sub(1));

    // Build the Active Display Lines
    let mut stage_lines = Vec::new();
    stage_lines.push(Line::from("")); // Top breathing room

    // Line 1: Previous completed words (dimmed green)
    if active_word_idx > 0 {
        let prev_start = active_word_idx.saturating_sub(3);
        let mut prev_spans = Vec::new();
        prev_spans.push(Span::raw("    "));
        for w in &words[prev_start..active_word_idx] {
            prev_spans.push(Span::styled(format!("✓ {}      ", w.text), Style::default().fg(Color::DarkGray)));
        }
        stage_lines.push(Line::from(prev_spans));
    } else {
        stage_lines.push(Line::from(Span::styled("    [ Beginning of Exercise ]", Style::default().fg(Color::DarkGray))));
    }

    stage_lines.push(Line::from(""));

    // Line 2: The Main Stage Line (Large spacing, active word with pointer)
    let mut active_line_spans = Vec::new();
    active_line_spans.push(Span::raw("  "));

    // Render the active word with precise character formatting
    if let Some(active_w) = words.get(active_word_idx) {
        active_line_spans.push(Span::styled("▶ ", Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD)));

        let is_at_trailing_space = cursor >= active_w.char_end && active_w.has_trailing_space;

        for (char_offset, c) in active_w.text.chars().enumerate() {
            let global_char_idx = active_w.char_start + char_offset;

            if global_char_idx < cursor {
                match statuses.get(global_char_idx).copied().unwrap_or(CharStatus::Pending) {
                    CharStatus::Correct => {
                        active_line_spans.push(Span::styled(c.to_string(), Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD)));
                    }
                    CharStatus::Incorrect => {
                        active_line_spans.push(Span::styled(c.to_string(), Style::default().fg(Color::White).bg(Theme::ERROR).add_modifier(Modifier::BOLD)));
                    }
                    CharStatus::Pending => {
                        active_line_spans.push(Span::styled(c.to_string(), Style::default().fg(Color::White)));
                    }
                }
            } else if global_char_idx == cursor {
                // Inverted active cursor block
                active_line_spans.push(Span::styled(
                    c.to_string(),
                    Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                ));
            } else {
                active_line_spans.push(Span::styled(c.to_string(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));
            }
        }

        // Highlight trailing space prompt if waiting for space
        if is_at_trailing_space {
            active_line_spans.push(Span::styled(" ␣", Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)));
        }

        active_line_spans.push(Span::raw("        ")); // 8 spaces between words

        // Next 2 words on the active line
        let next_start = active_word_idx + 1;
        let next_end = (next_start + 3).min(words.len());
        for w in &words[next_start..next_end] {
            active_line_spans.push(Span::styled(format!("{}        ", w.text), Style::default().fg(Color::LightCyan)));
        }
    }

    stage_lines.push(Line::from(active_line_spans));
    stage_lines.push(Line::from(""));

    // Line 3: Lookahead preview row
    let lookahead_start = (active_word_idx + 4).min(words.len());
    let lookahead_end = (lookahead_start + 4).min(words.len());
    if lookahead_start < words.len() {
        let mut lookahead_spans = Vec::new();
        lookahead_spans.push(Span::raw("    "));
        for w in &words[lookahead_start..lookahead_end] {
            lookahead_spans.push(Span::styled(format!("{}        ", w.text), Style::default().fg(Color::DarkGray)));
        }
        stage_lines.push(Line::from(lookahead_spans));
    }

    let stage_block = Block::default()
        .borders(Borders::ALL)
        .title(" Typing Stage ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Theme::ACCENT));

    let stage_p = Paragraph::new(stage_lines).block(stage_block);
    frame.render_widget(stage_p, chunks[1]);

    // 3. Interactive Keyboard Widget
    let active_expected_char = session.current_expected_char();
    let keyboard = KeyboardWidget::new(active_expected_char);
    frame.render_widget(keyboard, chunks[2]);

    // 4. Finger Guide Pill Bar
    if let Some(target) = active_expected_char {
        let (finger_name, finger_color) = if let Some(finger) = get_finger_for_char(target) {
            (finger.name(), finger.color())
        } else {
            ("Strike Key", Color::White)
        };

        let target_disp = if target == ' ' { "SPACEBAR".to_string() } else { target.to_ascii_uppercase().to_string() };

        let guide_spans = vec![
            Span::raw("  FINGER PLACEMENT:  "),
            Span::styled(format!(" {} ", finger_name), Style::default().fg(Color::Black).bg(finger_color).add_modifier(Modifier::BOLD)),
            Span::raw("   ──►   Target Key: "),
            Span::styled(format!(" [{}] ", target_disp), Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD)),
        ];

        let guide_p = Paragraph::new(Line::from(guide_spans))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
        frame.render_widget(guide_p, chunks[3]);
    }
}

fn render_progress_sidebar(app: &App, frame: &mut Frame, area: Rect) {
    let session = match app.active_session.as_ref() {
        Some(s) => s,
        None => return,
    };

    let metrics = session.current_metrics();
    let cursor = session.cursor();
    let total_chars = session.target_text.chars().count().max(1);
    let progress_ratio = (cursor as f64 / total_chars as f64).clamp(0.0, 1.0);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Progress gauge
            Constraint::Length(4), // Big digital clock
            Constraint::Length(8), // Speed and accuracy metrics
            Constraint::Length(4), // Goal badge
            Constraint::Min(2),    // Shortcut
        ])
        .split(area);

    // 1. Progress Gauge
    let gauge_title = format!("Progress: {:.0}% ({}/{})", progress_ratio * 100.0, cursor, total_chars);
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(gauge_title).title_alignment(Alignment::Center))
        .gauge_style(Style::default().fg(Theme::SUCCESS).bg(Color::DarkGray))
        .ratio(progress_ratio);
    frame.render_widget(gauge, chunks[0]);

    // 2. Timer / Clock
    let (time_title, time_val) = if let Some(remaining) = session.remaining_time() {
        let s = remaining.as_secs();
        ("Time Remaining", format!("{:02}:{:02}", s / 60, s % 60))
    } else {
        let s = session.elapsed().as_secs();
        ("Elapsed Time", format!("{:02}:{:02}", s / 60, s % 60))
    };

    let clock_lines = vec![
        Line::from(Span::styled(time_title, Style::default().fg(Color::DarkGray))),
        Line::from(Span::styled(time_val, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
    ];
    let clock_p = Paragraph::new(clock_lines)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
    frame.render_widget(clock_p, chunks[1]);

    // 3. Metrics card
    let metrics_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Net Speed: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.1} WPM", metrics.net_wpm), Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Gross:     ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.1} WPM", metrics.gross_wpm), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("Accuracy:  ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.1}%", metrics.accuracy_pct), Style::default().fg(if metrics.accuracy_pct >= 94.0 { Theme::SUCCESS } else { Theme::WARNING }).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Errors:    ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", metrics.error_count), Style::default().fg(if metrics.error_count > 0 { Theme::ERROR } else { Color::Green })),
        ]),
    ];
    let metrics_p = Paragraph::new(metrics_lines)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(" Speed & Accuracy ").title_alignment(Alignment::Center));
    frame.render_widget(metrics_p, chunks[2]);

    // 4. Target Goal Card
    let min_required = app.current_exercise().map(|e| e.minimum_accuracy).unwrap_or(94.0);
    let on_track = metrics.accuracy_pct >= min_required;
    let goal_lines = vec![
        Line::from(vec![
            Span::styled("Target: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.0}% Min", min_required), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(if on_track {
            Span::styled(" [ ON TRACK ] ", Style::default().fg(Color::Black).bg(Theme::SUCCESS).add_modifier(Modifier::BOLD))
        } else {
            Span::styled(" [ FOCUS ACCURACY ] ", Style::default().fg(Color::Black).bg(Theme::WARNING).add_modifier(Modifier::BOLD))
        }),
    ];
    let goal_p = Paragraph::new(goal_lines)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(" Lesson Goal ").title_alignment(Alignment::Center));
    frame.render_widget(goal_p, chunks[3]);

    // 5. Shortcut
    let shortcut_p = Paragraph::new("[Esc] Cancel & Exit")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(shortcut_p, chunks[4]);
}

pub fn render_results(app: &App, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Exercise Scorecard & Evaluation ")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_words_structure() {
        let text = "asdf jkl; fads";
        let words = extract_words(text);
        assert_eq!(words.len(), 3);
        assert_eq!(words[0].text, "asdf");
        assert_eq!(words[0].char_start, 0);
        assert_eq!(words[0].char_end, 4);
        assert!(words[0].has_trailing_space);

        assert_eq!(words[1].text, "jkl;");
        assert_eq!(words[1].char_start, 5);
        assert_eq!(words[1].char_end, 9);
        assert!(words[1].has_trailing_space);

        assert_eq!(words[2].text, "fads");
        assert_eq!(words[2].char_start, 10);
        assert_eq!(words[2].char_end, 14);
        assert!(!words[2].has_trailing_space);
    }
}
