//! Desktop platform entry point
//! This is a minimal launcher that simply runs the shared UI app.

fn main() {
	#[cfg(feature = "desktop")]
	launch_desktop();

	#[cfg(not(feature = "desktop"))]
	dioxus::launch(frontend::App);
}

/// Launch the desktop window. On macOS, a transparent/full-size titlebar that keeps the real OS
/// traffic lights; on Windows, a borderless window whose custom chrome ([`frontend::TitleBar`])
/// draws the controls. Linux keeps its native title bar for now.
#[cfg(feature = "desktop")]
fn launch_desktop() {
	use dioxus::desktop::{Config, WindowBuilder};

	let builder = WindowBuilder::new().with_title("Onyx");

	#[cfg(target_os = "macos")]
	let builder = {
		use dioxus::desktop::tao::platform::macos::WindowBuilderExtMacOS;
		use dioxus::desktop::LogicalPosition;

		builder
			.with_titlebar_transparent(true)
			.with_fullsize_content_view(true)
			.with_title_hidden(true)
			// Traffic-light position (x right, y down from the top-left corner; macOS default
			// is ~20,20, so go above that to move inward). Keep in sync with the runtime value
			// in `frontend`'s `titlebar::apply_chrome`.
			.with_traffic_light_inset(LogicalPosition::new(44.0, 20.0))
	};

	#[cfg(target_os = "windows")]
	let builder = builder.with_decorations(false).with_resizable(true);

	let config = Config::new().with_window(builder);
	dioxus::LaunchBuilder::new().with_cfg(config).launch(frontend::App);
}
