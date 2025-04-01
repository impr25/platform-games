use crossterm::style::Color;

// #[derive(Clone, Copy)]
pub struct Bird {
    x: u16,
    y: u16,
    velocity: i16,
    gravity: i16,
    jump_force: i16,
    screen_size: (u16, u16),
}

impl Bird {
    pub fn new(screen_size: (u16, u16)) -> Self {
        Self {
            x: 25, // Fixed x position on the left
            y: screen_size.1 / 2, // Start in middle of screen
            velocity: 0,
            gravity: 1,
            jump_force: -3, // Negative because moving up is negative y
            screen_size,
        }
    }

    pub fn update(&mut self) {
        // Apply gravity
        self.velocity += self.gravity;
        
        // Update position
        let new_y = self.y as i16 + self.velocity;
        self.y = new_y.max(1).min((self.screen_size.1 - 4) as i16) as u16;
    }

    pub fn jump(&mut self) {
        self.velocity = self.jump_force;
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