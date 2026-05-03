// Clipboard abstraction for desktop and mobile platforms.
// On Android/iOS, clipboard operations are no-ops since the `arboard` crate
// doesn't support those platforms.

/// Clipboard wrapper that works on all platforms.
/// On mobile platforms, all operations are no-ops.
pub struct Clipboard {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    inner: arboard::Clipboard,
}

impl Clipboard {
    /// Create a new Clipboard instance.
    /// Returns None if the clipboard cannot be accessed.
    pub fn new() -> Option<Self> {
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            arboard::Clipboard::new().ok().map(|inner| Self { inner })
        }
        #[cfg(any(target_os = "android", target_os = "ios"))]
        {
            None
        }
    }

    /// Get text from the clipboard.
    pub fn get_text(&mut self) -> Result<String, ClipboardError> {
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            self.inner.get_text().map_err(|e| ClipboardError(e.to_string()))
        }
        #[cfg(any(target_os = "android", target_os = "ios"))]
        {
            Err(ClipboardError("Clipboard not available on this platform".to_string()))
        }
    }

    /// Set text to the clipboard.
    pub fn set_text(&mut self, text: String) -> Result<(), ClipboardError> {
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            self.inner.set_text(text).map_err(|e| ClipboardError(e.to_string()))
        }
        #[cfg(any(target_os = "android", target_os = "ios"))]
        {
            let _ = text;
            Err(ClipboardError("Clipboard not available on this platform".to_string()))
        }
    }

    /// Clear the clipboard.
    pub fn clear(&mut self) -> Result<(), ClipboardError> {
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            self.inner.clear().map_err(|e| ClipboardError(e.to_string()))
        }
        #[cfg(any(target_os = "android", target_os = "ios"))]
        {
            Err(ClipboardError("Clipboard not available on this platform".to_string()))
        }
    }
}

/// Error type for clipboard operations.
#[derive(Debug)]
pub struct ClipboardError(String);

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClipboardError: {}", self.0)
    }
}

impl std::error::Error for ClipboardError {}
