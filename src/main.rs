mod app;
mod selectable;
mod ui;

use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::event::KeyModifiers;
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
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // TODO print selected results here

    if let Err(err) = res {
        println!("{err:?}");
        return Err(anyhow::anyhow!("{:?}", err));
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> AppResult<()> {
    loop {
        app.tick(10);
        terminal.draw(|f| ui(f, &mut app))?;

        if let Ok(Event::Key(key)) = event::read() {
            match (key.code, key.modifiers) {
                (KeyCode::Char(key), KeyModifiers::NONE) => {
                    app.append_to_query(key);
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => {
                    app.delete_from_query();
                }
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    return Ok(());
                }
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    return Ok(());
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    // Print selected items and exit
                    for item in app.selected_items().iter() {
                        println!("{}", item);
                    }
                    return Ok(());
                }
                (KeyCode::Down, KeyModifiers::NONE) => {
                    app.next();
                }
                (KeyCode::Up, KeyModifiers::NONE) => {
                    app.previous();
                }
                (KeyCode::Tab, KeyModifiers::NONE) => {
                    app.toggle_selected();
                }

                // ignore other key codes
                _ => {}
            }
        };
    }
}
