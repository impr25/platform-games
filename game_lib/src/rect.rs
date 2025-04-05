use rand::Rng;
use crossterm::style::Color;

/// Represents a rectangular obstacle in the game
/// 
/// Rectangles are colored blocks that the bird must avoid.
/// They are randomly generated with varying widths and heights.
#[derive(Debug)]
pub struct Rect {
    x: u16,
    width: u16,
    height: u16,
    color: Color,
    screen_size: (u16, u16),
}

impl Rect {
    /// Creates a new rectangle with specified dimensions and properties
    /// 
    /// # Arguments
    /// * `x` - Horizontal position
    /// * `width` - Width of the rectangle
    /// * `height` - Height of the rectangle
    /// * `color` - Color of the rectangle
    /// * `screen_size` - Terminal dimensions (width, height)
    fn new(x: u16, width: u16, height: u16, color: Color, screen_size:(u16,u16)) -> Self {
        Self {
            x,
            width,
            height,
            color,
            screen_size,
        }
    }

    /// Creates a new rectangle with random properties
    /// 
    /// # Arguments
    /// * `max_width` - Maximum width of the generated rectangle
    /// * `screen_size` - Terminal dimensions (width, height)
    /// 
    /// # Example
    /// ```
    /// use game_lib::rect::Rect;
    /// 
    /// let screen_size = (80, 24);
    /// let rect = Rect::random(10, screen_size);
    /// assert!(rect.get_rightmost_x() <= screen_size.0);
    /// ```
    pub fn random(max_width: u16, screen_size:(u16,u16)) -> Self {
        let mut rng = rand::thread_rng();

        let rect_width = rng.gen_range(2..max_width);
        let rect_height = rng.gen_range(3..screen_size.1-10);
        
        let x = screen_size.0 - rect_width;
        let colors = [
            Color::Blue,
            Color::Green,
            Color::Yellow,
            Color::Magenta,
            Color::Cyan,
        ];
        let color = colors[rng.gen_range(0..colors.len())];

        Self::new(x, rect_width, rect_height, color, screen_size)
    }

    /// Returns a vector of points representing the rectangle for drawing
    /// 
    /// Each point contains:
    /// - x coordinate
    /// - y coordinate
    /// - character to draw
    /// - color of the point
    pub fn draw(&self) -> Vec<(u16, u16, char, Color)> {
        let mut points = Vec::new();
        
        for y in self.screen_size.1-self.height..self.screen_size.1 {
            for x in self.x..self.x + self.width {
                points.push((x, y, '*', self.color));
            }
        }
        
        points
    }
    
    /// Returns the current x position of the rectangle
    pub fn get_x(&self) -> u16 {
        self.x
    }
    
    /// Sets a new x position for the rectangle
    pub fn set_x(&mut self, x: u16) {
        self.x = x;
    }

    /// Returns the leftmost x coordinate of the rectangle
    pub fn get_leftmost_x(&self) -> u16 {
        self.x
    }

    /// Returns the rightmost x coordinate of the rectangle
    pub fn get_rightmost_x(&self) -> u16 {
        self.x + self.width - 1
    }

    /// Returns the top y coordinate of the rectangle
    pub fn get_top_y(&self) -> u16 {
        self.screen_size.1-self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_creation() {
        let screen_size = (80, 24);
        let rect = Rect::new(10, 5, 8, Color::Blue, screen_size);
        
        assert_eq!(rect.get_x(), 10);
        assert_eq!(rect.get_leftmost_x(), 10);
        assert_eq!(rect.get_rightmost_x(), 14);
        assert_eq!(rect.get_top_y(), 16);
    }

    #[test]
    fn test_rect_movement() {
        let screen_size = (80, 24);
        let mut rect = Rect::new(10, 5, 8, Color::Blue, screen_size);
        
        rect.set_x(15);
        assert_eq!(rect.get_x(), 15);
        assert_eq!(rect.get_leftmost_x(), 15);
        assert_eq!(rect.get_rightmost_x(), 19);
    }

    #[test]
    fn test_random_rect() {
        let screen_size = (80, 24);
        let max_width = 10;
        let rect = Rect::random(max_width, screen_size);
        
        // Check that the rectangle fits within screen bounds
        assert!(rect.get_rightmost_x() <= screen_size.0);
        assert!(rect.get_top_y() < screen_size.1);
        
        // Check that width is within specified range
        let width = rect.get_rightmost_x() - rect.get_leftmost_x() + 1;
        assert!(width <= max_width);
        assert!(width >= 2);
    }

    #[test]
    fn test_rect_drawing() {
        let screen_size = (10, 10);
        let rect = Rect::new(2, 3, 4, Color::Blue, screen_size);
        let points = rect.draw();
        
        // Check number of points (width * height)
        assert_eq!(points.len(), (3 * 4) as usize);
        
        // Check that all points are within rectangle bounds
        for (x, y, c, color) in points {
            assert!(x >= rect.get_leftmost_x());
            assert!(x <= rect.get_rightmost_x());
            assert!(y >= rect.get_top_y());
            assert!(y < screen_size.1);
            assert_eq!(c, '*');
            assert_eq!(color, Color::Blue);
        }
    }
}