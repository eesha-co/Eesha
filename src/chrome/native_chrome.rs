//! Native Chrome
//!
//! The main native chrome implementation that replaces the HTML-based panel.
//! Renders the browser toolbar, tab bar, address bar, and bookmark bar
//! directly using WebRender display list primitives.

use base::id::WebViewId;
use webrender_api::{
    ColorF, CommonItemProperties, DisplayListBuilder, FontInstanceKey, ImageKey, ImageRendering,
    AlphaType, LayoutPoint, LayoutRect, LayoutSize, SpaceAndClipInfo, SpatialId,
    PipelineId as WebRenderPipelineId,
};
use winit::dpi::PhysicalPosition;

use crate::bookmark::BookmarkId;
use super::theme::ChromeTheme;
use super::url_input::{UrlInputAction, UrlInputState};
use super::widgets;

/// Actions that can be triggered by interacting with the chrome
#[derive(Debug, Clone)]
pub enum ChromeAction {
    /// Navigate to URL
    Navigate(String),
    /// Go back in history
    GoBack,
    /// Go forward in history
    GoForward,
    /// Refresh current page
    Refresh,
    /// Create a new tab
    NewTab,
    /// Create a new window
    NewWindow,
    /// Close a specific tab
    CloseTab(WebViewId),
    /// Activate (switch to) a specific tab
    ActivateTab(WebViewId),
    /// Minimize the window
    Minimize,
    /// Toggle maximize/restore
    Maximize,
    /// Close the window
    CloseWindow,
    /// Start window drag
    DragWindow,
    /// Toggle bookmark on current page
    ToggleBookmark,
    /// Open bookmark manager
    OpenBookmarkManager,
    /// Open history menu
    OpenHistoryMenu,
    /// Open downloads
    OpenDownloads,
    /// Focus the URL bar
    FocusUrlBar,
}

/// Identifies a specific element in the chrome for hit testing
#[derive(Debug, Clone, PartialEq)]
pub enum ChromeElementId {
    BackButton,
    ForwardButton,
    RefreshButton,
    HomeButton,
    UrlInput,
    BookmarkStar,
    DownloadButton,
    MenuButton,
    Tab(WebViewId),
    TabClose(WebViewId),
    NewTabButton,
    BookmarkItem(BookmarkId),
    WindowMinimize,
    WindowMaximize,
    WindowClose,
    DragRegion,
}

/// Current focus target in the chrome
#[derive(Debug, Clone, PartialEq)]
pub enum ChromeFocusTarget {
    None,
    UrlInput,
}

/// Tab state for rendering
#[derive(Debug, Clone)]
pub struct TabState {
    pub id: WebViewId,
    pub title: String,
    pub url: String,
    pub is_active: bool,
    pub is_loading: bool,
}

/// The native chrome - replaces the HTML Panel WebView
pub struct NativeChrome {
    /// Visual theme
    pub theme: ChromeTheme,

    /// URL input state
    pub url_input: UrlInputState,

    /// Tab states
    pub tabs: Vec<TabState>,

    /// Currently active tab ID
    pub active_tab_id: Option<WebViewId>,

    /// Navigation state
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub is_loading: bool,

    /// Bookmark state
    pub show_bookmark_bar: bool,
    pub current_page_bookmarked: bool,

    /// Download state
    pub download_active: bool,

    /// Interaction state
    pub focused_element: ChromeFocusTarget,
    pub hover_element: Option<ChromeElementId>,

    /// Font keys (registered with WebRender)
    pub font_small: FontInstanceKey,
    pub font_normal: FontInstanceKey,
    pub font_url: FontInstanceKey,

    /// Icon keys
    pub icons: ChromeIconKeys,

    /// Whether we're on a platform with CSD (Client-Side Decorations)
    pub has_csd: bool,
}

/// Simplified icon keys (until full icon system is in place)
#[derive(Debug, Clone)]
pub struct ChromeIconKeys {
    pub back: Option<ImageKey>,
    pub forward: Option<ImageKey>,
    pub refresh: Option<ImageKey>,
    pub close: Option<ImageKey>,
    pub new_tab: Option<ImageKey>,
    pub bookmark: Option<ImageKey>,
    pub logo: Option<ImageKey>,
}

