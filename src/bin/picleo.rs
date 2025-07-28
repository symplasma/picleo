extern crate picleo;

use anyhow::Result;
use clap::Parser;
use picleo::{picker::Picker, requested_items::RequestedItems, selectable::SelectableItem};
use std::{
    fmt, fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
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

    /// Preview command with placeholders like {1}, {2}, or {column_name}
    #[arg(short, long)]
    preview: Option<String>,

    /// Keep ANSI color codes in preview output
    #[arg(long)]
    keep_colors: bool,
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
    let dirs = args.dirs.clone();
    let preview_command = args.preview;

    // Check if we have any files vs directories to determine picker type
    if has_files && !has_dirs {
        // Only files - use String picker for file contents
        let mut picker = Picker::<String>::new(true);
        picker.set_keep_colors(args.keep_colors);
        if let Some(preview_cmd) = preview_command.clone() {
            picker.set_preview_command(preview_cmd);
        }

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
        let mut picker = Picker::<DisplayPath>::new(true);
        picker.set_keep_colors(args.keep_colors);
        if let Some(preview_cmd) = preview_command {
            picker.set_preview_command(preview_cmd);
        }

        // Collect directories to use for completion sources
        let completion_dirs: Vec<PathBuf> = args
            .dirs
            .clone()
            .into_iter()
            .filter(|d| d.is_dir())
            .collect();

        // Setup the autocomplete function
        picker.set_autocomplete(move |query| {
            // Create the completion suggestions vec and add the default entry i.e. what the user typed at the default path
            let mut suggestions = RequestedItems::from_vec(vec![match completion_dirs.first() {
                Some(dir) => SelectableItem::new_requested(
                    dir.join(query.to_string()).to_string_lossy().to_string(),
                ),
                None => SelectableItem::new_requested(query.to_string()),
            }]);

            // Split query as file path
            let path_to_match = Path::new(query);
            match (path_to_match.parent(), path_to_match.file_name()) {
                // Perform completion with the given path and file name
                (Some(parent), Some(file_name)) => {
                    // Add completions for all provided directories
                    for dir in &completion_dirs {
                        // Check if the path, joined with the user provided text exists and is a directory
                        let new_path = dir.join(parent);
                        if new_path.exists() && new_path.is_dir() {
                            // Read the directory entries
                            if let Ok(files) = fs::read_dir(dir) {
                                // Add completion suggestions
                                suggestions.extend(files.filter_map(|entry| {
                                    // Ignore errors in individual directory entries
                                    entry.ok().and_then(|e| {
                                        match e
                                            .file_name()
                                            .to_string_lossy()
                                            .starts_with(&file_name.to_string_lossy().to_string())
                                        {
                                            // Add any directory entries that have a matching prefix
                                            true => {
                                                let mut parent_path = dir.clone();
                                                parent_path.push(e.file_name());
                                                Some(SelectableItem::new_requested(
                                                    parent_path.to_string_lossy().to_string(),
                                                ))
                                            }
                                            false => None,
                                        }
                                    })
                                }));
                            }
                        }
                    }
                }

                // seems like other cases are not important and we can ignore them (at least in testing)
                _ => {}
            }

            suggestions
        });

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
    let mut picker = Picker::<String>::new(true);
    picker.set_keep_colors(args.keep_colors);
    if let Some(preview_cmd) = args.preview {
        picker.set_preview_command(preview_cmd);
    }
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
