mod app;
mod selectable;
mod ui;

use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use selectable::Selectable;

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

    // Create app state
    let mut app = App::new();

    // TODO wrap item loading in a spawned thread so we don't block the UI
    // Load items
    if !args.dirs.is_empty() {
        app.inject_items(|i|
            // List files from directories
            for dir in args.dirs {
                // TODO: might want to do something about errors here instead of silently ignoring them
                // .with_context(|| format!("Failed to read directory: {}", dir.display()))?;
                if let Ok(entries) = fs::read_dir(&dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        let str = path.to_string_lossy();
                        i.push(Selectable::new(str.clone().into()), |columns| columns[0] = str.into());
                    }
                }
            }
        );
    } else {
        app.inject_items(|i| {
            // Read from stdin
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                if let Ok(line) = line {
                    i.push(Selectable::new(line.clone()), |columns| {
                        columns[0] = line.into()
                    });
                }
            }
        });
    }

    // Run app
    match app.run() {
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
