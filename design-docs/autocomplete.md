# Autocomplete

Add an `autocomplete` method on the picker:

- Should take lambda that receives the current string from the editor
- Should return a Vec of strings
- The lambda should be invoked when the text in the editing mode is modified
- While in the in the editing mode, the list of autocomplete strings should be displayed where the matches are normally displayed in the fuzzy find mode
