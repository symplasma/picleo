extern crate picleo;

use anyhow::Result;
use clap::Parser;
use picleo::{picker::Picker, requested_items::RequestedItems, selectable::SelectableItem};
use std::{
    fmt, fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
};
use std::collections::HashMap;

// Wrapper for PathBuf that stores both full path and display string
#[derive(Debug, Clone)]
struct DisplayPath {
    full_path: PathBuf,
    display_name: String,
}

impl DisplayPath {
    fn new(full_path: PathBuf, display_name: String) -> Self {
        Self {
            full_path,
            display_name,
        }
    }

    fn simple(path: PathBuf) -> Self {
        let display_name = path.display().to_string();
        Self {
            full_path: path,
            display_name,
        }
    }
}

impl fmt::Display for DisplayPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name)
    }
}

impl From<PathBuf> for DisplayPath {
    fn from(path: PathBuf) -> Self {
        DisplayPath::simple(path)
    }
}

impl AsRef<PathBuf> for DisplayPath {
    fn as_ref(&self) -> &PathBuf {
        &self.full_path
    }
}

/// Find the longest common path prefix among a list of paths
fn find_common_prefix(paths: &[PathBuf]) -> PathBuf {
    if paths.is_empty() {
        return PathBuf::new();
    }
    if paths.len() == 1 {
        // For a single path, return its parent directory
        return paths[0].parent().map(|p| p.to_path_buf()).unwrap_or_default();
    }

    let mut common_components: Vec<_> = paths[0].components().collect();

    for path in paths.iter().skip(1) {
        let path_components: Vec<_> = path.components().collect();
        let mut new_common = Vec::new();

        for (a, b) in common_components.iter().zip(path_components.iter()) {
            if a == b {
                new_common.push(*a);
            } else {
                break;
            }
        }
        common_components = new_common;
    }

    common_components.iter().collect()
}

