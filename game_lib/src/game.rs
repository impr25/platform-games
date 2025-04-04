use crossterm::{
    cursor::{Hide, Show},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Result};
// use std::cell::Cell;

// static LEVEL: Cell<u16> = Cell::new(1);

// pub fn increase_level() {
//     let current_level = LEVEL.get();
//     LEVEL.set(current_level + 1);
// }

// pub fn get_level() -> u16 {
//     LEVEL.get()
// }

pub struct Game {
    running: bool,
    level: u16,
}

impl Game {
    pub fn new() -> Self {
        Self { running: true, level: 1 }
    }

    pub fn init_terminal(&self) -> Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        stdout().execute(Hide)?;
        Ok(())
    }

    pub fn cleanup_terminal(&self) -> Result<()> {
        stdout().execute(Show)?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increase_level(&mut self) {
        self.level += 1;
    }

    pub fn get_level(&self) -> u16 {
        self.level
    }
}