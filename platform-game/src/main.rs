use std::io::{self, Write, Read};
use crossterm::{
    execute, queue,
    style::{self, Stylize},
    cursor,
    terminal::{Clear, ClearType, size},
    event::{self, Event, KeyCode, KeyEventKind},
};
use std::time::Duration;
use game_lib::animation::Animation;
use game_lib::game::GameSession;

use serialport::{self};


const DEFAULT_PORT: &str = "COM5";

fn main() -> Result<(), serialport::Error> {
    let mut stdout = io::stdout();
    let screen_size = size().unwrap();
    let (width, height) = screen_size;

    let mut game_session = GameSession::new();
    game_session.init_terminal()?;
    game_session.start();

    let mut animation = Animation::new(screen_size);
    let mut update_interval = 40;

   let mut current_level = game_session.get_level();

   let mut port = serialport::new(DEFAULT_PORT, 115200)
        .timeout(Duration::from_millis(10))
        .open()?;

    let mut buffer: Vec<u8> = vec![0; 1];

    while game_session.is_running() {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('i') => animation.jump(),
                        KeyCode::Esc => break,
                        KeyCode::Char('r') => {
                            animation.restart(&mut game_session);
                            update_interval = 40;
                            current_level = game_session.get_level();
                        },
                        _ => {}
                    }
                }
            }
        }

        match port.read(buffer.as_mut_slice()) {
            Ok(t) => {
                if t > 0 {
                    let received_char = buffer[0] as char;
                    match received_char {
                        'i' => {
                            animation.jump();
                        }
                        'j' => {
                            animation.jump();
                        }
                        _ => {} // Ignore other characters
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (), // Ignore timeouts
            Err(e) => eprintln!("Error reading from serial port: {}", e),
        }

        execute!(stdout, Clear(ClearType::All))?;

        // Draw border
        for y in 0..height {
            for x in 0..width {
                if (y == 0 || y == height - 1) || (x == 0 || x == width - 1) {
                    queue!(stdout, cursor::MoveTo(x, y), style::PrintStyledContent("*".magenta()))?;
                }
            }
        }

        // Update animation with game session
        animation.update(&mut game_session);

        // Draw score and level
        queue!(
            stdout,
            cursor::MoveTo(2, 1),
            style::PrintStyledContent(format!("Level: {}", game_session.get_level()).green()),
            cursor::MoveTo(2, 2),
            style::PrintStyledContent(format!("Score: {}", game_session.get_score()).green()),
        )?;

        // Draw rectangles
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
        for (x, y, char, color) in animation.get_bird().draw() {
            queue!(
                stdout,
                cursor::MoveTo(x, y),
                style::PrintStyledContent(char.with(color))
            )?;
        }

        // Draw game over message
        if animation.is_game_over() {
            let game_over_text = "GAME OVER! Press ESC to quit";
            let restart_text = "Press R to restart";
            let text_x = (width - game_over_text.len() as u16) / 2;
            let text_y = height / 2;
            
            queue!(
                stdout,
                cursor::MoveTo(text_x, text_y),
                style::PrintStyledContent(game_over_text.red()),
                cursor::MoveTo((width - restart_text.len() as u16) / 2, text_y + 2),
                style::PrintStyledContent(restart_text.green())
            )?;
        }

        let game_level = game_session.get_level();
        if game_level > current_level {
            current_level = game_level;
            update_interval = (update_interval as f32 * 0.9) as u64; // Increase speed by reducing interval
        }


        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(update_interval));
    }

    game_session.cleanup_terminal()?;
    Ok(())
}