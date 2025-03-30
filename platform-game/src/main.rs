use crossterm::{
    event::{self, Event, KeyCode},
    terminal::Clear,
    ExecutableCommand,
};
use game_lib::Game;
use std::io::{stdout, Result};

fn main() -> Result<()> {
    let mut game = Game::new();
    game.init_terminal()?;

    while game.is_running() {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => game.quit(),
                    _ => {}
                }
            }
        }

        // Clear the screen and draw game state
        stdout().execute(Clear(crossterm::terminal::ClearType::All))?;
        println!("Platform Game - Press 'q' to quit");
    }

    game.cleanup_terminal()?;
    Ok(())
} 