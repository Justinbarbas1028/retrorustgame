mod arcade;
mod games;
mod high_scores;
mod settings;

use arcade::ArcadeConsole;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::panic;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Establish custom panic hook to restore the terminal on unexpected crashes
    panic::set_hook(Box::new(|panic_info| {
        let _ = disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen);
        eprintln!("Arcade Crash Report:\n{}", panic_info);
    }));

    // 2. Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // 3. Initialize arcade
    let mut arcade = ArcadeConsole::new();
    let mut last_tick = Instant::now();
    let startup_time = Instant::now();

    // 4. Run real-time game loop capped at ~60 FPS
    loop {
        let now = Instant::now();
        let delta = now.duration_since(last_tick);
        last_tick = now;

        // Non-blocking tick update
        arcade.update(delta);

        // Render double-buffer frame
        terminal.draw(|f| {
            arcade.draw(f, f.size());
        })?;

        // Non-blocking event poll capped to 16.6ms (60 FPS)
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                // Ignore key release events (common on Windows)
                if key.kind == event::KeyEventKind::Press {
                    // Ignore query escape sequences interpreted as ESC key on startup
                    if startup_time.elapsed() > Duration::from_millis(200) {
                        let should_quit = arcade.handle_input(key.code);
                        if should_quit {
                            break;
                        }
                    }
                }
            }
        }
    }

    // 5. Restore terminal on graceful exits
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    println!("🕹️  Thanks for playing at the Rust Retro Arcade! Come back soon!");
    Ok(())
}
