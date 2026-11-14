use crate::app::{NamePromptPurpose, NamePromptState};
use crate::ui::theme::Theme;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub fn render_name_prompt_modal(prompt: &NamePromptState, frame: &mut Frame) {
    let area = frame.area();
    let modal_w = 64.min(area.width.saturating_sub(4)).max(30);
    let modal_h = 10.min(area.height.saturating_sub(2)).max(8);

    let popup_x = (area.width.saturating_sub(modal_w)) / 2;
    let popup_y = (area.height.saturating_sub(modal_h)) / 2;
    let popup_rect = Rect::new(popup_x, popup_y, modal_w, modal_h);

    // Clear background to prevent bleed-through
    frame.render_widget(Clear, popup_rect);

    let (title, subtitle): (&str, String) = match &prompt.purpose {
        NamePromptPurpose::FirstRun => (
            " WELCOME TO KKTYPR • TYPIST SETUP ",
            "Enter your typist name to personalize your progress:".to_string(),
        ),
        NamePromptPurpose::AfterReset { .. } => (
            " STATS RESET • TYPIST SETUP ",
            "Your statistics have been reset. Enter your name to begin:".to_string(),
        ),
        NamePromptPurpose::RenameProfile { old_name } => (
            " RENAME PROFILE ",
            format!("Enter new name for profile '{}':", old_name),
        ),
        NamePromptPurpose::AddNewProfile => (
            " CREATE NEW TYPIST PROFILE ",
            "Enter a unique name for the new typist profile:".to_string(),
        ),
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Theme::ACCENT));

    let inner = block.inner(popup_rect);
    frame.render_widget(block, popup_rect);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Subtitle
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Input field
            Constraint::Length(1), // Error message / spacer
            Constraint::Min(1),    // Controls hint
        ])
        .split(inner);

    // 1. Subtitle instruction
    let sub_p = Paragraph::new(subtitle)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    frame.render_widget(sub_p, rows[0]);

    // 2. High-contrast Input Field Box
    let display_input = format!("{}_", prompt.input);
    let input_line = Line::from(vec![
        Span::styled(
            " ▶ ",
            Style::default()
                .fg(Theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" {:<28} ", display_input),
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ]);
    let input_p = Paragraph::new(input_line).alignment(Alignment::Center);
    frame.render_widget(input_p, rows[2]);

    // 3. Error message or blank
    if let Some(ref err) = prompt.error_msg {
        let err_p = Paragraph::new(format!("⚠️ {}", err))
            .style(
                Style::default()
                    .fg(Color::LightRed)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);
        frame.render_widget(err_p, rows[3]);
    }

    // 4. Shortcut instructions
    let hint_text = match prompt.purpose {
        NamePromptPurpose::FirstRun => "[Enter] Save & Start  •  [Esc] Use 'Typist'",
        NamePromptPurpose::AfterReset { .. } => "[Enter] Confirm & Continue  •  [Esc] Use 'Typist'",
        _ => "[Enter] Confirm  •  [Esc] Cancel",
    };
    let hint_p = Paragraph::new(hint_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(hint_p, rows[4]);
}
