use rand::RngExt;
use rand::seq::SliceRandom;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TargetedCourse {
    #[default]
    Basics,
    Developer,
}

impl TargetedCourse {
    pub const ALL: [Self; 2] = [Self::Basics, Self::Developer];

    pub fn title(self) -> &'static str {
        match self {
            Self::Basics => "Course 1: Touch Typing Basics",
            Self::Developer => "Course 2: Developer Syntax & Code Symbols",
        }
    }
}

/// Generates custom targeted practice drills to remediate specific weak keys.
pub struct TargetedDrillGenerator;

impl TargetedDrillGenerator {
    const VOCABULARY: &'static [&'static str] = &[
        // A
        "abstract",
        "acquire",
        "adapt",
        "address",
        "adjust",
        "advance",
        "aerial",
        "agenda",
        "alarm",
        "alias",
        "align",
        "allocate",
        "anchor",
        "ancient",
        "angle",
        "arcade",
        "archive",
        // B
        "balance",
        "banner",
        "barrel",
        "beacon",
        "binary",
        "blossom",
        "border",
        "bounce",
        "branch",
        "brave",
        "breeze",
        "bridge",
        "bright",
        "brisk",
        "bronze",
        "bubble",
        "buffer",
        // C
        "cabinet",
        "cadence",
        "camera",
        "candle",
        "canyon",
        "capture",
        "carbon",
        "castle",
        "center",
        "chapter",
        "chrome",
        "circle",
        "circuit",
        "clean",
        "clock",
        "cloud",
        "craft",
        // D
        "daemon",
        "dagger",
        "dance",
        "decade",
        "declare",
        "default",
        "delta",
        "desert",
        "design",
        "detail",
        "device",
        "digest",
        "digital",
        "divide",
        "domain",
        "dragon",
        "drive",
        // E
        "eager",
        "eagle",
        "echo",
        "edge",
        "editor",
        "effect",
        "effort",
        "ember",
        "emerald",
        "enable",
        "encoder",
        "energy",
        "engine",
        "entry",
        "epoch",
        "escape",
        "expand",
        // F
        "fabric",
        "factor",
        "faculty",
        "feather",
        "feature",
        "fiber",
        "field",
        "figure",
        "filter",
        "finish",
        "flame",
        "flight",
        "fluid",
        "focus",
        "forest",
        "forge",
        "format",
        // G
        "gadget",
        "galaxy",
        "garden",
        "gather",
        "gauge",
        "gentle",
        "gesture",
        "glance",
        "glide",
        "global",
        "granite",
        "graph",
        "gravity",
        "ground",
        "growth",
        "guard",
        "guitar",
        // H
        "habit",
        "hammer",
        "handle",
        "harbor",
        "harmony",
        "harvest",
        "hazard",
        "health",
        "height",
        "helmet",
        "hidden",
        "hollow",
        "honest",
        "horizon",
        "host",
        "humble",
        "hybrid",
        // I
        "icon",
        "ideal",
        "ignite",
        "impact",
        "import",
        "impulse",
        "income",
        "index",
        "indigo",
        "infant",
        "infinite",
        "input",
        "insight",
        "inspect",
        "install",
        "intend",
        "island",
        // J
        "jacket",
        "jagged",
        "january",
        "jasmine",
        "jargon",
        "javelin",
        "jersey",
        "jester",
        "jigsaw",
        "jingle",
        "jockey",
        "joiner",
        "joint",
        "journal",
        "journey",
        "joyful",
        "jungle",
        // K
        "kaleidoscope",
        "kayak",
        "keeper",
        "kernel",
        "kettle",
        "keyboard",
        "keynote",
        "keypad",
        "khaki",
        "kilobyte",
        "kindle",
        "kinetic",
        "kingdom",
        "kitchen",
        "kitten",
        "knapsack",
        "knight",
        // L
        "ladder",
        "lantern",
        "laptop",
        "launch",
        "layer",
        "layout",
        "league",
        "legacy",
        "legend",
        "lemon",
        "lesson",
        "letter",
        "linear",
        "liquid",
        "listen",
        "locker",
        "lunar",
        // M
        "magnet",
        "manner",
        "manual",
        "marble",
        "margin",
        "market",
        "matrix",
        "meadow",
        "memory",
        "mentor",
        "mercury",
        "method",
        "metric",
        "mirror",
        "modern",
        "module",
        "motion",
        // N
        "narrow",
        "native",
        "nature",
        "nebula",
        "needle",
        "neither",
        "network",
        "neutral",
        "nexus",
        "nimble",
        "nickel",
        "nobody",
        "noise",
        "normal",
        "notable",
        "notice",
        "number",
        // O
        "object",
        "obtain",
        "obvious",
        "ocean",
        "octave",
        "offset",
        "online",
        "openly",
        "optimal",
        "option",
        "orange",
        "orbit",
        "order",
        "origin",
        "output",
        "outward",
        "oxygen",
        // P
        "packet",
        "paddle",
        "palace",
        "parcel",
        "parent",
        "parser",
        "patent",
        "pattern",
        "pencil",
        "pepper",
        "period",
        "permit",
        "pillar",
        "pixel",
        "planet",
        "player",
        "pocket",
        // Q
        "quaint",
        "qualify",
        "quality",
        "quantum",
        "quarry",
        "quarter",
        "quartz",
        "quench",
        "query",
        "quest",
        "queue",
        "quick",
        "quiet",
        "quill",
        "quilt",
        "quiver",
        "quota",
        // R
        "radar",
        "radius",
        "random",
        "rapid",
        "raster",
        "rather",
        "ratio",
        "reader",
        "reason",
        "recipe",
        "record",
        "render",
        "repair",
        "rescue",
        "rhythm",
        "ripple",
        "robust",
        // S
        "saddle",
        "safety",
        "sample",
        "sandal",
        "scalar",
        "school",
        "screen",
        "script",
        "search",
        "season",
        "sector",
        "server",
        "shadow",
        "shield",
        "signal",
        "silver",
        "simple",
        // T
        "tablet",
        "tackle",
        "talent",
        "target",
        "temper",
        "temple",
        "thread",
        "timber",
        "token",
        "torque",
        "tracer",
        "travel",
        "treaty",
        "tunnel",
        "turf",
        "turtle",
        "typing",
        // U
        "ubuntu",
        "ultima",
        "umbrella",
        "unable",
        "unfold",
        "union",
        "unique",
        "unit",
        "unlock",
        "unpack",
        "update",
        "uplift",
        "upload",
        "upper",
        "urban",
        "urgent",
        "useful",
        // V
        "vacuum",
        "valley",
        "value",
        "vanilla",
        "vector",
        "velvet",
        "vendor",
        "verify",
        "vertex",
        "vessel",
        "vibrant",
        "victor",
        "viewer",
        "violet",
        "virtual",
        "vision",
        "volume",
        // W
        "wallet",
        "walnut",
        "wander",
        "warden",
        "warmth",
        "weapon",
        "weather",
        "weight",
        "whistle",
        "widget",
        "window",
        "winner",
        "winter",
        "wisdom",
        "witness",
        "wizard",
        "wonder",
        // X
        "boxcar",
        "duplex",
        "exact",
        "exam",
        "excel",
        "excess",
        "exhaust",
        "exist",
        "exotic",
        "expand",
        "expert",
        "export",
        "extend",
        "extra",
        "luxury",
        "oxford",
        "syntax",
        // Y
        "yacht",
        "yankee",
        "yard",
        "yearling",
        "yearly",
        "yearn",
        "yeast",
        "yellow",
        "yes",
        "yesterday",
        "yield",
        "yodel",
        "yonder",
        "young",
        "youth",
        "yucca",
        "yummy",
        // Z
        "zealous",
        "zebra",
        "zenith",
        "zephyr",
        "zero",
        "zest",
        "zigzag",
        "zinc",
        "zinger",
        "zipper",
        "zodiac",
        "zombie",
        "zonal",
        "zone",
        "zoning",
        "zoom",
        "zucchini",
    ];

    const BASICS_PUNCTUATION_WORDS: &'static [&'static str] = &[
        "aside;", "clear,", "daily.", "done;", "even,", "fair.", "fast,", "field;", "first.",
        "focus;", "glide,", "guide.", "honest,", "keep.", "quiet,", "ready.", "steady,", "still;",
        "today.", "true;", "until,", "value.", "wise,", "work;",
    ];

    pub const DEVELOPER_DRILLS: &'static [&'static str] = &[
        "std::fmt;",
        "std::io::Result<()>",
        "std::sync::Arc",
        "vec![0,1];",
        "vec![a,b,c]",
        "fn(x,y)",
        "main()",
        "{ok:true}",
        "{id,name}",
        "Result<T,E>",
        "Option<usize>",
        "Vec<String>",
        "HashMap<K,V>",
        "a==b",
        "c!=d",
        "i>=0",
        "n<=10",
        "x->y",
        "k=>v,",
        "x+=1;",
        "n-=1;",
        "v*=2;",
        "/etc/conf",
        "~/bin/run",
        "user_id",
        "max-retries",
        "\"quoted\"",
        "'char'",
        "`inline`",
        "r#\"raw\"#",
        "err?",
        "!valid",
        "&mut_buf",
        "&'a_str",
        "a|b",
        "x&&y",
        "p||q",
        "1^2",
        "~mask",
        "$HOME/bin",
        "100%",
        "#[cfg(test)]",
        "#[derive(Clone)]",
        "*ptr",
        "a+b",
        "items.len()",
        ".map(|x|x)",
        ".filter(|v|v>0)",
        ".unwrap_or_default()",
        ".await?",
        ".into_iter()",
        "tokio::join!",
        "println!(\"{}\")",
        "Ok(val)=>val,",
        "Err(e)=>return",
        "let_mut_buf",
    ];

    const DEFAULT_DRILLS: &'static [&'static str] = &[
        "the quick brown fox jumps over the lazy dog repeatedly with precision.",
        "sphinx of black quartz, judge my vow while keeping steady home-row posture.",
        "pack my box with five dozen liquor jugs and focus on smooth rhythm.",
        "glistening waves quiz bright fjord nymphs before dusk settles softly.",
        "clean touch typing comes from relaxed wrists, deliberate keystrokes, and steady pacing.",
    ];

    const DEVELOPER_DEFAULT_DRILLS: &'static [&'static str] = &[
        "fn parse(raw: &str) -> Result<Vec<usize>, Error> { Ok(raw.split(',').filter_map(|s| s.parse().ok()).collect()) }",
        "match state.poll()? { Ready(v) if v.len() > 0 => dispatch(&v), _ => backoff(Duration::from_millis(50)) }",
        "let mut map: HashMap<String, Vec<u8>> = HashMap::new(); map.entry(key).or_insert_with(Vec::new).push(1);",
        "pub async fn fetch_all(urls: &[&str]) -> io::Result<Vec<String>> { let out = Vec::new(); Ok(out) }",
        "git log --oneline -n 10 | grep -E 'fix|feat' && cargo test --all-targets -- -D warnings",
    ];

    fn pick_from_bank(
        bank: &[&'static str],
        targets: &[char],
        word_count: usize,
    ) -> Vec<&'static str> {
        let mut rng = rand::rng();
        let mut seen = HashSet::new();
        let mut matching: Vec<(&'static str, usize)> = bank
            .iter()
            .copied()
            .filter(|token| seen.insert(*token))
            .filter_map(|token| {
                let hits = targets.iter().filter(|&&t| token.contains(t)).count();
                (hits > 0).then_some((token, hits))
            })
            .collect();

        if matching.is_empty() {
            return Vec::new();
        }

        // Randomize order before grouping by hit count descending so repeated
        // weak-key sessions surface fresh word combinations instead of fixed loops.
        matching.shuffle(&mut rng);
        matching.sort_by_key(|(_, hits)| std::cmp::Reverse(*hits));

        matching
            .iter()
            .map(|(w, _)| *w)
            .cycle()
            .take(word_count)
            .collect()
    }

    /// Generates a targeted training drill strictly isolated to the chosen course.
    pub fn generate_for_course(
        course: TargetedCourse,
        weak_keys: &[char],
        word_count: usize,
    ) -> String {
        let mut rng = rand::rng();
        match course {
            TargetedCourse::Basics => {
                let targets: Vec<char> = weak_keys
                    .iter()
                    .map(|c| c.to_ascii_lowercase())
                    .filter(|c| c.is_ascii_alphabetic() || matches!(c, '.' | ',' | ';'))
                    .collect();

                if !targets.is_empty() {
                    let bank: Vec<&'static str> = Self::VOCABULARY
                        .iter()
                        .chain(Self::BASICS_PUNCTUATION_WORDS.iter())
                        .copied()
                        .collect();
                    let selected = Self::pick_from_bank(&bank, &targets, word_count);
                    if !selected.is_empty() {
                        return selected.join(" ");
                    }
                }
                let idx = rng.random_range(0..Self::DEFAULT_DRILLS.len());
                Self::DEFAULT_DRILLS[idx].to_string()
            }
            TargetedCourse::Developer => {
                let targets: Vec<char> = weak_keys
                    .iter()
                    .map(|c| c.to_ascii_lowercase())
                    .filter(|c| !c.is_whitespace())
                    .collect();

                if !targets.is_empty() {
                    let selected =
                        Self::pick_from_bank(Self::DEVELOPER_DRILLS, &targets, word_count);
                    if !selected.is_empty() {
                        return selected.join(" ");
                    }
                }
                let idx = rng.random_range(0..Self::DEVELOPER_DEFAULT_DRILLS.len());
                Self::DEVELOPER_DEFAULT_DRILLS[idx].to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drill_generator_contains_weak_keys() {
        let weak_keys = vec!['z', 'q', 'x'];
        let drill =
            TargetedDrillGenerator::generate_for_course(TargetedCourse::Basics, &weak_keys, 15);
        let words: Vec<&str> = drill.split_whitespace().collect();
        assert_eq!(words.len(), 15);

        // Verify that selected words actually contain the weak keys
        let has_weak_key = words
            .iter()
            .any(|word| word.contains('z') || word.contains('q') || word.contains('x'));
        assert!(has_weak_key);
    }

    #[test]
    fn test_targeted_drill_supports_symbols_and_expanded_vocabulary() {
        assert!(
            TargetedDrillGenerator::VOCABULARY.len() >= 350,
            "Targeted vocabulary should have rich word variety, got {}",
            TargetedDrillGenerator::VOCABULARY.len()
        );

        let symbol_drill = TargetedDrillGenerator::generate_for_course(
            TargetedCourse::Developer,
            &[';', '[', ']'],
            12,
        );
        let tokens: Vec<&str> = symbol_drill.split_whitespace().collect();
        assert_eq!(tokens.len(), 12);
        assert!(
            tokens
                .iter()
                .any(|t| t.contains(';') || t.contains('[') || t.contains(']')),
            "Targeted drill for punctuation/symbol keys must include matching tokens"
        );
    }

    #[test]
    fn test_course_targeted_drills_keep_basics_and_developer_isolated() {
        // Basics option must only emit touch-typing vocabulary (never code symbol tokens)
        let basics_with_symbol_noise = TargetedDrillGenerator::generate_for_course(
            TargetedCourse::Basics,
            &['q', '{', '['],
            16,
        );
        assert!(basics_with_symbol_noise.contains('q'));
        assert!(
            !basics_with_symbol_noise
                .chars()
                .any(|c| "{}[]<>$|\\^~`".contains(c)),
            "Touch Typing Basics drill must not contain developer syntax characters: {}",
            basics_with_symbol_noise
        );

        let basics_empty =
            TargetedDrillGenerator::generate_for_course(TargetedCourse::Basics, &[], 16);
        assert!(!basics_empty.chars().any(|c| "{}[]<>$|\\^~`".contains(c)));

        // Developer option must emit developer syntax tokens, never Touch Typing Basics prose
        let dev_empty =
            TargetedDrillGenerator::generate_for_course(TargetedCourse::Developer, &[], 12);
        assert!(
            dev_empty.chars().any(|c| "{}[]()<>=+-*/:;|&!?".contains(c)),
            "Developer fallback drill must be code syntax, got: {}",
            dev_empty
        );

        let dev_with_keys = TargetedDrillGenerator::generate_for_course(
            TargetedCourse::Developer,
            &['{', ':', 'a'],
            12,
        );
        let dev_tokens: Vec<&str> = dev_with_keys.split_whitespace().collect();
        assert_eq!(dev_tokens.len(), 12);
        assert!(
            dev_tokens
                .iter()
                .all(|tok| TargetedDrillGenerator::DEVELOPER_DRILLS.contains(tok)),
            "Developer targeted drill must strictly use developer code tokens: {}",
            dev_with_keys
        );
    }
}
