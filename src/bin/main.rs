extern crate picleo;

use std::fs;
use std::fmt;
use std::io::{self, BufRead};
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use picleo::selectable::Selectable;

use picleo::picker::Picker;

// Wrapper for PathBuf that implements Display
#[derive(Debug, Clone)]
struct DisplayPath(PathBuf);

impl fmt::Display for DisplayPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl From<PathBuf> for DisplayPath {
    fn from(path: PathBuf) -> Self {
        DisplayPath(path)
    }
}

impl AsRef<PathBuf> for DisplayPath {
    fn as_ref(&self) -> &PathBuf {
        &self.0
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directories to list files from
    #[arg(name = "DIRS")]
    dirs: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // TODO wrap item loading in a spawned thread so we don't block the UI
    // Load items
    if !args.dirs.is_empty() {
        // Create app state
        let mut picker = Picker::<DisplayPath>::new();

        picker.inject_items(|i|
            // List files from directories
            for dir in args.dirs {
                // TODO: might want to do something about errors here instead of silently ignoring them
                // .with_context(|| format!("Failed to read directory: {}", dir.display()))?;
                if let Ok(entries) = fs::read_dir(&dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        let display_path = DisplayPath(path);
                        i.push(Selectable::new(display_path), |columns| columns[0] = display_path.to_string().into());
                    }
                }
            }
        );

        // Run app
        match picker.run() {
            Ok(paths) => {
                for path in paths {
                    println!("{}", path.0.display())
                }
            }
            Err(err) => {
                println!("{err:?}");
                return Err(anyhow::anyhow!("{:?}", err));
            }
        }
    } else {
        // Create app state
        let mut picker = Picker::<String>::new();

        picker.inject_items(|i| {
            // Read from stdin
            // TODO: might want to handle read errors from stdin
            for line in io::stdin().lock().lines().map_while(Result::ok) {
                i.push(Selectable::new(line.clone()), |columns| {
                    columns[0] = line.into()
                });
            }
        });

        // Run app
        match picker.run() {
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
    }

    Ok(())
}
