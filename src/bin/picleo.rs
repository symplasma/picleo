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

    /// Use threaded injection for better performance
    #[arg(short, long)]
    threaded: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Load items
    if !args.dirs.is_empty() {
        load_from_args(args)?
    } else {
        load_from_stdin(args)?
    }

    Ok(())
}

fn load_from_args(args: Args) -> Result<(), anyhow::Error> {
    let has_files = args.dirs.iter().any(|path| path.is_file());
    let has_dirs = args.dirs.iter().any(|path| path.is_dir());

    // these are here to prevent lifetime issues since args is a reference, error: borrowed data escapes outside of function
    let dirs = args.dirs;

    // Check if we have any files vs directories to determine picker type
    if has_files && !has_dirs {
        // Only files - use String picker for file contents
        let mut picker = Picker::<String>::new();

        for file_path in dirs {
            if file_path.is_file() {
                if args.threaded {
                    picker.inject_items_threaded(move |i| {
                        read_file_lines(&file_path, i);
                    });
                } else {
                    picker.inject_items(|i| {
                        read_file_lines(&file_path, i);
                    });
                }
            }
        }

        // Run app
        match picker.run() {
            Ok(selected_items) => {
                for line in selected_items.existing_values() {
                    println!("{}", line)
                }
                for requested_line in selected_items.requested_values() {
                    println!("{}", requested_line)
                }
            }
            Err(err) => {
                println!("{err:?}");
                return Err(anyhow::anyhow!("{:?}", err));
            }
        }
    } else {
        // Has directories or mixed - use DisplayPath picker for file paths
        let mut picker = Picker::<DisplayPath>::new();

        for path in dirs {
            if path.is_file() {
                // Add the file itself to the picker
                if args.threaded {
                    picker.inject_items_threaded(move |i| {
                        i.push(SelectableItem::new(DisplayPath(path)), |item, columns| {
                            columns[0] = item.to_string().into()
                        });
                    });
                } else {
                    picker.inject_items(|i| {
                        i.push(
                            SelectableItem::new(DisplayPath(path.clone())),
                            |item, columns| columns[0] = item.to_string().into(),
                        );
                    });
                }
            } else if path.is_dir() {
                // Handle directory as before
                if args.threaded {
                    picker.inject_items_threaded(move |i| {
                        if args.recursive {
                            // Recursively walk the directory
                            walk_dir_recursive(path, i);
                        } else {
                            // Non-recursive: only list direct children
                            walk_dir(path, i);
                        }
                    });
                } else {
                    picker.inject_items(|i| {
                        if args.recursive {
                            // Recursively walk the directory
                            walk_dir_recursive(path, i);
                        } else {
                            // Non-recursive: only list direct children
                            walk_dir(path, i);
                        }
                    });
                }
            }
        }

        // Run app
        match picker.run() {
            Ok(selected_items) => {
                for path in selected_items.existing_values() {
                    println!("{}", path.0.display())
                }
                for requested_path in selected_items.requested_values() {
                    println!("{}", requested_path)
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

fn load_from_stdin(args: Args) -> Result<(), anyhow::Error> {
    let mut picker = Picker::<String>::new();
    if args.threaded {
        picker.inject_items_threaded(|i| {
            // Read from stdin
            // TODO: might want to handle read errors from stdin
            for line in io::stdin().lock().lines().map_while(Result::ok) {
                i.push(SelectableItem::new(line), |item, columns| {
                    columns[0] = item.to_string().into()
                });
            }
        });
    } else {
        picker.inject_items(|i| {
            // Read from stdin
            // TODO: might want to handle read errors from stdin
            for line in io::stdin().lock().lines().map_while(Result::ok) {
                i.push(SelectableItem::new(line), |item, columns| {
                    columns[0] = item.to_string().into()
                });
            }
        });
    }
    // Create app state

    // Run app
    match picker.run() {
        Ok(selected_items) => {
            for line in selected_items.existing_values() {
                println!("{}", line)
            }
            for requested_line in selected_items.requested_values() {
                println!("{}", requested_line)
            }
        }
        Err(err) => {
            println!("{err:?}");
            return Err(anyhow::anyhow!("{:?}", err));
        }
    }

    Ok(())
}

fn walk_dir(dir: PathBuf, i: &nucleo::Injector<SelectableItem<DisplayPath>>) {
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            i.push(SelectableItem::new(DisplayPath(path)), |item, columns| {
                columns[0] = item.to_string().into()
            });
        }
    }
}

fn walk_dir_recursive(dir: PathBuf, injector: &nucleo::Injector<SelectableItem<DisplayPath>>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_dir_recursive(path, injector);
            } else {
                injector.push(SelectableItem::new(DisplayPath(path)), |item, columns| {
                    columns[0] = item.to_string().into()
                });
            }
        }
    }
}

fn read_file_lines(file_path: &PathBuf, injector: &nucleo::Injector<SelectableItem<String>>) {
    if let Ok(contents) = fs::read_to_string(file_path) {
        for line in contents.lines() {
            injector.push(SelectableItem::new(line.to_string()), |item, columns| {
                columns[0] = item.to_string().into()
            });
        }
    }
}
