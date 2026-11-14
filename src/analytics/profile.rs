use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use crate::analytics::heatmap::KeyHeatmap;
use crate::engine::metrics::TypingMetrics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub name: String,
    pub completed_exercises: HashSet<String>,
    pub best_gross_wpm: f64,
    pub best_net_wpm: f64,
    pub total_sessions: usize,
    pub game_high_score: u32,
    pub heatmap: KeyHeatmap,
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            name: "Typist".to_string(),
            completed_exercises: HashSet::new(),
            best_gross_wpm: 0.0,
            best_net_wpm: 0.0,
            total_sessions: 0,
            game_high_score: 0,
            heatmap: KeyHeatmap::new(),
        }
    }
}

impl UserProfile {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn exercise_key(course_id: &str, lesson_num: usize, exercise_idx: usize) -> String {
        format!("{}:{}:{}", course_id, lesson_num, exercise_idx)
    }

    pub fn is_exercise_completed(&self, course_id: &str, lesson_num: usize, exercise_idx: usize) -> bool {
        let key = Self::exercise_key(course_id, lesson_num, exercise_idx);
        self.completed_exercises.contains(&key)
    }

    pub fn record_exercise_completion(
        &mut self,
        course_id: &str,
        lesson_num: usize,
        exercise_idx: usize,
        metrics: &TypingMetrics,
    ) {
        let key = Self::exercise_key(course_id, lesson_num, exercise_idx);
        self.completed_exercises.insert(key);
        self.record_session_metrics(metrics);
    }

    pub fn record_session_metrics(&mut self, metrics: &TypingMetrics) {
        self.total_sessions += 1;
        if metrics.gross_wpm > self.best_gross_wpm {
            self.best_gross_wpm = metrics.gross_wpm;
        }
        if metrics.net_wpm > self.best_net_wpm {
            self.best_net_wpm = metrics.net_wpm;
        }
    }

    pub fn default_storage_path() -> PathBuf {
        if let Some(proj) = ProjectDirs::from("com", "kktypr", "kktypr") {
            let data_dir = proj.data_local_dir();
            data_dir.join("profile.json")
        } else {
            PathBuf::from("profile.json")
        }
    }

    pub fn load_or_default() -> Self {
        let path = Self::default_storage_path();
        if path.exists()
            && let Ok(content) = fs::read_to_string(&path)
                && let Ok(profile) = serde_json::from_str(&content) {
                    return profile;
                }
        Self::default()
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::default_storage_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::other(e.to_string()))?;

        // Atomic write: write to temp file then rename
        let tmp_path = path.with_extension("tmp");
        fs::write(&tmp_path, json)?;
        fs::rename(&tmp_path, &path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_exercise_completion() {
        let mut profile = UserProfile::new("Kushagra");
        assert!(!profile.is_exercise_completed("basics", 1, 0));

        let metrics = TypingMetrics {
            gross_wpm: 65.0,
            net_wpm: 62.0,
            accuracy_pct: 98.0,
            total_keystrokes: 200,
            correct_keystrokes: 196,
            error_count: 4,
            elapsed: std::time::Duration::from_secs(30),
        };

        profile.record_exercise_completion("basics", 1, 0, &metrics);
        assert!(profile.is_exercise_completed("basics", 1, 0));
        assert_eq!(profile.total_sessions, 1);
        assert_eq!(profile.best_gross_wpm, 65.0);
        assert_eq!(profile.best_net_wpm, 62.0);
    }
}
