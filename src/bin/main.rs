extern crate picleo;

use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use picleo::selectable::Selectable;

use picleo::picker::Picker;

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
        let mut picker = Picker::<PathBuf>::new();

        picker.inject_items(|i|
            // List files from directories
            for dir in args.dirs {
                // TODO: might want to do something about errors here instead of silently ignoring them
                // .with_context(|| format!("Failed to read directory: {}", dir.display()))?;
                if let Ok(entries) = fs::read_dir(&dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        let str = path.to_string_lossy();
                        i.push(Selectable::new(str.to_string()), |columns| columns[0] = str.into());
                    }
                }
            }
        );

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
