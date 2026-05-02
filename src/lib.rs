//! Eesha Web Browser
//!
//! This is the documentation of Eesha's types and items.
//! See [GitHub repository](https://github.com/eesha-browser/eesha) for more general introduction.

#![deny(missing_docs)]

/// Android JNI entry point
#[cfg(target_os = "android")]
mod android;

/// Eesha's compositor component to handle webrender.
pub mod compositor;
/// Utilities to read options and preferences.
pub mod config;
/// Error and result types.
pub mod errors;
/// Utilities to handle keyboard inputs and states.
pub mod keyboard;
/// Eesha's rendering context.
pub mod rendering;
/// Utilities to handle touch inputs and states.
pub mod touch;
/// Main entry types and functions.
pub mod app;
/// Web view types to handle web browsing contexts.
pub mod webview;
/// Eesha's window types to handle Winit's window.
pub mod window;
pub use errors::{Error, Result};
/// Utilities to write tests.
// pub mod test;
pub use app::Eesha;
/// Re-exporting Winit for the sake of convenience.
pub use winit;
/// Bookmark manager
pub mod bookmark;
/// Download manager
pub mod download;
/// Storage manager, handles all the storage operations,
/// such as reading and writing bookmarks, preferences, etc.
pub(crate) mod storage;
/// Window tabs manager
pub mod tab;
/// Utilities
pub(crate) mod utils;
/// Native browser chrome (navigation bar, tab bar, URL bar)
pub mod chrome;
