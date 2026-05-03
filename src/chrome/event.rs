//! Chrome Event Handler
//!
//! Handles mouse and keyboard events for the native browser chrome.
//! This replaces the JavaScript-based event handling of the HTML panel.

use base::id::WebViewId;
use constellation_traits::{EmbedderToConstellationMessage, TraversalDirection};
use crossbeam_channel::Sender;
use embedder_traits::MouseButton;
use servo_url::ServoUrl;

use super::state::ChromeState;
use super::widget::{WidgetId, WidgetKind};

/// Result of handling a chrome event
#[derive(Clone, Debug)]
pub enum ChromeEventResult {
    /// No action needed
    None,
    /// Navigate back
    GoBack,
    /// Navigate forward
    GoForward,
    /// Refresh the current page
    Refresh,
    /// Stop loading the current page
    StopLoading,
    /// Navigate to home page
    GoHome,
    /// Navigate to a URL
    NavigateToUrl(String),
    /// Open a new tab
    NewTab,
    /// Switch to a tab by index
    SwitchTab(usize),
    /// Close a tab by index
    CloseTab(usize),
    /// Focus the URL bar
    FocusUrlBar,
    /// Request repaint
    RepaintNeeded,
}

/// Event handler for the native browser chrome
pub struct ChromeEventHandler;

impl ChromeEventHandler {
    /// Handle a mouse move event within the chrome area.
    /// Returns true if the event was consumed by the chrome.
    pub fn handle_mouse_move(
        state: &ChromeState,
        x: f32,
        y: f32,
    ) -> bool {
        // Find widget under cursor
        let widget = state.widget_at_point(x, y);
        let widget_id = widget.as_ref().map(|w| w.id);

        // Update hover state
        let old_hovered = state.hovered_widget();
        if old_hovered != widget_id {
            state.set_hovered(widget_id);
        }

        // If we're over a chrome widget, consume the event
        widget.is_some()
    }

    /// Check if a mouse button is the left button using pattern matching
    fn is_left_button(button: MouseButton) -> bool {
        matches!(button, MouseButton::Left)
    }

    /// Handle a mouse button down event within the chrome area.
    /// Returns (consumed, result) where consumed indicates the event was handled
    /// and result contains any action to take.
    pub fn handle_mouse_down(
        state: &ChromeState,
        x: f32,
        y: f32,
        button: MouseButton,
    ) -> (bool, ChromeEventResult) {
        if !Self::is_left_button(button) {
            return (false, ChromeEventResult::None);
        }

        // Find widget under cursor
        let widget = state.widget_at_point(x, y);
        if let Some(widget) = widget {
            state.set_pressed(Some(widget.id));
            (true, ChromeEventResult::None)
        } else {
            (false, ChromeEventResult::None)
        }
    }

    /// Handle a mouse button up event within the chrome area.
    /// Returns (consumed, result).
    pub fn handle_mouse_up(
        state: &ChromeState,
        x: f32,
        y: f32,
        button: MouseButton,
    ) -> (bool, ChromeEventResult) {
        if !Self::is_left_button(button) {
            return (false, ChromeEventResult::None);
        }

        let pressed = state.pressed_widget();
        let widget = state.widget_at_point(x, y);

        // Only trigger click if releasing on the same widget that was pressed
        if let (Some(pressed_id), Some(release_widget)) = (pressed, &widget) {
            if pressed_id == release_widget.id {
                let result = Self::handle_widget_click(state, &release_widget.kind);
                state.set_pressed(None);
                return (true, result);
            }
        }

        state.set_pressed(None);
        (widget.is_some(), ChromeEventResult::None)
    }

