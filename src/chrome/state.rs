//! Chrome State
//!
//! Manages the state of the native browser chrome, including
//! navigation history, tab information, URL bar state, and
//! widget layout.

use std::cell::RefCell;

use base::id::WebViewId;
use webrender_api::units::DeviceRect;

use super::widget::{WidgetId, WidgetIdCounter, WidgetRect, WidgetKind, ChromeUnit};
use super::theme::ChromeTheme;
use super::{TAB_BAR_HEIGHT, NAV_BAR_HEIGHT, BOOKMARK_BAR_HEIGHT, CHROME_PADDING};

/// Information about a single tab
#[derive(Clone, Debug)]
pub struct TabInfo {
    /// The WebView ID for this tab
    pub webview_id: WebViewId,
    /// The title of the page
    pub title: String,
    /// The URL of the page
    pub url: String,
    /// Whether this tab is loading
    pub loading: bool,
    /// Whether this tab can go back
    pub can_go_back: bool,
    /// Whether this tab can go forward
    pub can_go_forward: bool,
}

/// State of the URL bar
#[derive(Clone, Debug)]
pub struct UrlBarState {
    /// The current text in the URL bar
    pub text: String,
    /// Whether the URL bar is focused
    pub focused: bool,
    /// The cursor position
    pub cursor_pos: usize,
    /// The selection start (if any)
    pub selection_start: Option<usize>,
}

impl Default for UrlBarState {
    fn default() -> Self {
        Self {
            text: String::new(),
            focused: false,
            cursor_pos: 0,
            selection_start: None,
        }
    }
}

/// Navigation state for the current tab
#[derive(Clone, Debug, Default)]
pub struct NavigationState {
    /// Can navigate back
    pub can_go_back: bool,
    /// Can navigate forward
    pub can_go_forward: bool,
    /// Current page is loading
    pub loading: bool,
    /// Loading progress (0.0 to 1.0)
    pub loading_progress: f32,
}

/// The overall state of the browser chrome
pub struct ChromeState {
    /// The visual theme
    pub theme: ChromeTheme,
    /// Tab information
    pub tabs: Vec<TabInfo>,
    /// Index of the active tab
    pub active_tab_index: usize,
    /// Navigation state
    pub nav_state: NavigationState,
    /// URL bar state
    pub url_bar: UrlBarState,
    /// Whether the bookmark bar is visible
    pub show_bookmark_bar: bool,
    /// Widget ID counter
    widget_id_counter: WidgetIdCounter,
    /// Current widget layout (updated after each paint)
    widgets: RefCell<Vec<WidgetRect>>,
    /// Currently hovered widget ID
    hovered_widget: RefCell<Option<WidgetId>>,
    /// Currently pressed widget ID
    pressed_widget: RefCell<Option<WidgetId>>,
    /// Whether the chrome needs repainting
    dirty: RefCell<bool>,
}

impl ChromeState {
    /// Create a new ChromeState with default dark theme
    pub fn new() -> Self {
        Self {
            theme: ChromeTheme::default(),
            tabs: Vec::new(),
            active_tab_index: 0,
            nav_state: NavigationState::default(),
            url_bar: UrlBarState::default(),
            show_bookmark_bar: false,
            widget_id_counter: WidgetIdCounter::default(),
            widgets: RefCell::new(Vec::new()),
            hovered_widget: RefCell::new(None),
            pressed_widget: RefCell::new(None),
            dirty: RefCell::new(true),
        }
    }

    /// Create a new ChromeState with light theme
    pub fn light_theme() -> Self {
        let mut state = Self::new();
        state.theme = ChromeTheme::light();
        state
    }

    /// Add a new tab
    pub fn add_tab(&mut self, webview_id: WebViewId, url: String) {
        let title = if url == "eesha://newtab" {
            "New Tab".to_string()
        } else {
            url.clone()
        };
        self.tabs.push(TabInfo {
            webview_id,
            title,
            url: url.clone(),
            loading: false,
            can_go_back: false,
            can_go_forward: false,
        });
        self.active_tab_index = self.tabs.len() - 1;
        self.url_bar.text = url;
        self.mark_dirty();
    }

    /// Close a tab by index
    pub fn close_tab(&mut self, index: usize) -> Option<TabInfo> {
        if self.tabs.is_empty() {
            return None;
        }
        if index >= self.tabs.len() {
            return None;
        }
        let tab = self.tabs.remove(index);
        if self.active_tab_index >= self.tabs.len() && !self.tabs.is_empty() {
            self.active_tab_index = self.tabs.len() - 1;
        }
        // Update URL bar to show active tab URL
        if let Some(active) = self.tabs.get(self.active_tab_index) {
            self.url_bar.text = active.url.clone();
        }
        self.mark_dirty();
        Some(tab)
    }

