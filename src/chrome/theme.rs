//! Chrome Theme
//!
//! Defines the visual theme (colors, spacing, font sizes) for the native chrome.
//! Supports dark and light modes.

use webrender_api::ColorF;

/// Chrome visual theme
#[derive(Debug, Clone)]
pub struct ChromeTheme {
    // Tab bar
    pub tab_bar_bg: ColorF,
    pub active_tab_bg: ColorF,
    pub inactive_tab_bg: ColorF,
    pub active_tab_text: ColorF,
    pub inactive_tab_text: ColorF,
    pub tab_close_hover_bg: ColorF,
    pub new_tab_btn_bg: ColorF,
    pub new_tab_btn_hover_bg: ColorF,

    // Navbar
    pub navbar_bg: ColorF,
    pub nav_btn_bg: ColorF,
    pub nav_btn_hover_bg: ColorF,
    pub nav_btn_disabled: ColorF,
    pub nav_btn_icon: ColorF,

    // URL input
    pub url_input_bg: ColorF,
    pub url_input_border: ColorF,
    pub url_input_focused_border: ColorF,
    pub url_input_text: ColorF,
    pub url_input_placeholder: ColorF,

    // Bookmark bar
    pub bookmark_bar_bg: ColorF,
    pub bookmark_item_bg: ColorF,
    pub bookmark_item_hover_bg: ColorF,
    pub bookmark_item_text: ColorF,

    // Window controls (CSD)
    pub win_btn_hover_bg: ColorF,
    pub win_close_hover_bg: ColorF,
    pub win_btn_icon: ColorF,

    // Separator
    pub separator_color: ColorF,

    // Dimensions
    pub tab_bar_height: f32,
    pub navbar_height: f32,
    pub bookmark_bar_height: f32,
    pub panel_padding: f32,
    pub button_size: f32,
    pub icon_size: f32,
    pub tab_min_width: f32,
    pub tab_max_width: f32,
    pub border_radius: f32,
}

impl ChromeTheme {
    /// Total height of the chrome area
    pub fn total_chrome_height(&self, show_bookmark_bar: bool) -> f32 {
        let mut height = self.tab_bar_height + self.navbar_height + self.panel_padding;
        if show_bookmark_bar {
            height += self.bookmark_bar_height;
        }
        height
    }

    /// Dark theme (default)
    pub fn dark() -> Self {
        Self {
            // Tab bar
            tab_bar_bg: ColorF::new(0.11, 0.11, 0.12, 1.0),
            active_tab_bg: ColorF::new(0.18, 0.18, 0.20, 1.0),
            inactive_tab_bg: ColorF::new(0.11, 0.11, 0.12, 1.0),
            active_tab_text: ColorF::new(0.95, 0.95, 0.95, 1.0),
            inactive_tab_text: ColorF::new(0.55, 0.55, 0.60, 1.0),
            tab_close_hover_bg: ColorF::new(0.25, 0.25, 0.28, 1.0),
            new_tab_btn_bg: ColorF::new(0.15, 0.15, 0.17, 1.0),
            new_tab_btn_hover_bg: ColorF::new(0.22, 0.22, 0.25, 1.0),

            // Navbar
            navbar_bg: ColorF::new(0.15, 0.15, 0.17, 1.0),
            nav_btn_bg: ColorF::new(0.15, 0.15, 0.17, 0.0),
            nav_btn_hover_bg: ColorF::new(0.25, 0.25, 0.28, 1.0),
            nav_btn_disabled: ColorF::new(0.35, 0.35, 0.38, 1.0),
            nav_btn_icon: ColorF::new(0.85, 0.85, 0.88, 1.0),

            // URL input
            url_input_bg: ColorF::new(0.10, 0.10, 0.12, 1.0),
            url_input_border: ColorF::new(0.25, 0.25, 0.28, 1.0),
            url_input_focused_border: ColorF::new(0.91, 0.34, 0.16, 1.0), // Eesha orange
            url_input_text: ColorF::new(0.92, 0.92, 0.94, 1.0),
            url_input_placeholder: ColorF::new(0.40, 0.40, 0.45, 1.0),

            // Bookmark bar
            bookmark_bar_bg: ColorF::new(0.13, 0.13, 0.15, 1.0),
            bookmark_item_bg: ColorF::new(0.13, 0.13, 0.15, 0.0),
            bookmark_item_hover_bg: ColorF::new(0.20, 0.20, 0.23, 1.0),
            bookmark_item_text: ColorF::new(0.82, 0.82, 0.85, 1.0),

            // Window controls
            win_btn_hover_bg: ColorF::new(0.25, 0.25, 0.28, 1.0),
            win_close_hover_bg: ColorF::new(0.90, 0.20, 0.15, 1.0),
            win_btn_icon: ColorF::new(0.85, 0.85, 0.88, 1.0),

            // Separator
            separator_color: ColorF::new(0.22, 0.22, 0.25, 1.0),

            // Dimensions (in CSS pixels, will be scaled by device pixel ratio)
            tab_bar_height: 36.0,
            navbar_height: 42.0,
            bookmark_bar_height: 28.0,
            panel_padding: 2.0,
            button_size: 32.0,
            icon_size: 16.0,
            tab_min_width: 100.0,
            tab_max_width: 240.0,
            border_radius: 6.0,
        }
    }

