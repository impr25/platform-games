use crossterm::{
    cursor::{Hide, Show},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Result};

pub enum GameStatus {
    Start,
    Running,
    End,
}

pub struct GameSession {
    status: GameStatus,
    score: u32,
    level: u16,
}

impl GameSession {
    pub fn new() -> Self {
        Self { 
            status: GameStatus::Start, 
            score: 0, 
            level: 1
        } 
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
        matches!(self.status, GameStatus::Running)
    }

    pub fn start(&mut self) {
        self.status = GameStatus::Running;
        self.score = 0;
        self.level = 1;
    }

    pub fn end(&mut self) {
        self.status = GameStatus::End;
    }

    pub fn increase_level(&mut self) {
        self.level += 1;
    }

    pub fn increase_score(&mut self) {
        self.score += 1;
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_level(&self) -> u16 {
        self.level
    }
}