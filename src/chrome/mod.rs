//! Native Chrome Module
//!
//! This module implements the browser's UI chrome (toolbar, tabs, address bar)
//! using native WebRender display list primitives instead of HTML-based rendering.
//!
//! ## Architecture
//!
//! The native chrome is rendered directly into WebRender's display list alongside
//! content WebViews. This eliminates the need for an HTML-based panel WebView and
//! the insecure `window.prompt()` message bus that was previously used for
//! communication between the browser chrome and the Rust backend.
//!
//! ## Security Benefits
//!
//! - **No JavaScript execution in chrome**: The native chrome is pure Rust, making
//!   it immune to XSS attacks that could compromise the browser UI.
//! - **No prompt() message bus**: Eliminates string injection vectors.
//! - **No navigation in chrome**: The HTML panel had to explicitly block navigation;
//!   native chrome has no concept of navigation.
//! - **Input isolation**: Chrome hit-testing is separate from content hit-testing.

pub mod native_chrome;
pub mod theme;
pub mod widgets;
pub mod url_input;
pub mod icons;
pub mod keyboard;

pub use native_chrome::{NativeChrome, ChromeAction, ChromeElementId, ChromeFocusTarget};
pub use theme::ChromeTheme;
