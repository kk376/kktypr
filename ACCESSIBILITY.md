# Accessibility Statement

`kktypr` is committed to ensuring a comfortable, accessible, and ergonomic typing experience for developers of all backgrounds, including those with visual sensitivities, color vision deficiencies, motor impairments, or those working exclusively in terminal environments.

---

## Ergonomic & Accessibility Principles

### 1. Stationary Centered Cursor Physics
Traditional typing tutors force the user's eyes to scan rapidly across lines and jump across the screen, causing eye fatigue and visual disorientation.

`kktypr` implements a **stationary typewriter cursor layout**:
- The active character position remains anchored horizontally and vertically in the center of the terminal viewport.
- Upcoming text advances smoothly toward the cursor point, minimizing saccadic eye movement and reducing visual strain during extended practice sessions.

### 2. High-Contrast Semantic Color Palette
Visual feedback is designed for clarity across both light and dark terminal emulators (`src/ui/theme.rs`):
- **Background Respect**: The canvas background uses `Color::Reset`, ensuring full compatibility with user-defined terminal themes without unwanted color banding or jarring background mismatches.
- **Distinct Character States**:
  - Correct input: Bright high-visibility green (`Color::Green`).
  - Incorrect input: Bold white text on high-contrast red background (`Color::Red`).
  - Upcoming text: Muted gray (`Color::DarkGray`), maintaining a clear visual hierarchy.
  - Active target character: Inverse highlight (Black text on White background) with underline modifier for unambiguous focus.

### 3. Finger Zone Mapping & Color Distinction
For learners using visual finger cues (`src/ui/keyboard.rs`):
- On-screen keyboard keys are categorized by finger responsibility (Pinky, Ring, Middle, Index, Thumb).
- Color accents are paired with explicit on-screen text labels indicating finger names (e.g., "Left Index", "Thumb (Space)"), ensuring accessibility for users with color vision deficiencies.

### 4. 100% Keyboard-Driven Navigation
`kktypr` requires zero mouse interaction:
- **Full Navigation**: Menus, lesson selectors, speed tests, and profile dashboards operate entirely through standard arrow keys or Vim-style navigation (`h`, `j`, `k`, `l`).
- **Quick Controls**: Instant escape / exit via `Esc` or `q`, and immediate lesson restart via `Tab` or dedicated keyboard shortcut.

### 5. Terminal & Screen Reader Considerations
- `kktypr` operates within raw terminal alternate screens via `crossterm`.
- A panic hook ensures that even in the event of unexpected termination, terminal raw mode is safely disabled and the alternate screen is exited cleanly, leaving the user's shell in a stable, usable state.
- We welcome feedback and contributions from users running specialized terminal screen readers to improve live feedback announcements during typing drills.

---

## Feedback & Accessibility Issues

If you encounter an accessibility barrier or have suggestions for enhancing contrast, keyboard ergonomics, or screen reader support, please file an issue on GitHub or reach out to Kushagra Kumar ([@kk376](https://github.com/kk376)) at `kkushagra86@gmail.com`.
