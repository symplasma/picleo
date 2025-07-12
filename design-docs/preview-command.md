# Preview Command

Add a `--preview` flag that allows the specification of a preview command.

- The preview argument should be a string.
- Placeholders in the form of `{<COLUMN_NUMBER>}` or `{<COLUMN_NAME>}` will be replaced with appropriate values from the currently selected item.
- Column numbers start at 1, with the placeholders `{}` and `{0}` referring to the whole selected line.
- After substitution the line will be executed within a subshell.
- The output from this command will be displayed in a text area that takes up the right half of the screen.
