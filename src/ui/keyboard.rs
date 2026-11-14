use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use crate::analytics::heatmap::KeyHeatmap;
use crate::ui::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Finger {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    Thumb,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
}

impl Finger {
    pub fn name(&self) -> &'static str {
        match self {
            Self::LeftPinky => "Left Pinky",
            Self::LeftRing => "Left Ring",
            Self::LeftMiddle => "Left Middle",
            Self::LeftIndex => "Left Index",
            Self::Thumb => "Thumb (Space)",
            Self::RightIndex => "Right Index",
            Self::RightMiddle => "Right Middle",
            Self::RightRing => "Right Ring",
            Self::RightPinky => "Right Pinky",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::LeftPinky | Self::RightPinky => Color::Magenta,
            Self::LeftRing | Self::RightRing => Color::LightBlue,
            Self::LeftMiddle | Self::RightMiddle => Color::Cyan,
            Self::LeftIndex | Self::RightIndex => Color::Green,
            Self::Thumb => Color::Yellow,
        }
    }
}

pub fn get_finger_for_char(c: char) -> Option<Finger> {
    if c == ' ' {
        return Some(Finger::Thumb);
    }
    let lower = c.to_ascii_lowercase();
    match lower {
        '1' | 'q' | 'a' | 'z' => Some(Finger::LeftPinky),
        '2' | 'w' | 's' | 'x' => Some(Finger::LeftRing),
        '3' | 'e' | 'd' | 'c' => Some(Finger::LeftMiddle),
        '4' | '5' | 'r' | 't' | 'f' | 'g' | 'v' | 'b' => Some(Finger::LeftIndex),
        '6' | '7' | 'y' | 'u' | 'h' | 'j' | 'n' | 'm' => Some(Finger::RightIndex),
        '8' | 'i' | 'k' | ',' => Some(Finger::RightMiddle),
        '9' | 'o' | 'l' | '.' => Some(Finger::RightRing),
        '0' | '-' | '=' | 'p' | '[' | ']' | ';' | '\'' | '/' => Some(Finger::RightPinky),
        _ => None,
    }
}

pub struct KeyboardWidget<'a> {
    pub active_char: Option<char>,
    pub heatmap: Option<&'a KeyHeatmap>,
}

impl<'a> KeyboardWidget<'a> {
    pub fn new(active_char: Option<char>) -> Self {
        Self {
            active_char,
            heatmap: None,
        }
    }

    pub fn with_heatmap(mut self, heatmap: &'a KeyHeatmap) -> Self {
        self.heatmap = Some(heatmap);
        self
    }
}

