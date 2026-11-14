# kktypr ⚡

> Clean-room terminal touch-typing tutor and training engine written in **Rust 2024 Edition**. 
> Pedagogically inspired by TypingMaster Pro mechanics with 100% original curriculum, dynamic weak-key diagnostics, multi-profile management, and an arcade falling-word game.

---

## Highlights

- **Pure Native Rust (Edition 2024)**: Built on `ratatui` + `crossterm`. Instant sub-millisecond input polling, zero windowing latency, 60 FPS delta-time game loop, and a standalone ~1.4MB release binary.
- **Strict Clean-Room Architecture**: Zero proprietary code, zero reverse-engineering traps, and zero trademarked assets. All course texts, pangrams, and drills are original, public-domain, or developer-tailored.
- **Authentic TypingMaster Block Stage**:
  - Displays structured 8-word stationary blocks (4 words × 2 lines) so upcoming text remains fixed on screen.
  - Active slot highlighted with `▶`, letter tracking cursor, and explicit `[␣ SPACE]` prompt at word boundaries.
  - Moving cursor tracks typing position without disorienting text shifts.
- **12-Lesson Progressive Academy**:
  - *Phase 1*: Key Introduction (visual keyboard highlights & finger assignment)
  - *Phase 2*: Warm-Up Drill (rhythmic muscle-memory repetition)
  - *Phase 3*: Word Drill (real words utilizing learned keys)
  - *Phase 4*: Sentence Drill (fluid natural prose)
  - *Phase 5*: Lesson Mastery Test (1–2 minute timed certification with 94%+ accuracy thresholds)
- **Developer Syntax Course**: Dedicated drills for coding punctuation: brackets `{}[]()`, arrows `->` `=>`, path operators `::`, semicolons, and language keywords.
- **Interactive Terminal Keyboard**: Box-drawn Unicode keyboard with finger-zone color coding (Pinky, Ring, Middle, Index, Thumbs), active key glow, centered spacebar, and live finger placement guides.
- **Typist Profile Manager & Multi-Profile Support**:
  - First-run setup modal prompts for the typist's name.
  - Switch between multiple typist profiles, register new profiles, or rename existing accounts.
  - Reset stats option with high-contrast safety confirmation banner (`fg(Black).bg(Yellow)`) and deferred name prompt upon starting the next exercise.
  - Atomic JSON persistence in standard XDG data directories (`~/.local/share/kktypr/profiles.json`) with automatic legacy migration.
- **Pro Weak-Key Heatmap**: Tracks per-key accuracy and error matrices. Keys turn green (95%+), yellow (80–95%), or red (<80%) based on your lifetime error distribution.
- **Targeted Practice Generator**: Automatically analyzes your weakest 3–5 keys and synthesizes focused drill sentences targeting those exact finger transitions.
- **Word Drop Arcade Game**: TUI recreation of classic typing bubbles with falling word mechanics, dynamic terminal geometry adaptation, high-contrast target pills, combo streak multipliers, and high score tracking.

---

## Navigation & Controls

