use ratatui::style::{Color, Modifier, Style};

#[allow(dead_code)]
pub struct Theme;

#[allow(dead_code)]
impl Theme {
    pub const ACCENT: Color = Color::Cyan;
    pub const SUCCESS: Color = Color::Green;
    pub const ERROR: Color = Color::Red;
    pub const WARNING: Color = Color::Yellow;
    pub const MUTED: Color = Color::DarkGray;
    pub const FG: Color = Color::White;
    pub const BG: Color = Color::Reset;

    pub fn title() -> Style {
        Style::default()
            .fg(Self::ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn header_tab(active: bool) -> Style {
        if active {
            Style::default()
                .fg(Color::Black)
                .bg(Self::ACCENT)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        }
    }

    pub fn correct_char() -> Style {
        Style::default().fg(Self::SUCCESS)
    }

    pub fn incorrect_char() -> Style {
        Style::default().fg(Color::White).bg(Self::ERROR).add_modifier(Modifier::BOLD)
    }

    pub fn pending_char() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    pub fn active_cursor_char() -> Style {
        Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
    }

    pub fn finger_style(finger_idx: usize) -> Style {
        match finger_idx {
            0 => Style::default().fg(Color::Magenta), // Left Pinky
            1 => Style::default().fg(Color::Blue),    // Left Ring
            2 => Style::default().fg(Color::Cyan),    // Left Middle
            3 => Style::default().fg(Color::Green),   // Left Index
            4 => Style::default().fg(Color::Green),   // Right Index
            5 => Style::default().fg(Color::Cyan),    // Right Middle
            6 => Style::default().fg(Color::Blue),    // Right Ring
            7 => Style::default().fg(Color::Magenta), // Right Pinky
            _ => Style::default().fg(Color::Yellow),  // Thumbs
        }
    }
}
