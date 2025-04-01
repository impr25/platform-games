use std::io::{self, Write};
use crossterm::{
    execute, queue,
    style::{self, Stylize, Color},
    cursor,
    terminal::{
       EnterAlternateScreen,
       LeaveAlternateScreen,
       size,
       Clear,
       ClearType},
    event::{self, Event, KeyCode, KeyEventKind},
};
use std::time::{Duration, Instant};
use game_lib::{animation::Animation};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let screen_size = size().unwrap();
    let (width, height) = screen_size;

    execute!(stdout, EnterAlternateScreen)?;

    let mut animation = Animation::new(screen_size);
    let mut last_update = Instant::now();
    let update_interval = Duration::from_millis(50); // Update every 50ms

    while last_update.elapsed() < Duration::from_secs(30) { // Run for 30 seconds
        // Handle input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(' ') => animation.jump(), // Space to jump
                        KeyCode::Esc => break, // ESC to quit
                        KeyCode::Char('r') => {
                            animation.restart();
                        }
                        _ => {}
                    }
                }
            }
        }

        // Clear the screen
        execute!(stdout, Clear(ClearType::All))?;

        // Draw border
        for y in 0..height {
            for x in 0..width {
                if (y == 0 || y == height - 1) || (x == 0 || x == width - 1) {
                    queue!(stdout, cursor::MoveTo(x, y), style::PrintStyledContent("*".magenta()))?;
                }
            }
        }

        // Update animation
        animation.update();

        // Draw all rectangles
        for rect in animation.get_rectangles() {
            for (x, y, char, color) in rect.draw() {
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent(char.with(color))
                )?;
            }
        }

        // Draw bird
        let bird = animation.get_bird();
        for (x, y, char, color) in bird.draw() {
            queue!(
                stdout,
                cursor::MoveTo(x, y),
                style::PrintStyledContent(char.with(color))
            )?;
        }

        // Draw game over message if game is over
        if animation.is_game_over() {
            let game_over_text = "GAME OVER! Press ESC to quit";
            let restart_text = "Press R to restart";
            let text_x = (width - game_over_text.len() as u16) / 2;
            let text_y = height / 2;
            queue!(
                stdout,
                cursor::MoveTo(text_x, text_y),
                style::PrintStyledContent(game_over_text.red())
            )?;
            let restart_text_x = (width - restart_text.len() as u16) / 2;
            let restart_text_y = text_y + 2;
            queue!(
                stdout,
                cursor::MoveTo(restart_text_x, restart_text_y),
                style::PrintStyledContent(restart_text.green())
            )?;
        }

        stdout.flush()?;
        last_update = Instant::now();
        std::thread::sleep(update_interval);
    }

    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}