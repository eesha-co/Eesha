//! Chrome Widgets
//!
//! Defines the widget types used in the native browser chrome.
//! Each widget has an ID, a kind, and a bounding rectangle for hit testing.

use euclid::Rect;

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
    Tab { index: usize },
    /// Close button on a tab
    TabClose { index: usize },
    /// New tab button
    NewTabButton,
    /// Bookmark button in bookmark bar
    BookmarkButton { index: usize },
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
    pub rect: Rect<f32, euclid::DeviceUnit>,
    /// Whether the widget is currently hovered
    pub hovered: bool,
    /// Whether the widget is currently pressed
    pub pressed: bool,
    /// Whether the widget is disabled
    pub disabled: bool,
}

impl WidgetRect {
    /// Create a new widget rect
    pub fn new(id: WidgetId, kind: WidgetKind, rect: Rect<f32, euclid::DeviceUnit>) -> Self {
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
        self.rect.contains(euclid::Point2D::new(x, y))
    }
}

/// Widget ID counter for generating unique IDs
#[derive(Default)]
pub struct WidgetIdCounter {
    next_id: u64,
}

impl WidgetIdCounter {
    /// Generate a new unique widget ID
    pub fn next(&mut self) -> WidgetId {
        let id = self.next_id;
        self.next_id += 1;
        WidgetId(id)
    }
}
