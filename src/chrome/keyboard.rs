//! Chrome Keyboard Handling
//!
//! Handles keyboard shortcuts for the browser chrome (Ctrl+T, Ctrl+W, Ctrl+L, etc.)

use keyboard_types::{Code, Modifiers};

/// Actions that can be triggered by keyboard shortcuts in the chrome
#[derive(Debug, Clone, PartialEq)]
pub enum ChromeKeyAction {
    /// Open a new tab (Ctrl+T)
    NewTab,
    /// Close current tab (Ctrl+W)
    CloseTab,
    /// Focus the URL bar (Ctrl+L)
    FocusUrlBar,
    /// Navigate back (Alt+Left)
    GoBack,
    /// Navigate forward (Alt+Right)
    GoForward,
    /// Refresh page (Ctrl+R / F5)
    Refresh,
    /// Hard refresh (Ctrl+Shift+R / Ctrl+F5)
    HardRefresh,
    /// Open new window (Ctrl+N)
    NewWindow,
    /// Toggle bookmark (Ctrl+D)
    ToggleBookmark,
    /// Open bookmark manager (Ctrl+Shift+O)
    OpenBookmarkManager,
    /// Open history (Ctrl+H)
    OpenHistory,
    /// Select next tab (Ctrl+Tab)
    NextTab,
    /// Select previous tab (Ctrl+Shift+Tab)
    PrevTab,
    /// Select tab by index (Ctrl+1-8)
    SelectTab(usize),
    /// Select last tab (Ctrl+9)
    SelectLastTab,
    /// Find in page (Ctrl+F)
    FindInPage,
    /// No action
    None,
}

/// Process a keyboard event and return the corresponding chrome action
pub fn handle_chrome_key_event(code: &Code, modifiers: &Modifiers) -> ChromeKeyAction {
    let ctrl = modifiers.contains(Modifiers::CONTROL);
    let alt = modifiers.contains(Modifiers::ALT);
    let shift = modifiers.contains(Modifiers::SHIFT);

    match code {
        Code::KeyT if ctrl && !shift => ChromeKeyAction::NewTab,
        Code::KeyW if ctrl && !shift => ChromeKeyAction::CloseTab,
        Code::KeyL if ctrl && !shift => ChromeKeyAction::FocusUrlBar,
        Code::KeyR if ctrl && !shift => ChromeKeyAction::Refresh,
        Code::KeyR if ctrl && shift => ChromeKeyAction::HardRefresh,
        Code::F5 if !ctrl && !shift => ChromeKeyAction::Refresh,
        Code::F5 if ctrl => ChromeKeyAction::HardRefresh,
        Code::KeyN if ctrl && !shift => ChromeKeyAction::NewWindow,
        Code::KeyD if ctrl && !shift => ChromeKeyAction::ToggleBookmark,
        Code::KeyO if ctrl && shift => ChromeKeyAction::OpenBookmarkManager,
        Code::KeyH if ctrl && !shift => ChromeKeyAction::OpenHistory,
        Code::KeyF if ctrl && !shift => ChromeKeyAction::FindInPage,
        Code::Tab if ctrl && !shift => ChromeKeyAction::NextTab,
        Code::Tab if ctrl && shift => ChromeKeyAction::PrevTab,
        Code::ArrowLeft if alt => ChromeKeyAction::GoBack,
        Code::ArrowRight if alt => ChromeKeyAction::GoForward,
        Code::Digit1 if ctrl => ChromeKeyAction::SelectTab(0),
        Code::Digit2 if ctrl => ChromeKeyAction::SelectTab(1),
        Code::Digit3 if ctrl => ChromeKeyAction::SelectTab(2),
        Code::Digit4 if ctrl => ChromeKeyAction::SelectTab(3),
        Code::Digit5 if ctrl => ChromeKeyAction::SelectTab(4),
        Code::Digit6 if ctrl => ChromeKeyAction::SelectTab(5),
        Code::Digit7 if ctrl => ChromeKeyAction::SelectTab(6),
        Code::Digit8 if ctrl => ChromeKeyAction::SelectTab(7),
        Code::Digit9 if ctrl => ChromeKeyAction::SelectLastTab,
        _ => ChromeKeyAction::None,
    }
}
