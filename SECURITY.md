# Security Policy & Defensive Threat Model

`kktypr` adheres to zero-trust defensive engineering standards.

---

## Supported Versions

Security updates and patches are provided for the current active release series.

| Version | Supported |
| :--- | :--- |
| **0.1.x** | :white_check_mark: |
| **< 0.1.0** | :x: |

---

## Threat Model & Security Posture

### 1. Terminal Raw Mode & Panic Recovery
- **Architecture**: `kktypr` uses Crossterm and Ratatui to enter terminal raw mode and alternate screen buffer for low-latency keystroke handling.
- **Risk**: Unhandled application panic or SIGINT leaving the host terminal emulator in a bricked state (raw mode active, cursor hidden, mouse capture trapped, echoing disabled).
- **Mitigation & Invariants**:
  - Global panic hook (`std::panic::set_hook`) intercepts all panics, guaranteeing immediate execution of `disable_raw_mode()`, `LeaveAlternateScreen`, and `ShowCursor` before propagating panic traces.
  - Signal traps ensure clean terminal cleanup on graceful SIGINT/SIGTERM termination.

### 2. State Persistence & Deserialization Boundaries
- **Architecture**: Typing accuracy, weak-key heatmaps, and lesson history persist to disk (`~/.local/state/kktypr/` or platform data directory).
- **Risk**: Concurrent process race conditions, symlink traversal, or partially written state files causing deserialization failure or data truncation.
- **Mitigation & Invariants**:
  - Atomic write strategy: Data is serialized via `serde_json` to a unique temporary file alongside the target directory, synced, and renamed atomically via `std::fs::rename`.
  - Fallback error isolation: Corrupted or unreadable JSON files are gracefully logged and backed up without aborting the typing engine.

### 3. Curriculum & Input Processing
- **Architecture**: Developers practice typing across custom or built-in programming syntax tokens (Rust, Python, Bash, SQL).
- **Risk**: Malformed UTF-8 payloads, ANSI terminal escape injection, or boundary out-of-bounds access in text rendering buffers.
- **Mitigation & Invariants**:
  - Keystroke and cursor slicing are strictly character- and grapheme-boundary checked.
  - Zero-subprocess execution: `kktypr` does not spawn external shells or invoke `/bin/sh` to parse or evaluate drill text.

---

## Reporting a Vulnerability

If you discover a security vulnerability or potential defensive flaw in `kktypr`, please report it responsibly:

1. **GitHub Security Advisory (Recommended)**:
   - Navigate to the **Security** tab of the `kktypr` repository (`https://github.com/kk376/kktypr/security/advisories`).
   - Click **Report a vulnerability** to file a private advisory.
2. **Direct Contact**:
   - Email maintainer Kushagra Kumar directly at [kkushagra86@gmail.com](mailto:kkushagra86@gmail.com).

### Response SLA
- **Initial Acknowledgment**: Within 24-48 hours.
- **Triage & Patch Timeline**: Patch provided in a private branch within 7 days, followed by coordinated disclosure and release tagging.
- Please do not open public GitHub issues detailing reproducible exploit payloads until a patch is released.
