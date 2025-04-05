use game_lib::{
    animation::Animation,
    game::GameSession,
    RECT_INTERVAL,
};
use std::{thread, time::Duration};

#[test]
fn test_game_flow() {
    // Set up game components
    let screen_size = (80, 24);
    let mut game_session = GameSession::new();
    let mut animation = Animation::new(screen_size);
    
    // Start game
    game_session.start();
    assert!(game_session.is_running());
    
    // Initial state checks
    assert!(!animation.is_game_over());
    assert!(animation.get_rectangles().is_empty());
    
    // Run a few game cycles
    for _ in 0..5 {
        animation.update(&mut game_session);
        thread::sleep(Duration::from_millis(RECT_INTERVAL));
    }
    
    // Verify obstacles are being generated
    assert!(!animation.get_rectangles().is_empty());
    
    // Test jumping
    let initial_y = animation.get_bird().get_bottom_y();
    animation.jump();
    animation.update(&mut game_session);
    assert!(animation.get_bird().get_bottom_y() < initial_y);
    
    // Test level progression
    let initial_level = game_session.get_level();
    for _ in 0..10 {
        animation.update(&mut game_session);
        thread::sleep(Duration::from_millis(50));
    }
    assert!(game_session.get_level() >= initial_level);
    
    // Test game over and restart
    animation.restart(&mut game_session);
    assert!(!animation.is_game_over());
    assert!(animation.get_rectangles().is_empty());
    assert!(game_session.is_running());
}

#[test]
fn test_collision_mechanics() {
    let screen_size = (80, 24);
    let mut game_session = GameSession::new();
    let mut animation = Animation::new(screen_size);
    game_session.start();
    
    // Let some obstacles generate
    for _ in 0..3 {
        animation.update(&mut game_session);
        thread::sleep(Duration::from_millis(RECT_INTERVAL));
    }
    
    // Verify game continues while no collision
    assert!(!animation.is_game_over());
    assert!(game_session.is_running());
    
    // Eventually the game should end due to collision
    let max_attempts = 200;
    let mut attempts = 0;
    while !animation.is_game_over() && attempts < max_attempts {
        animation.update(&mut game_session);
        attempts += 1;
    }

    // Verify game over state
    assert!(animation.is_game_over());
}

#[test]
fn test_game_speed_progression() {
    let screen_size = (80, 24);
    let mut game_session = GameSession::new();
    let mut animation = Animation::new(screen_size);
    game_session.start();
    
    // Record initial state
    let initial_level = game_session.get_level();
    
    // Force level increases
    for _ in 0..5 {
        game_session.increase_level();
        animation.update(&mut game_session);
    }
    
    // Verify level progression
    assert!(game_session.get_level() > initial_level);
    
    // Game should still be running after level increases
    assert!(!animation.is_game_over());
    assert!(game_session.is_running());
}