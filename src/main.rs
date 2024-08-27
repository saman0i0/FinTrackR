mod app;
mod data;
mod ui;

use app::App;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::error::Error;
use std::io::stdout;

fn main() -> Result<(), Box<dyn Error>> {
    // Enable raw mode for the terminal to handle input directly
    enable_raw_mode()?;

    // Get a handle to the standard output
    let mut stdout = stdout();

    // Enter the alternate screen buffer to avoid modifying the user's screen
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    // Create a new App instance and run it
    // The `and_then` method ensures that `app.run()` is only called if `App::new()` succeeds
    let result = App::new().and_then(|mut app| app.run());

    // Restore the terminal to its previous state
    execute!(stdout, crossterm::terminal::LeaveAlternateScreen)?;
    disable_raw_mode()?;

    // Print error running the application
    if let Err(err) = result {
        println!("Error: {:?}", err);
    }

    Ok(())
}