    /// Handle a click on a specific widget
    fn handle_widget_click(
        state: &ChromeState,
        kind: &WidgetKind,
    ) -> ChromeEventResult {
        match kind {
            WidgetKind::BackButton => {
                if state.nav_state.can_go_back {
                    ChromeEventResult::GoBack
                } else {
                    ChromeEventResult::None
                }
            }
            WidgetKind::ForwardButton => {
                if state.nav_state.can_go_forward {
                    ChromeEventResult::GoForward
                } else {
                    ChromeEventResult::None
                }
            }
            WidgetKind::RefreshButton => {
                if state.nav_state.loading {
                    ChromeEventResult::StopLoading
                } else {
                    ChromeEventResult::Refresh
                }
            }
            WidgetKind::HomeButton => {
                ChromeEventResult::GoHome
            }
            WidgetKind::UrlBar => {
                // Focus URL bar
                ChromeEventResult::FocusUrlBar
            }
            WidgetKind::NewTabButton => {
                ChromeEventResult::NewTab
            }
            WidgetKind::Tab { index } => {
                ChromeEventResult::SwitchTab(*index)
            }
            WidgetKind::TabClose { index } => {
                ChromeEventResult::CloseTab(*index)
            }
            WidgetKind::BookmarkButton { index: _ } => {
                // TODO: Navigate to bookmark URL
                ChromeEventResult::None
            }
            WidgetKind::MenuButton => {
                // TODO: Show menu
                ChromeEventResult::None
            }
            WidgetKind::DownloadsButton => {
                // TODO: Show downloads
                ChromeEventResult::None
            }
        }
    }

    /// Handle a keyboard event while the URL bar is focused.
    /// Returns (consumed, result).
    pub fn handle_url_bar_key_event(
        state: &mut ChromeState,
        key: &keyboard_types::Key,
        state_key: keyboard_types::KeyState,
        _modifiers: keyboard_types::Modifiers,
    ) -> (bool, ChromeEventResult) {
        if !state.url_bar.focused {
            return (false, ChromeEventResult::None);
        }

        if state_key == keyboard_types::KeyState::Up {
            return (true, ChromeEventResult::None);
        }

        match key {
            keyboard_types::Key::Enter => {
                let url = state.url_bar.text.trim().to_string();
                if !url.is_empty() {
                    state.url_bar.focused = false;
                    state.mark_dirty();
                    return (true, ChromeEventResult::NavigateToUrl(url));
                }
                (true, ChromeEventResult::None)
            }
            keyboard_types::Key::Escape => {
                // Restore original URL and unfocus
                if let Some(tab) = state.active_tab() {
                    state.url_bar.text = tab.url.clone();
                }
                state.url_bar.focused = false;
                state.mark_dirty();
                (true, ChromeEventResult::None)
            }
            keyboard_types::Key::Backspace => {
                if let Some(sel_start) = state.url_bar.selection_start {
                    // Delete selection
                    let start = sel_start.min(state.url_bar.cursor_pos);
                    let end = sel_start.max(state.url_bar.cursor_pos);
                    state.url_bar.text.replace_range(start..end, "");
                    state.url_bar.cursor_pos = start;
                    state.url_bar.selection_start = None;
                } else if state.url_bar.cursor_pos > 0 {
                    // Delete character before cursor
                    let pos = state.url_bar.cursor_pos;
                    state.url_bar.text.remove(pos - 1);
                    state.url_bar.cursor_pos -= 1;
                }
                state.mark_dirty();
                (true, ChromeEventResult::None)
            }
            keyboard_types::Key::Delete => {
                if let Some(sel_start) = state.url_bar.selection_start {
                    let start = sel_start.min(state.url_bar.cursor_pos);
                    let end = sel_start.max(state.url_bar.cursor_pos);
                    state.url_bar.text.replace_range(start..end, "");
                    state.url_bar.cursor_pos = start;
                    state.url_bar.selection_start = None;
                } else if state.url_bar.cursor_pos < state.url_bar.text.len() {
                    state.url_bar.text.remove(state.url_bar.cursor_pos);
                }
                state.mark_dirty();
                (true, ChromeEventResult::None)
            }
            keyboard_types::Key::ArrowLeft => {
                if state.url_bar.cursor_pos > 0 {
                    state.url_bar.cursor_pos -= 1;
                }
                state.mark_dirty();
                (true, ChromeEventResult::None)
            }
            keyboard_types::Key::ArrowRight => {
                if state.url_bar.cursor_pos < state.url_bar.text.len() {
                    state.url_bar.cursor_pos += 1;
                }
                state.mark_dirty();
                (true, ChromeEventResult::None)
            }
            keyboard_types::Key::Home => {
                state.url_bar.cursor_pos = 0;
                state.mark_dirty();
                (true, ChromeEventResult::None)
            }
            keyboard_types::Key::End => {
                state.url_bar.cursor_pos = state.url_bar.text.len();
                state.mark_dirty();
                (true, ChromeEventResult::None)
            }
            keyboard_types::Key::Character(c) => {
                if let Some(sel_start) = state.url_bar.selection_start {
                    let start = sel_start.min(state.url_bar.cursor_pos);
                    let end = sel_start.max(state.url_bar.cursor_pos);
                    state.url_bar.text.replace_range(start..end, c);
                    state.url_bar.cursor_pos = start + c.len();
                    state.url_bar.selection_start = None;
                } else {
                    state.url_bar.text.insert_str(state.url_bar.cursor_pos, c);
                    state.url_bar.cursor_pos += c.len();
                }
                state.mark_dirty();
                (true, ChromeEventResult::None)
            }
            _ => (true, ChromeEventResult::None),
        }
    }

