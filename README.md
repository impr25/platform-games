# Platform Game Library

A Rust library for creating terminal-based platformer games with physics-based movement and obstacle avoidance.

## Features

- Physics-based bird movement with gravity and jumping mechanics
- Procedurally generated obstacles with random properties
- Level progression system with increasing difficulty
- Collision detection
- Terminal-based rendering using crossterm
- Customizable game parameters

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- A terminal that supports ANSI escape codes

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
game_lib = { path = "path/to/game_lib" }
```

### Basic Usage

```rust
use game_lib::{animation::Animation, game::GameSession};

fn main() -> std::io::Result<()> {
    let screen_size = (80, 24);
    let mut game_session = GameSession::new();
    game_session.init_terminal()?;
    game_session.start();

    let mut animation = Animation::new(screen_size, game_session.get_level());

    while game_session.is_running() {
        animation.update(&mut game_session);
        // Handle input, drawing, etc.
    }

    game_session.cleanup_terminal()?;
    Ok(())
}
```

## Running the Example

To run the included example game:

```bash
cargo run --example basic_game
```

Controls:
- Space: Make the bird jump
- R: Restart game (when game over)
- ESC: Quit game

## Testing

Run the test suite:

```bash
cargo test
```

This includes both unit tests and integration tests that verify the game mechanics work correctly.

## Documentation

Generate and view the documentation:

```bash
cargo doc --open
```

## Project Structure

- `game_lib/` - The core game library
  - `src/`
    - `animation.rs` - Game state and animation management
    - `bird.rs` - Player character physics and rendering
    - `game.rs` - Game session and state management
    - `rect.rs` - Obstacle generation and management
  - `examples/` - Example implementations
  - `tests/` - Integration tests

## License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details.
