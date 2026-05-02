//! Chrome Icons
//!
//! Manages icon loading and caching as WebRender ImageKeys.
//! Icons are embedded as PNG bytes and registered with WebRender.

use webrender_api::{ImageDescriptor, ImageData, ImageFormat, ImageKey, RenderApi, Transaction};

/// Icon keys used by the native chrome
#[derive(Debug, Clone)]
pub struct ChromeIcons {
    pub back: ImageKey,
    pub forward: ImageKey,
    pub refresh: ImageKey,
    pub close: ImageKey,
    pub close_tab: ImageKey,
    pub new_tab: ImageKey,
    pub bookmark: ImageKey,
    pub bookmark_filled: ImageKey,
    pub download: ImageKey,
    pub home: ImageKey,
    pub menu: ImageKey,
    pub shield: ImageKey,
    pub logo: ImageKey,
}

impl ChromeIcons {
    /// Generate and register all chrome icons with WebRender
    pub fn register(api: &mut RenderApi, txn: &mut Transaction) -> Self {
        let back = Self::add_icon(api, txn, ICON_BACK);
        let forward = Self::add_icon(api, txn, ICON_FORWARD);
        let refresh = Self::add_icon(api, txn, ICON_REFRESH);
        let close = Self::add_icon(api, txn, ICON_CLOSE);
        let close_tab = Self::add_icon(api, txn, ICON_CLOSE_TAB);
        let new_tab = Self::add_icon(api, txn, ICON_NEW_TAB);
        let bookmark = Self::add_icon(api, txn, ICON_BOOKMARK);
        let bookmark_filled = Self::add_icon(api, txn, ICON_BOOKMARK_FILLED);
        let download = Self::add_icon(api, txn, ICON_DOWNLOAD);
        let home = Self::add_icon(api, txn, ICON_HOME);
        let menu = Self::add_icon(api, txn, ICON_MENU);
        let shield = Self::add_icon(api, txn, ICON_SHIELD);
        let logo = Self::add_icon_from_data(api, txn, include_bytes!("../../icons/icon32x32.png"));

        Self {
            back,
            forward,
            refresh,
            close,
            close_tab,
            new_tab,
            bookmark,
            bookmark_filled,
            download,
            home,
            menu,
            shield,
            logo,
        }
    }

    fn add_icon(api: &mut RenderApi, txn: &mut Transaction, data: &[u8]) -> ImageKey {
        let key = api.generate_image_key();
        let desc = ImageDescriptor::new(16, 16, ImageFormat::BGRA8, true);
        txn.add_image(key, desc, ImageData::new(data.to_vec()), None);
        key
    }

    fn add_icon_from_data(api: &mut RenderApi, txn: &mut Transaction, data: &[u8]) -> ImageKey {
        // Decode PNG to get dimensions
        let key = api.generate_image_key();
        // For now, use raw data - in production, decode PNG to BGRA
        let desc = ImageDescriptor::new(32, 32, ImageFormat::BGRA8, true);
        txn.add_image(key, desc, ImageData::new(data.to_vec()), None);
        key
    }
}

// Placeholder icon data - these would be replaced with actual pre-rendered BGRA8 icon data
// In production, these would be generated at build time from SVG sources

const _16X16: usize = 16 * 16 * 4; // 16x16 BGRA8

// Back arrow icon (16x16 BGRA8)
const ICON_BACK: &[u8] = include_bytes!("icons/back.rgba");
const ICON_FORWARD: &[u8] = include_bytes!("icons/forward.rgba");
const ICON_REFRESH: &[u8] = include_bytes!("icons/refresh.rgba");
const ICON_CLOSE: &[u8] = include_bytes!("icons/close.rgba");
const ICON_CLOSE_TAB: &[u8] = include_bytes!("icons/close_tab.rgba");
const ICON_NEW_TAB: &[u8] = include_bytes!("icons/new_tab.rgba");
const ICON_BOOKMARK: &[u8] = include_bytes!("icons/bookmark.rgba");
const ICON_BOOKMARK_FILLED: &[u8] = include_bytes!("icons/bookmark_filled.rgba");
const ICON_DOWNLOAD: &[u8] = include_bytes!("icons/download.rgba");
const ICON_HOME: &[u8] = include_bytes!("icons/home.rgba");
const ICON_MENU: &[u8] = include_bytes!("icons/menu.rgba");
const ICON_SHIELD: &[u8] = include_bytes!("icons/shield.rgba");