| Screen | Keybindings | Action |
|---|---|---|
| **Global** | `Ctrl + C` | Force quit application safely |
| **Main Menu** | `↑` / `k`, `↓` / `j` | Navigate menu items |
| | `Enter` | Select menu option |
| | `r` | Quick trigger stats reset confirmation |
| | `q` / `Esc` | Exit application |
| **Name Prompt Modal** | Alphanumeric, `-`, `_` | Type typist name (max 24 characters) |
| | `Backspace` | Erase character |
| | `Enter` | Confirm and proceed |
| | `Esc` | Cancel / use default 'Typist' |
| **Profile Manager** | `↑` / `k`, `↓` / `j` | Browse typist profiles |
| | `Enter` | Switch active profile |
| | `a` | Create new profile |
| | `r` | Rename selected profile |
| | `d` | Delete selected profile (safeguarded) |
| | `Esc` / `q` | Return to Main Menu |
| **Lesson Select** | `↑` / `k`, `↓` / `j` | Browse 12 progressive lessons |
| | `Enter` | Start selected lesson |
| | `Esc` | Return to course selection |
| **Typing Pane** | Printable Keys | Type expected characters (Green = Correct, Red = Error) |
| | `Backspace` | Correct previous character |
| | `Esc` | Abort session and return to previous screen (e.g. Lesson Select) |
| **Scorecard** | `n` / `Enter` | Advance to next exercise / lesson |
| | `r` | Retry current exercise |
| | `Esc` / `q` | Return to previous screen (e.g. Lesson Select) |
| | `m` | Jump directly to main menu |
| **Speed Tests** | `1`, `2`, `5` | Start 1-minute, 2-minute, or 5-minute typing test |
| | `Esc` | Return to main menu |
| **Targeted Review** | `Enter` / `p` | Generate dynamic targeted drill for weakest keys |
| | `Esc` | Return to main menu |
| **Word Drop Game** | Keys `a-z` | Type falling words to pop them before they breach the floor |
| | `r` (Game Over) | Play again |
| | `Esc` | Exit game to main menu |

---

## Architectural Layout

```text
kktypr/
├── Cargo.toml                  # Rust 2024 edition, ratatui 0.30, crossterm 0.29
├── src/
│   ├── main.rs                 # Terminal setup, raw mode, panic hook, 60 FPS event loop
│   ├── app.rs                  # Application state controller, screen router, key dispatch
│   ├── engine/
│   │   ├── mod.rs
│   │   ├── session.rs          # TypingSession state machine and backspace support
│   │   ├── metrics.rs          # Standardized Gross/Net WPM and accuracy calculations
│   │   └── curriculum.rs       # 12-Lesson Course, Developer Special & drill models
│   ├── analytics/
│   │   ├── mod.rs
│   │   ├── heatmap.rs          # Key-by-key accuracy & error frequency matrices
│   │   ├── targeted.rs         # Dynamic weak-key practice drill synthesizer
│   │   └── profile.rs          # Multi-profile registry and atomic JSON storage
│   ├── game/
│   │   ├── mod.rs
│   │   └── word_drop.rs        # Terminal falling-word arcade physics loop
│   └── ui/
│       ├── mod.rs              # Frame router and overlay compositor
│       ├── theme.rs            # Terminal color tokens & styling rules
│       ├── keyboard.rs         # Interactive box-drawn visual keyboard widget
│       └── views/
│           ├── mod.rs
│           ├── menu_view.rs    # Main menu, course and lesson browsers
│           ├── lesson_view.rs  # Block typing stage, HUD, and scorecards
│           ├── dashboard.rs    # Weak-key diagnostic reports & typist statistics
│           ├── game_view.rs    # Word Drop arcade canvas & danger baseline
│           ├── modal_view.rs   # Floating dialogs for typist profile creation/rename
│           └── profile_view.rs # Profile Manager view (switch, add, rename, delete)
```

---

## Building & Running

### Requirements
- Rust 1.85+ (Rust 2024 edition)
- Any standard Linux/macOS terminal with UTF-8 support (Ghostty, Foot, Kitty, Alacritty, GNOME Terminal)

### Commands
```bash
# Run in development mode
cargo run

# Run unit tests (100% pass)
cargo test

# Check linting and idioms
cargo clippy --all-targets

# Build optimized release binary
cargo build --release

# Run release binary
./target/release/kktypr
```

---

## Clean-Room Pedagogical Design

| Dimension | Implementation Standard |
|---|---|
| **Curriculum** | 100% original sentences and code drills; zero copied or trademarked lessons. |
| **Input Physics** | Real-time delta-time event polling with zero-latency keystroke feedback. |
| **Error Handling** | Atomic writes via temporary files prevent JSON corruption during sudden disconnects. |
| **Test Suite** | Comprehensive unit tests for metrics, session lifecycles, profile CRUD, and curriculum completeness. |

---

## License

Open-sourced under the [MIT License](LICENSE).
