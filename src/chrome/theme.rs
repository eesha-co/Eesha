//! Chrome Theme
//!
//! Defines the visual theme for the native browser chrome,
//! including colors, dimensions, and font settings.

use webrender_api::ColorF;

/// Visual theme for the browser chrome
#[derive(Clone, Debug)]
pub struct ChromeTheme {
    /// Background color of the tab bar
    pub tab_bar_bg: ColorF,
    /// Background color of the active tab
    pub tab_active_bg: ColorF,
    /// Background color of inactive tabs
    pub tab_inactive_bg: ColorF,
    /// Text color for active tab
    pub tab_active_text: ColorF,
    /// Text color for inactive tabs
    pub tab_inactive_text: ColorF,
    /// Tab close button color
    pub tab_close_color: ColorF,
    /// Tab close button hover color
    pub tab_close_hover: ColorF,
    /// Background color of the navigation bar
    pub nav_bar_bg: ColorF,
    /// Button background color
    pub button_bg: ColorF,
    /// Button background color on hover
    pub button_hover_bg: ColorF,
    /// Button background color when pressed
    pub button_pressed_bg: ColorF,
    /// Button icon/text color
    pub button_color: ColorF,
    /// Button icon/text color when disabled
    pub button_disabled_color: ColorF,
    /// URL bar background color
    pub url_bar_bg: ColorF,
    /// URL bar border color
    pub url_bar_border: ColorF,
    /// URL bar border color when focused
    pub url_bar_focus_border: ColorF,
    /// URL bar text color
    pub url_bar_text: ColorF,
    /// URL bar placeholder text color
    pub url_bar_placeholder: ColorF,
    /// Bookmark bar background color
    pub bookmark_bar_bg: ColorF,
    /// Bookmark text color
    pub bookmark_text: ColorF,
    /// Separator/divider color
    pub separator_color: ColorF,
    /// Loading progress bar color
    pub loading_bar_color: ColorF,
    /// Chrome brand color (Eesha orange)
    pub brand_color: ColorF,
    /// Shadow color for chrome borders
    pub shadow_color: ColorF,
    /// New tab button color
    pub new_tab_button_color: ColorF,
}

impl Default for ChromeTheme {
    fn default() -> Self {
        Self {
            tab_bar_bg: ColorF::new(0.12, 0.12, 0.14, 1.0),       // Dark (#1F1F24)
            tab_active_bg: ColorF::new(0.16, 0.16, 0.19, 1.0),     // Slightly lighter (#292930)
            tab_inactive_bg: ColorF::new(0.10, 0.10, 0.12, 1.0),   // Darker (#1A1A1F)
            tab_active_text: ColorF::new(0.93, 0.93, 0.95, 1.0),   // White-ish (#EDEDF2)
            tab_inactive_text: ColorF::new(0.55, 0.55, 0.60, 1.0), // Gray (#8C8C9A)
            tab_close_color: ColorF::new(0.50, 0.50, 0.55, 1.0),
            tab_close_hover: ColorF::new(0.90, 0.34, 0.16, 1.0),   // Brand orange
            nav_bar_bg: ColorF::new(0.16, 0.16, 0.19, 1.0),        // (#292930)
            button_bg: ColorF::new(0.0, 0.0, 0.0, 0.0),            // Transparent
            button_hover_bg: ColorF::new(0.25, 0.25, 0.28, 1.0),   // Light gray
            button_pressed_bg: ColorF::new(0.18, 0.18, 0.21, 1.0),
            button_color: ColorF::new(0.80, 0.80, 0.83, 1.0),      // Light gray
            button_disabled_color: ColorF::new(0.35, 0.35, 0.38, 1.0),
            url_bar_bg: ColorF::new(0.08, 0.08, 0.10, 1.0),        // Very dark (#141418)
            url_bar_border: ColorF::new(0.25, 0.25, 0.28, 1.0),    // Subtle border
            url_bar_focus_border: ColorF::new(0.91, 0.34, 0.16, 1.0), // Brand orange (#E8572A)
            url_bar_text: ColorF::new(0.90, 0.90, 0.92, 1.0),
            url_bar_placeholder: ColorF::new(0.45, 0.45, 0.48, 1.0),
            bookmark_bar_bg: ColorF::new(0.14, 0.14, 0.16, 1.0),
            bookmark_text: ColorF::new(0.70, 0.70, 0.73, 1.0),
            separator_color: ColorF::new(0.20, 0.20, 0.23, 1.0),
            loading_bar_color: ColorF::new(0.91, 0.34, 0.16, 1.0), // Brand orange
            brand_color: ColorF::new(0.91, 0.34, 0.16, 1.0),       // #E8572A
            shadow_color: ColorF::new(0.0, 0.0, 0.0, 0.3),
            new_tab_button_color: ColorF::new(0.55, 0.55, 0.60, 1.0),
        }
    }
}

impl ChromeTheme {
    /// Light theme variant
    pub fn light() -> Self {
        Self {
            tab_bar_bg: ColorF::new(0.95, 0.95, 0.96, 1.0),
            tab_active_bg: ColorF::new(1.0, 1.0, 1.0, 1.0),
            tab_inactive_bg: ColorF::new(0.90, 0.90, 0.92, 1.0),
            tab_active_text: ColorF::new(0.13, 0.13, 0.14, 1.0),
            tab_inactive_text: ColorF::new(0.45, 0.45, 0.48, 1.0),
            tab_close_color: ColorF::new(0.50, 0.50, 0.53, 1.0),
            tab_close_hover: ColorF::new(0.91, 0.34, 0.16, 1.0),
            nav_bar_bg: ColorF::new(1.0, 1.0, 1.0, 1.0),
            button_bg: ColorF::new(0.0, 0.0, 0.0, 0.0),
            button_hover_bg: ColorF::new(0.90, 0.90, 0.92, 1.0),
            button_pressed_bg: ColorF::new(0.85, 0.85, 0.87, 1.0),
            button_color: ColorF::new(0.25, 0.25, 0.28, 1.0),
            button_disabled_color: ColorF::new(0.65, 0.65, 0.68, 1.0),
            url_bar_bg: ColorF::new(0.95, 0.95, 0.96, 1.0),
            url_bar_border: ColorF::new(0.80, 0.80, 0.82, 1.0),
            url_bar_focus_border: ColorF::new(0.91, 0.34, 0.16, 1.0),
            url_bar_text: ColorF::new(0.13, 0.13, 0.14, 1.0),
            url_bar_placeholder: ColorF::new(0.55, 0.55, 0.58, 1.0),
            bookmark_bar_bg: ColorF::new(0.96, 0.96, 0.97, 1.0),
            bookmark_text: ColorF::new(0.30, 0.30, 0.33, 1.0),
            separator_color: ColorF::new(0.85, 0.85, 0.87, 1.0),
            loading_bar_color: ColorF::new(0.91, 0.34, 0.16, 1.0),
            brand_color: ColorF::new(0.91, 0.34, 0.16, 1.0),
            shadow_color: ColorF::new(0.0, 0.0, 0.0, 0.08),
            new_tab_button_color: ColorF::new(0.45, 0.45, 0.48, 1.0),
        }
    }
}
