use crossterm::style::Color;

use crate::GRAVITY;
use crate::JUMP_VELOCITY;
use crate::MAX_VELOCITY;
use crate::UPDATE_INTERVAL;

/// Represents the player-controlled bird in the game
/// 
/// The bird moves with physics-based motion, affected by gravity and jump impulses.
/// It maintains a constant x position but can move vertically.
#[derive(Debug)]
pub struct Bird {
    x: u16,
    y: u16,
    velocity: f32,
    screen_size: (u16, u16),
}

impl Bird {
    /// Creates a new bird instance
    /// 
    /// # Arguments
    /// * `screen_size` - Terminal dimensions (width, height)
    /// 
    /// # Example
    /// ```
    /// use game_lib::bird::Bird;
    /// 
    /// let screen_size = (80, 24);
    /// let bird = Bird::new(screen_size);
    /// assert_eq!(bird.get_leftmost_x(), 25); // Fixed x position
    /// ```
    pub fn new(screen_size: (u16, u16)) -> Self {
        Self {
            x: 25, // Fixed x position on the left
            y: screen_size.1 / 2, // Start in middle of screen
            velocity: 0.0,
            screen_size,
        }
    }

    /// Updates the bird's position based on physics
    /// 
    /// Applies gravity and updates position using basic physics equations.
    /// Prevents the bird from moving outside screen bounds.
    pub fn update(&mut self) {
        // Apply gravity and clamp to terminal velocity
        self.velocity = self.velocity.min(MAX_VELOCITY);
        
        // Update position using physics equation: y = y0 + v0*t + 0.5*a*t^2
        let delta_y = self.velocity*UPDATE_INTERVAL + 0.5 * GRAVITY*UPDATE_INTERVAL*UPDATE_INTERVAL;
        let new_y = self.y as f32 + delta_y;
        
        if new_y > (self.screen_size.1 - 4) as f32 {
            self.velocity = 0.0;
            self.y = (self.screen_size.1 - 4) as u16;
        } else if new_y < 1.0 {
            self.velocity = 0.0;
            self.y = 1;
        } else {
            self.velocity += GRAVITY;
            self.y = new_y as u16;
        }
    }

    /// Makes the bird jump by setting its velocity to JUMP_VELOCITY
    pub fn jump(&mut self) {
        self.velocity = JUMP_VELOCITY;
    }

    /// Returns a vector of points representing the bird for drawing
    /// 
    /// Each point contains:
    /// - x coordinate
    /// - y coordinate
    /// - character to draw
    /// - color of the point
    pub fn draw(&self) -> Vec<(u16, u16, char, Color)> {
        let mut points = Vec::new();
        let color = Color::Yellow;

        // Bird shape using ASCII characters
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

    /// Returns the rightmost x coordinate of the bird
    pub fn get_rightmost_x(&self) -> u16 {
        self.x + 4 // Bird width is 5 characters (0-4)
    }
 
    /// Returns the bottom y coordinate of the bird
    pub fn get_bottom_y(&self) -> u16 {
        self.y + 2 // Bird height is 3 characters (0-2)
    }

    /// Returns the leftmost x coordinate of the bird
    pub fn get_leftmost_x(&self) -> u16 {
        self.x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bird_creation() {
        let screen_size = (80, 24);
        let bird = Bird::new(screen_size);
        
        assert_eq!(bird.x, 25);
        assert_eq!(bird.y, screen_size.1 / 2);
        assert_eq!(bird.velocity, 0.0);
    }

    #[test]
    fn test_bird_boundaries() {
        let screen_size = (80, 24);
        let mut bird = Bird::new(screen_size);
        
        // Test floor collision
        bird.y = screen_size.1 - 1;
        bird.velocity = MAX_VELOCITY;
        bird.update();
        assert_eq!(bird.y, screen_size.1 - 4);
        assert_eq!(bird.velocity, 0.0);

        // Test ceiling collision
        bird.y = 0;
        bird.velocity = JUMP_VELOCITY;
        bird.update();
        assert_eq!(bird.y, 1);
        assert_eq!(bird.velocity, 0.0);
    }

    #[test]
    fn test_bird_jump() {
        let screen_size = (80, 24);
        let mut bird = Bird::new(screen_size);
        let initial_y = bird.y;
        
        bird.jump();
        assert_eq!(bird.velocity, JUMP_VELOCITY);
        
        // After jumping, bird should move up
        bird.update();
        assert!(bird.y < initial_y);
    }

    #[test]
    fn test_bird_drawing() {
        let screen_size = (80, 24);
        let bird = Bird::new(screen_size);
        let points = bird.draw();
        
        // Check that we have the right number of points
        assert_eq!(points.len(), 10);
        
        // Check that all points are within bird bounds
        for (x, y, _, _) in points {
            assert!(x >= bird.get_leftmost_x());
            assert!(x <= bird.get_rightmost_x());
            assert!(y >= bird.y);
            assert!(y <= bird.get_bottom_y());
        }
    }
}