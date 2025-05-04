mod app;
mod selectable;
mod ui;

use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::App;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directories to list files from
    #[arg(name = "DIRS")]
    dirs: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // TODO wrap item loading in a spawned thread so we don't block the UI
    // Load items
    if !args.dirs.is_empty() {
        // List files from directories
        for dir in args.dirs {
            let entries = fs::read_dir(&dir)
                .with_context(|| format!("Failed to read directory: {}", dir.display()))?;

            for entry in entries {
                if let Ok(entry) = entry {
                    app.push(entry.path().to_string_lossy().as_ref());
                }
            }
        }
    } else {
        // Read from stdin
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                app.push(&line);
            }
        }
    }

    // Run app
    let res = app.run(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match res {
        Ok(lines) => {
            for line in lines {
                println!("{}", line)
            }
        }
        Err(err) => {
            println!("{err:?}");
            return Err(anyhow::anyhow!("{:?}", err));
        }
    }

    Ok(())
}
