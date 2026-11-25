use crate::engine::curriculum::{Course, ExerciseType, Lesson, multi_drill, single_drill};
use rand::seq::SliceRandom;

const SPEED_TEST_PARAGRAPHS: &[&str] = &[
    "Software engineering is the systematic application of structured methods to the design, testing, and maintenance of reliable programs.",
    "Clean architecture separates business logic from terminal rendering so every component can be verified independently and changed with confidence.",
    "Consistent touch typing builds muscle memory across home, top, and bottom rows, letting engineers express complex ideas at the speed of thought.",
    "Predictable keyboard ergonomics reduce finger travel on punctuation and brackets when writing nested expressions, closures, and type signatures.",
    "Good command line tools start quickly, avoid unnecessary allocations in tight loops, and restore terminal state cleanly if interrupted.",
    "Strong test suites catch edge cases early by exercising boundary conditions, Unicode string slicing, and error recovery paths.",
    "Practical ergonomics come from rhythm rather than bursts of speed: keep wrists neutral, strike lightly, and focus on steady accuracy first.",
    "Careful version control keeps changesets cohesive and small so reviewers can trace why each invariant exists without guessing.",
    "Readable code optimizes for the next human engineer who has to debug a production incident at two in the morning with half their coffee left.",
    "Fast iterative feedback loops turn daunting refactors into routine steps because every failing assertion points straight to the broken assumption.",
    "Deep ocean trenches hide hydrothermal vents where mineral-rich plumes support entire ecosystems without a single ray of sunlight.",
    "Early mechanical typewriters arranged keys to keep adjacent typebars from jamming together when typists struck rapid common letter pairs.",
    "Observatories high in arid mountain ranges capture sharp infrared starlight by rising above most atmospheric water vapor and city glare.",
    "Resilient distributed systems treat partial network failure as normal weather rather than an exceptional edge case.",
    "Well-chosen data structures simplify algorithms dramatically; when state invariants become impossible to violate, defensive checks melt away.",
    "Woodworkers sharpen plane irons until shavings curl thin as translucent silk, proving that tool preparation defines finish quality.",
    "Coastal lighthouses rotate precision Fresnel lenses above rugged headlands so mariners can fix their bearings through fog and squalls.",
    "Careful telemetry measures latency percentiles instead of bare averages, exposing the long tail that dominates real user perception.",
    "Typography balances subtle proportions between letterforms, line height, and whitespace so pages invite calm sustained reading.",
    "Modern terminal emulators composite textured glyphs at sixty frames per second while keeping input latency under a single frame budget.",
    "Sound engineering judgment means knowing which abstraction genuinely removes accidental complexity and which layer merely hides it.",
    "Glacial valleys carve broad U-shaped corridors through granite peaks over millennia of patient ice movement and seasonal thaws.",
    "Automated static analysis complements human code review by spotting unused branches, integer overflows, and subtle lifetime mismatches early.",
    "Precision acoustic instruments depend on seasoned tonewood dried for years before a luthier shapes a single soundboard brace.",
    "Deterministic builds guarantee that identical source trees and locked dependencies produce byte-identical artifacts across every build host.",
    "Monsoon winds reverse direction twice each year, carrying seasonal rain across fertile river deltas and mountain foothills.",
    "Strict interface boundaries allow teams to evolve internal storage formats freely without breaking external scripts or downstream consumers.",
    "Steady deliberate practice rewires motor pathways slowly during rest, which makes fifteen focused minutes beat one exhausting marathon.",
    "Submarine fiber cables span abyssal plains beneath thousands of meters of saltwater to carry transoceanic packets in milliseconds.",
    "Clear error messages name both what failed and how to recover, saving operators precious minutes during unfamiliar outages.",
    "Botanical conservatories simulate desert droughts and cloud-forest mist under adjacent glass domes through zoned climate valves.",
    "Simplicity is rarely effortless; arriving at a compact, obvious module usually takes subtracting three clever ideas that did not belong.",
];