    /// Light theme
    pub fn light() -> Self {
        let dark = Self::dark();
        Self {
            // Tab bar
            tab_bar_bg: ColorF::new(0.93, 0.93, 0.94, 1.0),
            active_tab_bg: ColorF::new(1.0, 1.0, 1.0, 1.0),
            inactive_tab_bg: ColorF::new(0.93, 0.93, 0.94, 1.0),
            active_tab_text: ColorF::new(0.12, 0.12, 0.14, 1.0),
            inactive_tab_text: ColorF::new(0.50, 0.50, 0.55, 1.0),
            tab_close_hover_bg: ColorF::new(0.85, 0.85, 0.87, 1.0),
            new_tab_btn_bg: ColorF::new(0.96, 0.96, 0.97, 1.0),
            new_tab_btn_hover_bg: ColorF::new(0.88, 0.88, 0.90, 1.0),

            // Navbar
            navbar_bg: ColorF::new(1.0, 1.0, 1.0, 1.0),
            nav_btn_bg: ColorF::new(1.0, 1.0, 1.0, 0.0),
            nav_btn_hover_bg: ColorF::new(0.90, 0.90, 0.92, 1.0),
            nav_btn_disabled: ColorF::new(0.70, 0.70, 0.73, 1.0),
            nav_btn_icon: ColorF::new(0.25, 0.25, 0.28, 1.0),

            // URL input
            url_input_bg: ColorF::new(0.96, 0.96, 0.97, 1.0),
            url_input_border: ColorF::new(0.82, 0.82, 0.85, 1.0),
            url_input_focused_border: ColorF::new(0.91, 0.34, 0.16, 1.0),
            url_input_text: ColorF::new(0.12, 0.12, 0.14, 1.0),
            url_input_placeholder: ColorF::new(0.60, 0.60, 0.63, 1.0),

            // Bookmark bar
            bookmark_bar_bg: ColorF::new(0.96, 0.96, 0.97, 1.0),
            bookmark_item_bg: ColorF::new(0.96, 0.96, 0.97, 0.0),
            bookmark_item_hover_bg: ColorF::new(0.88, 0.88, 0.90, 1.0),
            bookmark_item_text: ColorF::new(0.20, 0.20, 0.23, 1.0),

            // Window controls
            win_btn_hover_bg: ColorF::new(0.90, 0.90, 0.92, 1.0),
            win_close_hover_bg: ColorF::new(0.90, 0.20, 0.15, 1.0),
            win_btn_icon: ColorF::new(0.25, 0.25, 0.28, 1.0),

            // Separator
            separator_color: ColorF::new(0.82, 0.82, 0.85, 1.0),

            // Keep same dimensions
            ..dark
        }
    }
}
