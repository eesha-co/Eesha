//! Chrome Widgets
//!
//! Defines the widget types used in the native browser chrome.
//! Each widget has an ID, a kind, and a bounding rectangle for hit testing.

use euclid::{Point2D, Rect, UnknownUnit};

/// Marker type for device-pixel rectangles in chrome widgets
pub type ChromeUnit = UnknownUnit;

/// Unique identifier for a widget within the chrome
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WidgetId(pub u64);

/// The kind of a chrome widget
#[derive(Clone, Debug, PartialEq)]
pub enum WidgetKind {
    /// Back navigation button
    BackButton,
    /// Forward navigation button
    ForwardButton,
    /// Refresh/stop button
    RefreshButton,
    /// Home button
    HomeButton,
    /// URL bar (text input)
    UrlBar,
    /// Tab in the tab bar
    Tab {
        /// Index of the tab in the tab list
        index: usize,
    },
    /// Close button on a tab
    TabClose {
        /// Index of the tab to close
        index: usize,
    },
    /// New tab button
    NewTabButton,
    /// Bookmark button in bookmark bar
    BookmarkButton {
        /// Index of the bookmark item
        index: usize,
    },
    /// Menu button (hamburger)
    MenuButton,
    /// Downloads button
    DownloadsButton,
}

/// A rectangular widget in the chrome with its ID, kind, and bounds
#[derive(Clone, Debug)]
pub struct WidgetRect {
    /// Unique widget ID
    pub id: WidgetId,
    /// Kind of widget
    pub kind: WidgetKind,
    /// Bounding rectangle in device pixels
    pub rect: Rect<f32, ChromeUnit>,
    /// Whether the widget is currently hovered
    pub hovered: bool,
    /// Whether the widget is currently pressed
    pub pressed: bool,
    /// Whether the widget is disabled
    pub disabled: bool,
}

impl WidgetRect {
    /// Create a new widget rect
    pub fn new(id: WidgetId, kind: WidgetKind, rect: Rect<f32, ChromeUnit>) -> Self {
        Self {
            id,
            kind,
            rect,
            hovered: false,
            pressed: false,
            disabled: false,
        }
    }

    /// Check if a point is inside this widget
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        self.rect.contains(Point2D::new(x, y))
    }
}

/// Widget ID counter for generating unique IDs
#[derive(Default)]
pub struct WidgetIdCounter {
    next_id: std::cell::Cell<u64>,
}

impl WidgetIdCounter {
    /// Generate a new unique widget ID
    pub fn next(&self) -> WidgetId {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        WidgetId(id)
    }
}
