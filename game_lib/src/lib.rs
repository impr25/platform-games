use crossterm::{
    cursor::{Hide, Show},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Result};

pub struct Game {
    running: bool,
}

impl Game {
    pub fn new() -> Self {
        Self { running: true }
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
} 