//! Native Chrome Module
//!
//! This module implements the browser chrome (navigation bar, tab bar, URL bar)
//! using native WebRender display lists instead of HTML-based UI.
//!
//! This provides:
//! - Better performance (no HTML rendering overhead for chrome)
//! - Better security (no web content in chrome context)
//! - Native look and feel
//! - Direct event handling without JavaScript bridge

mod state;
mod painter;
mod event;
mod widget;
mod theme;

pub use state::{ChromeState, NavigationState, TabInfo};
pub use painter::ChromePainter;
pub use event::{ChromeEventHandler, ChromeEventResult};
pub use widget::{WidgetId, WidgetKind, WidgetRect};
pub use theme::ChromeTheme;

/// Height of the tab bar in logical pixels
pub const TAB_BAR_HEIGHT: f32 = 34.0;
/// Height of the navigation bar in logical pixels
pub const NAV_BAR_HEIGHT: f32 = 44.0;
/// Height of the bookmark bar in logical pixels
pub const BOOKMARK_BAR_HEIGHT: f32 = 28.0;
/// Total chrome height (tab bar + nav bar)
pub const CHROME_HEIGHT: f32 = TAB_BAR_HEIGHT + NAV_BAR_HEIGHT;
/// Padding around widgets
pub const CHROME_PADDING: f32 = 4.0;
/// Spacing between widgets
pub const WIDGET_SPACING: f32 = 6.0;
