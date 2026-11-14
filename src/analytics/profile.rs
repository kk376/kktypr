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

    pub fn reset(&mut self) {
        let preserved_name = self.name.clone();
        *self = Self {
            name: preserved_name,
            ..Default::default()
        };
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

    #[cfg_attr(test, allow(dead_code))]
    pub fn default_storage_path() -> PathBuf {
        if let Some(proj) = ProjectDirs::from("com", "kktypr", "kktypr") {
            let data_dir = proj.data_local_dir();
            data_dir.join("profile.json")
        } else {
            PathBuf::from("profile.json")
        }
    }
}

/// Registry of multiple typist profiles, managing profile switching, adding, and renaming.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileStore {
    pub active_name: String,
    pub profiles: Vec<UserProfile>,
    #[serde(skip)]
    pub is_first_run: bool,
}

impl Default for ProfileStore {
    fn default() -> Self {
        let def = UserProfile::default();
        Self {
            active_name: def.name.clone(),
            profiles: vec![def],
            is_first_run: true,
        }
    }
}

impl ProfileStore {
    pub fn active(&self) -> &UserProfile {
        self.profiles
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(&self.active_name))
            .or_else(|| self.profiles.first())
            .expect("ProfileStore must contain at least one profile")
    }

    pub fn update_active(&mut self, profile: &UserProfile) {
        if let Some(idx) = self.profiles.iter().position(|p| p.name.eq_ignore_ascii_case(&self.active_name)) {
            self.profiles[idx] = profile.clone();
            self.active_name = profile.name.clone();
        } else {
            self.profiles.push(profile.clone());
            self.active_name = profile.name.clone();
        }
    }

    pub fn switch_profile(&mut self, name: &str) -> bool {
        if let Some(p) = self.profiles.iter().find(|p| p.name.eq_ignore_ascii_case(name)) {
            self.active_name = p.name.clone();
            true
        } else {
            false
        }
    }

    pub fn add_profile(&mut self, name: &str) -> Result<(), &'static str> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err("Profile name cannot be blank.");
        }
        if trimmed.chars().count() > 24 {
            return Err("Profile name must be 24 characters or less.");
        }
        if self.profiles.iter().any(|p| p.name.eq_ignore_ascii_case(trimmed)) {
            return Err("A profile with this name already exists.");
        }
        let new_profile = UserProfile::new(trimmed);
        self.active_name = new_profile.name.clone();
        self.profiles.push(new_profile);
        Ok(())
    }

    pub fn rename_profile(&mut self, old_name: &str, new_name: &str) -> Result<(), &'static str> {
        let trimmed = new_name.trim();
        if trimmed.is_empty() {
            return Err("Profile name cannot be blank.");
        }
        if trimmed.chars().count() > 24 {
            return Err("Profile name must be 24 characters or less.");
        }
        if !old_name.eq_ignore_ascii_case(trimmed)
            && self.profiles.iter().any(|p| p.name.eq_ignore_ascii_case(trimmed))
        {
            return Err("A profile with this name already exists.");
        }

        if let Some(p) = self.profiles.iter_mut().find(|p| p.name.eq_ignore_ascii_case(old_name)) {
            p.name = trimmed.to_string();
            if self.active_name.eq_ignore_ascii_case(old_name) {
                self.active_name = trimmed.to_string();
            }
            Ok(())
        } else {
            Err("Profile not found.")
        }
    }

    pub fn delete_profile(&mut self, name: &str) -> Result<(), &'static str> {
        if self.profiles.len() <= 1 {
            return Err("Cannot delete the only existing profile.");
        }
        let pos = self
            .profiles
            .iter()
            .position(|p| p.name.eq_ignore_ascii_case(name))
            .ok_or("Profile not found.")?;
        self.profiles.remove(pos);
        if self.active_name.eq_ignore_ascii_case(name) {
            self.active_name = self.profiles[0].name.clone();
        }
        Ok(())
    }

    #[cfg_attr(test, allow(dead_code))]
    pub fn storage_path() -> PathBuf {
        if let Some(proj) = ProjectDirs::from("com", "kktypr", "kktypr") {
            let data_dir = proj.data_local_dir();
            data_dir.join("profiles.json")
        } else {
            PathBuf::from("profiles.json")
        }
    }

    pub fn load_or_default() -> Self {
        #[cfg(test)]
        {
            Self::default()
        }
        #[cfg(not(test))]
        {
            let path = Self::storage_path();
            if path.exists()
                && let Ok(content) = fs::read_to_string(&path)
                && let Ok(mut store) = serde_json::from_str::<ProfileStore>(&content)
            {
                if store.profiles.is_empty() {
                    store = Self::default();
                }
                store.is_first_run = false;
                return store;
            }

            // Check legacy single-profile file
            let legacy_path = UserProfile::default_storage_path();
            if legacy_path.exists()
                && let Ok(content) = fs::read_to_string(&legacy_path)
                && let Ok(legacy_profile) = serde_json::from_str::<UserProfile>(&content)
            {
                let store = ProfileStore {
                    active_name: legacy_profile.name.clone(),
                    profiles: vec![legacy_profile],
                    is_first_run: false,
                };
                let _ = store.save();
                return store;
            }

            // Clean first run
            Self::default()
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        #[cfg(test)]
        {
            Ok(())
        }
        #[cfg(not(test))]
        {
            self.save_to_path(&Self::storage_path())
        }
    }

    pub fn save_to_path(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::other(e.to_string()))?;

        let tmp_path = path.with_extension("tmp");
        fs::write(&tmp_path, json)?;
        fs::rename(&tmp_path, path)?;
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

        profile.reset();
        assert!(!profile.is_exercise_completed("basics", 1, 0));
        assert_eq!(profile.total_sessions, 0);
        assert_eq!(profile.best_gross_wpm, 0.0);
        assert_eq!(profile.best_net_wpm, 0.0);
        assert_eq!(profile.name, "Kushagra");
    }

    #[test]
    fn test_profile_store_crud() {
        let mut store = ProfileStore::default();
        assert_eq!(store.active().name, "Typist");

        // Add profile
        assert!(store.add_profile("Kushagra").is_ok());
        assert_eq!(store.active().name, "Kushagra");
        assert_eq!(store.profiles.len(), 2);

        // Disallow duplicate names or overly long names
        assert!(store.add_profile("kushagra").is_err());
        assert!(store.add_profile("").is_err());
        assert!(store.add_profile("ThisNameIsWayTooLongForATypistProfile").is_err());

        // Switch profile
        assert!(store.switch_profile("Typist"));
        assert_eq!(store.active().name, "Typist");

        // Rename profile
        assert!(store.rename_profile("Typist", "Guest").is_ok());
        assert_eq!(store.active().name, "Guest");

        // Delete profile
        assert!(store.delete_profile("Guest").is_ok());
        assert_eq!(store.profiles.len(), 1);
        assert_eq!(store.active().name, "Kushagra");

        // Cannot delete only remaining profile
        assert!(store.delete_profile("Kushagra").is_err());
    }

    #[test]
    fn test_profile_store_serialization_to_path() {
        let mut store = ProfileStore::default();
        store.add_profile("TestTypist").unwrap();

        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join(format!("kktypr_test_{}.json", std::process::id()));

        // Save
        assert!(store.save_to_path(&test_file).is_ok());
        assert!(test_file.exists());

        // Read back
        let content = fs::read_to_string(&test_file).unwrap();
        let loaded: ProfileStore = serde_json::from_str(&content).unwrap();
        assert_eq!(loaded.active().name, "TestTypist");
        assert_eq!(loaded.profiles.len(), 2);

        // Clean up
        let _ = fs::remove_file(test_file);
    }
}
