use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct KeyMetric {
    pub total: usize,
    pub errors: usize,
    pub correct: usize,
}

impl KeyMetric {
    pub fn accuracy_pct(&self) -> f64 {
        if self.total == 0 {
            100.0
        } else {
            (self.correct as f64 / self.total as f64) * 100.0
        }
    }

    pub fn record_press(&mut self, is_correct: bool) {
        self.total += 1;
        if is_correct {
            self.correct += 1;
        } else {
            self.errors += 1;
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyHeatmap {
    pub key_metrics: HashMap<char, KeyMetric>,
}

impl KeyHeatmap {
    pub fn new() -> Self {
        Self {
            key_metrics: HashMap::new(),
        }
    }

    pub fn record(&mut self, key: char, is_correct: bool) {
        let key_lower = key.to_ascii_lowercase();
        self.key_metrics
            .entry(key_lower)
            .or_default()
            .record_press(is_correct);
    }

    pub fn get_accuracy(&self, key: char) -> f64 {
        let key_lower = key.to_ascii_lowercase();
        self.key_metrics
            .get(&key_lower)
            .map(|m| m.accuracy_pct())
            .unwrap_or(100.0)
    }

    /// Returns keys with highest error rates, filtered to keys with at least `min_attempts`.
    pub fn weakest_keys(&self, count: usize, min_attempts: usize) -> Vec<(char, f64, usize)> {
        let mut candidates: Vec<(char, f64, usize)> = self
            .key_metrics
            .iter()
            .filter(|(_, m)| m.total >= min_attempts && m.errors > 0)
            .map(|(&k, m)| (k, m.accuracy_pct(), m.errors))
            .collect();

        // Sort ascending by accuracy, then descending by error count
        candidates.sort_by(|a, b| {
            a.1.partial_cmp(&b.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.2.cmp(&a.2))
        });

        candidates.into_iter().take(count).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heatmap_recording_and_accuracy() {
        let mut heatmap = KeyHeatmap::new();
        // Record 10 presses of 'a': 9 correct, 1 error -> 90%
        for _ in 0..9 {
            heatmap.record('a', true);
        }
        heatmap.record('a', false);
        assert!((heatmap.get_accuracy('a') - 90.0).abs() < 1e-4);

        // Record 10 presses of 'z': 5 correct, 5 error -> 50%
        for _ in 0..5 {
            heatmap.record('z', true);
            heatmap.record('z', false);
        }
        assert!((heatmap.get_accuracy('z') - 50.0).abs() < 1e-4);

        let weakest = heatmap.weakest_keys(5, 5);
        assert_eq!(weakest.len(), 2);
        assert_eq!(weakest[0].0, 'z'); // lowest accuracy first
        assert_eq!(weakest[1].0, 'a');
    }
}
