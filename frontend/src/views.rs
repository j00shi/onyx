mod frame;
mod home;
mod newtab;
mod settings;

pub use frame::Frame;
pub use home::Home;
pub use newtab::NewTab;
pub use settings::{
	Settings, ACCENT_HUE, BORDER_RADIUS_SCALE, CUSTOM_TITLEBAR, DARK_MODE, MOBILE_REACH_LAYOUT, SHOW_WALLPAPER,
	SIDEBAR_SWAPPED, SIDEBAR_VARIANT,
};
