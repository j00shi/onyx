//! Desktop window setup.

/// Launch the desktop window. On macOS, a transparent/full-size titlebar that keeps the real OS
/// traffic lights; on Windows, a borderless window whose custom chrome ([`frontend::TitleBar`])
/// draws the controls. Linux keeps its native title bar for now.
pub fn launch_desktop() {
	use dioxus::desktop::{Config, LogicalSize, WindowBuilder};

	let builder: WindowBuilder =
		WindowBuilder::new().with_title("Onyx").with_min_inner_size(LogicalSize::new(400.0, 300.0));

	// Window transparency backs the native blur on macOS/Windows and the opaque body simply paints
	// over it when the effect is off. Linux's WebKitGTK renders transparent windows with severe
	// flicker (elements glitching in and out) and has no native blur anyway, so keep it opaque there.
	#[cfg(any(target_os = "macos", target_os = "windows"))]
	let builder = builder.with_transparent(true);

	#[cfg(target_os = "macos")]
	let builder = {
		use dioxus::desktop::tao::platform::macos::WindowBuilderExtMacOS;

		builder.with_titlebar_transparent(true).with_fullsize_content_view(true).with_title_hidden(true)
	};

	#[cfg(target_os = "windows")]
	let builder = builder.with_decorations(false).with_resizable(true);

	let config = Config::new().with_window(builder);

	// Transparent initial webview background to avoid a flash before the page renders. Only where the
	// window is transparent — on Linux this would reintroduce the WebKitGTK flicker.
	#[cfg(any(target_os = "macos", target_os = "windows"))]
	let config = config.with_background_color((0, 0, 0, 0));

	// Drop the default menu bar on Linux — it renders as a Window/Edit/Help strip inside the window.
	// macOS keeps its native top menu; Windows already hides it (borderless, no decorations).
	#[cfg(target_os = "linux")]
	let config = config.with_menu(None::<dioxus::desktop::muda::Menu>);

	dioxus::LaunchBuilder::new().with_cfg(config).launch(frontend::App);
}
