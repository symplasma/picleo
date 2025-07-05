extern crate picleo;

use anyhow::Result;
use clap::Parser;
use picleo::{picker::Picker, selectable::SelectableItem};
use std::{
    fmt, fs,
    io::{self, BufRead},
    path::PathBuf,
};

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

    /// Recursively index files in directories
    #[arg(short, long)]
    recursive: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // TODO wrap item loading in a spawned thread so we don't block the UI
    // Load items
    if !args.dirs.is_empty() {
        // Create app state
        let mut picker = Picker::<DisplayPath>::new();

        picker.inject_items(|i| {
            // List files from directories
            for dir in args.dirs {
                if args.recursive {
                    // Recursively walk the directory
                    fn walk_dir(
                        dir: &PathBuf,
                        injector: &nucleo::Injector<SelectableItem<DisplayPath>>,
                    ) {
                        if let Ok(entries) = fs::read_dir(dir) {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if path.is_dir() {
                                    walk_dir(&path, injector);
                                } else {
                                    injector.push(
                                        SelectableItem::new(DisplayPath(path)),
                                        |item, columns| columns[0] = item.to_string().into(),
                                    );
                                }
                            }
                        }
                    }
                    walk_dir(&dir, i);
                } else {
                    // Non-recursive: only list direct children
                    if let Ok(entries) = fs::read_dir(&dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            i.push(SelectableItem::new(DisplayPath(path)), |item, columns| {
                                columns[0] = item.to_string().into()
                            });
                        }
                    }
                }
            }
        });

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
                i.push(SelectableItem::new(line), |item, columns| {
                    columns[0] = item.to_string().into()
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
