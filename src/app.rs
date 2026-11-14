use std::time::Duration;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::analytics::profile::{ProfileStore, UserProfile};
use crate::analytics::targeted::TargetedDrillGenerator;
use crate::engine::curriculum::{Course, Exercise};
use crate::engine::metrics::TypingMetrics;
use crate::engine::session::TypingSession;
use crate::game::word_drop::WordDropGame;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    MainMenu,
    CourseSelect,
    LessonSelect,
    TypingActive,
    Results,
    SpeedTestSelect,
    TargetedReview,
    WordDropGame,
    Stats,
    ProfileManager,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamePromptPurpose {
    FirstRun,
    AfterReset { pending_screen: AppScreen },
    RenameProfile { old_name: String },
    AddNewProfile,
}

#[derive(Debug, Clone)]
pub struct NamePromptState {
    pub purpose: NamePromptPurpose,
    pub input: String,
    pub error_msg: Option<String>,
}

pub struct App {
    pub screen: AppScreen,
    pub profile_store: ProfileStore,
    pub profile: UserProfile,
    pub courses: Vec<Course>,
    pub selected_menu_idx: usize,
    pub selected_course_idx: usize,
    pub selected_lesson_idx: usize,
    pub selected_exercise_idx: usize,
    pub selected_profile_idx: usize,
    pub active_session: Option<TypingSession>,
    pub last_metrics: Option<TypingMetrics>,
    pub game: WordDropGame,
    pub confirm_reset_stats: bool,
    pub needs_name_prompt_on_next_mode: bool,
    pub name_prompt: Option<NamePromptState>,
    pub status_notification: Option<String>,
    pub session_origin: AppScreen,
    pub last_speed_test_secs: u64,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let profile_store = ProfileStore::load_or_default();
        let profile = profile_store.active().clone();
        let is_first_run = profile_store.is_first_run;
        let courses = vec![
            Course::touch_typing_basics(),
            Course::developer_special(),
        ];

        let name_prompt = if is_first_run {
            Some(NamePromptState {
                purpose: NamePromptPurpose::FirstRun,
                input: String::new(),
                error_msg: None,
            })
        } else {
            None
        };

