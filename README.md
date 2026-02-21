# Picleo

A fuzzy picker/matcher CLI using [nucleo](https://crates.io/crates/nucleo), as well as a library to implement this functionality in other programs.

Picleo was inspired by and is most similar to [nucleo-picker](https://lib.rs/crates/nucleo-picker) but with different/expanded functionality e.g. selection of multiple items, and adding non-existing items to the "selection" that should be created.

Additionally, this crate is a test for AI coding using [Aider](https://aider.chat/) and Claude 3.7 (at least initially). Code that is AI generated is marked with `(aider)` in the author field of git commit messages. Though all code is checked and refactored by the author.

## Status

This is currently **alpha** phase software. It is feature incomplete compared to [Skim](https://lib.rs/crates/skim), [fzf](https://github.com/junegunn/fzf), and other fuzzy finders, though it does handle some basic tasks.

The current development focus is on integrating this into a larger project as a selector widget. As such, it's unlikely to gain feature parity with other CLI fuzzy finders in the near future.

## Features and TODOs

This is a mix of feature list and roadmap. Checked features are present in the current version. Unchecked features may be added in any order.

- [x] Can handle selection of multiple items
- [x] Can handle input on `stdin`
- [x] List directory contents, with recursive option
- [x] List file contents
- [x] Preview command functionality
- [x] Allows for the creation of new items not originally in the input
- [x] Is fast for large numbers of items
- [x] Allows the core functionality to be embedded in other software as a library
- [x] Can wrap arbitrary object types and return the whole objects after matches
- [x] Supports mouse scroll wheel
- [x] Middle-click to toggle item selection
- [x] Click on items to toggle item selection
- [ ] Running commands on selected entries
- [ ] Support config via args and file
- [ ] Add interactive modification of config options
  - [ ] displaying ASCII colors
  - [ ] column delimiters
  - [ ] columns to display
  - [ ] columns to output
  - [ ] post-processing commands for existing items
  - [ ] post-processing commands for requested items
- [ ] Builder pattern for config when used as a library
- [ ] Allows custom sorting and re-sorting of items
- [ ] Customizable headers and header lines
- [ ] Support multi-column chooser with column naming
- [ ] Choose columns to display with delimiter pattern
- [ ] Choose columns to output with join string
- [ ] Support prefix based filters and args e.g. `title:`
- [ ] Has filtering options for fields besides those shown and fuzzy matched against
- [ ] Need to ensure that we restore the normal screen before printing error messages on panic
- [ ] Add help command to remind users of available options in both search and editing mode
- [ ] Add default prefix to be used with/instead of autocomplete
- [ ] Allow non-prefixed output? Maybe this should be a flag or just allow it if the prefix is not set.
- [ ] When finishing editing, take the version with the prefix
- [ ] Make all selected matches sort to the top of the results...unless there are too many?
- [ ] Need to ensure that lines do not mess up display via multi-width chars or other odd bytes e.g. when asked to take the head of binary files
- [ ] Ensure that displayed lines fit within the space alloted
- [ ] Limit the amount of data read from commands to about what fits in the preview area
- [ ] Provide preview command variables e.g. &LINES
- [ ] Make preview command execution async
  - [ ] Cache command output for a few seconds so rapidly moving arrows up or down does not re-run commands unnecessarily
- [ ] Allow preview commands to interrogate the terminal to get proper size and width
- [ ] Add flag to run command in shell and respect shell functions and aliases if possible
- [ ] Update match as indexing completes
- [ ] Make autocomplete in editing mode case insensitive, or allow toggling
- [ ] Allow editing of the preview command without restarting picleo
- [ ] Allow specification of post-processing commands for
  - [ ] Selected items
  - [ ] Requested items (double check whether requested items exist, edit mode can add them)
  - [ ] Allow invoking processing commands with the entire output in one go
  - [ ] Also allow invoking processing commands with lines as separate, possible parallel invocations

Due to the excellent design of [nucleo](https://docs.rs/nucleo/latest/nucleo/) we are able to load matches in separate threads while the user starts searching. Currently, the `--threaded` option controls whether we perform item loads in separate threads. Only one thread is used to load arguments from `STDIN` while one thread per arg is used when passing file path arguments.

## Usage

Picleo allows fuzzy finding of items from potentially enormous item lists. It's been tested with 300k+ items and the performance is good i.e. no user-noticeable delay.

- The currently selected item will be returned when pressing the `return` key.
- Multiple items can be selected by typing `Tab` or middle-clicking via the mouse.
- Escape causes picleo to exit and return nothing.

The TUI is rendered on `STDERR` in Alternate Screen Mode.

- Rendering on `STDERR` allows for easy redirection of the output on `STDOUT` into other files or programs.
- Alternate Screen mode is used so as not to interfere with other terminal output and scrollback.

### Search Syntax

Picleo supports the [fzf style search syntax](https://github.com/junegunn/fzf?tab=readme-ov-file#search-syntax) that [nucleo::pattern::AtomKind](https://docs.rs/nucleo/0.5.0/nucleo/pattern/enum.AtomKind.html#variants) supports.

Picleo can currently be used in two modes:

### Input Mode

Lines can be piped directly into picleo via `STDIN`.

```zsh
ls | picleo
```

### Directory Listing Mode

If one or more directories are specified on the command line, their contents will be listed in picleo. Recursive traversal can be achieved with the `--recursive` flag.

```zsh
picleo --recursive ~/Movies
```
