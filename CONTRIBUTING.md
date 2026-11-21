# Contributing to `kktypr`

Thank you for your interest in contributing to `kktypr`! This guide outlines our architecture, development workflow, testing standards, and pull request guidelines.

---

## 1. Project Architecture

`kktypr` is a terminal touch typing academy and developer syntax tutor written in pure Rust with `ratatui` and `crossterm`. The codebase is structured into five core modules:

- **Engine Core (`src/engine/`)**:
  - `curriculum.rs`: Defines progressive lesson courses (Course 1: Touch Typing Fundamentals, Course 2: Developer Syntax & Code Symbols), exercises, and drill classification.
  - `metrics.rs`: Calculates Gross WPM, Net WPM, accuracy percentage, error penalty formulas, and keystroke cadence.
  - `session.rs`: Tracks keystroke state, character status (`Correct`, `Incorrect`, `Pending`), and live session timers.
- **Analytics & Persistence (`src/analytics/`)**:
  - `profile.rs`: Manages multi-user profile switching, stats tracking, and atomic XDG state persistence (`~/.local/state/kktypr/profiles.json`).
  - `heatmap.rs`: Aggregates per-key error counts and accuracy percentages to identify weak finger transitions.
  - `targeted.rs`: Algorithmic procedural generator creating personalized drill sentences targeting user-specific weak keys.
- **Arcade Engine (`src/game/`)**:
  - `word_drop.rs`: Real-time 60 FPS falling-word arcade challenge with progressive speed scaling, score tracking, and collision detection.
- **Terminal User Interface (`src/ui/`)**:
  - `theme.rs`: Semantic color palette and styles (cyan accent, green success, bold red error, dark gray muted).
  - `keyboard.rs`: Visual on-screen keyboard widget mapping characters to specific fingers and color-coded zones.
  - `views/`: Dedicated screen views for `lesson_view`, `menu_view`, `dashboard`, `profile_view`, and `game_view`.
- **Application Controller (`src/app.rs`, `src/main.rs`)**:
  - Drives the 60 FPS tick loop, clean panic hooks that restore raw mode, and keyboard event dispatch.

---

## 2. Toolchain & Prerequisites

- **Rust Version**: Rust Edition 2024 with MSRV **1.88.0** or later.
- Toolchain configuration is pinned in `rust-toolchain.toml`:
  ```toml
  [toolchain]
  channel = "stable"
  components = ["rustfmt", "clippy"]
  ```

---

## 3. Local Development Workflow

Clone the repository and verify your local environment:

```bash
# Compile in debug mode
cargo check

# Run the test suite
cargo test

# Run linter with strict warning enforcement
cargo clippy --all-targets --all-features -- -D warnings

# Verify code formatting
cargo fmt --check

# Launch kktypr locally
cargo run
```

---

## 4. Contributing New Lessons & Syntax Drills

To propose or add new developer language drills:
1. Locate `src/engine/curriculum.rs`.
2. Add exercises to the appropriate course (`create_touch_typing_course()` or `create_developer_course()`).
3. Ensure text samples emphasize real-world syntactical patterns (e.g. operators, brackets, generics, or idiomatic language keywords).
4. Add unit test coverage in `src/engine/curriculum.rs` verifying lesson registration and exercise classification.

---

## 5. Engineering Invariants

- **Zero Unhandled Panics**: Terminal raw mode must always be restored cleanly on exit or error.
- **No Pass-Through Wrappers**: Keep module interfaces deep and cohesive.
- **Strict TDD**: Write deterministic tests before fixing bugs or introducing new scoring calculations.
- **Theme Respect**: Use `Color::Reset` as default background so user terminal colorschemes are respected.

---

## 6. Submitting Pull Requests

1. Create a focused branch (`feat/your-feature` or `fix/issue-description`).
2. Keep diffs minimal, clean, and directly traceable to the feature or bug fix.
3. Ensure all tests, clippy lints, and format checks pass cleanly.
4. Open a pull request using the provided PR template.