/// Generates non-repetitive multi-paragraph text for timed speed benchmarks.
pub fn generate_speed_test_text(duration_secs: u64, previous: Option<&str>) -> String {
    let needed = match duration_secs {
        0..=90 => 5,
        91..=180 => 8,
        _ => 14,
    };

    let mut rng = rand::rng();
    let mut indices: Vec<usize> = (0..SPEED_TEST_PARAGRAPHS.len()).collect();
    indices.shuffle(&mut rng);

    let build = |idxs: &[usize]| -> String {
        let mut combined = String::new();
        for i in 0..needed {
            if !combined.is_empty() {
                combined.push(' ');
            }
            combined.push_str(SPEED_TEST_PARAGRAPHS[idxs[i % idxs.len()]]);
        }
        combined
    };

    let mut candidate = build(&indices);
    if let Some(prev) = previous
        && candidate == prev
    {
        indices.rotate_left(1);
        candidate = build(&indices);
    }
    candidate
}

impl Course {
    pub fn developer_special() -> Self {
        Self {
            id: "developer".to_string(),
            title: "Course 2: Developer Syntax & Code Symbols".to_string(),
            description: "Targeted drills for brackets, curly braces, arrows, and punctuation used in real programming."
                .to_string(),
            lessons: vec![
                Lesson {
                    number: 1,
                    title: "Brackets and Parens: () [] {}".to_string(),
                    new_keys: vec!['(', ')', '[', ']', '{', '}'],
                    finger_guidance: "Right pinky reaches top row for (), and bracket keys [ ] { }.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "Bracket Warm Up",
                            "Type matching delimiter pairs.",
                            "() [] {} (()) [[]] {{}} ([{}]) {([()])} () [] {}",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Code Expression Drill",
                            "Type common programming function calls and index expressions.",
                            94.0,
                            &[
                                "fn main() -> Result<(), Box<dyn Error>> { let items = vec![1, 2, 3]; }",
                                "const grid = [[1, 0], [0, 1]]; function get(row, col) { return grid[row][col]; }",
                                "def batch(items, size): return [items[i:i + size] for i in range(0, len(items), size)]",
                                "let buckets = vec![Vec::with_capacity(8), Vec::with_capacity(16)];",
                                "type Entry = { id: number; tags: string[]; meta: { active: boolean } };",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Rust Idioms",
                            "Type real idiomatic statements with nested brackets and blocks.",
                            94.0,
                            &[
                                "pub fn calculate(&self, count: usize) -> Option<Duration> { match self.timer { Some(t) => Some(t.elapsed()), None => None } }",
                                "pub fn swap_pair(slice: &mut [u32], i: usize, j: usize) { let tmp = slice[i]; slice[i] = slice[j]; slice[j] = tmp; }",
                                "async function fetchUser(id) { const res = await client.get(`/users/${id}`, { headers: { accept: 'json' } }); return res.data; }",
                                "func sumMatrix(rows [][]int) int { total := 0; for _, r := range rows { for _, v := range r { total += v } } return total }",
                                "pub fn clamp_window(vals: &[i32], lo: i32, hi: i32) -> Vec<i32> { vals.iter().copied().map(|x| x.clamp(lo, hi)).collect() }",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Syntax Certification",
                            "Attain high accuracy on symbol-dense code.",
                            94.0,
                            &[
                                "impl<T: Clone + Send> DataPipe<T> { pub async fn forward(&mut self, payload: T) -> Result<usize, ChannelError> { Ok(1) } }",
                                "pub fn group_by_key<K: Eq + Hash, V>(rows: Vec<(K, V)>) -> HashMap<K, Vec<V>> { let mut out = HashMap::new(); for (k, v) in rows { out.entry(k).or_insert_with(Vec::new).push(v); } out }",
                                "export const buildConfig = (opts) => ({ host: opts.host ?? 'localhost', ports: [8080, 8443], tls: { enabled: true } });",
                                "SELECT users.id, json_agg(orders.total) FROM users JOIN orders ON orders.user_id = users.id WHERE status IN ('paid', 'shipped') GROUP BY users.id;",
                                "for (idx, chunk) in buffer.chunks_exact_mut(4).enumerate() { chunk[0] = (idx & 0xff) as u8; chunk[3] = 0x7f; }",
                            ],
                        ),
                    ],
                },
                Lesson {
                    number: 2,
                    title: "Arrows & Comparison: -> => <= >= == !=".to_string(),
                    new_keys: vec!['-', '>', '=', '<', '!', '+'],
                    finger_guidance: "Left pinky reaches ! and +, right pinky/ring reaches = < >.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "Arrow & Operator Warm Up",
                            "Build muscle memory for compound operators and arrows.",
                            "-> => == != <= >= += -= *= /= -> => == != <= >=",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Comparison & Assignment Drill",
                            "Type comparison statements and assignment operators.",
                            94.0,
                            &[
                                "x => x * 2; if a == b && c != d || e <= f { return Ok(1); }",
                                "fn step(cur: i64, limit: i64) -> i64 { if cur + 1 <= limit { cur + 1 } else { limit } }",
                                "const inRange = (v, min, max) => v >= min && v <= max && v !== null;",
                                "count += 1; balance -= fee; if balance <= 0 || flag != true { status = 1; }",
                                "def is_sorted(xs: list[int]) -> bool: return all(xs[i] <= xs[i + 1] for i in range(len(xs) - 1))",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Match Arms & Lambda Expressions",
                            "Type pattern matching arms with fat arrows and guards.",
                            94.0,
                            &[
                                "match signal { Ok(val) => val.process(), Err(e) if e.is_timeout() => retry(), Err(_) => abort() }",
                                "match state { Status::Ready if retries < 3 => dispatch(), Status::Busy => backoff(), _ => fail_fast() }",
                                "const score = items.filter((x) => x.weight > 0).reduce((acc, x) => acc + x.weight * 2, 0);",
                                "fn binary_search(nums: &[i32], target: i32) -> Option<usize> { let mut lo = 0; while lo < nums.len() { if nums[lo] == target { return Some(lo); } lo += 1; } None }",
                                "switch (code) { case 200: return 'ok'; case 404: return 'missing'; default: return code >= 500 ? 'server' : 'client'; }",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Control Flow Certification",
                            "Demonstrate accuracy on compound logical conditions.",
                            94.0,
                            &[
                                "let delta = (next_val - prev_val) >= threshold && max_limit != 0 || offset < baseline;",
                                "fn within_budget(spent: f64, cap: f64, reserve: f64) -> bool { spent >= 0.0 && spent + reserve <= cap }",
                                "const compareDates = (a, b) => (a.year !== b.year ? a.year - b.year : a.day - b.day);",
                                "if latency_ms > 250 || error_rate >= 0.05 { alert_level += 1; throttle = true; } else { healthy_ticks += 1; }",
                                "let verdict = match accuracy { a if a >= 100.0 => \"Perfect\", a if a >= 94.0 => \"Pass\", _ => \"Retry\" };",
                            ],
                        ),
                    ],
                },
                Lesson {
                    number: 3,
                    title: "Semicolons, Colons & Path Operators: ; : :: , .".to_string(),
                    new_keys: vec![';', ':', ',', '.'],
                    finger_guidance: "Right pinky strikes ; and Shift+; for :. Right middle and ring reach , and . on bottom row.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "Scope & Path Warm Up",
                            "Type colons, semicolons, and path qualification symbols.",
                            ":: : ; :: : ; std::sync::Arc; std::collections::HashMap; std::path::PathBuf;",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Module Imports & Declarations",
                            "Type nested use declarations and qualified module paths.",
                            94.0,
                            &[
                                "use std::sync::atomic::{AtomicBool, Ordering}; use std::time::{Duration, Instant};",
                                "use std::collections::{BTreeMap, HashSet, VecDeque}; use std::io::{self, BufRead, Write};",
                                "use std::path::{Component, Path, PathBuf}; use std::fs::{File, OpenOptions};",
                                "std::thread::Builder::new().name(\"worker\".into()).stack_size(1024 * 64);",
                                "from collections import defaultdict, deque; from pathlib import Path;",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Struct Definition & Fields",
                            "Type typed struct fields and visibility specifiers.",
                            94.0,
                            &[
                                "pub struct Config { pub host: String, pub port: u16, pub timeout_ms: u64, pub enabled: bool }",
                                "pub struct MetricPoint { pub timestamp: u64, pub p95_latency: f64, pub p99_latency: f64, pub count: usize }",
                                "interface ServerOptions { host: string; port: number; keepAliveMs: number; tlsCert: string; }",
                                "pub enum RetryPolicy { Immediate, FixedDelay { millis: u64 }, Exponential { base_ms: u64, max_ms: u64 } }",
                                "CREATE TABLE sessions (id UUID PRIMARY KEY, user_id INT NOT NULL, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Trait Bounds & Scopes Certification",
                            "Type generic struct implementations with trait bounds.",
                            94.0,
                            &[
                                "impl<K: std::hash::Hash + Eq, V: Clone> Cache<K, V> { pub fn new() -> Self { Self { map: HashMap::new() } } }",
                                "impl std::fmt::Display for Endpoint { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, \"{}:{}\", self.host, self.port) } }",
                                "pub fn resolve_root(base: &std::path::Path, child: &str) -> std::io::Result<std::path::PathBuf> { Ok(base.join(child)) }",
                                "type Handler = std::sync::Arc<dyn Fn(&Request) -> Response + Send + Sync>;",
                                "serde_json::to_writer_pretty(std::io::BufWriter::new(file), &snapshot)?;",
                            ],
                        ),
                    ],
                },
                Lesson {
                    number: 4,
                    title: "Pipes, Ampersands & Bitwise Operators: | || & && ! ~ ^".to_string(),
                    new_keys: vec!['|', '&', '!', '~', '^'],
                    finger_guidance: "Right pinky reaches | with Shift+\\. Left pinky and index reach ! ~ ^ &.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "Bitwise & Logic Warm Up",
                            "Type boolean and bitwise operator pairs.",
                            "& && | || ! ~ ^ & && | || ! ~ ^ & && | || ! ~ ^",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Boolean Condition Drill",
                            "Type compound conditional statements.",
                            94.0,
                            &[
                                "if (is_valid && !is_empty) || (has_token && has_access) { proceed(); }",
                                "let ready = !is_closed && (queue_len > 0 || force_flush) && !has_error;",
                                "if (!user.banned && (user.verified || isAdmin)) || overrideFlag { allow(); }",
                                "while running && !shutdown.load(Ordering::Acquire) { tick(); }",
                                "let keep = (row.score >= min_score) && !(row.archived || row.deleted);",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Bitmasking & Flags",
                            "Type low-level bit shifting and masking expressions.",
                            94.0,
                            &[
                                "let mask: u32 = (1 << 4) | (1 << 2) | (1 << 0); let flags = header.flags() & !mask;",
                                "let high = (word >> 8) & 0xff; let low = word & 0xff; let swapped = (low << 8) | high;",
                                "hash ^= val.wrapping_mul(0x9e37_79b9); hash = hash.rotate_left(13) ^ (hash >> 17);",
                                "const READ = 1 << 0; const WRITE = 1 << 1; const EXEC = 1 << 2; let mode = READ | EXEC;",
                                "if (perms & WRITE_FLAG) != 0 && (perms & LOCK_FLAG) == 0 { perms ^= DIRTY_FLAG; }",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Pattern Matching with Pipes",
                            "Type multi-pattern match arms and pipelines.",
                            94.0,
                            &[
                                "match token { Token::Ident(id) | Token::Keyword(id) => emit(id), Token::Eof | Token::Whitespace => skip() }",
                                "match code { KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('h') => go_back(), _ => {} }",
                                "grep -E 'WARN|ERROR' app.log | sort | uniq -c | sort -nr | head -n 20",
                                "let toggled = (flags | ENABLE_BIT) ^ (stale_bits & !PRESERVE_MASK);",
                                "journalctl -u nginx -n 100 --no-pager | grep -E 'timeout|reset' && systemctl is-active nginx",
                            ],
                        ),
                    ],
                },
                Lesson {
                    number: 5,
                    title: "Quotes, Strings & Regex Tokens: \" ' ` \\ / $".to_string(),
                    new_keys: vec!['"', '\'', '`', '\\', '/', '$'],
                    finger_guidance: "Right pinky strikes ' and \", left pinky reaches ` and ~, right index reaches / and pinky \\.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "Quote & Slash Warm Up",
                            "Type quotation marks, backslashes, and path delimiters.",
                            "\"str\" 'c' `lit` \\n \\t \\r /path/to/file \"$var\" 'a' \"b\"",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "String Interpolation & Regex",
                            "Type formatted string templates and regular expressions.",
                            94.0,
                            &[
                                "let path = format!(\"{}/{}.json\", base_dir, filename); let re = Regex::new(r\"^\\d{4}-\\d{2}-\\d{2}$\");",
                                "const url = `https://${host}:${port}/v1/items?filter=${encodeURIComponent(tag)}`;",
                                "let re = Regex::new(r\"^[a-zA-Z0-9_.-]+@[a-zA-Z0-9-]+\\.[a-z]{2,}$\").unwrap();",
                                "let sep = if cfg!(windows) { '\\\\' } else { '/' }; let clean = raw.trim_matches('\"');",
                                "sed -n 's/^[[:space:]]*version[[:space:]]*=[[:space:]]*\"\\([^\"]*\\)\"/\\1/p' Cargo.toml",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Shell Automation & Path Builders",
                            "Type shell command strings and quoted path parameters.",
                            94.0,
                            &[
                                "git commit -m \"feat(core): sanitize path inputs\" && printf \"%s\\n\" \"$output\" > build.log",
                                "tar -czf \"${OUT_DIR}/release-${VERSION}.tar.gz\" -C \"$STAGING\" README.md LICENSE",
                                "find \"$HOME/.config\" -maxdepth 2 -type f -name '*.json' -exec ls -lh {} +",
                                "curl --proto '=https' --tlsv1.2 -fsSL \"https://api.example.com/v2/status\" -o \"$TMP_JSON\"",
                                "awk -F'/' '{ printf \"%-20s %s\\n\", $NF, $0 }' /var/log/packages.list",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Escape Sequences Certification",
                            "Type complex escaped logging statements and URLs.",
                            94.0,
                            &[
                                "println!(\"Response [status={}]: \\\"{}\\\" from endpoint /api/v1/auth\", code, body);",
                                r#"let json = format!("{{\"event\":\"{}\",\"path\":\"/tmp/{}\"}}", kind, file);"#,
                                "const msg = `User '${name}' updated path \"${base}/${sub}\"\\nStatus: OK`;",
                                "let sql = \"SELECT id, 'active' AS state FROM \\\"users\\\" WHERE home = '/home/kk376';\";",
                                "printf '{\\n  \"status\": \"%s\",\\n  \"file\": \"%s\"\\n}\\n' \"$STATE\" \"$TARGET_FILE\"",
                            ],
                        ),
                    ],
                },
                Lesson {
                    number: 6,
                    title: "Generics, Turbofish & Lifetimes: < > ::< > 'a &mut".to_string(),
                    new_keys: vec!['<', '>', ':', '\'', '&'],
                    finger_guidance: "Shift comma and dot for < >, right pinky for : and '.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "Generic Syntax Warm Up",
                            "Type generic angle brackets and lifetime annotations.",
                            "<T> <K, V> ::<T> ::<Vec<String>> &'a str &'a mut [u8] <T: Clone>",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Turbofish & Collection Transforms",
                            "Type turbofish parsing and collection collectors.",
                            94.0,
                            &[
                                "let nums = raw.split(',').map(str::trim).filter_map(|s| s.parse::<usize>().ok()).collect::<Vec<_>>();",
                                "let ports = env_list.split(':').filter_map(|p| p.parse::<u16>().ok()).collect::<HashSet<u16>>();",
                                "let buf = std::mem::take(&mut state.pending).into_iter().collect::<VecDeque<String>>();",
                                "let size = header.parse::<u64>().unwrap_or_default().min(MAX_FRAME_SIZE);",
                                "function pluck<T, K extends keyof T>(items: readonly T[], key: K): Array<T[K]> { return items.map((x) => x[key]); }",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Generic Lifetime Bounds",
                            "Type generic function signatures with explicit lifetime bounds.",
                            94.0,
                            &[
                                "pub fn filter_active<'a, T: Identifiable + 'a>(items: &'a [T]) -> Vec<&'a T> { items.iter().filter(|i| i.is_active()).collect() }",
                                "pub struct Cursor<'a, T> { slice: &'a [T], pos: usize }",
                                "pub fn split_header<'a>(input: &'a str, sep: char) -> Option<(&'a str, &'a str)> { input.split_once(sep) }",
                                "impl<'a> Iterator for TokenStream<'a> { type Item = Token<'a>; fn next(&mut self) -> Option<Self::Item> { None } }",
                                "pub fn borrow_mut_or_insert<'a, K: Eq + Hash, V: Default>(map: &'a mut HashMap<K, V>, key: K) -> &'a mut V { map.entry(key).or_default() }",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Zero Cost Generic Pipeline",
                            "Type deserialization and typed container declarations.",
                            94.0,
                            &[
                                "let parsed: Result<HashMap<String, Vec<u8>>, Error> = serde_json::from_str(&payload);",
                                "pub fn transform<I, T, U, F>(iter: I, f: F) -> Vec<U> where I: IntoIterator<Item = T>, F: FnMut(T) -> U { iter.into_iter().map(f).collect() }",
                                "type SharedQueue<T> = Arc<Mutex<VecDeque<Result<T, ChannelError>>>>;",
                                "let map: BTreeMap<&'static str, Box<dyn Fn(usize) -> usize>> = BTreeMap::new();",
                                "const mergeMaps = <V>(a: Map<string, V>, b: Map<string, V>): Map<string, V> => new Map([...a, ...b]);",
                            ],
                        ),
                    ],
                },
                Lesson {
                    number: 7,
                    title: "Async/Await & Functional Pipelines: ? .map .filter .await".to_string(),
                    new_keys: vec!['?', '.', '_', '-'],
                    finger_guidance: "Right pinky reaches ? with Shift+/, right ring for dot, left pinky for _.".to_string(),
                    exercises: vec![
                        single_drill(
                            ExerciseType::WarmUp,
                            "Pipeline Operator Warm Up",
                            "Type method chaining calls and error try operators.",
                            ".map() .filter() .unwrap() .unwrap_or_default() .ok()? .await? .as_ref()",
                            92.0,
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Async File Operations",
                            "Type async I/O function calls with error propagation.",
                            94.0,
                            &[
                                "let mut file = File::open(&path).await?; file.read_to_end(&mut buffer).await?; Ok(buffer)",
                                "let text = tokio::fs::read_to_string(&cfg_path).await?; let parsed = serde_json::from_str(&text)?;",
                                "let resp = client.get(&endpoint).timeout(Duration::from_secs(5)).send().await?.error_for_status()?;",
                                "let conn = pool.acquire().await?; sqlx::query(\"SELECT 1\").execute(&mut *conn).await?;",
                                "let body = req.text().await?.trim().to_string();",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Functional Stream Aggregation",
                            "Type functional filter and mapping pipeline expressions.",
                            94.0,
                            &[
                                "let total = entries.into_iter().filter(|e| e.is_valid).map(|e| e.score * 2).sum::<u64>();",
                                "let names: Vec<String> = users.iter().filter(|u| u.active).map(|u| u.name.trim().to_owned()).collect();",
                                "let max_latency = samples.iter().copied().fold(0.0_f64, f64::max);",
                                "let lines = raw.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')).collect::<Vec<_>>();",
                                "const totals = orders.filter((o) => o.paid).map((o) => o.amount_cents).reduce((a, b) => a + b, 0);",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Async Join & Concurrency",
                            "Type concurrent future join operations with error unwraps.",
                            94.0,
                            &[
                                "let (res1, res2) = tokio::join!(fetch_node(&addr1), fetch_node(&addr2)); let combined = res1? + res2?;",
                                "let (meta, body) = tokio::try_join!(load_manifest(&root), read_payload(&blob_path))?;",
                                "tokio::select! { msg = rx.recv() => handle(msg), _ = sleep(Duration::from_secs(10)) => return Err(Timeout) }",
                                "let handles: Vec<_> = jobs.into_iter().map(|j| tokio::spawn(run_job(j))).collect();",
                                "futures::future::try_join_all(tasks).await?.into_iter().flatten().collect()",
                            ],
                        ),
                    ],
                },
                Lesson {
                    number: 8,
                    title: "Developer Certification: Idiomatic Systems Code".to_string(),
                    new_keys: vec!['*', '&', '[', ']', '{', '}', '!', '?'],
                    finger_guidance: "Comprehensive integration of all developer syntax, symbols, and operators.".to_string(),
                    exercises: vec![
                        multi_drill(
                            ExerciseType::WarmUp,
                            "Systems Code Warm Up",
                            "Type systems network connection and result return.",
                            94.0,
                            &[
                                "fn connect(addr: &str) -> io::Result<TcpStream> { TcpStream::connect(addr) }",
                                "fn bind_udp(host: &str, port: u16) -> io::Result<UdpSocket> { UdpSocket::bind((host, port)) }",
                                "pub fn read_env(key: &str, fallback: &str) -> String { std::env::var(key).unwrap_or_else(|_| fallback.to_string()) }",
                                "fn page_align(size: usize, align: usize) -> usize { (size + align - 1) & !(align - 1) }",
                                "pub fn canonicalize_dir(p: &Path) -> io::Result<PathBuf> { std::fs::canonicalize(p) }",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::WordDrill,
                            "Macro & Unsafe Pointer Ops",
                            "Type attribute macros and low-level memory operations.",
                            94.0,
                            &[
                                "#[inline(always)] pub unsafe fn fast_copy<T: Copy>(src: *const T, dst: *mut T, count: usize) { std::ptr::copy_nonoverlapping(src, dst, count); }",
                                "#[repr(C)] #[derive(Clone, Copy)] pub struct FrameHeader { magic: u32, len: u16, flags: u16, crc: u32 }",
                                "macro_rules! ensure { ($cond:expr, $err:expr) => { if !($cond) { return Err($err); } }; }",
                                "#[must_use] pub fn slice_as_bytes<T: Sized>(val: &T) -> &[u8] { let ptr = (val as *const T) as *const u8; unsafe { std::slice::from_raw_parts(ptr, std::mem::size_of::<T>()) } }",
                                "#[cfg(target_os = \"linux\")] pub fn set_nonblocking(fd: RawFd) -> io::Result<()> { let _ = fd; Ok(()) }",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::SentenceDrill,
                            "Concurrent Channel Dispatch",
                            "Type async task spawn and channel message processing loops.",
                            94.0,
                            &[
                                "tokio::spawn(async move { while let Some(msg) = rx.recv().await { if let Err(e) = tx.send(msg).await { eprintln!(\"Err: {}\", e); break; } } });",
                                "while !stop.load(Ordering::Relaxed) { match rx.try_recv() { Ok(job) => process(job), Err(TryRecvError::Empty) => thread::yield_now(), Err(_) => break } }",
                                "let guard = state.lock().map_err(|_| AppError::Poisoned)?; guard.pending.push_back(task);",
                                "std::thread::scope(|s| { for chunk in chunks { s.spawn(move || process_chunk(chunk)); } });",
                                "let (tx, mut rx) = tokio::sync::mpsc::channel::<Command>(256); tx.send(Command::Flush).await?;",
                            ],
                        ),
                        multi_drill(
                            ExerciseType::LessonReview,
                            "Production Systems Certification",
                            "Prove mastery of complex systems syntax under strict accuracy criteria.",
                            95.0,
                            &[
                                "impl<T: Send + Sync + 'static> Drop for ThreadWorkerPool<T> { fn drop(&mut self) { for w in &mut self.workers { if let Some(t) = w.handle.take() { let _ = t.join(); } } } }",
                                "pub fn write_atomic(path: &Path, data: &[u8]) -> io::Result<()> { let tmp = path.with_extension(\"tmp\"); fs::write(&tmp, data)?; fs::rename(&tmp, path) }",
                                "impl<S> Router<S> where S: Clone + Send + Sync + 'static { pub fn route(mut self, path: &str, handler: HandlerFn<S>) -> Self { self.table.insert(path.into(), handler); self } }",
                                "fn retry_with_backoff<T, E, F: FnMut() -> Result<T, E>>(mut op: F, attempts: usize) -> Result<T, E> { loop { match op() { Ok(v) => return Ok(v), Err(e) if attempts == 0 => return Err(e), _ => {} } } }",
                                "pub async fn shutdown_graceful(listener: TcpListener, mut signal: watch::Receiver<bool>) -> io::Result<()> { loop { tokio::select! { res = listener.accept() => { let _ = res?; }, _ = signal.changed() => return Ok(()) } } }",
                            ],
                        ),
                    ],
                },
            ],
        }
    }
}
