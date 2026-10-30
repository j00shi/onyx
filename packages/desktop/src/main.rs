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
	use dioxus::desktop::{Config, LogicalSize, WindowBuilder};

	// Launch transparent on every platform an stay that way. Opaque background simply paints over
	let builder = WindowBuilder::new()
		.with_title("Onyx")
		.with_transparent(true)
		.with_min_inner_size(LogicalSize::new(400.0, 30.0));

	#[cfg(target_os = "macos")]
	let builder = {
		use dioxus::desktop::tao::platform::macos::WindowBuilderExtMacOS;

		builder.with_titlebar_transparent(true).with_fullsize_content_view(true).with_title_hidden(true)
	};

	#[cfg(target_os = "windows")]
	let builder = builder.with_decorations(false).with_resizable(true);

	// Transparent initial webview background to avoid a flash before the page renders
	let config = Config::new().with_window(builder).with_background_color((0, 0, 0, 0));
	dioxus::LaunchBuilder::new().with_cfg(config).launch(frontend::App);
}
