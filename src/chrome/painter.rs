//! Chrome Painter
//!
//! Builds WebRender display lists for the browser chrome.
//! This is the core rendering module that draws the navigation bar,
//! tab bar, URL bar, and other chrome elements using WebRender primitives.

use euclid::{Rect, Point2D, Size2D, vec2};
use webrender_api::{
    BorderRadius, ColorF, CommonItemProperties, ComplexClipRegion,
    ClipMode, SpaceAndClipInfo, SpatialId,
    units::{DeviceRect, LayoutRect, LayoutSize, LayoutPoint},
};

use super::state::ChromeState;
use super::widget::{WidgetId, WidgetKind, WidgetRect, ChromeUnit};
use super::theme::ChromeTheme;
use super::{TAB_BAR_HEIGHT, NAV_BAR_HEIGHT, BOOKMARK_BAR_HEIGHT, CHROME_PADDING, WIDGET_SPACING};

/// Paints the native browser chrome using WebRender display list primitives
pub struct ChromePainter;

impl ChromePainter {
    /// Paint the entire chrome UI and return the list of widgets for hit testing
    pub fn paint(
        state: &ChromeState,
        builder: &mut webrender::api::DisplayListBuilder,
        spatial_id: SpatialId,
        clip_chain_id: webrender_api::ClipChainId,
        viewport_size: LayoutSize,
        scale_factor: f32,
    ) -> Vec<WidgetRect> {
        let mut widgets = Vec::new();
        let theme = &state.theme;

        let space_and_clip = SpaceAndClipInfo {
            spatial_id,
            clip_chain_id,
        };

        // Calculate chrome regions
        let tab_bar_height = TAB_BAR_HEIGHT * scale_factor;
        let nav_bar_y = tab_bar_height;
        let nav_bar_height = NAV_BAR_HEIGHT * scale_factor;

        // 1. Draw tab bar background
        let tab_bar_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::zero(),
            LayoutSize::new(viewport_size.width, tab_bar_height),
        );
        Self::draw_rect(builder, &space_and_clip, tab_bar_rect, theme.tab_bar_bg);

        // 2. Draw tabs
        Self::draw_tabs(state, builder, &space_and_clip, viewport_size.width, scale_factor, &mut widgets);