    /// Execute a chrome event result by sending the appropriate constellation messages
    pub fn execute_result(
        result: ChromeEventResult,
        state: &mut ChromeState,
        constellation_sender: &Sender<EmbedderToConstellationMessage>,
    ) {
        match result {
            ChromeEventResult::GoBack => {
                if let Some(webview_id) = state.active_webview_id() {
                    let _ = constellation_sender.send(
                        EmbedderToConstellationMessage::TraverseHistory(
                            webview_id,
                            TraversalDirection::Back(1),
                        )
                    );
                }
            }
            ChromeEventResult::GoForward => {
                if let Some(webview_id) = state.active_webview_id() {
                    let _ = constellation_sender.send(
                        EmbedderToConstellationMessage::TraverseHistory(
                            webview_id,
                            TraversalDirection::Forward(1),
                        )
                    );
                }
            }
            ChromeEventResult::Refresh => {
                if let Some(webview_id) = state.active_webview_id() {
                    let _ = constellation_sender.send(
                        EmbedderToConstellationMessage::Reload(webview_id)
                    );
                }
            }
            ChromeEventResult::StopLoading => {
                // Servo doesn't have a StopLoading message; reload is the closest equivalent
                // or we can just ignore this for now
                log::debug!("StopLoading requested but not directly supported by constellation");
            }
            ChromeEventResult::GoHome => {
                let home_url = ServoUrl::parse("eesha://newtab").unwrap();
                if let Some(webview_id) = state.active_webview_id() {
                    let _ = constellation_sender.send(
                        EmbedderToConstellationMessage::LoadUrl(webview_id, home_url)
                    );
                }
            }
            ChromeEventResult::NavigateToUrl(url) => {
                let parsed_url = if url.contains("://") {
                    ServoUrl::parse(&url)
                } else if url.contains('.') && !url.contains(' ') {
                    ServoUrl::parse(&format!("https://{}", url))
                } else {
                    // Treat as search query - for now, use a simple search URL
                    ServoUrl::parse(&format!("https://duckduckgo.com/?q={}", url.replace(' ', "+")))
                };

                if let Ok(servo_url) = parsed_url {
                    if let Some(webview_id) = state.active_webview_id() {
                        let _ = constellation_sender.send(
                            EmbedderToConstellationMessage::LoadUrl(webview_id, servo_url)
                        );
                    }
                }
            }
            ChromeEventResult::NewTab => {
                // NewTab needs to be handled at the Window/App level because it needs
                // a WebViewId and ViewportDetails. Send a simpler approach - we just log
                // and the keyboard shortcut (Ctrl+T) handles this at the window level.
                log::debug!("NewTab requested via chrome - should be handled at window level");
            }
            ChromeEventResult::SwitchTab(index) => {
                state.set_active_tab(index);
            }
            ChromeEventResult::CloseTab(index) => {
                if let Some(tab) = state.tabs.get(index) {
                    let _ = constellation_sender.send(
                        EmbedderToConstellationMessage::CloseWebView(tab.webview_id)
                    );
                }
            }
            ChromeEventResult::FocusUrlBar => {
                state.url_bar.focused = true;
                state.url_bar.cursor_pos = state.url_bar.text.len();
                state.url_bar.selection_start = Some(0);
                state.mark_dirty();
            }
            ChromeEventResult::RepaintNeeded | ChromeEventResult::None => {}
        }
    }
}