impl NativeChrome {
    /// Create a new native chrome instance
    pub fn new(
        font_small: FontInstanceKey,
        font_normal: FontInstanceKey,
        font_url: FontInstanceKey,
        has_csd: bool,
    ) -> Self {
        Self {
            theme: ChromeTheme::dark(),
            url_input: UrlInputState::new(),
            tabs: Vec::new(),
            active_tab_id: None,
            can_go_back: false,
            can_go_forward: false,
            is_loading: false,
            show_bookmark_bar: false,
            current_page_bookmarked: false,
            download_active: false,
            focused_element: ChromeFocusTarget::None,
            hover_element: None,
            font_small,
            font_normal,
            font_url,
            icons: ChromeIconKeys {
                back: None,
                forward: None,
                refresh: None,
                close: None,
                new_tab: None,
                bookmark: None,
                logo: None,
            },
            has_csd,
        }
    }

    /// Total height of the chrome area in CSS pixels
    pub fn total_height(&self) -> f32 {
        self.theme.total_chrome_height(self.show_bookmark_bar)
    }

    /// Add a tab to the chrome
    pub fn add_tab(&mut self, id: WebViewId, active: bool) {
        if active {
            // Deactivate all other tabs
            for tab in &mut self.tabs {
                tab.is_active = false;
            }
            self.active_tab_id = Some(id);
        }
        self.tabs.push(TabState {
            id,
            title: "New Tab".to_string(),
            url: "eesha://newtab".to_string(),
            is_active: active,
            is_loading: false,
        });
    }

    /// Remove a tab from the chrome
    pub fn close_tab(&mut self, id: WebViewId) -> Option<WebViewId> {
        let idx = self.tabs.iter().position(|t| t.id == id)?;
        self.tabs.remove(idx);

        // If we closed the active tab, activate another
        if self.active_tab_id == Some(id) {
            if self.tabs.is_empty() {
                self.active_tab_id = None;
            } else {
                let new_idx = idx.min(self.tabs.len() - 1);
                self.tabs[new_idx].is_active = true;
                self.active_tab_id = Some(self.tabs[new_idx].id);
            }
        }

        self.active_tab_id
    }

    /// Activate a specific tab
    pub fn activate_tab(&mut self, id: WebViewId) {
        for tab in &mut self.tabs {
            tab.is_active = tab.id == id;
        }
        self.active_tab_id = Some(id);
    }

    /// Set the URL displayed in the address bar
    pub fn set_url(&mut self, url: &str) {
        if self.focused_element != ChromeFocusTarget::UrlInput {
            self.url_input.set_url(url);
        }
    }

