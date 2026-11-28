mod frame;
mod home;
mod newtab;
mod settings;

pub use frame::Frame;
pub use home::Home;
pub use newtab::NewTab;
pub use settings::{
	Settings, ACCENT_HUE, BORDER_RADIUS_SCALE, CUSTOM_TITLEBAR, DARK_MODE, DURATION_SCALE, MOBILE_REACH_LAYOUT,
	SHOW_WALLPAPER, SIDEBAR_SWAPPED, SIDEBAR_VARIANT, SPACING_SCALE, WINDOW_EFFECT,
};

/// View ids for the built-in pages the app shell registers (see `plugin.rs`). Shared so the
/// workspace model can seed default tabs without hard-coding string literals.
pub const HOME_VIEW: &str = "app:home";
pub const NEW_TAB_VIEW: &str = "app:new-tab";
pub const SETTINGS_VIEW: &str = "app:settings";
