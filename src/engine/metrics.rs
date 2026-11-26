use std::time::Duration;

/// Comprehensive typing metrics snapshot calculated from session input.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TypingMetrics {
    pub gross_wpm: f64,
    pub net_wpm: f64,
    pub accuracy_pct: f64,
    pub total_keystrokes: usize,
    pub correct_keystrokes: usize,
    pub error_count: usize,
    pub elapsed: Duration,
}

impl Default for TypingMetrics {
    fn default() -> Self {
        Self {
            gross_wpm: 0.0,
            net_wpm: 0.0,
            accuracy_pct: 100.0,
            total_keystrokes: 0,
            correct_keystrokes: 0,
            error_count: 0,
            elapsed: Duration::ZERO,
        }
    }
}

impl TypingMetrics {
    /// Computes typing metrics following standardized touch typing rules:
    /// - Standard word length is 5 characters (including spaces).
    /// - Gross WPM = (Total Keystrokes / 5) / Elapsed Minutes.
    /// - Net WPM = max(0, (Correct Keystrokes / 5 - Error Count) / Elapsed Minutes).
    /// - Accuracy = (Correct Keystrokes / Total Keystrokes) * 100%.
    pub fn calculate(
        total_keystrokes: usize,
        correct_keystrokes: usize,
        error_count: usize,
        elapsed: Duration,
    ) -> Self {
        let accuracy_pct = if total_keystrokes > 0 {
            ((correct_keystrokes as f64 / total_keystrokes as f64) * 100.0).clamp(0.0, 100.0)
        } else {
            100.0
        };

        if total_keystrokes == 0 || elapsed.as_secs_f64() < 0.1 {
            return Self {
                accuracy_pct,
                total_keystrokes,
                correct_keystrokes,
                error_count,
                elapsed,
                ..Default::default()
            };
        }

        let elapsed_minutes = elapsed.as_secs_f64() / 60.0;
        let gross_wpm = (total_keystrokes as f64 / 5.0) / elapsed_minutes;

        let net_chars = (correct_keystrokes as f64 / 5.0) - (error_count as f64);
        let net_wpm = (net_chars / elapsed_minutes).max(0.0);

        Self {
            gross_wpm,
            net_wpm,
            accuracy_pct,
            total_keystrokes,
            correct_keystrokes,
            error_count,
            elapsed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_time_or_zero_keystrokes_gives_default_zeros() {
        let metrics = TypingMetrics::calculate(0, 0, 0, Duration::from_secs(0));
        assert_eq!(metrics.gross_wpm, 0.0);
        assert_eq!(metrics.net_wpm, 0.0);
        assert_eq!(metrics.accuracy_pct, 100.0);
    }

    #[test]
    fn test_perfect_typing_metrics() {
        // 300 characters in 60 seconds = 60 Gross WPM and 60 Net WPM at 100% accuracy
        let metrics = TypingMetrics::calculate(300, 300, 0, Duration::from_secs(60));
        assert!((metrics.gross_wpm - 60.0).abs() < 1e-4);
        assert!((metrics.net_wpm - 60.0).abs() < 1e-4);
        assert!((metrics.accuracy_pct - 100.0).abs() < 1e-4);
        assert_eq!(metrics.error_count, 0);
    }

    #[test]
    fn test_metrics_with_errors() {
        // 300 characters typed in 60s, but 20 errors made (280 correct)
        // Gross WPM: (300 / 5) / 1.0 = 60.0
        // Net WPM: ((280 / 5) - 20) / 1.0 = (56 - 20) = 36.0 Net WPM
        // Accuracy: (280 / 300) * 100 = 93.333%
        let metrics = TypingMetrics::calculate(300, 280, 20, Duration::from_secs(60));
        assert!((metrics.gross_wpm - 60.0).abs() < 1e-4);
        assert!((metrics.net_wpm - 36.0).abs() < 1e-4);
        assert!((metrics.accuracy_pct - 93.3333).abs() < 1e-2);
        assert_eq!(metrics.error_count, 20);
    }

    #[test]
    fn test_net_wpm_clamps_at_zero_when_errors_exceed_correct() {
        // 100 total, 40 correct, 60 errors
        // (40 / 5) - 60 = 8 - 60 = -52 -> clamped to 0.0
        let metrics = TypingMetrics::calculate(100, 40, 60, Duration::from_secs(60));
        assert_eq!(metrics.net_wpm, 0.0);
        assert!((metrics.accuracy_pct - 40.0).abs() < 1e-4);
    }
}
