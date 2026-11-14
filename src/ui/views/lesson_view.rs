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

const WORDS_PER_LINE: usize = 4;
const LINES_PER_BLOCK: usize = 2;
const WORDS_PER_BLOCK: usize = WORDS_PER_LINE * LINES_PER_BLOCK; // 8 words per block

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

fn format_spaced(text: &str) -> String {
    let mut out = String::with_capacity(text.len() * 3);
    for (i, c) in text.chars().enumerate() {
        if i > 0 {
            out.push_str("  ");
        }
        out.push(c);
    }
    out
}

fn render_word_slot(
    word: &TargetWord<'_>,
    word_idx: usize,
    active_word_idx: usize,
    cursor: usize,
    statuses: &[CharStatus],
    slot_width: usize,
) -> Vec<Span<'static>> {
    let mut spans = Vec::new();

    if word_idx < active_word_idx {
        // Completed word in this block
        spans.push(Span::styled("✓ ", Style::default().fg(Theme::SUCCESS)));
        let word_display = format_spaced(word.text);
        let formatted = format!("{:<width$}", word_display, width = slot_width.saturating_sub(2));
        spans.push(Span::styled(formatted, Style::default().fg(Color::DarkGray)));
    } else if word_idx == active_word_idx {
        // Active word: stationary slot with prominent cursor tracking
        spans.push(Span::styled("▶ ", Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD)));
        let mut rendered_chars_count = 2; // for "▶ "

        let is_at_trailing_space = cursor >= word.char_end && word.has_trailing_space;
        let char_count = word.text.chars().count();

        for (offset, c) in word.text.chars().enumerate() {
            let global_idx = word.char_start + offset;
            let is_last = offset + 1 == char_count;
            let trailing = if is_last { "" } else { "  " };

            if global_idx < cursor {
                match statuses.get(global_idx).copied().unwrap_or(CharStatus::Pending) {
                    CharStatus::Correct => {
                        spans.push(Span::styled(
                            format!("{}{}", c, trailing),
                            Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD),
                        ));
                    }
                    CharStatus::Incorrect => {
                        spans.push(Span::styled(
                            format!("{}{}", c, trailing),
                            Style::default().fg(Color::White).bg(Theme::ERROR).add_modifier(Modifier::BOLD),
                        ));
                    }
                    CharStatus::Pending => {
                        spans.push(Span::styled(format!("{}{}", c, trailing), Style::default().fg(Color::White)));
                    }
                }
            } else if global_idx == cursor {
                // Inverted highlighted cursor block
                spans.push(Span::styled(
                    format!("{}", c),
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                ));
                if !is_last {
                    spans.push(Span::raw("  "));
                }
            } else {
                spans.push(Span::styled(
                    format!("{}{}", c, trailing),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ));
            }
            rendered_chars_count += 1 + trailing.len();
        }

        // Highlight trailing space indicator if waiting for spacebar
        if is_at_trailing_space {
            spans.push(Span::styled(
                " [␣ SPACE]",
                Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD),
            ));
            rendered_chars_count += 10;
        }

        // Pad slot with spaces to maintain fixed column alignment for subsequent words
        let pad_len = slot_width.saturating_sub(rendered_chars_count);
        if pad_len > 0 {
            spans.push(Span::raw(" ".repeat(pad_len)));
        }
    } else {
        // Upcoming word in this block
        spans.push(Span::raw("  "));
        let word_display = format_spaced(word.text);
        let formatted = format!("{:<width$}", word_display, width = slot_width.saturating_sub(2));
        spans.push(Span::styled(formatted, Style::default().fg(Color::Rgb(140, 160, 190))));
    }

    spans
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
            Constraint::Length(3),  // Instruction header
            Constraint::Length(9),  // Clean, focused Block Typing Stage
            Constraint::Min(16),    // Centered interactive boxed keyboard
            Constraint::Length(3),  // Finger Guide Pill
        ])
        .split(area);

    // 1. Instruction prompt banner
    let prompt_desc = if let Some(ex) = app.current_exercise() {
        format!("▶ {} • {}  |  {}", ex.exercise_type.display_name(), ex.title, ex.prompt)
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

    // 2. The Structured Block Typing Stage (Fixed positions, no treadmill shifting)
    let words = extract_words(&session.target_text);
    let cursor = session.cursor();
    let statuses = session.char_statuses();
    let total_words = words.len().max(1);

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
    active_word_idx = active_word_idx.min(total_words.saturating_sub(1));

    // Page / Block calculations
    let current_block_idx = active_word_idx / WORDS_PER_BLOCK;
    let total_blocks = total_words.div_ceil(WORDS_PER_BLOCK);
    let block_start = current_block_idx * WORDS_PER_BLOCK;
    let block_end = (block_start + WORDS_PER_BLOCK).min(total_words);
    let block_words = &words[block_start..block_end];

    // Determine slot width based on longest word in current block
    let max_word_len = block_words.iter().map(|w| w.text.chars().count()).max().unwrap_or(4);
    let slot_width = (max_word_len * 3 + 12).max(20);

    // Build Line 1: words from block_start to min(block_start + 4, block_end)
    let line1_end = (block_start + WORDS_PER_LINE).min(block_end);
    let mut line1_spans = vec![Span::raw("    ")];
    for (w_idx, w) in words.iter().enumerate().take(line1_end).skip(block_start) {
        line1_spans.extend(render_word_slot(
            w,
            w_idx,
            active_word_idx,
            cursor,
            statuses,
            slot_width,
        ));
    }

    // Build Line 2: words from min(block_start + 4, block_end) to block_end
    let line2_start = (block_start + WORDS_PER_LINE).min(block_end);
    let mut line2_spans = vec![Span::raw("    ")];
    if line2_start < block_end {
        for (w_idx, w) in words.iter().enumerate().take(block_end).skip(line2_start) {
            line2_spans.extend(render_word_slot(
                w,
                w_idx,
                active_word_idx,
                cursor,
                statuses,
                slot_width,
            ));
        }
    }

    let mut stage_lines = vec![
        Line::from(""),
        Line::from(line1_spans),
        Line::from(""),
    ];
    if line2_start < block_end {
        stage_lines.push(Line::from(line2_spans));
        stage_lines.push(Line::from(""));
    }

    let stage_title = format!(
        " Typing Stage  •  Block {} of {} ",
        current_block_idx + 1,
        total_blocks.max(1)
    );
    let stage_block = Block::default()
        .borders(Borders::ALL)
        .title(stage_title)
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

        let target_disp = if target == ' ' {
            "SPACEBAR".to_string()
        } else {
            target.to_string()
        };

        let guide_spans = vec![
            Span::raw("  FINGER PLACEMENT:  "),
            Span::styled(
                format!(" {} ", finger_name),
                Style::default().fg(Color::Black).bg(finger_color).add_modifier(Modifier::BOLD),
            ),
            Span::raw("   ──►   Target Key: "),
            Span::styled(
                format!(" [{}] ", target_disp),
                Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD),
            ),
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

    let is_tall = area.height >= 30;
    let chunks = if is_tall {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // Progress gauge
                Constraint::Length(4), // Big digital clock
                Constraint::Length(8), // Speed and accuracy metrics
                Constraint::Length(4), // Goal badge
                Constraint::Min(4),    // Performance Cadence Insights
                Constraint::Length(2), // Shortcut
            ])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // Progress gauge
                Constraint::Length(4), // Big digital clock
                Constraint::Length(8), // Speed and accuracy metrics
                Constraint::Length(4), // Goal badge
                Constraint::Min(2),    // Shortcut
            ])
            .split(area)
    };

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

    // 5. Cadence Insights Card (when tall) or Shortcut Hint
    if is_tall {
        let cadence_lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Keystrokes: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{}", cursor), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("Pace Status: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    if metrics.net_wpm >= 30.0 { "HIGH CADENCE" } else if metrics.net_wpm >= 15.0 { "STEADY FLOW" } else { "CALIBRATING" },
                    Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD),
                ),
            ]),
        ];
        let cadence_p = Paragraph::new(cadence_lines)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title(" Cadence ").title_alignment(Alignment::Center));
        frame.render_widget(cadence_p, chunks[4]);

        let shortcut_p = Paragraph::new("[Esc] Back")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(shortcut_p, chunks[5]);
    } else {
        let shortcut_p = Paragraph::new("[Esc] Back")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(shortcut_p, chunks[4]);
    }
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

    let back_label = match app.session_origin {
        crate::app::AppScreen::LessonSelect => "[Esc] Lessons  ",
        crate::app::AppScreen::SpeedTestSelect => "[Esc] Speed Tests  ",
        crate::app::AppScreen::TargetedReview => "[Esc] Targeted Review  ",
        _ => "[Esc] Back  ",
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
            Span::styled("[r] Retry  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("[n/Enter] Next  ", Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD)),
            Span::styled(back_label, Style::default().fg(Color::Cyan)),
            Span::styled("[m] Main Menu", Style::default().fg(Color::Gray)),
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

    #[test]
    fn test_format_spaced() {
        assert_eq!(format_spaced("asdf"), "a  s  d  f");
        assert_eq!(format_spaced("a"), "a");
    }

    #[test]
    fn test_render_word_slot() {
        let word = TargetWord {
            text: "asdf",
            char_start: 0,
            char_end: 4,
            has_trailing_space: true,
        };
        let statuses = vec![CharStatus::Correct, CharStatus::Pending, CharStatus::Pending, CharStatus::Pending];
        let slot = render_word_slot(&word, 0, 0, 1, &statuses, 24);
        assert!(!slot.is_empty());
    }
}