        // 3. Draw navigation bar background
        let nav_bar_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(0.0, nav_bar_y),
            LayoutSize::new(viewport_size.width, nav_bar_height),
        );
        Self::draw_rect(builder, &space_and_clip, nav_bar_rect, theme.nav_bar_bg);

        // 4. Draw navigation buttons
        Self::draw_nav_buttons(state, builder, &space_and_clip, nav_bar_y, scale_factor, &mut widgets);

        // 5. Draw URL bar
        Self::draw_url_bar(state, builder, &space_and_clip, nav_bar_y, viewport_size.width, scale_factor, &mut widgets);

        // 6. Draw loading progress bar if loading
        if state.nav_state.loading {
            Self::draw_loading_bar(state, builder, &space_and_clip, nav_bar_y + nav_bar_height - 2.0 * scale_factor, viewport_size.width, scale_factor);
        }

        // 7. Draw separator between tab bar and nav bar
        let separator_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(0.0, nav_bar_y),
            LayoutSize::new(viewport_size.width, 1.0 * scale_factor),
        );
        Self::draw_rect(builder, &space_and_clip, separator_rect, theme.separator_color);

        // 8. Draw bookmark bar if visible
        let chrome_bottom = if state.show_bookmark_bar {
            let bookmark_y = nav_bar_y + nav_bar_height;
            let bookmark_height = BOOKMARK_BAR_HEIGHT * scale_factor;
            let bookmark_rect = LayoutRect::from_origin_and_size(
                LayoutPoint::new(0.0, bookmark_y),
                LayoutSize::new(viewport_size.width, bookmark_height),
            );
            Self::draw_rect(builder, &space_and_clip, bookmark_rect, theme.bookmark_bar_bg);
            nav_bar_y + nav_bar_height + bookmark_height
        } else {
            nav_bar_y + nav_bar_height
        };

        // 9. Draw bottom shadow/border of chrome
        let shadow_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(0.0, chrome_bottom),
            LayoutSize::new(viewport_size.width, 1.0 * scale_factor),
        );
        Self::draw_rect(builder, &space_and_clip, shadow_rect, theme.shadow_color);

        // Store the widget layout
        state.set_widgets(widgets.clone());

        widgets
    }

    /// Draw a filled rectangle
    fn draw_rect(
        builder: &mut webrender::api::DisplayListBuilder,
        space_and_clip: &SpaceAndClipInfo,
        rect: LayoutRect,
        color: ColorF,
    ) {
        if color.a <= 0.0 {
            return;
        }
        let props = CommonItemProperties::new(rect, space_and_clip.clone());
        builder.push_rect(&props, rect, color);
    }

    /// Draw a rounded rectangle
    fn draw_rounded_rect(
        builder: &mut webrender::api::DisplayListBuilder,
        space_and_clip: &SpaceAndClipInfo,
        rect: LayoutRect,
        color: ColorF,
        radius: f32,
    ) {
        if color.a <= 0.0 {
            return;
        }

        let complex = ComplexClipRegion::new(
            rect,
            BorderRadius::uniform(radius),
            ClipMode::Clip,
        );
        let clip_id = builder.define_clip_rounded_rect(space_and_clip.spatial_id, complex);
        let clip_chain_id = builder.define_clip_chain(Some(space_and_clip.clip_chain_id), [clip_id]);
        let clipped_space = SpaceAndClipInfo {
            spatial_id: space_and_clip.spatial_id,
            clip_chain_id,
        };

        let props = CommonItemProperties::new(rect, clipped_space);
        builder.push_rect(&props, rect, color);
    }

    /// Draw the tabs in the tab bar
    fn draw_tabs(
        state: &ChromeState,
        builder: &mut webrender::api::DisplayListBuilder,
        space_and_clip: &SpaceAndClipInfo,
        viewport_width: f32,
        scale_factor: f32,
        widgets: &mut Vec<WidgetRect>,
    ) {
        let theme = &state.theme;
        let tab_count = state.tabs.len();
        if tab_count == 0 {
            return;
        }

        let tab_bar_height = TAB_BAR_HEIGHT * scale_factor;
        let new_tab_btn_width = 28.0 * scale_factor;
        let tabs_width = viewport_width - new_tab_btn_width;
        let max_tab_width = 220.0 * scale_factor;
        let min_tab_width = 60.0 * scale_factor;
        let tab_width = (tabs_width / tab_count as f32).min(max_tab_width).max(min_tab_width);

        for (i, tab) in state.tabs.iter().enumerate() {
            let is_active = i == state.active_tab_index;
            let tab_x = i as f32 * tab_width;
            let bg_color = if is_active { theme.tab_active_bg } else { theme.tab_inactive_bg };
            let text_color = if is_active { theme.tab_active_text } else { theme.tab_inactive_text };

            // Tab background
            let tab_rect = LayoutRect::from_origin_and_size(
                LayoutPoint::new(tab_x, 0.0),
                LayoutSize::new(tab_width, tab_bar_height),
            );

            // Draw tab with rounded top corners
            let tab_radius = 8.0 * scale_factor;
            Self::draw_rounded_rect(builder, space_and_clip, tab_rect, bg_color, tab_radius);

            // Active tab indicator line at top
            if is_active {
                let indicator_rect = LayoutRect::from_origin_and_size(
                    LayoutPoint::new(tab_x + tab_radius, 0.0),
                    LayoutSize::new(tab_width - 2.0 * tab_radius, 2.0 * scale_factor),
                );
                Self::draw_rect(builder, space_and_clip, indicator_rect, theme.brand_color);
            }

            // Tab close button (X)
            let close_btn_size = 16.0 * scale_factor;
            let close_btn_margin = 6.0 * scale_factor;
            let close_btn_x = tab_x + tab_width - close_btn_size - close_btn_margin;
            let close_btn_y = (tab_bar_height - close_btn_size) / 2.0;

            let close_widget_id = state.next_widget_id();
            let close_rect = Rect::new(
                Point2D::new(close_btn_x, close_btn_y),
                Size2D::new(close_btn_size, close_btn_size),
            );
            let is_close_hovered = state.hovered_widget() == Some(close_widget_id);

            // Draw close button background on hover
            if is_close_hovered {
                Self::draw_rounded_rect(
                    builder, space_and_clip,
                    LayoutRect::from_origin_and_size(
                        LayoutPoint::new(close_btn_x, close_btn_y),
                        LayoutSize::new(close_btn_size, close_btn_size),
                    ),
                    theme.tab_close_hover,
                    2.0 * scale_factor,
                );
            }

            // Draw X symbol using two small rectangles
            let x_margin = 4.0 * scale_factor;
            let x_color = if is_close_hovered { ColorF::WHITE } else { theme.tab_close_color };
            // Horizontal line of X (we'll just draw a small rect for simplicity)
            let x_rect1 = LayoutRect::from_origin_and_size(
                LayoutPoint::new(close_btn_x + x_margin, close_btn_y + close_btn_size / 2.0 - 1.0 * scale_factor),
                LayoutSize::new(close_btn_size - 2.0 * x_margin, 2.0 * scale_factor),
            );
            Self::draw_rect(builder, space_and_clip, x_rect1, x_color);

            widgets.push(WidgetRect::new(
                close_widget_id,
                WidgetKind::TabClose { index: i },
                close_rect,
            ));

            // Tab widget for hit testing (the whole tab area minus close button)
            let tab_widget_id = state.next_widget_id();
            let tab_hit_rect = Rect::new(
                Point2D::new(tab_x, 0.0),
                Size2D::new(tab_width - close_btn_size - close_btn_margin, tab_bar_height),
            );
            widgets.push(WidgetRect::new(
                tab_widget_id,
                WidgetKind::Tab { index: i },
                tab_hit_rect,
            ));

            // Separator between tabs
            if i < tab_count - 1 && !is_active {
                let sep_rect = LayoutRect::from_origin_and_size(
                    LayoutPoint::new(tab_x + tab_width - 0.5 * scale_factor, 4.0 * scale_factor),
                    LayoutSize::new(1.0 * scale_factor, tab_bar_height - 8.0 * scale_factor),
                );
                Self::draw_rect(builder, space_and_clip, sep_rect, theme.separator_color);
            }
        }

        // New tab button (+)
        let new_tab_x = tab_count as f32 * tab_width;
        let new_tab_size = 20.0 * scale_factor;
        let new_tab_btn_x = new_tab_x + (new_tab_btn_width - new_tab_size) / 2.0;
        let new_tab_btn_y = (tab_bar_height - new_tab_size) / 2.0;

        let new_tab_widget_id = state.next_widget_id();
        let is_new_tab_hovered = state.hovered_widget() == Some(new_tab_widget_id);

        if is_new_tab_hovered {
            Self::draw_rounded_rect(
                builder, space_and_clip,
                LayoutRect::from_origin_and_size(
                    LayoutPoint::new(new_tab_btn_x, new_tab_btn_y),
                    LayoutSize::new(new_tab_size, new_tab_size),
                ),
                theme.button_hover_bg,
                4.0 * scale_factor,
            );
        }

        // Draw + symbol
        let plus_margin = 5.0 * scale_factor;
        let plus_color = theme.new_tab_button_color;
        // Horizontal line
        let h_line = LayoutRect::from_origin_and_size(
            LayoutPoint::new(new_tab_btn_x + plus_margin, new_tab_btn_y + new_tab_size / 2.0 - 1.0 * scale_factor),
            LayoutSize::new(new_tab_size - 2.0 * plus_margin, 2.0 * scale_factor),
        );
        Self::draw_rect(builder, space_and_clip, h_line, plus_color);
        // Vertical line
        let v_line = LayoutRect::from_origin_and_size(
            LayoutPoint::new(new_tab_btn_x + new_tab_size / 2.0 - 1.0 * scale_factor, new_tab_btn_y + plus_margin),
            LayoutSize::new(2.0 * scale_factor, new_tab_size - 2.0 * plus_margin),
        );
        Self::draw_rect(builder, space_and_clip, v_line, plus_color);

        widgets.push(WidgetRect::new(
            new_tab_widget_id,
            WidgetKind::NewTabButton,
            Rect::new(
                Point2D::new(new_tab_btn_x, new_tab_btn_y),
                Size2D::new(new_tab_size, new_tab_size),
            ),
        ));
    }

    /// Draw navigation buttons (back, forward, refresh, home)
    fn draw_nav_buttons(
        state: &ChromeState,
        builder: &mut webrender::api::DisplayListBuilder,
        space_and_clip: &SpaceAndClipInfo,
        nav_bar_y: f32,
        scale_factor: f32,
        widgets: &mut Vec<WidgetRect>,
    ) {
        let theme = &state.theme;
        let nav_bar_height = NAV_BAR_HEIGHT * scale_factor;
        let button_size = 28.0 * scale_factor;
        let button_y = nav_bar_y + (nav_bar_height - button_size) / 2.0;
        let mut x = CHROME_PADDING * scale_factor;

        // Back button
        x = Self::draw_nav_button(
            state, builder, space_and_clip,
            x, button_y, button_size, scale_factor,
            WidgetKind::BackButton,
            !state.nav_state.can_go_back,
            theme,
            widgets,
            NavButtonIcon::LeftArrow,
        );
        x += WIDGET_SPACING * scale_factor;

        // Forward button
        x = Self::draw_nav_button(
            state, builder, space_and_clip,
            x, button_y, button_size, scale_factor,
            WidgetKind::ForwardButton,
            !state.nav_state.can_go_forward,
            theme,
            widgets,
            NavButtonIcon::RightArrow,
        );
        x += WIDGET_SPACING * scale_factor;

        // Refresh/Stop button
        let refresh_kind = if state.nav_state.loading {
            WidgetKind::RefreshButton // Acts as stop when loading
        } else {
            WidgetKind::RefreshButton
        };
        x = Self::draw_nav_button(
            state, builder, space_and_clip,
            x, button_y, button_size, scale_factor,
            refresh_kind,
            false,
            theme,
            widgets,
            if state.nav_state.loading { NavButtonIcon::Cross } else { NavButtonIcon::Refresh },
        );
        x += WIDGET_SPACING * scale_factor;

        // Home button
        Self::draw_nav_button(
            state, builder, space_and_clip,
            x, button_y, button_size, scale_factor,
            WidgetKind::HomeButton,
            false,
            theme,
            widgets,
            NavButtonIcon::Home,
        );
    }

    /// Draw a single navigation button and return the new x position
    fn draw_nav_button(
        state: &ChromeState,
        builder: &mut webrender::api::DisplayListBuilder,
        space_and_clip: &SpaceAndClipInfo,
        x: f32,
        y: f32,
        size: f32,
        scale_factor: f32,
        kind: WidgetKind,
        disabled: bool,
        theme: &ChromeTheme,
        widgets: &mut Vec<WidgetRect>,
        icon: NavButtonIcon,
    ) -> f32 {
        let widget_id = state.next_widget_id();
        let is_hovered = state.hovered_widget() == Some(widget_id);
        let is_pressed = state.pressed_widget() == Some(widget_id);

        let bg_color = if disabled {
            ColorF::TRANSPARENT
        } else if is_pressed {
            theme.button_pressed_bg
        } else if is_hovered {
            theme.button_hover_bg
        } else {
            theme.button_bg
        };

        // Button background
        if bg_color.a > 0.0 {
            Self::draw_rounded_rect(
                builder, space_and_clip,
                LayoutRect::from_origin_and_size(
                    LayoutPoint::new(x, y),
                    LayoutSize::new(size, size),
                ),
                bg_color,
                size / 2.0, // Circular
            );
        }

        // Draw icon
        let icon_color = if disabled {
            theme.button_disabled_color
        } else {
            theme.button_color
        };

        let icon_margin = 7.0 * scale_factor;
        let icon_x = x + icon_margin;
        let icon_y = y + icon_margin;
        let icon_size = size - 2.0 * icon_margin;
        let line_width = 2.0 * scale_factor;

        match icon {
            NavButtonIcon::LeftArrow => {
                // Draw < shape
                let x1 = icon_x + icon_size;
                let y1 = icon_y;
                let x2 = icon_x;
                let y2 = icon_y + icon_size / 2.0;
                let x3 = icon_x + icon_size;
                let y3 = icon_y + icon_size;
                // Top line of <
                let top_line = LayoutRect::from_origin_and_size(
                    LayoutPoint::new(x2, y1),
                    LayoutSize::new(line_width * 2.0, (y2 - y1).abs() + line_width),
                );
                // Approximate with a rotated approach - use two small rects
                let mid_y = icon_y + icon_size / 2.0 - line_width / 2.0;
                let mid_rect = LayoutRect::from_origin_and_size(
                    LayoutPoint::new(icon_x, mid_y),
                    LayoutSize::new(icon_size, line_width),
                );
                Self::draw_rect(builder, space_and_clip, mid_rect, icon_color);
                // Top-left diagonal
                let top_diag = LayoutRect::from_origin_and_size(
                    LayoutPoint::new(icon_x + icon_size * 0.2, mid_y - icon_size * 0.25),
                    LayoutSize::new(icon_size * 0.5, line_width),
                );
                Self::draw_rect(builder, space_and_clip, top_diag, icon_color);
                // Bottom-left diagonal
                let bot_diag = LayoutRect::from_origin_and_size(
                    LayoutPoint::new(icon_x + icon_size * 0.2, mid_y + icon_size * 0.25),
                    LayoutSize::new(icon_size * 0.5, line_width),
                );
                Self::draw_rect(builder, space_and_clip, bot_diag, icon_color);
            }
            NavButtonIcon::RightArrow => {
                let mid_y = icon_y + icon_size / 2.0 - line_width / 2.0;
                let mid_rect = LayoutRect::from_origin_and_size(
                    LayoutPoint::new(icon_x, mid_y),
                    LayoutSize::new(icon_size, line_width),
                );
                Self::draw_rect(builder, space_and_clip, mid_rect, icon_color);
                // Top-right diagonal
                let top_diag = LayoutRect::from_origin_and_size(
                    LayoutPoint::new(icon_x + icon_size * 0.3, mid_y - icon_size * 0.25),
                    LayoutSize::new(icon_size * 0.5, line_width),
                );
                Self::draw_rect(builder, space_and_clip, top_diag, icon_color);
                // Bottom-right diagonal
                let bot_diag = LayoutRect::from_origin_and_size(
                    LayoutPoint::new(icon_x + icon_size * 0.3, mid_y + icon_size * 0.25),
                    LayoutSize::new(icon_size * 0.5, line_width),
                );
                Self::draw_rect(builder, space_and_clip, bot_diag, icon_color);
            }
            NavButtonIcon::Refresh => {
                // Draw a circular arrow (approximate with arcs)
                let cx = icon_x + icon_size / 2.0;
                let cy = icon_y + icon_size / 2.0;
                let r = icon_size / 2.5;
                // Top arc
                let arc_rect = LayoutRect::from_origin_and_size(
                    LayoutPoint::new(cx - r, cy - r),
                    LayoutSize::new(r * 2.0, r * 2.0),
                );
                Self::draw_rounded_rect(builder, space_and_clip,
                    LayoutRect::from_origin_and_size(
                        LayoutPoint::new(cx - r, cy - r - line_width / 2.0),
                        LayoutSize::new(r * 2.0, line_width),
                    ),
                    icon_color, line_width / 2.0,
                );
                // Bottom arc
                Self::draw_rounded_rect(builder, space_and_clip,
                    LayoutRect::from_origin_and_size(
                        LayoutPoint::new(cx - r, cy + r - line_width / 2.0),
                        LayoutSize::new(r * 2.0, line_width),
                    ),
                    icon_color, line_width / 2.0,
                );
                // Left arc
                Self::draw_rounded_rect(builder, space_and_clip,
                    LayoutRect::from_origin_and_size(
                        LayoutPoint::new(cx - r - line_width / 2.0, cy - r),
                        LayoutSize::new(line_width, r * 2.0),
                    ),
                    icon_color, line_width / 2.0,
                );
                // Right arc
                Self::draw_rounded_rect(builder, space_and_clip,
                    LayoutRect::from_origin_and_size(
                        LayoutPoint::new(cx + r - line_width / 2.0, cy - r),
                        LayoutSize::new(line_width, r * 2.0),
                    ),
                    icon_color, line_width / 2.0,
                );
                // Arrow tip at top-right
                let arrow = LayoutRect::from_origin_and_size(
                    LayoutPoint::new(cx + r - 3.0 * scale_factor, cy - r - 2.0 * scale_factor),
                    LayoutSize::new(6.0 * scale_factor, 4.0 * scale_factor),
                );
                Self::draw_rect(builder, space_and_clip, arrow, icon_color);
            }
            NavButtonIcon::Cross => {
                // Draw X for stop
                let mid_x = icon_x + icon_size / 2.0;
                let mid_y = icon_y + icon_size / 2.0;
                let half = icon_size * 0.35;
                // Diagonal 1
                Self::draw_rect(builder, space_and_clip,
                    LayoutRect::from_origin_and_size(
                        LayoutPoint::new(mid_x - half, mid_y - line_width / 2.0),
                        LayoutSize::new(half * 2.0, line_width),
                    ),
                    icon_color,
                );
                // Diagonal 2
                Self::draw_rect(builder, space_and_clip,
                    LayoutRect::from_origin_and_size(
                        LayoutPoint::new(mid_x - line_width / 2.0, mid_y - half),
                        LayoutSize::new(line_width, half * 2.0),
                    ),
                    icon_color,
                );
            }
            NavButtonIcon::Home => {
                // Draw house shape
                let cx = icon_x + icon_size / 2.0;
                let roof_y = icon_y;
                let base_y = icon_y + icon_size * 0.45;
                let bottom_y = icon_y + icon_size;
                let hw = icon_size * 0.45;
                // Roof (triangle approximation with rects)
                let roof = LayoutRect::from_origin_and_size(
                    LayoutPoint::new(cx - line_width / 2.0, roof_y),
                    LayoutSize::new(line_width, base_y - roof_y),
                );
                Self::draw_rect(builder, space_and_clip, roof, icon_color);
                // Roof left diagonal
                Self::draw_rect(builder, space_and_clip,
                    LayoutRect::from_origin_and_size(
                        LayoutPoint::new(cx - hw, base_y - line_width),
                        LayoutSize::new(hw, line_width),
                    ),
                    icon_color,
                );
                // Roof right diagonal
                Self::draw_rect(builder, space_and_clip,
                    LayoutRect::from_origin_and_size(
                        LayoutPoint::new(cx, base_y - line_width),
                        LayoutSize::new(hw, line_width),
                    ),
                    icon_color,
                );
                // Base
                let base = LayoutRect::from_origin_and_size(
                    LayoutPoint::new(cx - hw + line_width, base_y),
                    LayoutSize::new(hw * 2.0 - 2.0 * line_width, bottom_y - base_y),
                );
                Self::draw_rect(builder, space_and_clip, base, icon_color);
            }
        }

        // Register widget for hit testing
        let rect = Rect::new(
            Point2D::new(x, y),
            Size2D::new(size, size),
        );
        let mut widget = WidgetRect::new(widget_id, kind, rect);
        widget.disabled = disabled;
        widget.hovered = is_hovered;
        widget.pressed = is_pressed;
        widgets.push(widget);

        x + size
    }

    /// Draw the URL bar
    fn draw_url_bar(
        state: &ChromeState,
        builder: &mut webrender::api::DisplayListBuilder,
        space_and_clip: &SpaceAndClipInfo,
        nav_bar_y: f32,
        viewport_width: f32,
        scale_factor: f32,
        widgets: &mut Vec<WidgetRect>,
    ) {
        let theme = &state.theme;
        let nav_bar_height = NAV_BAR_HEIGHT * scale_factor;

        // URL bar positioning
        let url_bar_margin = CHROME_PADDING * scale_factor;
        let nav_buttons_width = (4.0 * (28.0 + WIDGET_SPACING) + CHROME_PADDING) * scale_factor;
        let url_bar_x = nav_buttons_width;
        let url_bar_right_margin = 8.0 * scale_factor;
        let url_bar_width = viewport_width - url_bar_x - url_bar_right_margin;
        let url_bar_height = nav_bar_height - 2.0 * url_bar_margin;
        let url_bar_y = nav_bar_y + url_bar_margin;

        // URL bar background
        let url_bar_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(url_bar_x, url_bar_y),
            LayoutSize::new(url_bar_width, url_bar_height),
        );

        let border_color = if state.url_bar.focused {
            theme.url_bar_focus_border
        } else {
            theme.url_bar_border
        };

        // Draw URL bar border
        Self::draw_rounded_rect(builder, space_and_clip, url_bar_rect, border_color, url_bar_height / 2.0);

        // Draw URL bar inner background (slightly smaller)
        let border_width = 1.0 * scale_factor;
        let inner_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(url_bar_x + border_width, url_bar_y + border_width),
            LayoutSize::new(url_bar_width - 2.0 * border_width, url_bar_height - 2.0 * border_width),
        );
        Self::draw_rounded_rect(builder, space_and_clip, inner_rect, theme.url_bar_bg, (url_bar_height - 2.0 * border_width) / 2.0);

        // Register URL bar widget
        let url_widget_id = state.next_widget_id();
        let rect = Rect::new(
            Point2D::new(url_bar_x, url_bar_y),
            Size2D::new(url_bar_width, url_bar_height),
        );
        let mut widget = WidgetRect::new(url_widget_id, WidgetKind::UrlBar, rect);
        widget.hovered = state.hovered_widget() == Some(url_widget_id);
        widgets.push(widget);
    }

    /// Draw the loading progress bar
    fn draw_loading_bar(
        state: &ChromeState,
        builder: &mut webrender::api::DisplayListBuilder,
        space_and_clip: &SpaceAndClipInfo,
        y: f32,
        viewport_width: f32,
        scale_factor: f32,
    ) {
        let theme = &state.theme;
        let bar_height = 2.0 * scale_factor;
        let progress = state.nav_state.loading_progress;
        let bar_width = viewport_width * progress;

        let bar_rect = LayoutRect::from_origin_and_size(
            LayoutPoint::new(0.0, y),
            LayoutSize::new(bar_width, bar_height),
        );
        Self::draw_rect(builder, space_and_clip, bar_rect, theme.loading_bar_color);
    }
}

/// Navigation button icon types
enum NavButtonIcon {
    LeftArrow,
    RightArrow,
    Refresh,
    Cross,
    Home,
}
