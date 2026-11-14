# kktypr ⚡

> Clean-room terminal touch-typing tutor and training engine written in **Rust 2024 Edition**. 
> Pedagogically inspired by TypingMaster Pro mechanics with 100% original curriculum, dynamic weak-key diagnostics, and an arcade falling-word game.

---

## Highlights

- **Pure Native Rust (Edition 2024)**: Built on `ratatui` + `crossterm`. Instant sub-millisecond input polling, zero windowing latency, 60 FPS delta-time game loop, and a standalone ~1.4MB release binary.
- **Strict Clean-Room Architecture**: Zero proprietary code, zero reverse-engineering traps, and zero trademarked assets. All course texts, pangrams, and drills are original, public-domain, or developer-tailored.
- **12-Lesson Progressive Academy**:
  - *Phase 1*: Key Introduction (visual keyboard highlights & finger assignment)
  - *Phase 2*: Warm-Up Drill (rhythmic muscle-memory repetition)
  - *Phase 3*: Word Drill (real words utilizing learned keys)
  - *Phase 4*: Sentence Drill (fluid natural prose)
  - *Phase 5*: Lesson Mastery Test (1–2 minute timed certification with 94%+ accuracy thresholds)
- **Developer Syntax Course**: Dedicated drills for coding punctuation: brackets `{}[]()`, arrows `->` `=>`, path operators `::`, semicolons, and language keywords.
- **Interactive Terminal Keyboard**: Box-drawn Unicode keyboard with finger-zone color coding (Pinky, Ring, Middle, Index, Thumbs), active key glow, and live finger placement guides.
- **Pro Weak-Key Heatmap**: Tracks per-key accuracy and error matrices. Keys turn green (95%+), yellow (80–95%), or red (<80%) based on your lifetime error distribution.
- **Targeted Practice Generator**: Automatically analyzes your weakest 3–5 keys and synthesizes focused drill sentences targeting those exact finger transitions.
- **Word Drop Arcade Game**: TUI recreation of classic typing bubbles with falling word mechanics, combo streak multipliers, and high score tracking.
- **Zero-Bloat Persistence**: Atomic JSON storage in standard XDG data directories (`~/.local/share/kktypr/profile.json`).

---

## Navigation & Controls

| Screen | Keybindings | Action |
|---|---|---|
| **Global** | `Ctrl + C` | Force quit application safely |
| **Main Menu** | `↑` / `k`, `↓` / `j` | Navigate menu items |
| | `Enter` | Select menu option |
| | `q` / `Esc` | Exit application |
| **Lesson Select** | `↑` / `k`, `↓` / `j` | Browse 12 progressive lessons |
| | `Enter` | Start selected lesson |
| | `Esc` | Return to course selection |
| **Typing Pane** | Printable Keys | Type expected characters (Green = Correct, Red = Error) |
| | `Backspace` | Correct previous character |
| | `Esc` | Abort session and return to menu |
| **Scorecard** | `n` / `Enter` | Advance to next exercise / lesson |
| | `r` | Retry current exercise |
| | `m` / `Esc` | Return to main menu |
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
│   ├── app.rs                  # Application state controller and screen router
│   ├── engine/
│   │   ├── mod.rs
│   │   ├── session.rs          # TypingSession state machine and backspace support
│   │   ├── metrics.rs          # Standardized Gross/Net WPM and accuracy calculations
│   │   └── curriculum.rs       # 12-Lesson Course, Developer Special & drill models
│   ├── analytics/
│   │   ├── mod.rs
│   │   ├── heatmap.rs          # Key-by-key accuracy & error frequency matrices
│   │   ├── targeted.rs         # Dynamic weak-key practice drill synthesizer
│   │   └── profile.rs          # Atomic JSON profile persistence (~/.local/share/kktypr)
│   ├── game/
│   │   ├── mod.rs
│   │   └── word_drop.rs        # Terminal falling-word arcade physics loop
│   └── ui/
│       ├── mod.rs
│       ├── theme.rs            # Terminal color tokens & styling rules
│       ├── keyboard.rs         # Interactive box-drawn visual keyboard widget
│       └── views/
│           ├── mod.rs
│           ├── menu_view.rs    # Main menu, course and lesson browsers
│           ├── lesson_view.rs  # Active typing HUD, text stream, and scorecards
│           ├── dashboard.rs    # Weak-key diagnostic reports & typist statistics
│           └── game_view.rs    # Word Drop arcade canvas & danger baseline
```

---

## Building & Running

### Requirements
- Rust 1.85+ (Rust 2024 edition)
- Any standard Linux/macOS terminal with UTF-8 support (Foot, Ghostty, Kitty, Alacritty, GNOME Terminal)

### Commands
```bash
# Run in development mode
cargo run

# Run unit tests (100% pass)
cargo test

# Build optimized release binary
cargo build --release

# Run release binary
./target/release/kktypr
```

---

## License

Open-sourced under the [MIT License](LICENSE).
