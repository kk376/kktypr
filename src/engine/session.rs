use std::time::{Duration, Instant};
use crate::engine::metrics::TypingMetrics;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharStatus {
    Pending,
    Correct,
    Incorrect,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeystrokeRecord {
    pub expected: char,
    pub actual: char,
    pub is_correct: bool,
    pub timestamp: Duration,
}

#[derive(Debug, Clone)]
pub struct TypingSession {
    pub target_text: String,
    target_chars: Vec<char>,
    input_chars: Vec<char>,
    char_statuses: Vec<CharStatus>,
    keystroke_history: Vec<KeystrokeRecord>,
    start_time: Option<Instant>,
    finish_time: Option<Instant>,
    pub time_limit: Option<Duration>,
    pub error_count: usize,
    pub correct_count: usize,
}

impl TypingSession {
    pub fn new(target_text: &str, time_limit: Option<Duration>) -> Self {
        let target_chars: Vec<char> = target_text.chars().collect();
        let len = target_chars.len();
        Self {
            target_text: target_text.to_string(),
            target_chars,
            input_chars: Vec::with_capacity(len),
            char_statuses: vec![CharStatus::Pending; len],
            keystroke_history: Vec::with_capacity(len * 2),
            start_time: None,
            finish_time: None,
            time_limit,
            error_count: 0,
            correct_count: 0,
        }
    }

    pub fn cursor(&self) -> usize {
        self.input_chars.len()
    }

    #[allow(dead_code)]
    pub fn is_started(&self) -> bool {
        self.start_time.is_some()
    }

    pub fn is_finished(&self) -> bool {
        if self.cursor() >= self.target_chars.len() {
            return true;
        }
        if let Some(limit) = self.time_limit
            && self.elapsed() >= limit {
                return true;
            }
        false
    }

    pub fn elapsed(&self) -> Duration {
        match (self.start_time, self.finish_time) {
            (None, _) => Duration::ZERO,
            (Some(start), Some(finish)) => finish.saturating_duration_since(start),
            (Some(start), None) => {
                let current = start.elapsed();
                if let Some(limit) = self.time_limit {
                    current.min(limit)
                } else {
                    current
                }
            }
        }
    }

    pub fn remaining_time(&self) -> Option<Duration> {
        self.time_limit.map(|limit| {
            let elapsed = self.elapsed();
            limit.saturating_sub(elapsed)
        })
    }

    pub fn target_char_at(&self, index: usize) -> Option<char> {
        self.target_chars.get(index).copied()
    }

    pub fn current_expected_char(&self) -> Option<char> {
        self.target_char_at(self.cursor())
    }

    pub fn char_statuses(&self) -> &[CharStatus] {
        &self.char_statuses
    }

    pub fn keystrokes(&self) -> &[KeystrokeRecord] {
        &self.keystroke_history
    }

    pub fn push_char(&mut self, c: char) -> bool {
        if self.is_finished() {
            return false;
        }

        let now = Instant::now();
        if self.start_time.is_none() {
            self.start_time = Some(now);
        }

        let idx = self.cursor();
        if idx >= self.target_chars.len() {
            return false;
        }

        let expected = self.target_chars[idx];
        let is_correct = c == expected;
        let elapsed = self.elapsed();

        if is_correct {
            self.correct_count += 1;
            self.char_statuses[idx] = CharStatus::Correct;
        } else {
            self.error_count += 1;
            self.char_statuses[idx] = CharStatus::Incorrect;
        }

        self.input_chars.push(c);
        self.keystroke_history.push(KeystrokeRecord {
            expected,
            actual: c,
            is_correct,
            timestamp: elapsed,
        });

        if self.cursor() >= self.target_chars.len() {
            self.finish_time = Some(now);
        }

        is_correct
    }

    pub fn pop_char(&mut self) -> bool {
        if self.input_chars.is_empty() || self.finish_time.is_some() {
            return false;
        }

        let removed_idx = self.input_chars.len() - 1;
        self.input_chars.pop();
        self.char_statuses[removed_idx] = CharStatus::Pending;
        true
    }

    pub fn current_metrics(&self) -> TypingMetrics {
        TypingMetrics::calculate(
            self.keystroke_history.len(),
            self.correct_count,
            self.error_count,
            self.elapsed(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_lifecycle_and_correctness() {
        let mut session = TypingSession::new("hello", None);
        assert!(!session.is_started());
        assert!(!session.is_finished());
        assert_eq!(session.cursor(), 0);
        assert_eq!(session.current_expected_char(), Some('h'));

        assert!(session.push_char('h'));
        assert!(session.is_started());
        assert_eq!(session.cursor(), 1);
        assert_eq!(session.char_statuses()[0], CharStatus::Correct);

        assert!(!session.push_char('x')); // incorrect
        assert_eq!(session.cursor(), 2);
        assert_eq!(session.char_statuses()[1], CharStatus::Incorrect);
        assert_eq!(session.error_count, 1);
        assert_eq!(session.correct_count, 1);

        assert!(session.pop_char()); // backspace
        assert_eq!(session.cursor(), 1);
        assert_eq!(session.char_statuses()[1], CharStatus::Pending);

        assert!(session.push_char('e'));
        assert!(session.push_char('l'));
        assert!(session.push_char('l'));
        assert!(session.push_char('o'));

        assert!(session.is_finished());
        assert_eq!(session.cursor(), 5);
        assert_eq!(session.char_statuses()[4], CharStatus::Correct);
    }
}
