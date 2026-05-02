//! URL Input State Machine
//!
//! Manages the text input state for the URL/address bar, including
//! cursor position, text selection, and IME composition.

use url::Url;

/// Actions that can result from URL input interaction
#[derive(Debug, Clone)]
pub enum UrlInputAction {
    /// No action needed
    None,
    /// Navigate to the given URL
    Navigate(String),
    /// The URL bar lost focus
    Blur,
    /// The URL bar gained focus, select all text
    FocusSelectAll,
}

/// State of the URL text input field
#[derive(Debug, Clone)]
pub struct UrlInputState {
    /// Current text content
    pub text: String,
    /// Cursor position (byte offset)
    pub cursor_pos: usize,
    /// Start of selection (byte offset), if any
    pub selection_start: Option<usize>,
    /// Whether the input is currently focused
    pub is_focused: bool,
    /// IME composition text (if composing)
    pub composition_text: Option<String>,
    /// Placeholder text when empty
    pub placeholder: String,
}

impl Default for UrlInputState {
    fn default() -> Self {
        Self {
            text: String::new(),
            cursor_pos: 0,
            selection_start: None,
            is_focused: false,
            composition_text: None,
            placeholder: "Search or enter URL".to_string(),
        }
    }
}

impl UrlInputState {
    /// Create a new URL input state
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the URL text (e.g., when navigating to a new page)
    pub fn set_url(&mut self, url: &str) {
        self.text = url.to_string();
        self.cursor_pos = self.text.len();
        self.selection_start = None;
    }

    /// Focus the URL input and select all text
    pub fn focus(&mut self) {
        self.is_focused = true;
        self.selection_start = Some(0);
        self.cursor_pos = self.text.len();
    }

    /// Blur the URL input
    pub fn blur(&mut self) {
        self.is_focused = false;
        self.selection_start = None;
        self.composition_text = None;
    }

    /// Get the display text (shows composition text if composing)
    pub fn display_text(&self) -> String {
        if let Some(ref comp) = self.composition_text {
            let mut text = self.text.clone();
            if let Some(sel_start) = self.selection_start {
                text.replace_range(sel_start..self.cursor_pos, comp);
            } else {
                text.insert_str(self.cursor_pos, comp);
            }
            text
        } else {
            self.text.clone()
        }
    }

    /// Handle a character input
    pub fn handle_char(&mut self, ch: char) -> UrlInputAction {
        if !self.is_focused {
            return UrlInputAction::None;
        }

        // Delete selection if present
        self.delete_selection();

        // Insert character
        self.text.insert(self.cursor_pos, ch);
        self.cursor_pos += ch.len_utf8();
        self.selection_start = None;

        UrlInputAction::None
    }