    /// Set the active tab by index
    pub fn set_active_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab_index = index;
            let tab = &self.tabs[index];
            self.url_bar.text = tab.url.clone();
            self.nav_state.can_go_back = tab.can_go_back;
            self.nav_state.can_go_forward = tab.can_go_forward;
            self.nav_state.loading = tab.loading;
            self.mark_dirty();
        }
    }

    /// Update the URL of a tab
    pub fn update_tab_url(&mut self, webview_id: WebViewId, url: String) {
        if let Some(tab) = self.tabs.iter_mut().find(|t| t.webview_id == webview_id) {
            tab.url = url.clone();
            // If this is the active tab, update the URL bar
            let active_id = self.tabs.get(self.active_tab_index).map(|t| t.webview_id);
            if active_id == Some(webview_id) {
                if !self.url_bar.focused {
                    self.url_bar.text = url;
                }
            }
            self.mark_dirty();
        }
    }

    /// Update the title of a tab
    pub fn update_tab_title(&mut self, webview_id: WebViewId, title: String) {
        if let Some(tab) = self.tabs.iter_mut().find(|t| t.webview_id == webview_id) {
            tab.title = title;
            self.mark_dirty();
        }
    }

    /// Update navigation state
    pub fn update_nav_state(&mut self, can_go_back: bool, can_go_forward: bool) {
        self.nav_state.can_go_back = can_go_back;
        self.nav_state.can_go_forward = can_go_forward;
        // Also update the tab info
        if let Some(tab) = self.tabs.get_mut(self.active_tab_index) {
            tab.can_go_back = can_go_back;
            tab.can_go_forward = can_go_forward;
        }
        self.mark_dirty();
    }

    /// Set loading state for a tab
    pub fn set_loading(&mut self, webview_id: WebViewId, loading: bool) {
        // First, update the tab's loading state
        let mut found_active = false;
        for tab in &mut self.tabs {
            if tab.webview_id == webview_id {
                tab.loading = loading;
                found_active = true;
                break;
            }
        }
        
        // Check if this is the active tab and update nav state
        if found_active {
            if let Some(active_tab) = self.tabs.get(self.active_tab_index) {
                if active_tab.webview_id == webview_id {
                    self.nav_state.loading = loading;
                }
            }
        }
        self.mark_dirty();
    }

    /// Get the active tab's WebView ID
    pub fn active_webview_id(&self) -> Option<WebViewId> {
        self.tabs.get(self.active_tab_index).map(|t| t.webview_id)
    }

    /// Generate a new widget ID
    pub fn next_widget_id(&self) -> WidgetId {
        self.widget_id_counter.next()
    }

    /// Get the current widget layout
    pub fn widgets(&self) -> std::cell::Ref<'_, Vec<WidgetRect>> {
        self.widgets.borrow()
    }

    /// Set the widget layout (called after painting)
    pub fn set_widgets(&self, new_widgets: Vec<WidgetRect>) {
        *self.widgets.borrow_mut() = new_widgets;
    }

    /// Find the widget at a given point
    pub fn widget_at_point(&self, x: f32, y: f32) -> Option<WidgetRect> {
        let widgets = self.widgets.borrow();
        // Iterate in reverse so topmost widgets are found first
        for widget in widgets.iter().rev() {
            if widget.contains_point(x, y) {
                return Some(widget.clone());
            }
        }
        None
    }

    /// Set the hovered widget
    pub fn set_hovered(&self, widget_id: Option<WidgetId>) {
        let mut current = self.hovered_widget.borrow_mut();
        if *current != widget_id {
            *current = widget_id;
            self.mark_dirty();
        }
    }

    /// Get the hovered widget ID
    pub fn hovered_widget(&self) -> Option<WidgetId> {
        *self.hovered_widget.borrow()
    }

    /// Set the pressed widget
    pub fn set_pressed(&self, widget_id: Option<WidgetId>) {
        *self.pressed_widget.borrow_mut() = widget_id;
        self.mark_dirty();
    }

    /// Get the pressed widget ID
    pub fn pressed_widget(&self) -> Option<WidgetId> {
        *self.pressed_widget.borrow()
    }

    /// Mark the chrome as needing repaint
    pub fn mark_dirty(&self) {
        *self.dirty.borrow_mut() = true;
    }

    /// Check if the chrome needs repaint and clear the dirty flag
    pub fn take_dirty(&self) -> bool {
        let dirty = *self.dirty.borrow();
        *self.dirty.borrow_mut() = false;
        dirty
    }

    /// Get the total chrome height in device pixels
    pub fn chrome_height(&self, scale_factor: f32) -> f32 {
        let mut height = TAB_BAR_HEIGHT + NAV_BAR_HEIGHT;
        if self.show_bookmark_bar {
            height += BOOKMARK_BAR_HEIGHT;
        }
        height * scale_factor
    }

    /// Get the content area rect (the area where web content is drawn)
    pub fn content_rect(&self, window_rect: DeviceRect, scale_factor: f32) -> DeviceRect {
        let chrome_height = self.chrome_height(scale_factor);
        let origin = webrender_api::units::DevicePoint::new(
            window_rect.min.x,
            window_rect.min.y + chrome_height,
        );
        let size = webrender_api::units::DeviceSize::new(
            window_rect.width(),
            window_rect.height() - chrome_height,
        );
        DeviceRect::from_origin_and_size(origin, size)
    }

    /// Get the active tab info
    pub fn active_tab(&self) -> Option<&TabInfo> {
        self.tabs.get(self.active_tab_index)
    }

    /// Get the tab index by WebView ID
    pub fn tab_index_by_webview_id(&self, webview_id: WebViewId) -> Option<usize> {
        self.tabs.iter().position(|t| t.webview_id == webview_id)
    }
}

impl Default for ChromeState {
    fn default() -> Self {
        Self::new()
    }
}