/// Compute minimal unique display names for paths by removing common prefix
/// but keeping enough path components to make names unambiguous
fn compute_display_names(paths: &[PathBuf]) -> Vec<(PathBuf, String)> {
    if paths.is_empty() {
        return Vec::new();
    }

    let common_prefix = find_common_prefix(paths);
    let prefix_len = common_prefix.components().count();

    // First pass: strip common prefix from all paths
    let mut stripped: Vec<(PathBuf, Vec<String>)> = paths
        .iter()
        .map(|p| {
            let components: Vec<String> = p
                .components()
                .skip(prefix_len)
                .map(|c| c.as_os_str().to_string_lossy().to_string())
                .collect();
            (p.clone(), components)
        })
        .collect();

    // Check for ambiguity: if multiple paths have the same file name,
    // we need to include more path components
    let mut result: Vec<(PathBuf, String)> = Vec::with_capacity(paths.len());

    // Group by file name to detect ambiguity
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for (_, components) in &stripped {
        if let Some(name) = components.last() {
            *name_counts.entry(name.clone()).or_insert(0) += 1;
        }
    }

    for (full_path, components) in stripped.drain(..) {
        if components.is_empty() {
            result.push((full_path.clone(), full_path.display().to_string()));
            continue;
        }

        let file_name = components.last().unwrap().clone();
        let is_ambiguous = name_counts.get(&file_name).map(|&c| c > 1).unwrap_or(false);

        let display_name = if is_ambiguous && components.len() > 1 {
            // Include parent directory to disambiguate
            // Find minimum components needed for uniqueness
            let mut needed_components = 1;
            'outer: for n in 2..=components.len() {
                let suffix: Vec<_> = components.iter().skip(components.len() - n).collect();
                let suffix_str: String = suffix.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("/");

                // Check if this suffix is unique among all paths
                let mut count = 0;
                for (_, other_components) in paths.iter().zip(
                    paths.iter().map(|p| {
                        p.components()
                            .skip(prefix_len)
                            .map(|c| c.as_os_str().to_string_lossy().to_string())
                            .collect::<Vec<_>>()
                    }),
                ) {
                    if other_components.len() >= n {
                        let other_suffix: String = other_components
                            .iter()
                            .skip(other_components.len() - n)
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join("/");
                        if other_suffix == suffix_str {
                            count += 1;
                        }
                    }
                }
                if count == 1 {
                    needed_components = n;
                    break 'outer;
                }
            }
            components
                .iter()
                .skip(components.len().saturating_sub(needed_components))
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join("/")
        } else {
            components.join("/")
        };

        result.push((full_path, display_name));
    }

    result
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

        // Collect all file paths first to compute common prefix
        let mut all_paths: Vec<PathBuf> = Vec::new();
        for path in &dirs {
            if path.is_file() {
                let abs_path = fs::canonicalize(path).unwrap_or_else(|_| path.clone());
                all_paths.push(abs_path);
            } else if path.is_dir() {
                collect_paths_from_dir(path, args.recursive, &mut all_paths);
            }
        }

        // Compute display names with common prefix removed
        let display_names = compute_display_names(&all_paths);

        // Create a map from full path to display name for quick lookup
        let path_to_display: HashMap<PathBuf, String> = display_names.into_iter().collect();

        // Inject items with computed display names
        for path in dirs {
            if path.is_file() {
                let abs_path = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
                let display_name = path_to_display
                    .get(&abs_path)
                    .cloned()
                    .unwrap_or_else(|| abs_path.display().to_string());
                let display_path = DisplayPath::new(abs_path, display_name);

                if args.threaded {
                    picker.inject_items_threaded(move |i| {
                        i.push(SelectableItem::new(display_path), |item, columns| {
                            columns[0] = item.to_string().into()
                        });
                    });
                } else {
                    picker.inject_items(|i| {
                        i.push(SelectableItem::new(display_path.clone()), |item, columns| {
                            columns[0] = item.to_string().into()
                        });
                    });
                }
            } else if path.is_dir() {
                let path_to_display = path_to_display.clone();
                let recursive = args.recursive;

                if args.threaded {
                    picker.inject_items_threaded(move |i| {
                        if recursive {
                            walk_dir_recursive_with_display(&path, i, &path_to_display);
                        } else {
                            walk_dir_with_display(&path, i, &path_to_display);
                        }
                    });
                } else {
                    picker.inject_items(|i| {
                        if recursive {
                            walk_dir_recursive_with_display(&path, i, &path_to_display);
                        } else {
                            walk_dir_with_display(&path, i, &path_to_display);
                        }
                    });
                }
            }
        }

        // Run app
        match picker.run() {
            Ok(selected_items) => {
                for path in selected_items.existing_values() {
                    // Print the full absolute path
                    println!("{}", path.full_path.display())
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

/// Collect all file paths from a directory (used for computing common prefix)
fn collect_paths_from_dir(dir: &PathBuf, recursive: bool, paths: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && recursive {
                collect_paths_from_dir(&path, recursive, paths);
            } else if path.is_file() {
                let abs_path = fs::canonicalize(&path).unwrap_or_else(|_| path);
                paths.push(abs_path);
            }
        }
    }
}

fn walk_dir_with_display(
    dir: &PathBuf,
    i: &nucleo::Injector<SelectableItem<DisplayPath>>,
    path_to_display: &HashMap<PathBuf, String>,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let abs_path = fs::canonicalize(&path).unwrap_or_else(|_| path);
            let display_name = path_to_display
                .get(&abs_path)
                .cloned()
                .unwrap_or_else(|| abs_path.display().to_string());
            let display_path = DisplayPath::new(abs_path, display_name);
            i.push(SelectableItem::new(display_path), |item, columns| {
                columns[0] = item.to_string().into()
            });
        }
    }
}

fn walk_dir_recursive_with_display(
    dir: &PathBuf,
    injector: &nucleo::Injector<SelectableItem<DisplayPath>>,
    path_to_display: &HashMap<PathBuf, String>,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_dir_recursive_with_display(&path, injector, path_to_display);
            } else {
                let abs_path = fs::canonicalize(&path).unwrap_or_else(|_| path);
                let display_name = path_to_display
                    .get(&abs_path)
                    .cloned()
                    .unwrap_or_else(|| abs_path.display().to_string());
                let display_path = DisplayPath::new(abs_path, display_name);
                injector.push(SelectableItem::new(display_path), |item, columns| {
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
