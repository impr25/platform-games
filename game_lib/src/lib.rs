//! Game Library for Platform-based 2D Games
//! 
//! This library provides the core functionality for creating simple 2D platform games
//! that can run in a terminal. It includes:
//! 
//! - Physics-based bird movement with gravity and jumping
//! - Obstacle generation and collision detection
//! - Game state management
//! - Terminal-based animation system
//! 
//! # Example
//! 
//! ```no_run
//! use game_lib::{animation::Animation, game::GameSession};
//! 
//! let mut game_session = GameSession::new();
//! let mut animation = Animation::new(screen_size);
//! 
//! // Game loop
//! while game_session.is_running() {
//!     animation.update(&mut game_session);
//! }
//! ```

pub mod rect;
pub mod animation;
pub mod bird;
pub mod game;

/// Gravity constant affecting bird's vertical movement
/// 
/// Higher values make the bird fall faster
pub const GRAVITY: f32 = 1.0;

/// Initial upward velocity when the bird jumps
/// 
/// Negative value represents upward movement
pub const JUMP_VELOCITY: f32 = -3.0;

/// Time interval between physics updates in seconds
/// 
/// Used to maintain consistent game speed
pub const UPDATE_INTERVAL: f32 = 1.0;

/// Maximum falling speed of the bird
/// 
/// Prevents the bird from falling too quickly
pub const MAX_VELOCITY: f32 = 8.0;

/// Time interval between rectangles are created in ms
/// 
/// Used to maintain consistent generate speed
pub const RECT_INTERVAL: u64 = 1000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_constants() {
        assert!(GRAVITY > 0.0, "Gravity should be positive");
        assert!(JUMP_VELOCITY < 0.0, "Jump velocity should be negative (upward)");
        assert!(UPDATE_INTERVAL > 0.0, "Update interval should be positive");
        assert!(MAX_VELOCITY > GRAVITY, "Max velocity should be greater than gravity");
    }
}
