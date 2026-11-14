use std::time::Duration;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::analytics::profile::UserProfile;
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
}

pub struct App {
    pub screen: AppScreen,
    pub profile: UserProfile,
    pub courses: Vec<Course>,
    pub selected_menu_idx: usize,
    pub selected_course_idx: usize,
    pub selected_lesson_idx: usize,
    pub selected_exercise_idx: usize,
    pub active_session: Option<TypingSession>,
    pub last_metrics: Option<TypingMetrics>,
    pub game: WordDropGame,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let profile = UserProfile::load_or_default();
        let courses = vec![
            Course::touch_typing_basics(),
            Course::developer_special(),
        ];

        Self {
            screen: AppScreen::MainMenu,
            profile,
            courses,
            selected_menu_idx: 0,
            selected_course_idx: 0,
            selected_lesson_idx: 0,
            selected_exercise_idx: 0,
            active_session: None,
            last_metrics: None,
            game: WordDropGame::new(80, 24),
            should_quit: false,
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

    pub fn start_current_exercise(&mut self) {
        if let Some(exercise) = self.current_exercise() {
            let session = TypingSession::new(&exercise.target_text, None);
            self.active_session = Some(session);
            self.screen = AppScreen::TypingActive;
        }
    }

    pub fn start_speed_test(&mut self, duration_secs: u64) {
        let test_text = "Software engineering is the systematic application of engineering approaches to the development of software. Clean room design ensures that software implementations remain strictly independent of proprietary reference code. High performance terminal user interfaces provide immediate feedback, predictable latency, and maximum developer focus. Touch typing empowers engineers to express ideas without cognitive friction.";
        let session = TypingSession::new(test_text, Some(Duration::from_secs(duration_secs)));
        self.active_session = Some(session);
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
        self.screen = AppScreen::TypingActive;
    }

    pub fn on_tick(&mut self) {
        if self.screen == AppScreen::WordDropGame {
            self.game.update();
            if self.game.score > self.profile.game_high_score {
                self.profile.game_high_score = self.game.score;
                let _ = self.profile.save();
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
            if self.selected_course_idx < self.courses.len() {
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

            let _ = self.profile.save();
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
        }
    }

    fn handle_menu_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_menu_idx > 0 {
                    self.selected_menu_idx -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_menu_idx < 5 {
                    self.selected_menu_idx += 1;
                }
            }
            KeyCode::Enter => match self.selected_menu_idx {
                0 => self.screen = AppScreen::CourseSelect,
                1 => self.screen = AppScreen::SpeedTestSelect,
                2 => self.screen = AppScreen::TargetedReview,
                3 => {
                    self.game.reset(80, 24);
                    self.screen = AppScreen::WordDropGame;
                }
                4 => self.screen = AppScreen::Stats,
                5 => self.should_quit = true,
                _ => {}
            },
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
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
                    self.screen = AppScreen::MainMenu;
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
                self.start_current_exercise();
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Enter => {
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
            KeyCode::Esc | KeyCode::Char('m') | KeyCode::Char('q') => {
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
