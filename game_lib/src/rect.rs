use rand::Rng;
use crossterm::style::Color;

pub struct Rect {
    x: u16,
    width: u16,
    height: u16,
    color: Color,
    screen_size: (u16,u16),
}

impl Rect {
    fn new(x: u16, width: u16, height: u16, color: Color, screen_size:(u16,u16)) -> Self {
        Self {
            x,
            width,
            height,
            color,
            screen_size,
        }
    }

    pub fn random(max_width: u16, screen_size:(u16,u16)) -> Self {
        let mut rng = rand::thread_rng();

        let rect_width = rng.gen_range(2..max_width);
        let rect_height = rng.gen_range(3..screen_size.1-10);
        
        let x = screen_size.0 - rect_width;
        // Random color from a predefined set
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

    pub fn draw(&self) -> Vec<(u16, u16, char, Color)> {
        let mut points = Vec::new();
        
        // Draw the rectangle
        for y in self.screen_size.1-self.height..self.screen_size.1{
            for x in self.x..self.x + self.width {
                let char = '*'; // Border
                points.push((x, y, char, self.color));
            }
        }
        
        points
    }
    
    pub fn get_x(&self) -> u16 {
        self.x
    }
    
    pub fn set_x(&mut self, x: u16) {
        self.x = x;
    }

    pub fn get_leftmost_x(&self) -> u16 {
        self.x
    }

    pub fn get_rightmost_x(&self) -> u16 {
        self.x + self.width - 1
    }

    pub fn get_top_y(&self) -> u16 {
        self.screen_size.1-self.height
    }

} 