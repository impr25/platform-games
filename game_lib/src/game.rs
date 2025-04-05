use crossterm::{
    cursor::{Hide, Show},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Result};

/// Represents the current state of the game
/// 
/// Used to control game flow and determine what actions are allowed
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameStatus {
    /// Initial state before the game begins
    Start,
    /// Game is actively being played
    Running,
    /// Game has ended (either by collision or user action)
    End,
}

/// Manages the game session state and terminal setup
/// 
/// Handles:
/// - Game status transitions
/// - Level progression
/// - Terminal initialization and cleanup
/// - Score tracking
#[derive(Debug)]
pub struct GameSession {
    status: GameStatus,
    score: u32,
    level: u16,
}

impl GameSession {
    /// Creates a new game session with initial state
    /// 
    /// # Example
    /// ```
    /// use game_lib::game::GameSession;
    /// 
    /// let session = GameSession::new();
    /// assert_eq!(session.get_level(), 1);
    /// ```
    pub fn new() -> Self {
        Self { 
            status: GameStatus::Start, 
            score: 0, 
            level: 1
        } 
    }

    /// Sets up the terminal for game display
    /// 
    /// Enters alternate screen mode and hides the cursor
    pub fn init_terminal(&self) -> Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        stdout().execute(Hide)?;
        Ok(())
    }

    /// Restores terminal to original state
    /// 
    /// Shows cursor and leaves alternate screen mode
    pub fn cleanup_terminal(&self) -> Result<()> {
        stdout().execute(Show)?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    /// Checks if the game is currently being played
    /// 
    /// # Returns
    /// `true` if the game status is Running, `false` otherwise
    pub fn is_running(&self) -> bool {
        matches!(self.status, GameStatus::Running)
    }

    /// Transitions the game to Running state
    pub fn start(&mut self) {
        self.status = GameStatus::Running;
        self.score = 0;
        self.level = 1;
    }

    /// Transitions the game to End state
    pub fn end(&mut self) {
        self.status = GameStatus::End;
    }

    /// Increases the current level by 1
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_session() {
        let session = GameSession::new();
        assert_eq!(session.get_level(), 1);
        assert!(!session.is_running());
    }

    #[test]
    fn test_game_state_transitions() {
        let mut session = GameSession::new();
        assert!(!session.is_running());
        
        session.start();
        assert!(session.is_running());
        
        session.end();
        assert!(!session.is_running());
    }

    #[test]
    fn test_level_progression() {
        let mut session = GameSession::new();
        assert_eq!(session.get_level(), 1);
        
        session.increase_level();
        assert_eq!(session.get_level(), 2);
        
        // Test multiple level increases
        for _ in 0..5 {
            session.increase_level();
        }
        assert_eq!(session.get_level(), 7);
    }
}