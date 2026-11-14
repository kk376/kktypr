/// Generates custom targeted practice drills to remediate specific weak keys.
pub struct TargetedDrillGenerator;

impl TargetedDrillGenerator {
    const VOCABULARY: &'static [&'static str] = &[
        // A
        "adapt",
        "alarm",
        "action",
        "artist",
        "anchor",
        "arrow",
        "arcade",
        // B
        "brave",
        "bridge",
        "bright",
        "build",
        "border",
        "branch",
        "bounce",
        "bubble",
        // C
        "clean",
        "craft",
        "circle",
        "cloud",
        "center",
        "candle",
        "custom",
        "crater",
        // D
        "dance",
        "drive",
        "depth",
        "domain",
        "divide",
        "dragon",
        "desert",
        "detail",
        // E
        "energy",
        "engine",
        "echo",
        "expert",
        "escape",
        "expand",
        "effort",
        "eagle",
        // F
        "focus",
        "flame",
        "forest",
        "flight",
        "factor",
        "figure",
        "future",
        "forge",
        // G
        "great",
        "glide",
        "ground",
        "garden",
        "galaxy",
        "gentle",
        "global",
        "glance",
        // H
        "harbor",
        "honest",
        "hybrid",
        "horizon",
        "hammer",
        "hazard",
        "hollow",
        "health",
        // I
        "impact",
        "island",
        "insight",
        "invent",
        "income",
        "intend",
        "iron",
        "infinite",
        // J
        "journey",
        "jungle",
        "jacket",
        "judge",
        "junior",
        "joint",
        "joyful",
        "jigsaw",
        // K
        "knight",
        "kernel",
        "kinetic",
        "kingdom",
        "keeper",
        "kettle",
        "keyboard",
        "kitchen",
        // L
        "linear",
        "liquid",
        "legend",
        "lunar",
        "legacy",
        "laptop",
        "lesson",
        "ladder",
        // M
        "matrix",
        "memory",
        "motion",
        "module",
        "modern",
        "magnet",
        "meadow",
        "marble",
        // N
        "native",
        "nature",
        "needle",
        "nebula",
        "number",
        "novel",
        "network",
        "nexus",
        // O
        "orbit",
        "origin",
        "option",
        "orange",
        "outward",
        "ocean",
        "object",
        "optimal",
        // P
        "pixel",
        "planet",
        "puzzle",
        "player",
        "portal",
        "python",
        "proper",
        "pocket",
        // Q
        "quest",
        "quick",
        "quiet",
        "quiver",
        "quota",
        "quarry",
        "quantum",
        "quaint",
        // R
        "rust",
        "robust",
        "rapid",
        "rhythm",
        "radar",
        "rocket",
        "rescue",
        "ripple",
        // S
        "system",
        "solid",
        "stream",
        "server",
        "spirit",
        "simple",
        "shield",
        "stride",
        // T
        "timber",
        "target",
        "talent",
        "temple",
        "tunnel",
        "theory",
        "tracer",
        "torque",
        // U
        "unique",
        "update",
        "urgent",
        "uplift",
        "unfold",
        "utopia",
        "unlock",
        "useful",
        // V
        "vector",
        "vision",
        "valley",
        "vibrant",
        "vertex",
        "vessel",
        "velvet",
        "volume",
        // W
        "winter",
        "window",
        "wisdom",
        "wizard",
        "wonder",
        "weight",
        "walnut",
        "weapon",
        // X
        "matrix",
        "expert",
        "index",
        "vertex",
        "luxury",
        "syntax",
        "galaxy",
        "oxygen",
        // Y
        "yellow",
        "yield",
        "yard",
        "yearly",
        "young",
        "yonder",
        "yacht",
        "yesterday",
        // Z
        "zebra",
        "zenith",
        "zone",
        "zipper",
        "zealous",
        "zodiac",
        "zigzag",
        "zero",
    ];

    /// Generates a targeted training drill focused on the specified weak keys.
    pub fn generate_drill(weak_keys: &[char], word_count: usize) -> String {
        if weak_keys.is_empty() {
            return "the quick brown fox jumps over the lazy dog repeatedly with precision."
                .to_string();
        }

        let lowercase_targets: Vec<char> = weak_keys
            .iter()
            .map(|c| c.to_ascii_lowercase())
            .filter(|c| c.is_alphabetic())
            .collect();

        let mut matching_words: Vec<&'static str> = Self::VOCABULARY
            .iter()
            .copied()
            .filter(|word| {
                lowercase_targets
                    .iter()
                    .any(|&target| word.contains(target))
            })
            .collect();

        if matching_words.is_empty() {
            return "practice standard touch typing with steady rhythm and focus.".to_string();
        }

        // Sort words by how many weak keys they contain
        matching_words.sort_by_key(|word| {
            let matches = lowercase_targets
                .iter()
                .filter(|&&target| word.contains(target))
                .count();
            std::cmp::Reverse(matches)
        });

        // Pick alternating words to form a smooth practice sequence
        let mut selected: Vec<&'static str> = Vec::with_capacity(word_count);
        let mut idx = 0;
        while selected.len() < word_count {
            selected.push(matching_words[idx % matching_words.len()]);
            idx += 1;
        }

        selected.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drill_generator_contains_weak_keys() {
        let weak_keys = vec!['z', 'q', 'x'];
        let drill = TargetedDrillGenerator::generate_drill(&weak_keys, 15);
        let words: Vec<&str> = drill.split_whitespace().collect();
        assert_eq!(words.len(), 15);

        // Verify that selected words actually contain the weak keys
        let has_weak_key = words
            .iter()
            .any(|word| word.contains('z') || word.contains('q') || word.contains('x'));
        assert!(has_weak_key);
    }
}
