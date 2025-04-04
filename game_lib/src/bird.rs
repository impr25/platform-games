use crossterm::style::Color;

use crate::GRAVITY;
use crate::JUMP_VELOCITY;
use crate::MAX_VELOCITY;
use crate::UPDATE_INTERVAL;

pub struct Bird {
    x: u16,
    y: u16,
    velocity: f32,
    screen_size: (u16, u16),
}

impl Bird {
    pub fn new(screen_size: (u16, u16)) -> Self {
        Self {
            x: 25, // Fixed x position on the left
            y: screen_size.1 / 2, // Start in middle of screen
            velocity: 0.0,
            screen_size,
        }
    }

    pub fn update(&mut self) {
        // Apply gravity
        // Clamp velocity to terminal velocity
        self.velocity = self.velocity.min(MAX_VELOCITY);
        
        // Update position using physics equation: y = y0 + v0*t + 0.5*a*t^2
        let delta_y = self.velocity*UPDATE_INTERVAL + 0.5 * GRAVITY*UPDATE_INTERVAL*UPDATE_INTERVAL;
        let new_y = self.y as f32 + delta_y;
        
        if new_y > (self.screen_size.1 - 4) as f32 {
            self.velocity = 0.0;
            self.y = (self.screen_size.1 - 4) as u16;
        } else if new_y < 1.0 {
            self.velocity = 0.0;
            self.y = 1 as u16;
        } else {
            self.velocity += GRAVITY;
            self.y = new_y as u16;
        }
    }

    pub fn jump(&mut self) {
        self.velocity = JUMP_VELOCITY;
    }

    pub fn draw(&self) -> Vec<(u16, u16, char, Color)> {
        let mut points = Vec::new();
        let color = Color::Yellow;

        // Bird shape using ASCII characters
        //  ><
        //  ^
        //  v
        let bird_shape = [
            (0, 0, ' '),
            (1, 0, ' '),
            (2, 0, ' '),
            (3, 0, '|'),
            (4, 0, '>'),
            (0, 1, '|'),
            (1, 1, ':'),
            (2, 1, ':'),
            (3, 1, '|'),
            (4, 1, ' '),
        ];

        for (dx, dy, char) in bird_shape {
            points.push((self.x + dx, self.y + dy, char, color));
        }

        points
    }

    pub fn get_rightmost_x(&self) -> u16 {
        self.x + 4 // Bird width is 5 characters (0-4)
    }
 
    pub fn get_bottom_y(&self) -> u16 {
        self.y + 2 // Bird height is 3 characters (0-2)
    }
    pub fn get_leftmost_x(&self) -> u16 {
        self.x
    }
} 