    /// Handle a key event (for special keys)
    pub fn handle_key(&mut self, key_code: &keyboard_types::Code, modifiers: &keyboard_types::Modifiers) -> UrlInputAction {
        if !self.is_focused {
            return UrlInputAction::None;
        }

        use keyboard_types::{Code, Modifiers};

        match key_code {
            Code::Enter | Code::NumpadEnter => {
                let url_text = self.text.trim().to_string();
                if !url_text.is_empty() {
                    let url = self.normalize_url(&url_text);
                    self.blur();
                    UrlInputAction::Navigate(url)
                } else {
                    UrlInputAction::None
                }
            }
            Code::Escape => {
                self.blur();
                UrlInputAction::Blur
            }
            Code::Backspace => {
                if self.has_selection() {
                    self.delete_selection();
                } else if self.cursor_pos > 0 {
                    // Find previous character boundary
                    let prev = self.text[..self.cursor_pos]
                        .char_indices()
                        .last()
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    self.text.drain(prev..self.cursor_pos);
                    self.cursor_pos = prev;
                }
                UrlInputAction::None
            }
            Code::Delete => {
                if self.has_selection() {
                    self.delete_selection();
                } else if self.cursor_pos < self.text.len() {
                    let next = self.text[self.cursor_pos..]
                        .char_indices()
                        .nth(1)
                        .map(|(i, _)| self.cursor_pos + i)
                        .unwrap_or(self.text.len());
                    self.text.drain(self.cursor_pos..next);
                }
                UrlInputAction::None
            }
            Code::ArrowLeft => {
                if modifiers.contains(Modifiers::SHIFT) {
                    // Extend selection
                    if self.selection_start.is_none() {
                        self.selection_start = Some(self.cursor_pos);
                    }
                } else {
                    self.selection_start = None;
                }
                if self.cursor_pos > 0 {
                    let prev = self.text[..self.cursor_pos]
                        .char_indices()
                        .last()
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    self.cursor_pos = prev;
                }
                UrlInputAction::None
            }
            Code::ArrowRight => {
                if modifiers.contains(Modifiers::SHIFT) {
                    if self.selection_start.is_none() {
                        self.selection_start = Some(self.cursor_pos);
                    }
                } else {
                    self.selection_start = None;
                }
                if self.cursor_pos < self.text.len() {
                    let next = self.text[self.cursor_pos..]
                        .char_indices()
                        .nth(1)
                        .map(|(i, _)| self.cursor_pos + i)
                        .unwrap_or(self.text.len());
                    self.cursor_pos = next;
                }
                UrlInputAction::None
            }
            Code::Home => {
                if !modifiers.contains(Modifiers::SHIFT) {
                    self.selection_start = None;
                }
                self.cursor_pos = 0;
                UrlInputAction::None
            }
            Code::End => {
                if !modifiers.contains(Modifiers::SHIFT) {
                    self.selection_start = None;
                }
                self.cursor_pos = self.text.len();
                UrlInputAction::None
            }
            Code::KeyA if modifiers.contains(Modifiers::CONTROL) => {
                // Select all
                self.selection_start = Some(0);
                self.cursor_pos = self.text.len();
                UrlInputAction::None
            }
            _ => UrlInputAction::None,
        }
    }

    /// Handle IME composition start
    pub fn handle_ime_composition(&mut self, text: String) {
        self.composition_text = Some(text);
    }

    /// Handle IME composition end (commit)
    pub fn handle_ime_commit(&mut self, text: String) {
        self.composition_text = None;
        self.delete_selection();
        self.text.insert_str(self.cursor_pos, &text);
        self.cursor_pos += text.len();
    }

    /// Check if there is a text selection
    pub fn has_selection(&self) -> bool {
        self.selection_start.map_or(false, |start| start != self.cursor_pos)
    }

    /// Get the selected text range
    pub fn selection_range(&self) -> Option<(usize, usize)> {
        self.selection_start.map(|start| {
            if start <= self.cursor_pos {
                (start, self.cursor_pos)
            } else {
                (self.cursor_pos, start)
            }
        })
    }

    /// Delete the current selection
    fn delete_selection(&mut self) {
        if let Some((start, end)) = self.selection_range() {
            self.text.drain(start..end);
            self.cursor_pos = start;
            self.selection_start = None;
        }
    }

    /// Normalize a URL string (add scheme if missing)
    fn normalize_url(&self, input: &str) -> String {
        if input.starts_with("http://") || input.starts_with("https://") || input.starts_with("eesha://") {
            input.to_string()
        } else if input.starts_with("://") {
            format!("https{}", input)
        } else if input.contains('.') && !input.contains(' ') {
            // Looks like a URL
            format!("https://{}", input)
        } else {
            // Treat as search query (TODO: use search engine)
            format!("https://www.google.com/search?q={}", urlencoding::encode(input))
        }
    }
}

/// Simple URL encoding for search queries
mod urlencoding {
    pub fn encode(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
                    c.to_string()
                } else {
                    format!("%{:02X}", c as u32)
                }
            })
            .collect()
    }
}
