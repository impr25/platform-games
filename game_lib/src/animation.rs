use std::time::{Duration, Instant};
use crate::{rect::Rect, bird::Bird};
use crate::game;

pub struct Animation {
    rectangles: Vec<Rect>,
    bird: Bird,
    last_spawn_time: Instant,
    game_level: u16,
    screen_size: (u16, u16),
    game_over: bool,
}

impl Animation {
    pub fn new(screen_size: (u16, u16), game_level: u16) -> Self {
        Self {
            rectangles: Vec::new(),
            bird: Bird::new(screen_size),
            last_spawn_time: Instant::now(),
            game_level,
            screen_size,
            game_over: false,
        }
    }

    pub fn update(&mut self) {

        if self.game_over {
            return;
        }
        // Remove rectangles that have moved off screen
        self.rectangles.retain(|rect| rect.get_x() > 0);

        // Move all rectangles to the left
        for rect in &mut self.rectangles {
            rect.set_x(rect.get_x().saturating_sub(1));
        }

        // Update bird position
        self.bird.update();

        // Check for collisions
        if self.check_collision() {
            self.game_over = true;
        }

        // Spawn new rectangle if enough time has passed
        if self.last_spawn_time.elapsed() >= Duration::from_secs(self.game_level as u64) {
            let max_width = 10;
            // Create new rectangle at the right edge
            let new_rect = Rect::random(max_width, self.screen_size);
            
            self.rectangles.push(new_rect);
            self.last_spawn_time = Instant::now();
        }
    }

    pub fn update_level(&mut self) {
        self.game_level = game::get_level();
    }

    fn check_collision(&self) -> bool {
        let bird_right = self.bird.get_rightmost_x();
        let bird_left = self.bird.get_leftmost_x();
        let bird_bottom = self.bird.get_bottom_y();

        for rect in &self.rectangles {
            let rect_left = rect.get_leftmost_x();
            let rect_right = rect.get_rightmost_x();
            let rect_top = rect.get_top_y();

            // Check if bird's right edge collides with rectangle's left edge
            if bird_right >= rect_left && bird_left <= rect_right && bird_bottom >= rect_top {
                return true;
            }
            else {
                continue;
            }
        }

        false
    }

    pub fn jump(&mut self) {
        if !self.game_over {
            self.bird.jump();
        }
    }

    pub fn get_rectangles(&self) -> &[Rect] {
        &self.rectangles
    }

    pub fn get_bird(&self) -> &Bird {
        &self.bird
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn restart(&mut self) {
        self.game_over = false;
        self.rectangles.clear();
        self.last_spawn_time = Instant::now();
    }
} 