//! Chrome Widgets
//!
//! Low-level widget primitives for building the native chrome display list.
//! These use WebRender's `DisplayListBuilder` API directly.

use webrender_api::{
    BorderDetails, BorderWidths, BorderRadius, ClipMode, ColorF, CommonItemProperties,
    ComplexClipRegion, DisplayListBuilder, LayoutPoint, LayoutRect, LayoutSize, SpaceAndClipInfo,
};

/// Push a filled rectangle to the display list
pub fn push_rect(
    builder: &mut DisplayListBuilder,
    space_and_clip: &SpaceAndClipInfo,
    rect: LayoutRect,
    color: ColorF,
) {
    builder.push_rect(
        &CommonItemProperties::new(rect, space_and_clip.clone()),
        rect,
        color,
    );
}

/// Push a rounded rectangle to the display list
pub fn push_rounded_rect(
    builder: &mut DisplayListBuilder,
    space_and_clip: &SpaceAndClipInfo,
    rect: LayoutRect,
    color: ColorF,
    radius: f32,
) {
    if radius <= 0.0 {
        push_rect(builder, space_and_clip, rect, color);
        return;
    }

    let complex = ComplexClipRegion::new(
        rect,
        BorderRadius::uniform(radius),
        ClipMode::Clip,
    );
    let clip_id = builder.define_clip_rounded_rect(space_and_clip.spatial_id, complex);
    let clip_chain_id = builder.define_clip_chain(
        Some(space_and_clip.clip_chain_id),
        [clip_id],
    );
    let clipped_space = SpaceAndClipInfo {
        spatial_id: space_and_clip.spatial_id,
        clip_chain_id,
    };

    builder.push_rect(
        &CommonItemProperties::new(rect, clipped_space),
        rect,
        color,
    );
}

/// Push a bordered rounded rectangle
pub fn push_bordered_rounded_rect(
    builder: &mut DisplayListBuilder,
    space_and_clip: &SpaceAndClipInfo,
    rect: LayoutRect,
    bg_color: ColorF,
    border_color: ColorF,
    border_width: f32,
    radius: f32,
) {
    // Push background
    push_rounded_rect(builder, space_and_clip, rect, bg_color, radius);

    // Push border
    if border_width > 0.0 {
        let border_details = BorderDetails::Normal(webrender_api::NormalBorder {
            top: webrender_api::BorderSide {
                color: border_color,
                style: webrender_api::BorderStyle::Solid,
            },
            right: webrender_api::BorderSide {
                color: border_color,
                style: webrender_api::BorderStyle::Solid,
            },
            bottom: webrender_api::BorderSide {
                color: border_color,
                style: webrender_api::BorderStyle::Solid,
            },
            left: webrender_api::BorderSide {
                color: border_color,
                style: webrender_api::BorderStyle::Solid,
            },
            radius: BorderRadius::uniform(radius),
            do_aa: true,
        });

        builder.push_border(
            &CommonItemProperties::new(rect, space_and_clip.clone()),
            rect,
            BorderWidths::uniform(border_width),
            border_details,
        );
    }
}

/// Push a horizontal separator line
pub fn push_separator(
    builder: &mut DisplayListBuilder,
    space_and_clip: &SpaceAndClipInfo,
    y: f32,
    width: f32,
    color: ColorF,
) {
    let rect = LayoutRect::from_origin_and_size(
        LayoutPoint::new(0.0, y),
        LayoutSize::new(width, 1.0),
    );
    push_rect(builder, space_and_clip, rect, color);
}

/// Calculate the width for each tab given the total available width and tab count
pub fn calculate_tab_width(total_width: f32, tab_count: usize, min_width: f32, max_width: f32) -> f32 {
    if tab_count == 0 {
        return max_width;
    }
    let ideal_width = total_width / tab_count as f32;
    ideal_width.clamp(min_width, max_width)
}