impl<'a> Widget for KeyboardWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Interactive Keyboard & Finger Guide ")
            .title_alignment(Alignment::Center)
            .border_style(Style::default().fg(Theme::ACCENT));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 6 || inner.width < 50 {
            return;
        }

        let row1 = [
            ('`', '~', Finger::LeftPinky),
            ('1', '!', Finger::LeftPinky),
            ('2', '@', Finger::LeftRing),
            ('3', '#', Finger::LeftMiddle),
            ('4', '$', Finger::LeftIndex),
            ('5', '%', Finger::LeftIndex),
            ('6', '^', Finger::RightIndex),
            ('7', '&', Finger::RightIndex),
            ('8', '*', Finger::RightMiddle),
            ('9', '(', Finger::RightRing),
            ('0', ')', Finger::RightPinky),
            ('-', '_', Finger::RightPinky),
            ('=', '+', Finger::RightPinky),
        ];

        let row2 = [
            ('q', 'Q', Finger::LeftPinky),
            ('w', 'W', Finger::LeftRing),
            ('e', 'E', Finger::LeftMiddle),
            ('r', 'R', Finger::LeftIndex),
            ('t', 'T', Finger::LeftIndex),
            ('y', 'Y', Finger::RightIndex),
            ('u', 'U', Finger::RightIndex),
            ('i', 'I', Finger::RightMiddle),
            ('o', 'O', Finger::RightRing),
            ('p', 'P', Finger::RightPinky),
            ('[', '{', Finger::RightPinky),
            (']', '}', Finger::RightPinky),
        ];

        let row3 = [
            ('a', 'A', Finger::LeftPinky),
            ('s', 'S', Finger::LeftRing),
            ('d', 'D', Finger::LeftMiddle),
            ('f', 'F', Finger::LeftIndex),
            ('g', 'G', Finger::LeftIndex),
            ('h', 'H', Finger::RightIndex),
            ('j', 'J', Finger::RightIndex),
            ('k', 'K', Finger::RightMiddle),
            ('l', 'L', Finger::RightRing),
            (';', ':', Finger::RightPinky),
            ('\'', '"', Finger::RightPinky),
        ];

        let row4 = [
            ('z', 'Z', Finger::LeftPinky),
            ('x', 'X', Finger::LeftRing),
            ('c', 'C', Finger::LeftMiddle),
            ('v', 'V', Finger::LeftIndex),
            ('b', 'B', Finger::LeftIndex),
            ('n', 'N', Finger::RightIndex),
            ('m', 'M', Finger::RightIndex),
            (',', '<', Finger::RightMiddle),
            ('.', '>', Finger::RightRing),
            ('/', '?', Finger::RightPinky),
        ];

        let render_key = |key_char: char, display_char: char, finger: Finger| -> Span<'static> {
            let is_active = self.active_char.is_some_and(|c| {
                c.eq_ignore_ascii_case(&key_char)
            });

            if is_active {
                Span::styled(
                    format!(" [{}] ", display_char.to_ascii_uppercase()),
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
            } else if let Some(hm) = self.heatmap {
                let acc = hm.get_accuracy(key_char);
                let color = if acc >= 95.0 {
                    Color::Green
                } else if acc >= 80.0 {
                    Color::Yellow
                } else {
                    Color::Red
                };
                Span::styled(
                    format!(" [{}] ", display_char.to_ascii_uppercase()),
                    Style::default().fg(color),
                )
            } else {
                Span::styled(
                    format!(" [{}] ", display_char.to_ascii_uppercase()),
                    Style::default().fg(finger.color()),
                )
            }
        };

        let mut lines = Vec::new();

        // Row 1
        let mut r1_spans = Vec::new();
        r1_spans.push(Span::raw(" "));
        for (k, d, f) in row1 {
            r1_spans.push(render_key(k, d, f));
        }
        lines.push(Line::from(r1_spans));

        // Row 2
        let mut r2_spans = Vec::new();
        r2_spans.push(Span::raw("  "));
        for (k, d, f) in row2 {
            r2_spans.push(render_key(k, d, f));
        }
        lines.push(Line::from(r2_spans));

        // Row 3
        let mut r3_spans = Vec::new();
        r3_spans.push(Span::raw("    "));
        for (k, d, f) in row3 {
            r3_spans.push(render_key(k, d, f));
        }
        lines.push(Line::from(r3_spans));

        // Row 4
        let mut r4_spans = Vec::new();
        r4_spans.push(Span::raw("      "));
        for (k, d, f) in row4 {
            r4_spans.push(render_key(k, d, f));
        }
        lines.push(Line::from(r4_spans));

        // Row 5: Spacebar
        let is_space_active = self.active_char == Some(' ');
        let space_style = if is_space_active {
            Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Yellow)
        };
        let r5_spans = vec![
            Span::raw("             "),
            Span::styled("[                SPACEBAR                ]", space_style),
        ];
        lines.push(Line::from(r5_spans));

        // Guidance line
        if let Some(target) = self.active_char {
            let guide = if let Some(finger) = get_finger_for_char(target) {
                let key_disp = if target == ' ' { "SPACE".to_string() } else { target.to_string() };
                format!("👉 Target: [{}]  |  Finger: {} ", key_disp, finger.name())
            } else {
                format!("👉 Target: [{}] ", target)
            };
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled(guide, Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD)),
            ]));
        }

        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .render(inner, buf);
    }
}
