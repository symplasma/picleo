# TODO

- [ ] Need to ensure that we restore the normal screen before printing error messages on panic
- [ ] Add help command to remind users of available options in both search and editing mode
- [ ] Need to ensure that lines do not mess up display via multi-width chars or other odd bytes e.g. when asked to take the head of binary files
- [ ] Ensure that displayed lines fit within the space alloted
- [ ] Limit the amount of data read from commands to about what fits in the preview area
- [ ] Provide preview command variables e.g. &LINES
- [ ] Allow preview commands to interrogate the terminal to get proper size and width
- [ ] Update match as indexing completes
- [ ] Make autocomplete in editing mode case insensitive, or allow toggling
- [ ] Allow editing of the preview command without restarting picleo
- [ ] Allow specification of post-processing commands for
  - [ ] Selected items
  - [ ] Requested items (double check whether requested items exist, edit mode can add them)
  - [ ] Allow invoking processing commands with the entire output in one go
  - [ ] Also allow invoking processing commands with lines as separate, possible parallel invocations
