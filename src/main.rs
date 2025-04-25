mod app;
mod ui;

use std::io::{self, BufRead};
use std::path::PathBuf;
use std::fs;
use std::time::Duration;

use anyhow::{Result, Context};
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::{App, AppResult};
use crate::ui::ui;

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
    
    // Load items
    if !args.dirs.is_empty() {
        // List files from directories
        for dir in args.dirs {
            let entries = fs::read_dir(&dir)
                .with_context(|| format!("Failed to read directory: {}", dir.display()))?;
            
            for entry in entries {
                if let Ok(entry) = entry {
                    app.items.push(entry.path().display().to_string());
                }
            }
        }
    } else {
        // Read from stdin
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                app.items.push(line);
            }
        }
    }

    // Run app
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
        return Err(err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> AppResult<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        // Print selected items and exit
                        for (i, item) in app.items.iter().enumerate() {
                            if app.selected.contains(&i) {
                                println!("{}", item);
                            }
                        }
                        return Ok(());
                    }
                    KeyCode::Down => {
                        app.next();
                    }
                    KeyCode::Up => {
                        app.previous();
                    }
                    KeyCode::Tab => {
                        app.toggle_selected();
                    }
                    _ => {}
                }
            }
        }
    }
}
