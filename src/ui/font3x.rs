//! 3-Row Tall Block Typography for Terminal Touch Typing.
//!
//! Provides multi-row high-contrast block glyphs for letters, digits, and symbols,
//! allowing text in the typing stage to be magnified ~3x for enhanced focus and readability.

/// Returns the 3-row block representation `[row0, row1, row2]` for a character.
pub fn get_glyph_3x(c: char) -> [&'static str; 3] {
    let lower = c.to_ascii_lowercase();
    match lower {
        'a' => ["▄▀█", "█▀█", "▀ ▀"],
        'b' => ["█▀▄", "█▀▄", "▀▀ "],
        'c' => ["█▀▀", "█  ", "▀▀▀"],
        'd' => ["█▀▄", "█ █", "▀▀ "],
        'e' => ["█▀▀", "█▀▀", "▀▀▀"],
        'f' => ["█▀▀", "█▀▀", "▀  "],
        'g' => ["█▀▀", "█ █", "▀▀▀"],
        'h' => ["█ █", "█▀█", "▀ ▀"],
        'i' => [" █ ", " █ ", "▀▀▀"],
        'j' => ["  █", "▄ █", " ▀▀"],
        'k' => ["█▄▀", "█▀▄", "▀ ▀"],
        'l' => ["█  ", "█  ", "▀▀▀"],
        'm' => ["█▀▄▀█", "█ ▀ █", "▀   ▀"],
        'n' => ["█▀▄", "█ █", "▀ ▀"],
        'o' => ["█▀█", "█ █", "▀▀▀"],
        'p' => ["█▀█", "█▀▀", "▀  "],
        'q' => ["█▀█", "▀▀█", "  ▀"],
        'r' => ["█▀▄", "█▀ ", "▀  "],
        's' => ["█▀▀", "▀▀█", "▀▀▀"],
        't' => ["▀█▀", " █ ", "  ▀"],
        'u' => ["█ █", "█ █", "▀▀▀"],
        'v' => ["█ █", "█ █", " ▀ "],
        'w' => ["█   █", "█ █ █", " ▀ ▀ "],
        'x' => ["▀▄▀", " █ ", "▄▀▄"],
        'y' => ["█ █", "▀█▀", " ▀ "],
        'z' => ["▀▀█", " ▄▀", "█▄▄"],
        '0' => ["█▀█", "█ █", "▀▀▀"],
        '1' => [" █ ", " █ ", "▀▀▀"],
        '2' => ["▀▀█", "█▀▀", "▀▀▀"],
        '3' => ["▀▀█", " ▀█", "▀▀▀"],
        '4' => ["█ █", "▀▀█", "  ▀"],
        '5' => ["█▀▀", "▀▀█", "▀▀▀"],
        '6' => ["█▀▀", "█▀█", "▀▀▀"],
        '7' => ["▀▀█", "  █", "  ▀"],
        '8' => ["█▀█", "█▀█", "▀▀▀"],
        '9' => ["█▀█", "▀▀█", "▀▀▀"],
        ';' => [" ▄ ", " ▄ ", "▀  "],
        ':' => [" ▄ ", "   ", " ▄ "],
        ',' => ["   ", " ▄ ", "▀  "],
        '.' => ["   ", "   ", " ▀ "],
        '\'' => [" █ ", " ▀ ", "   "],
        '"' => ["█ █", "▀ ▀", "   "],
        '-' => ["   ", "▀▀▀", "   "],
        '_' => ["   ", "   ", "▀▀▀"],
        '/' => ["  █", " █ ", "█  "],
        '\\' => ["█  ", " █ ", "  █"],
        '?' => ["▀▀█", " ▄▀", " ▀ "],
        '!' => [" █ ", " █ ", " ▀ "],
        '(' => [" ▄▀", " █ ", " ▀▄"],
        ')' => ["▀▄ ", " █ ", "▄▀ "],
        '[' => [" █▀", " █ ", " █▄"],
        ']' => ["▀█ ", " █ ", "▄█ "],
        '{' => [" ▄▀", "▀▄ ", " ▀▄"],
        '}' => ["▀▄ ", " ▄▀", "▄▀ "],
        '<' => ["  ▀", " ▀▄", "  ▀"],
        '>' => ["▀  ", "▄▀ ", "▀  "],
        '=' => ["   ", "▀▀▀", "▀▀▀"],
        '+' => [" ▄ ", "▀█▀", " ▀ "],
        '*' => ["▀▄▀", " █ ", "▄▀▄"],
        '&' => ["▄▀▄", " ▄▀", "▀ ▀"],
        '%' => ["█ ▀", " █ ", "▀ █"],
        '$' => ["▄█▄", " █ ", "▀█▀"],
        '#' => ["█ █", "▀█▀", "█ █"],
        '@' => ["▄▀█", "█▄█", " ▀▀"],
        '~' => ["▀▄▀", "   ", "   "],
        '^' => [" ▄ ", "▀ ▀", "   "],
        '|' => [" █ ", " █ ", " █ "],
        ' ' => ["   ", "   ", "   "],
        _ => ["   ", " ? ", "   "],
    }
}

/// Returns the horizontal display width of the 3-row glyph.
pub fn glyph_width(c: char) -> usize {
    let g = get_glyph_3x(c);
    g[0].chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ascii_glyphs_have_consistent_row_lengths() {
        let test_chars = "abcdefghijklmnopqrstuvwxyz0123456789;:,.'\"-_/?!\\()[]{}<>=+*&%$#@~^| ";
        for c in test_chars.chars() {
            let g = get_glyph_3x(c);
            let l0 = g[0].chars().count();
            let l1 = g[1].chars().count();
            let l2 = g[2].chars().count();
            assert_eq!(
                l0, l1,
                "Glyph for '{}' row 0 length ({}) != row 1 length ({})",
                c, l0, l1
            );
            assert_eq!(
                l1, l2,
                "Glyph for '{}' row 1 length ({}) != row 2 length ({})",
                c, l1, l2
            );
            assert!(l0 >= 3, "Glyph width should be at least 3 for '{}'", c);
        }
    }

    #[test]
    fn test_uppercase_maps_cleanly() {
        let g_upper = get_glyph_3x('A');
        let g_lower = get_glyph_3x('a');
        assert_eq!(g_upper, g_lower);
    }
}
