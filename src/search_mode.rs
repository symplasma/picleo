use crate::picker::{EventResponse, Picker};
use crossterm::event::{Event, KeyCode, KeyModifiers, MouseButton, MouseEventKind};
use std::fmt::Display;

impl<T> Picker<T>
where
    T: Sync + Send + Display,
{
    /// Handle event processing when we are in search mode
    pub(crate) fn search_mode_handle_event(&mut self, event: Event) -> EventResponse {
        let mut event_response: EventResponse;

        match event {
            Event::Key(key) => {
                event_response = EventResponse::UpdateUI;

                match (key.code, key.modifiers) {
                    (KeyCode::Char(key), KeyModifiers::NONE)
                    | (KeyCode::Char(key), KeyModifiers::SHIFT) => {
                        self.append_to_query(key);
                        // NOTE: this probably doesn't need to saturate, that would require an absurdly long query
                        self.query_index = self.query_index.saturating_add(1);
                    }
                    (KeyCode::Backspace, KeyModifiers::NONE) => {
                        self.delete_from_query();
                        // NOTE: this needs to saturate to handle deleting when the query is empty
                        self.query_index = self.query_index.saturating_sub(1);
                    }
                    // TODO find out if it's a local keybinding that's preventing `Ctrl + Backspace` from working or if it's actually a bug
                    (KeyCode::Backspace, KeyModifiers::CONTROL)
                    | (KeyCode::Backspace, KeyModifiers::ALT) => {
                        self.delete_word_backward();
                    }
                    (KeyCode::Right, KeyModifiers::NONE) => {
                        // NOTE: this probably doesn't need to saturate, that would require an absurdly long query
                        self.query_index = self.query_index.saturating_add(1);
                    }
                    (KeyCode::Right, KeyModifiers::CONTROL)
                    | (KeyCode::Right, KeyModifiers::ALT) => {
                        self.jump_word_forward();
                    }
                    (KeyCode::Left, KeyModifiers::NONE) => {
                        // NOTE: this needs to saturate to handle deleting when the query is empty
                        self.query_index = self.query_index.saturating_sub(1);
                    }
                    (KeyCode::Left, KeyModifiers::CONTROL) | (KeyCode::Left, KeyModifiers::ALT) => {
                        self.jump_word_backward();
                    }
                    (KeyCode::Delete, KeyModifiers::CONTROL)
                    | (KeyCode::Delete, KeyModifiers::ALT) => {
                        self.delete_word_forward();
                    }
                    (KeyCode::Esc, KeyModifiers::NONE) => {
                        if self.query_is_empty() {
                            event_response = EventResponse::ExitProgram;
                        } else {
                            self.clear_query();
                        }
                    }
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        event_response = EventResponse::ExitProgram;
                    }
                    (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                        self.clear_query();
                        self.query_index = 0;
                    }
                    (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                        self.query_index = 0;
                    }
                    (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                        self.query_index = self.query.len();
                    }
                    (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                        self.delete_to_end();
                    }
                    (KeyCode::Enter, KeyModifiers::NONE) => {
                        // Print selected items and exit
                        event_response = EventResponse::ReturnSelectedItems;
                    }
                    (KeyCode::Down, KeyModifiers::NONE) => {
                        self.next();
                    }
                    (KeyCode::PageDown, KeyModifiers::NONE) => {
                        self.next_page();
                    }
                    (KeyCode::End, KeyModifiers::NONE) => {
                        self.end();
                    }
                    (KeyCode::Up, KeyModifiers::NONE) => {
                        self.previous();
                    }
                    (KeyCode::PageUp, KeyModifiers::NONE) => {
                        self.previous_page();
                    }
                    (KeyCode::Home, KeyModifiers::NONE) => {
                        self.home();
                    }
                    (KeyCode::Tab, KeyModifiers::NONE) => {
                        self.toggle_selected();
                        self.next();
                    }
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                        self.enter_editing_mode(self.current_item_text());
                    }
                    (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                        self.enter_editing_mode(String::new());
                    }

                    // ignore other key codes
                    _ => {
                        event_response = EventResponse::NoAction;
                    }
                }
            }
            Event::Mouse(mouse) => {
                event_response = EventResponse::UpdateUI;
                match mouse.kind {
                    MouseEventKind::ScrollUp => {
                        // Check if Shift or Control is held for page navigation
                        if mouse.modifiers.contains(KeyModifiers::SHIFT)
                            || mouse.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            if self.config.invert_scroll() {
                                self.next_page();
                            } else {
                                self.previous_page();
                            }
                        } else {
                            if self.config.invert_scroll() {
                                self.next();
                            } else {
                                self.previous();
                            }
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        // Check if Shift or Control is held for page navigation
                        if mouse.modifiers.contains(KeyModifiers::SHIFT)
                            || mouse.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            if self.config.invert_scroll() {
                                self.previous_page();
                            } else {
                                self.next_page();
                            }
                        } else {
                            if self.config.invert_scroll() {
                                self.previous();
                            } else {
                                self.next();
                            }
                        }
                    }
                    MouseEventKind::Down(MouseButton::Left) => {
                        // Handle left click on item lines to toggle selection
                        self.handle_item_click(mouse.row);
                    }
                    MouseEventKind::Down(MouseButton::Middle) => {
                        self.toggle_selected();
                    }
                    _ => {
                        event_response = EventResponse::NoAction;
                    }
                }
            }
            _ => {
                event_response = EventResponse::NoAction;
            }
        }
        event_response
    }
}