    /// Set tab title
    pub fn set_tab_title(&mut self, id: WebViewId, title: String) {
        if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == id) {
            tab.title = title;
        }
    }

    /// Set navigation button states
    pub fn set_nav_buttons(&mut self, back: bool, forward: bool) {
        self.can_go_back = back;
        self.can_go_forward = forward;
    }

    /// Focus the URL bar
    pub fn focus_url_bar(&mut self) {
        self.focused_element = ChromeFocusTarget::UrlInput;
        self.url_input.focus();
    }

    /// Build the WebRender display list for the chrome
    pub fn build_display_list(
        &self,
        builder: &mut DisplayListBuilder,
        pipeline: WebRenderPipelineId,
        viewport_width: f32,
        zoom_factor: f32,
        spatial_id: SpatialId,
    ) {
        let theme = &self.theme;
        let w = viewport_width / zoom_factor;

        let root_space = SpaceAndClipInfo {
            spatial_id,
            clip_chain_id: builder.define_clip_chain(
                None,
                [builder.define_clip_rect(
                    spatial_id,
                    LayoutRect::from_origin_and_size(
                        LayoutPoint::zero(),
                        LayoutSize::new(w, self.total_height()),
                    ),
                )],
            ),
        };

        // === Tab Bar ===
        self.build_tab_bar(builder, &root_space, w);

        // === Navbar ===
        let nav_y = theme.tab_bar_height;
        self.build_navbar(builder, &root_space, w, nav_y);

        // === Bookmark Bar ===
        if self.show_bookmark_bar {
            let bm_y = theme.tab_bar_height + theme.navbar_height;
            self.build_bookmark_bar(builder, &root_space, w, bm_y);
        }

        // === Bottom separator ===
        let sep_y = self.total_height() - 1.0;
        widgets::push_separator(builder, &root_space, sep_y, w, theme.separator_color);
    }

    /// Build the tab bar display list
    fn build_tab_bar(
        &self,
        builder: &mut DisplayListBuilder,
        space: &SpaceAndClipInfo,
        width: f32,
    ) {
        let theme = &self.theme;

        // Tab bar background
        let tab_bar_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::zero(),
            LayoutSize::new(width, theme.tab_bar_height),
        );
        widgets::push_rect(builder, space, tab_bar_rect, theme.tab_bar_bg);

        // Calculate tab width
        let csd_width = if self.has_csd { 138.0 } else { 0.0 };
        let available_width = width - 40.0 - csd_width; // new tab btn + window controls
        let tab_width = widgets::calculate_tab_width(
            available_width,
            self.tabs.len(),
            theme.tab_min_width,
            theme.tab_max_width,
        );

        // Render tabs
        for (i, tab) in self.tabs.iter().enumerate() {
            let tab_x = i as f32 * tab_width;
            let tab_rect = LayoutRect::from_origin_and_size(
                LayoutPoint::new(tab_x, 0.0),
                LayoutSize::new(tab_width, theme.tab_bar_height),
            );

            let bg_color = if tab.is_active {
                theme.active_tab_bg
            } else {
                theme.inactive_tab_bg
            };
            widgets::push_rect(builder, space, tab_rect, bg_color);

            // Tab title text
            let title_x = tab_x + 12.0;
            let title_width = tab_width - 36.0; // space for close btn
            let title_rect = LayoutRect::from_origin_and_size(
                LayoutPoint::new(title_x, 0.0),
                LayoutSize::new(title_width.max(0.0), theme.tab_bar_height),
            );
            let text_color = if tab.is_active {
                theme.active_tab_text
            } else {
                theme.inactive_tab_text
            };

            // Truncate title to fit
            let max_chars = (title_width / 8.0) as usize;
            let display_title = if tab.title.len() > max_chars && max_chars > 3 {
                format!("{}...", &tab.title[..max_chars - 3])
            } else {
                tab.title.clone()
            };

            builder.push_text(
                &CommonItemProperties::new(title_rect, space.clone()),
                title_rect,
                &display_title,
                self.font_small,
                text_color,
            );

            // Tab close button (×)
            let close_x = tab_x + tab_width - 24.0;
            let close_rect = LayoutRect::from_origin_and_size(
                LayoutPoint::new(close_x, (theme.tab_bar_height - 20.0) / 2.0),
                LayoutSize::new(20.0, 20.0),
            );
            let is_hover = self.hover_element.as_ref() == Some(&ChromeElementId::TabClose(tab.id));
            let close_bg = if is_hover {
                theme.tab_close_hover_bg
            } else {
                ColorF::TRANSPARENT
            };
            widgets::push_rounded_rect(builder, space, close_rect, close_bg, 3.0);

            builder.push_text(
                &CommonItemProperties::new(close_rect, space.clone()),
                close_rect,
                "×",
                self.font_small,
                text_color,
            );

            // Separator between tabs
            if i < self.tabs.len() - 1 {
                let sep_rect = LayoutRect::from_origin_and_size(
                    LayoutPoint::new(tab_x + tab_width - 0.5, 8.0),
                    LayoutSize::new(1.0, theme.tab_bar_height - 16.0),
                );
                widgets::push_rect(builder, space, sep_rect, theme.separator_color);
            }
        }

        // New tab button (+)
        let new_tab_x = self.tabs.len() as f32 * tab_width + 4.0;
        let new_tab_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(new_tab_x, 4.0),
            LayoutSize::new(28.0, theme.tab_bar_height - 8.0),
        );
        let is_hover = self.hover_element.as_ref() == Some(&ChromeElementId::NewTabButton);
        let btn_bg = if is_hover {
            theme.new_tab_btn_hover_bg
        } else {
            theme.new_tab_btn_bg
        };
        widgets::push_rounded_rect(builder, space, new_tab_rect, btn_bg, theme.border_radius);
        builder.push_text(
            &CommonItemProperties::new(new_tab_rect, space.clone()),
            new_tab_rect,
            "+",
            self.font_normal,
            theme.active_tab_text,
        );

        // Window controls (CSD only)
        if self.has_csd {
            self.build_window_controls(builder, space, width);
        }
    }

    /// Build window control buttons (minimize, maximize, close) for CSD
    fn build_window_controls(
        &self,
        builder: &mut DisplayListBuilder,
        space: &SpaceAndClipInfo,
        width: f32,
    ) {
        let theme = &self.theme;
        let btn_w = 46.0;
        let btn_h = theme.tab_bar_height;

        // Close button
        let close_x = width - btn_w;
        let close_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(close_x, 0.0),
            LayoutSize::new(btn_w, btn_h),
        );
        let is_hover = self.hover_element.as_ref() == Some(&ChromeElementId::WindowClose);
        let bg = if is_hover {
            theme.win_close_hover_bg
        } else {
            ColorF::TRANSPARENT
        };
        widgets::push_rect(builder, space, close_rect, bg);
        builder.push_text(
            &CommonItemProperties::new(close_rect, space.clone()),
            close_rect,
            "×",
            self.font_normal,
            theme.win_btn_icon,
        );

        // Maximize button
        let max_x = width - btn_w * 2.0;
        let max_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(max_x, 0.0),
            LayoutSize::new(btn_w, btn_h),
        );
        let is_hover = self.hover_element.as_ref() == Some(&ChromeElementId::WindowMaximize);
        let bg = if is_hover {
            theme.win_btn_hover_bg
        } else {
            ColorF::TRANSPARENT
        };
        widgets::push_rect(builder, space, max_rect, bg);
        builder.push_text(
            &CommonItemProperties::new(max_rect, space.clone()),
            max_rect,
            "□",
            self.font_normal,
            theme.win_btn_icon,
        );

        // Minimize button
        let min_x = width - btn_w * 3.0;
        let min_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(min_x, 0.0),
            LayoutSize::new(btn_w, btn_h),
        );
        let is_hover = self.hover_element.as_ref() == Some(&ChromeElementId::WindowMinimize);
        let bg = if is_hover {
            theme.win_btn_hover_bg
        } else {
            ColorF::TRANSPARENT
        };
        widgets::push_rect(builder, space, min_rect, bg);
        builder.push_text(
            &CommonItemProperties::new(min_rect, space.clone()),
            min_rect,
            "─",
            self.font_normal,
            theme.win_btn_icon,
        );
    }

    /// Build the navbar (address bar row)
    fn build_navbar(
        &self,
        builder: &mut DisplayListBuilder,
        space: &SpaceAndClipInfo,
        width: f32,
        y_offset: f32,
    ) {
        let theme = &self.theme;

        // Navbar background
        let nav_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(0.0, y_offset),
            LayoutSize::new(width, theme.navbar_height),
        );
        widgets::push_rect(builder, space, nav_rect, theme.navbar_bg);

        let btn_y = y_offset + (theme.navbar_height - theme.button_size) / 2.0;
        let mut x = 8.0;

        // Back button
        let back_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(x, btn_y),
            LayoutSize::new(theme.button_size, theme.button_size),
        );
        let is_hover = self.hover_element.as_ref() == Some(&ChromeElementId::BackButton);
        let icon_color = if self.can_go_back {
            if is_hover { theme.nav_btn_hover_bg } else { ColorF::TRANSPARENT }
        } else {
            ColorF::TRANSPARENT
        };
        widgets::push_rounded_rect(builder, space, back_rect, icon_color, theme.border_radius);
        let text_color = if self.can_go_back { theme.nav_btn_icon } else { theme.nav_btn_disabled };
        builder.push_text(
            &CommonItemProperties::new(back_rect, space.clone()),
            back_rect,
            "◀",
            self.font_normal,
            text_color,
        );
        x += theme.button_size + 2.0;

        // Forward button
        let fwd_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(x, btn_y),
            LayoutSize::new(theme.button_size, theme.button_size),
        );
        let is_hover = self.hover_element.as_ref() == Some(&ChromeElementId::ForwardButton);
        let icon_color = if self.can_go_forward {
            if is_hover { theme.nav_btn_hover_bg } else { ColorF::TRANSPARENT }
        } else {
            ColorF::TRANSPARENT
        };
        widgets::push_rounded_rect(builder, space, fwd_rect, icon_color, theme.border_radius);
        let text_color = if self.can_go_forward { theme.nav_btn_icon } else { theme.nav_btn_disabled };
        builder.push_text(
            &CommonItemProperties::new(fwd_rect, space.clone()),
            fwd_rect,
            "▶",
            self.font_normal,
            text_color,
        );
        x += theme.button_size + 2.0;

        // Refresh/Stop button
        let refresh_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(x, btn_y),
            LayoutSize::new(theme.button_size, theme.button_size),
        );
        let is_hover = self.hover_element.as_ref() == Some(&ChromeElementId::RefreshButton);
        let icon_color = if is_hover { theme.nav_btn_hover_bg } else { ColorF::TRANSPARENT };
        widgets::push_rounded_rect(builder, space, refresh_rect, icon_color, theme.border_radius);
        let refresh_text = if self.is_loading { "✕" } else { "⟳" };
        builder.push_text(
            &CommonItemProperties::new(refresh_rect, space.clone()),
            refresh_rect,
            refresh_text,
            self.font_normal,
            theme.nav_btn_icon,
        );
        x += theme.button_size + 4.0;

        // URL input field
        let url_end_x = if self.has_csd { width - 150.0 } else { width - 50.0 };
        let url_width = url_end_x - x;
        let url_input_height = 30.0;
        let url_y = y_offset + (theme.navbar_height - url_input_height) / 2.0;
        let url_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(x, url_y),
            LayoutSize::new(url_width, url_input_height),
        );

        let is_focused = self.focused_element == ChromeFocusTarget::UrlInput;
        let border_color = if is_focused {
            theme.url_input_focused_border
        } else {
            theme.url_input_border
        };
        widgets::push_bordered_rounded_rect(
            builder,
            space,
            url_rect,
            theme.url_input_bg,
            border_color,
            if is_focused { 2.0 } else { 1.0 },
            theme.border_radius,
        );

        // URL text or placeholder
        let text_padding = 10.0;
        let text_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(x + text_padding, url_y),
            LayoutSize::new(url_width - text_padding * 2.0, url_input_height),
        );

        let display_text = self.url_input.display_text();
        if display_text.is_empty() && !is_focused {
            builder.push_text(
                &CommonItemProperties::new(text_rect, space.clone()),
                text_rect,
                &self.url_input.placeholder,
                self.font_url,
                theme.url_input_placeholder,
            );
        } else {
            // Truncate URL to fit
            let max_chars = ((url_width - text_padding * 2.0) / 7.0) as usize;
            let display_url = if display_text.len() > max_chars && max_chars > 3 {
                format!("...{}", &display_text[display_text.len() - max_chars + 3..])
            } else {
                display_text
            };

            builder.push_text(
                &CommonItemProperties::new(text_rect, space.clone()),
                text_rect,
                &display_url,
                self.font_url,
                theme.url_input_text,
            );
        }

        // Right side buttons
        let mut right_x = if self.has_csd { width - 150.0 } else { width - 50.0 };

        // Bookmark star
        let bm_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(right_x, btn_y),
            LayoutSize::new(theme.button_size, theme.button_size),
        );
        let is_hover = self.hover_element.as_ref() == Some(&ChromeElementId::BookmarkStar);
        let bg = if is_hover { theme.nav_btn_hover_bg } else { ColorF::TRANSPARENT };
        widgets::push_rounded_rect(builder, space, bm_rect, bg, theme.border_radius);
        let star = if self.current_page_bookmarked { "★" } else { "☆" };
        builder.push_text(
            &CommonItemProperties::new(bm_rect, space.clone()),
            bm_rect,
            star,
            self.font_normal,
            theme.nav_btn_icon,
        );
        right_x += theme.button_size + 2.0;

        // Menu button (⋮)
        if !self.has_csd {
            let menu_rect = LayoutRect::from_origin_and_size(
                LayoutPoint::new(right_x, btn_y),
                LayoutSize::new(theme.button_size, theme.button_size),
            );
            let is_hover = self.hover_element.as_ref() == Some(&ChromeElementId::MenuButton);
            let bg = if is_hover { theme.nav_btn_hover_bg } else { ColorF::TRANSPARENT };
            widgets::push_rounded_rect(builder, space, menu_rect, bg, theme.border_radius);
            builder.push_text(
                &CommonItemProperties::new(menu_rect, space.clone()),
                menu_rect,
                "⋮",
                self.font_normal,
                theme.nav_btn_icon,
            );
        }
    }

    /// Build the bookmark bar
    fn build_bookmark_bar(
        &self,
        builder: &mut DisplayListBuilder,
        space: &SpaceAndClipInfo,
        width: f32,
        y_offset: f32,
    ) {
        let theme = &self.theme;

        // Background
        let bm_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(0.0, y_offset),
            LayoutSize::new(width, theme.bookmark_bar_height),
        );
        widgets::push_rect(builder, space, bm_rect, theme.bookmark_bar_bg);

        // Separator at top
        widgets::push_separator(builder, space, y_offset, width, theme.separator_color);

        // Note: Individual bookmark items will be rendered when
        // bookmark data is connected. For now, show a placeholder.
        let placeholder_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(10.0, y_offset),
            LayoutSize::new(width - 20.0, theme.bookmark_bar_height),
        );
        builder.push_text(
            &CommonItemProperties::new(placeholder_rect, space.clone()),
            placeholder_rect,
            "Bookmarks bar",
            self.font_small,
            theme.inactive_tab_text,
        );
    }

    /// Hit test: determine which chrome element is at the given point
    pub fn hit_test(&self, point: PhysicalPosition<f64>, viewport_width: f32, scale_factor: f64) -> Option<ChromeElementId> {
        let x = point.x as f32 / scale_factor as f32;
        let y = point.y as f32 / scale_factor as f32;
        let theme = &self.theme;

        // Tab bar
        if y < theme.tab_bar_height {
            return self.hit_test_tab_bar(x, y, viewport_width / scale_factor as f32);
        }

        // Navbar
        if y < theme.tab_bar_height + theme.navbar_height {
            return self.hit_test_navbar(x, y - theme.tab_bar_height, viewport_width / scale_factor as f32);
        }

        // Bookmark bar
        if self.show_bookmark_bar && y < self.total_height() {
            return self.hit_test_bookmark_bar(x, y - theme.tab_bar_height - theme.navbar_height);
        }

        None
    }

    fn hit_test_tab_bar(&self, x: f32, y: f32, width: f32) -> Option<ChromeElementId> {
        let theme = &self.theme;
        let csd_width = if self.has_csd { 138.0 } else { 0.0 };
        let available_width = width - 40.0 - csd_width;
        let tab_width = widgets::calculate_tab_width(
            available_width,
            self.tabs.len(),
            theme.tab_min_width,
            theme.tab_max_width,
        );

        for (i, tab) in self.tabs.iter().enumerate() {
            let tab_x = i as f32 * tab_width;
            if x >= tab_x && x < tab_x + tab_width {
                // Check close button
                let close_x = tab_x + tab_width - 24.0;
                if x >= close_x && x < close_x + 20.0 {
                    return Some(ChromeElementId::TabClose(tab.id));
                }
                return Some(ChromeElementId::Tab(tab.id));
            }
        }

        // New tab button
        let new_tab_x = self.tabs.len() as f32 * tab_width + 4.0;
        if x >= new_tab_x && x < new_tab_x + 28.0 {
            return Some(ChromeElementId::NewTabButton);
        }

        // Window controls (CSD)
        if self.has_csd {
            let btn_w = 46.0;
            if x >= width - btn_w {
                return Some(ChromeElementId::WindowClose);
            }
            if x >= width - btn_w * 2.0 {
                return Some(ChromeElementId::WindowMaximize);
            }
            if x >= width - btn_w * 3.0 {
                return Some(ChromeElementId::WindowMinimize);
            }
        }

        Some(ChromeElementId::DragRegion)
    }

    fn hit_test_navbar(&self, x: f32, y: f32, width: f32) -> Option<ChromeElementId> {
        let theme = &self.theme;
        let btn_y = (theme.navbar_height - theme.button_size) / 2.0;

        if y < btn_y || y > btn_y + theme.button_size {
            return None;
        }

        let mut pos = 8.0;

        // Back button
        if x >= pos && x < pos + theme.button_size {
            return Some(ChromeElementId::BackButton);
        }
        pos += theme.button_size + 2.0;

        // Forward button
        if x >= pos && x < pos + theme.button_size {
            return Some(ChromeElementId::ForwardButton);
        }
        pos += theme.button_size + 2.0;

        // Refresh button
        if x >= pos && x < pos + theme.button_size {
            return Some(ChromeElementId::RefreshButton);
        }
        pos += theme.button_size + 4.0;

        // URL input
        let url_end_x = if self.has_csd { width - 150.0 } else { width - 50.0 };
        if x >= pos && x < url_end_x {
            return Some(ChromeElementId::UrlInput);
        }

        // Right side buttons
        let mut right_x = url_end_x;

        // Bookmark star
        if x >= right_x && x < right_x + theme.button_size {
            return Some(ChromeElementId::BookmarkStar);
        }
        right_x += theme.button_size + 2.0;

        // Menu button
        if !self.has_csd && x >= right_x && x < right_x + theme.button_size {
            return Some(ChromeElementId::MenuButton);
        }

        None
    }

    fn hit_test_bookmark_bar(&self, _x: f32, _y: f32) -> Option<ChromeElementId> {
        // TODO: Implement bookmark bar hit testing when bookmark data is connected
        None
    }

    /// Handle a click on a chrome element
    pub fn handle_click(&mut self, element: &ChromeElementId) -> Option<ChromeAction> {
        match element {
            ChromeElementId::BackButton if self.can_go_back => Some(ChromeAction::GoBack),
            ChromeElementId::ForwardButton if self.can_go_forward => Some(ChromeAction::GoForward),
            ChromeElementId::RefreshButton => Some(ChromeAction::Refresh),
            ChromeElementId::UrlInput => {
                self.focused_element = ChromeFocusTarget::UrlInput;
                self.url_input.focus();
                None // URL input focus doesn't trigger a navigation action
            }
            ChromeElementId::NewTabButton => Some(ChromeAction::NewTab),
            ChromeElementId::Tab(id) => Some(ChromeAction::ActivateTab(*id)),
            ChromeElementId::TabClose(id) => Some(ChromeAction::CloseTab(*id)),
            ChromeElementId::BookmarkStar => Some(ChromeAction::ToggleBookmark),
            ChromeElementId::WindowMinimize => Some(ChromeAction::Minimize),
            ChromeElementId::WindowMaximize => Some(ChromeAction::Maximize),
            ChromeElementId::WindowClose => Some(ChromeAction::CloseWindow),
            ChromeElementId::DragRegion => Some(ChromeAction::DragWindow),
            ChromeElementId::DownloadButton => Some(ChromeAction::OpenDownloads),
            ChromeElementId::MenuButton => None, // Menu handled separately
            _ => None,
        }
    }

    /// Handle mouse move for hover effects
    pub fn handle_mouse_move(&mut self, element: Option<ChromeElementId>) {
        self.hover_element = element;
    }

    /// Handle a click that was in the chrome area (not content)
    pub fn handle_chrome_click_outside(&mut self) {
        if self.focused_element == ChromeFocusTarget::UrlInput {
            self.url_input.blur();
            self.focused_element = ChromeFocusTarget::None;
        }
    }

    /// Check if a point is within the chrome area
    pub fn is_in_chrome_area(&self, y: f32, scale_factor: f64) -> bool {
        y < (self.total_height() * scale_factor as f32) as f64
    }
}
