use std::time::{Duration, Instant};
use rand::RngExt;

const WORD_BANK: &[&str] = &[
    "rust", "code", "fast", "pure", "safe", "drop", "flow", "core",
    "grid", "byte", "loop", "type", "beam", "star", "neon", "zero",
    "async", "trait", "mutex", "spawn", "slice", "macro", "crate",
    "stack", "logic", "event", "pixel", "cache", "token", "guard",
];

#[derive(Debug, Clone)]
pub struct FallingWord {
    pub text: String,
    pub x: u16,
    pub y: f32,
    pub typed_len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Playing,
    GameOver,
}

pub struct WordDropGame {
    pub words: Vec<FallingWord>,
    pub score: u32,
    pub streak: u32,
    pub lives: u8,
    pub state: GameState,
    pub targeted_word_idx: Option<usize>,
    pub speed: f32, // rows per second
    last_spawn: Instant,
    last_update: Instant,
    spawn_interval: Duration,
    pub width: u16,
    pub height: u16,
}

impl WordDropGame {
    pub fn new(width: u16, height: u16) -> Self {
        let now = Instant::now();
        Self {
            words: Vec::new(),
            score: 0,
            streak: 0,
            lives: 3,
            state: GameState::Playing,
            targeted_word_idx: None,
            speed: 1.2,
            last_spawn: now,
            last_update: now,
            spawn_interval: Duration::from_millis(1800),
            width: width.max(30),
            height: height.max(12),
        }
    }

    pub fn reset(&mut self, width: u16, height: u16) {
        *self = Self::new(width, height);
    }

    pub fn spawn_word(&mut self) {
        if self.state != GameState::Playing {
            return;
        }

        let mut rng = rand::rng();
        let word_idx = rng.random_range(0..WORD_BANK.len());
        let text = WORD_BANK[word_idx].to_string();

        let max_x = self.width.saturating_sub(text.len() as u16 + 2).max(1);
        let x = rng.random_range(2..max_x);

        self.words.push(FallingWord {
            text,
            x,
            y: 0.0,
            typed_len: 0,
        });
    }

    pub fn update(&mut self) {
        if self.state != GameState::Playing {
            return;
        }

        let now = Instant::now();
        let dt = now.saturating_duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Periodic word spawning
        if now.saturating_duration_since(self.last_spawn) >= self.spawn_interval {
            self.spawn_word();
            self.last_spawn = now;
            // Gradually ramp difficulty
            self.speed = (self.speed + 0.02).min(3.5);
            let millis = (self.spawn_interval.as_millis().saturating_sub(15)).max(800) as u64;
            self.spawn_interval = Duration::from_millis(millis);
        }

        let floor = (self.height.saturating_sub(2)) as f32;
        let mut breached = false;

        for word in &mut self.words {
            word.y += self.speed * dt;
            if word.y >= floor {
                breached = true;
            }
        }

        if breached {
            // Remove fallen words and penalize life
            let prev_len = self.words.len();
            self.words.retain(|w| w.y < floor);
            let lost_count = prev_len - self.words.len();
            
            self.streak = 0;
            self.targeted_word_idx = None;
            self.lives = self.lives.saturating_sub(lost_count as u8);

            if self.lives == 0 {
                self.state = GameState::GameOver;
            }
        }
    }

    pub fn handle_char(&mut self, c: char) -> bool {
        if self.state != GameState::Playing {
            return false;
        }

        // If a word is already targeted, continue typing it
        if let Some(target_idx) = self.targeted_word_idx {
            if target_idx < self.words.len() {
                let word = &mut self.words[target_idx];
                if let Some(expected) = word.text.chars().nth(word.typed_len)
                    && expected == c {
                        word.typed_len += 1;
                        if word.typed_len >= word.text.len() {
                            // Word completed!
                            let pts = (word.text.len() as u32 * 10) * (1 + self.streak / 5);
                            self.score += pts;
                            self.streak += 1;
                            self.words.remove(target_idx);
                            self.targeted_word_idx = None;
                        }
                        return true;
                    }
            }
            // Mismatch resets streak and target
            self.streak = 0;
            self.targeted_word_idx = None;
            return false;
        }

        // Find a word starting with char `c` closest to the floor (highest y)
        let mut best_candidate: Option<(usize, f32)> = None;

        for (idx, word) in self.words.iter().enumerate() {
            if word.text.starts_with(c) {
                match best_candidate {
                    None => best_candidate = Some((idx, word.y)),
                    Some((_, best_y)) if word.y > best_y => best_candidate = Some((idx, word.y)),
                    _ => {}
                }
            }
        }

        if let Some((idx, _)) = best_candidate {
            let word = &mut self.words[idx];
            word.typed_len = 1;
            if word.typed_len >= word.text.len() {
                // 1-letter word completed
                self.score += 10;
                self.streak += 1;
                self.words.remove(idx);
            } else {
                self.targeted_word_idx = Some(idx);
            }
            true
        } else {
            self.streak = 0;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_word_matching_and_scoring() {
        let mut game = WordDropGame::new(80, 24);
        game.words.push(FallingWord {
            text: "rust".to_string(),
            x: 10,
            y: 5.0,
            typed_len: 0,
        });

        assert!(game.handle_char('r'));
        assert_eq!(game.targeted_word_idx, Some(0));
        assert!(game.handle_char('u'));
        assert!(game.handle_char('s'));
        assert!(game.handle_char('t'));

        assert_eq!(game.words.len(), 0);
        assert!(game.score > 0);
        assert_eq!(game.streak, 1);
        assert_eq!(game.targeted_word_idx, None);
    }
}
