pub mod rect;
pub mod animation;
pub mod bird;
pub mod game;
// Physics constants for Flappy Bird
pub const GRAVITY: f32 = 1.0;        // Moderate falling speed
pub const JUMP_VELOCITY: f32 = -3.0; // Quick, snappy jump
pub const UPDATE_INTERVAL: f32 = 1.0; // 60 FPS
pub const MAX_VELOCITY: f32 = 8.0;   // Terminal velocity