        Self {
            screen: AppScreen::MainMenu,
            profile_store,
            profile,
            courses,
            selected_menu_idx: 0,
            selected_course_idx: 0,
            selected_lesson_idx: 0,
            selected_exercise_idx: 0,
            selected_profile_idx: 0,
            active_session: None,
            last_metrics: None,
            game: WordDropGame::new(80, 24),
            confirm_reset_stats: false,
            needs_name_prompt_on_next_mode: false,
            name_prompt,
            status_notification: None,
            session_origin: AppScreen::MainMenu,
            last_speed_test_secs: 60,
            should_quit: false,
        }
    }

    pub fn sync_and_save_profile(&mut self) {
        self.profile_store.update_active(&self.profile);
        let _ = self.profile_store.save();
    }

    pub fn sync_active_from_store(&mut self) {
        self.profile = self.profile_store.active().clone();
    }

    pub fn navigate_to_mode(&mut self, target: AppScreen) {
        if self.needs_name_prompt_on_next_mode {
            self.name_prompt = Some(NamePromptState {
                purpose: NamePromptPurpose::AfterReset { pending_screen: target },
                input: String::new(),
                error_msg: None,
            });
        } else {
            self.screen = target;
        }
    }

    pub fn current_course(&self) -> &Course {
        &self.courses[self.selected_course_idx]
    }

    pub fn current_exercise(&self) -> Option<&Exercise> {
        self.courses
            .get(self.selected_course_idx)
            .and_then(|c| c.lessons.get(self.selected_lesson_idx))
            .and_then(|l| l.exercises.get(self.selected_exercise_idx))
    }

    pub fn is_current_exercise_key_drill(&self) -> bool {
        if self.session_origin != AppScreen::LessonSelect {
            return false;
        }
        self.current_exercise()
            .map(|ex| ex.is_key_drill())
            .unwrap_or(false)
    }

    pub fn start_current_exercise(&mut self) {
        if let Some(exercise) = self.current_exercise() {
            let session = TypingSession::new(&exercise.target_text, None);
            self.active_session = Some(session);
            self.session_origin = AppScreen::LessonSelect;
            self.screen = AppScreen::TypingActive;
        }
    }

    pub fn start_speed_test(&mut self, duration_secs: u64) {
        let test_text = "Software engineering is the systematic application of engineering approaches to the development of software. Clean room design ensures that software implementations remain strictly independent of proprietary reference code. High performance terminal user interfaces provide immediate feedback, predictable latency, and maximum developer focus. Touch typing empowers engineers to express ideas without cognitive friction.";
        let session = TypingSession::new(test_text, Some(Duration::from_secs(duration_secs)));
        self.active_session = Some(session);
        self.session_origin = AppScreen::SpeedTestSelect;
        self.last_speed_test_secs = duration_secs;
        self.screen = AppScreen::TypingActive;
    }

    pub fn start_targeted_drill(&mut self) {
        let weakest: Vec<char> = self
            .profile
            .heatmap
            .weakest_keys(5, 3)
            .into_iter()
            .map(|(c, _, _)| c)
            .collect();

        let drill_text = TargetedDrillGenerator::generate_drill(&weakest, 25);
        let session = TypingSession::new(&drill_text, None);
        self.active_session = Some(session);
        self.session_origin = AppScreen::TargetedReview;
        self.screen = AppScreen::TypingActive;
    }

    pub fn on_tick(&mut self) {
        if self.screen == AppScreen::WordDropGame {
            self.game.update();
            if self.game.score > self.profile.game_high_score {
                self.profile.game_high_score = self.game.score;
                self.sync_and_save_profile();
            }
        } else if self.screen == AppScreen::TypingActive
            && let Some(ref session) = self.active_session
                && session.is_finished() {
                    self.finish_active_session();
                }
    }

    pub fn finish_active_session(&mut self) {
        if let Some(session) = self.active_session.take() {
            let metrics = session.current_metrics();
            
            // Record keystrokes into user heatmap
            for k in session.keystrokes() {
                self.profile.heatmap.record(k.expected, k.is_correct);
            }

            // Record completion if in course mode and accuracy threshold met
            if self.session_origin == AppScreen::LessonSelect && self.selected_course_idx < self.courses.len() {
                let course_id = self.current_course().id.clone();
                if let Some(ex) = self.current_exercise()
                    && metrics.accuracy_pct >= ex.minimum_accuracy {
                        self.profile.record_exercise_completion(
                            &course_id,
                            self.selected_lesson_idx + 1,
                            self.selected_exercise_idx,
                            &metrics,
                        );
                    }
            } else {
                self.profile.record_session_metrics(&metrics);
            }

            self.sync_and_save_profile();
            self.last_metrics = Some(metrics);
            self.screen = AppScreen::Results;
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        // Global quit on Ctrl+C
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }

        // Modal name prompt intercepts all keys when active
        if self.name_prompt.is_some() {
            self.handle_name_prompt_key(key);
            return;
        }

        match self.screen {
            AppScreen::MainMenu => self.handle_menu_key(key),
            AppScreen::CourseSelect => self.handle_course_select_key(key),
            AppScreen::LessonSelect => self.handle_lesson_select_key(key),
            AppScreen::TypingActive => self.handle_typing_key(key),
            AppScreen::Results => self.handle_results_key(key),
            AppScreen::SpeedTestSelect => self.handle_speed_test_menu_key(key),
            AppScreen::TargetedReview => self.handle_targeted_review_key(key),
            AppScreen::WordDropGame => self.handle_game_key(key),
            AppScreen::Stats => self.handle_stats_key(key),
            AppScreen::ProfileManager => self.handle_profile_manager_key(key),
        }
    }

    fn handle_name_prompt_key(&mut self, key: KeyEvent) {
        let mut prompt = match self.name_prompt.take() {
            Some(p) => p,
            None => return,
        };

        match key.code {
            KeyCode::Esc => match prompt.purpose {
                NamePromptPurpose::FirstRun => {
                    self.profile.name = "Typist".to_string();
                    self.sync_and_save_profile();
                }
                NamePromptPurpose::AfterReset { pending_screen } => {
                    self.profile.name = "Typist".to_string();
                    self.needs_name_prompt_on_next_mode = false;
                    self.sync_and_save_profile();
                    self.screen = pending_screen;
                }
                _ => {}
            },
            KeyCode::Backspace => {
                prompt.input.pop();
                prompt.error_msg = None;
                self.name_prompt = Some(prompt);
            }
            KeyCode::Char(c) => {
                if prompt.input.len() < 24 && (c.is_alphanumeric() || c == ' ' || c == '_' || c == '-') {
                    prompt.input.push(c);
                    prompt.error_msg = None;
                }
                self.name_prompt = Some(prompt);
            }
            KeyCode::Enter => {
                let name = prompt.input.trim().to_string();
                match prompt.purpose {
                    NamePromptPurpose::FirstRun => {
                        let final_name = if name.is_empty() { "Typist".to_string() } else { name };
                        self.profile.name = final_name;
                        self.sync_and_save_profile();
                        self.status_notification = Some(format!("Profile registered for '{}'.", self.profile.name));
                    }
                    NamePromptPurpose::AfterReset { pending_screen } => {
                        let final_name = if name.is_empty() { "Typist".to_string() } else { name };
                        self.profile.name = final_name;
                        self.needs_name_prompt_on_next_mode = false;
                        self.sync_and_save_profile();
                        self.status_notification = Some(format!("Typist set to '{}'.", self.profile.name));
                        self.screen = pending_screen;
                    }
                    NamePromptPurpose::RenameProfile { ref old_name } => {
                        if name.is_empty() {
                            prompt.error_msg = Some("Name cannot be empty.".to_string());
                            self.name_prompt = Some(prompt);
                            return;
                        }
                        match self.profile_store.rename_profile(old_name, &name) {
                            Ok(()) => {
                                self.sync_active_from_store();
                                let _ = self.profile_store.save();
                                self.status_notification = Some(format!("Renamed profile to '{}'.", name));
                            }
                            Err(e) => {
                                prompt.error_msg = Some(e.to_string());
                                self.name_prompt = Some(prompt);
                            }
                        }
                    }
                    NamePromptPurpose::AddNewProfile => {
                        if name.is_empty() {
                            prompt.error_msg = Some("Name cannot be empty.".to_string());
                            self.name_prompt = Some(prompt);
                            return;
                        }
                        match self.profile_store.add_profile(&name) {
                            Ok(()) => {
                                self.sync_active_from_store();
                                let _ = self.profile_store.save();
                                self.selected_profile_idx = self.profile_store.profiles.len().saturating_sub(1);
                                self.status_notification = Some(format!("Created profile '{}'.", name));
                            }
                            Err(e) => {
                                prompt.error_msg = Some(e.to_string());
                                self.name_prompt = Some(prompt);
                            }
                        }
                    }
                }
            }
            _ => {
                self.name_prompt = Some(prompt);
            }
        }
    }

    fn handle_menu_key(&mut self, key: KeyEvent) {
        if self.confirm_reset_stats {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    self.profile.reset();
                    self.sync_and_save_profile();
                    self.confirm_reset_stats = false;
                    self.needs_name_prompt_on_next_mode = true;
                    self.status_notification = Some("Stats reset. You will be prompted for your name on your next exercise.".to_string());
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.confirm_reset_stats = false;
                    self.status_notification = Some("Reset cancelled.".to_string());
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_menu_idx > 0 {
                    self.selected_menu_idx -= 1;
                }
                self.status_notification = None;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_menu_idx < 7 {
                    self.selected_menu_idx += 1;
                }
                self.status_notification = None;
            }
            KeyCode::Char('r') => {
                self.confirm_reset_stats = true;
                self.status_notification = None;
            }
            KeyCode::Enter => match self.selected_menu_idx {
                0 => self.navigate_to_mode(AppScreen::CourseSelect),
                1 => self.navigate_to_mode(AppScreen::SpeedTestSelect),
                2 => self.navigate_to_mode(AppScreen::TargetedReview),
                3 => {
                    self.game.reset(80, 24);
                    self.navigate_to_mode(AppScreen::WordDropGame);
                }
                4 => self.screen = AppScreen::Stats,
                5 => {
                    self.selected_profile_idx = self
                        .profile_store
                        .profiles
                        .iter()
                        .position(|p| p.name.eq_ignore_ascii_case(&self.profile_store.active_name))
                        .unwrap_or(0);
                    self.screen = AppScreen::ProfileManager;
                }
                6 => {
                    self.confirm_reset_stats = true;
                    self.status_notification = None;
                }
                7 => self.should_quit = true,
                _ => {}
            },
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            _ => {}
        }
    }

    fn handle_profile_manager_key(&mut self, key: KeyEvent) {
        let total = self.profile_store.profiles.len();
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_profile_idx > 0 {
                    self.selected_profile_idx -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_profile_idx + 1 < total {
                    self.selected_profile_idx += 1;
                }
            }
            KeyCode::Enter => {
                if let Some(target_p) = self.profile_store.profiles.get(self.selected_profile_idx) {
                    let target_name = target_p.name.clone();
                    self.profile_store.switch_profile(&target_name);
                    self.sync_active_from_store();
                    let _ = self.profile_store.save();
                    self.status_notification = Some(format!("Switched to profile '{}'.", target_name));
                }
            }
            KeyCode::Char('a') => {
                self.name_prompt = Some(NamePromptState {
                    purpose: NamePromptPurpose::AddNewProfile,
                    input: String::new(),
                    error_msg: None,
                });
            }
            KeyCode::Char('r') => {
                if let Some(target_p) = self.profile_store.profiles.get(self.selected_profile_idx) {
                    let old_name = target_p.name.clone();
                    self.name_prompt = Some(NamePromptState {
                        purpose: NamePromptPurpose::RenameProfile { old_name },
                        input: String::new(),
                        error_msg: None,
                    });
                }
            }
            KeyCode::Char('d') => {
                if let Some(target_p) = self.profile_store.profiles.get(self.selected_profile_idx) {
                    let name_to_del = target_p.name.clone();
                    match self.profile_store.delete_profile(&name_to_del) {
                        Ok(()) => {
                            self.sync_active_from_store();
                            let _ = self.profile_store.save();
                            self.selected_profile_idx = self
                                .selected_profile_idx
                                .min(self.profile_store.profiles.len().saturating_sub(1));
                            self.status_notification = Some(format!("Profile '{}' deleted.", name_to_del));
                        }
                        Err(e) => {
                            self.status_notification = Some(format!("Cannot delete: {}", e));
                        }
                    }
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.screen = AppScreen::MainMenu;
            }
            _ => {}
        }
    }

    fn handle_course_select_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_course_idx > 0 {
                    self.selected_course_idx -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_course_idx + 1 < self.courses.len() {
                    self.selected_course_idx += 1;
                }
            }
            KeyCode::Enter => {
                self.selected_lesson_idx = 0;
                self.screen = AppScreen::LessonSelect;
            }
            KeyCode::Esc | KeyCode::Char('q') => self.screen = AppScreen::MainMenu,
            _ => {}
        }
    }

    fn handle_lesson_select_key(&mut self, key: KeyEvent) {
        let total_lessons = self.current_course().lessons.len();
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_lesson_idx > 0 {
                    self.selected_lesson_idx -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_lesson_idx + 1 < total_lessons {
                    self.selected_lesson_idx += 1;
                }
            }
            KeyCode::Enter => {
                self.selected_exercise_idx = 0;
                self.start_current_exercise();
            }
            KeyCode::Esc | KeyCode::Char('q') => self.screen = AppScreen::CourseSelect,
            _ => {}
        }
    }

    fn handle_typing_key(&mut self, key: KeyEvent) {
        if let Some(ref mut session) = self.active_session {
            match key.code {
                KeyCode::Esc => {
                    self.active_session = None;
                    self.screen = self.session_origin;
                }
                KeyCode::Backspace => {
                    session.pop_char();
                }
                KeyCode::Char(c) => {
                    session.push_char(c);
                    if session.is_finished() {
                        self.finish_active_session();
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_results_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('r') | KeyCode::Char('R') => {
                match self.session_origin {
                    AppScreen::LessonSelect => self.start_current_exercise(),
                    AppScreen::SpeedTestSelect => self.start_speed_test(self.last_speed_test_secs),
                    AppScreen::TargetedReview => self.start_targeted_drill(),
                    _ => self.start_current_exercise(),
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Enter => {
                match self.session_origin {
                    AppScreen::LessonSelect => {
                        // Advance to next exercise or lesson
                        if let Some(course) = self.courses.get(self.selected_course_idx)
                            && let Some(lesson) = course.lessons.get(self.selected_lesson_idx) {
                                if self.selected_exercise_idx + 1 < lesson.exercises.len() {
                                    self.selected_exercise_idx += 1;
                                    self.start_current_exercise();
                                } else if self.selected_lesson_idx + 1 < course.lessons.len() {
                                    self.selected_lesson_idx += 1;
                                    self.selected_exercise_idx = 0;
                                    self.start_current_exercise();
                                } else {
                                    self.screen = AppScreen::LessonSelect;
                                }
                            }
                    }
                    _ => {
                        self.screen = self.session_origin;
                    }
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.screen = self.session_origin;
            }
            KeyCode::Char('m') => {
                self.screen = AppScreen::MainMenu;
            }
            _ => {}
        }
    }

    fn handle_speed_test_menu_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('1') => self.start_speed_test(60),
            KeyCode::Char('2') => self.start_speed_test(120),
            KeyCode::Char('5') => self.start_speed_test(300),
            KeyCode::Esc | KeyCode::Char('q') => self.screen = AppScreen::MainMenu,
            _ => {}
        }
    }

    fn handle_targeted_review_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter | KeyCode::Char('p') => {
                self.start_targeted_drill();
            }
            KeyCode::Esc | KeyCode::Char('q') => self.screen = AppScreen::MainMenu,
            _ => {}
        }
    }

    fn handle_game_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.screen = AppScreen::MainMenu,
            KeyCode::Char(c) => {
                if self.game.state == crate::game::word_drop::GameState::GameOver {
                    if c == 'r' || c == 'R' {
                        self.game.reset(80, 24);
                    } else if c == 'q' || c == 'Q' {
                        self.screen = AppScreen::MainMenu;
                    }
                } else {
                    self.game.handle_char(c);
                }
            }
            _ => {}
        }
    }

    fn handle_stats_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter => self.screen = AppScreen::MainMenu,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyEventState;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
            kind: crossterm::event::KeyEventKind::Press,
            state: KeyEventState::empty(),
        }
    }

    #[test]
    fn test_app_reset_stats_workflow() {
        let mut app = App::new();
        app.name_prompt = None;
        app.profile.best_gross_wpm = 75.0;
        app.profile.total_sessions = 10;
        app.profile.completed_exercises.insert("basics:1:0".to_string());

        // Press 'r' in main menu triggers confirmation
        app.handle_key_event(key(KeyCode::Char('r')));
        assert!(app.confirm_reset_stats);

        // Cancel with 'n'
        app.handle_key_event(key(KeyCode::Char('n')));
        assert!(!app.confirm_reset_stats);
        assert_eq!(app.profile.best_gross_wpm, 75.0);

        // Select item 6 (Reset All Statistics) and press Enter
        app.selected_menu_idx = 6;
        app.handle_key_event(key(KeyCode::Enter));
        assert!(app.confirm_reset_stats);

        // Confirm with 'y'
        app.handle_key_event(key(KeyCode::Char('y')));
        assert!(!app.confirm_reset_stats);
        assert_eq!(app.profile.best_gross_wpm, 0.0);
        assert_eq!(app.profile.total_sessions, 0);
        assert!(app.profile.completed_exercises.is_empty());
        assert!(app.status_notification.is_some());
        assert!(app.needs_name_prompt_on_next_mode);

        // Now select a typing mode (e.g. Academy, index 0)
        app.selected_menu_idx = 0;
        app.handle_key_event(key(KeyCode::Enter));
        // Name prompt modal should appear before entering Academy!
        assert!(app.name_prompt.is_some());
        assert_eq!(app.screen, AppScreen::MainMenu);

        // Enter typist name "Linus" and submit Enter
        for c in "Linus".chars() {
            app.handle_key_event(key(KeyCode::Char(c)));
        }
        app.handle_key_event(key(KeyCode::Enter));
        assert!(app.name_prompt.is_none());
        assert_eq!(app.profile.name, "Linus");
        assert_eq!(app.screen, AppScreen::CourseSelect);
        assert!(!app.needs_name_prompt_on_next_mode);
    }

    #[test]
    fn test_profile_manager_navigation_and_crud() {
        let mut app = App::new();
        // Clear any first-run prompt in test
        app.name_prompt = None;

        // Navigate to Profile Manager (Menu index 5)
        app.selected_menu_idx = 5;
        app.handle_key_event(key(KeyCode::Enter));
        assert_eq!(app.screen, AppScreen::ProfileManager);

        // Press 'a' to add a profile
        app.handle_key_event(key(KeyCode::Char('a')));
        assert!(app.name_prompt.is_some());

        // Type "Alice" and Enter
        for c in "Alice".chars() {
            app.handle_key_event(key(KeyCode::Char(c)));
        }
        app.handle_key_event(key(KeyCode::Enter));
        assert!(app.name_prompt.is_none());
        assert_eq!(app.profile_store.profiles.len(), 2);
        assert_eq!(app.profile.name, "Alice");

        // Rename active profile: press 'r'
        app.handle_key_event(key(KeyCode::Char('r')));
        assert!(app.name_prompt.is_some());

        for c in "Bob".chars() {
            app.handle_key_event(key(KeyCode::Char(c)));
        }
        app.handle_key_event(key(KeyCode::Enter));
        assert!(app.name_prompt.is_none());
        assert_eq!(app.profile.name, "Bob");

        // Esc returns to MainMenu
        app.handle_key_event(key(KeyCode::Esc));
        assert_eq!(app.screen, AppScreen::MainMenu);
    }

    #[test]
    fn test_navigation_hierarchy_backwards_progression() {
        let mut app = App::new();
        app.name_prompt = None;

        // 1. From MainMenu -> Touch Typing Academy (Index 0)
        app.selected_menu_idx = 0;
        app.handle_key_event(key(KeyCode::Enter));
        assert_eq!(app.screen, AppScreen::CourseSelect);

        // 2. From CourseSelect -> Course 1 (Enter)
        app.selected_course_idx = 0;
        app.handle_key_event(key(KeyCode::Enter));
        assert_eq!(app.screen, AppScreen::LessonSelect);

        // 3. From LessonSelect -> Lesson 1 (Enter)
        app.selected_lesson_idx = 0;
        app.handle_key_event(key(KeyCode::Enter));
        assert_eq!(app.screen, AppScreen::TypingActive);
        assert!(app.active_session.is_some());

        // 4. In TypingActive, press Esc -> should return to LessonSelect!
        app.handle_key_event(key(KeyCode::Esc));
        assert_eq!(app.screen, AppScreen::LessonSelect);
        assert!(app.active_session.is_none());

        // 5. In LessonSelect, press Esc -> should return to CourseSelect!
        app.handle_key_event(key(KeyCode::Esc));
        assert_eq!(app.screen, AppScreen::CourseSelect);

        // 6. In CourseSelect, press Esc -> should return to MainMenu!
        app.handle_key_event(key(KeyCode::Esc));
        assert_eq!(app.screen, AppScreen::MainMenu);

        // 7. Results screen: finish an exercise, test Esc returns to LessonSelect
        app.selected_menu_idx = 0;
        app.handle_key_event(key(KeyCode::Enter)); // CourseSelect
        app.handle_key_event(key(KeyCode::Enter)); // LessonSelect
        app.handle_key_event(key(KeyCode::Enter)); // TypingActive
        app.finish_active_session();
        assert_eq!(app.screen, AppScreen::Results);
        app.handle_key_event(key(KeyCode::Esc));
        assert_eq!(app.screen, AppScreen::LessonSelect);

        // 8. Speed Tests backward progression
        app.screen = AppScreen::MainMenu;
        app.selected_menu_idx = 1;
        app.handle_key_event(key(KeyCode::Enter));
        assert_eq!(app.screen, AppScreen::SpeedTestSelect);
        app.handle_key_event(key(KeyCode::Char('1')));
        assert_eq!(app.screen, AppScreen::TypingActive);
        app.handle_key_event(key(KeyCode::Esc));
        assert_eq!(app.screen, AppScreen::SpeedTestSelect);
        app.handle_key_event(key(KeyCode::Esc));
        assert_eq!(app.screen, AppScreen::MainMenu);

        // 9. Targeted Review backward progression
        app.selected_menu_idx = 2;
        app.handle_key_event(key(KeyCode::Enter));
        assert_eq!(app.screen, AppScreen::TargetedReview);
        app.handle_key_event(key(KeyCode::Enter));
        assert_eq!(app.screen, AppScreen::TypingActive);
        app.handle_key_event(key(KeyCode::Esc));
        assert_eq!(app.screen, AppScreen::TargetedReview);
        app.handle_key_event(key(KeyCode::Esc));
        assert_eq!(app.screen, AppScreen::MainMenu);
    }

    #[test]
    fn test_app_key_drill_vs_prose_mode_resolution() {
        let mut app = App::new();

        // 1. In Lesson 1 Exercise 0 (Home Row Warm-Up): should be key drill
        app.selected_course_idx = 0;
        app.selected_lesson_idx = 0;
        app.selected_exercise_idx = 0;
        app.start_current_exercise();
        assert!(app.is_current_exercise_key_drill());

        // 2. In Lesson 1 Exercise 1 (Word Drill): should be prose
        app.selected_exercise_idx = 1;
        app.start_current_exercise();
        assert!(!app.is_current_exercise_key_drill());

        // 3. In Lesson 1 Exercise 2 (Sentence Drill): should be prose
        app.selected_exercise_idx = 2;
        app.start_current_exercise();
        assert!(!app.is_current_exercise_key_drill());

        // 4. In Speed Test: ALWAYS prose (never key drill)
        app.start_speed_test(60);
        assert!(!app.is_current_exercise_key_drill());

        // 5. In Targeted Review: ALWAYS prose (words)
        app.start_targeted_drill();
        assert!(!app.is_current_exercise_key_drill());
    }
}
