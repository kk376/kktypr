pub mod keyboard;
pub mod theme;
pub mod views;

use ratatui::Frame;
use crate::app::{App, AppScreen};

pub fn render(app: &mut App, frame: &mut Frame) {
    match app.screen {
        AppScreen::MainMenu => views::menu_view::render_main_menu(app, frame),
        AppScreen::CourseSelect => views::menu_view::render_course_select(app, frame),
        AppScreen::LessonSelect => views::menu_view::render_lesson_select(app, frame),
        AppScreen::TypingActive => views::lesson_view::render_typing_active(app, frame),
        AppScreen::Results => views::lesson_view::render_results(app, frame),
        AppScreen::SpeedTestSelect => views::menu_view::render_speed_test_select(app, frame),
        AppScreen::TargetedReview => views::dashboard::render_targeted_review(app, frame),
        AppScreen::WordDropGame => views::game_view::render_word_drop(app, frame),
        AppScreen::Stats => views::dashboard::render_stats(app, frame),
    }
}
