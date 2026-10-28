//! Custom, per-platform window chrome: a draggable title bar with window controls.
//!
//! This is desktop-only and currently covers macOS and Windows. On Linux, web, and mobile
//! [`TitleBar`] compiles down to nothing and the platform's native title bar is used (Linux
//! custom chrome will come later).
//!
//! Platform behaviour:
//! - **macOS**: keeps the *real* OS traffic lights (the window stays decorated but with a
//!   transparent, full-size-content titlebar). No custom buttons are drawn — the native
//!   lights sit at the top-left, inset to line up with the sidebar.
//! - **Windows**: no OS buttons exist for a borderless window, so faithful Windows 11-style
//!   caption buttons are drawn in the top-right.

use dioxus::prelude::*;

/// Full-width draggable window chrome. Renders nothing unless this is a macOS/Windows desktop
/// build and the user has opted into the custom title bar ([`crate::views::CUSTOM_TITLEBAR`]).
#[component]
pub fn TitleBar() -> Element {
	#[cfg(all(feature = "desktop", any(target_os = "macos", target_os = "windows")))]
	{
		desktop::title_bar()
	}
	#[cfg(not(all(feature = "desktop", any(target_os = "macos", target_os = "windows"))))]
	{
		rsx! {}
	}
}

#[cfg(all(feature = "desktop", any(target_os = "macos", target_os = "windows")))]
mod desktop {
	use dioxus::desktop::{use_window, DesktopContext};
	use dioxus::prelude::*;

	use crate::components::sidebar::SidebarVariant;
	use crate::views::{CUSTOM_TITLEBAR, SIDEBAR_VARIANT};

	const TITLEBAR_CSS: Asset = asset!("./titlebar.css");

	/// Runtime OS tag, used to switch styling in SCSS via `#titlebar[data-os="…"]`.
	const OS: &str = if cfg!(target_os = "macos") { "macos" } else { "windows" };

	/// A single window-control button.
	// On macOS the native traffic lights mean no custom buttons are ever built, so the variants
	// are legitimately unconstructed there (they are used on Windows).
	#[cfg_attr(target_os = "macos", allow(dead_code))]
	#[derive(Clone, Copy, PartialEq, Eq)]
	enum Control {
		Minimize,
		Maximize,
		Close,
	}

	pub(super) fn title_bar() -> Element {
		let window = use_window();

		// Apply the chosen chrome mode to the live window whenever the setting changes. Reads the
		// sidebar variant too: the traffic-light inset depends on how far the sidebar is from the
		// window edge (see `apply_chrome`).
		use_effect({
			let window = window.clone();
			move || {
				let custom = CUSTOM_TITLEBAR();
				let variant = SIDEBAR_VARIANT();
				apply_chrome(&window, custom, variant);
				// Toggling the custom title bar makes macOS relayout the title bar asynchronously,
				// which clobbers the inset we just set; re-apply it once that settles so the lights
				// land in the right spot without needing another toggle.
				#[cfg(target_os = "macos")]
				reapply_inset_after_relayout(window.clone(), custom, variant);
			}
		});

		let right = use_hook(controls);
		let on_drag = window.clone();
		let on_double = window.clone();

		rsx! {
			document::Link { rel: "stylesheet", href: TITLEBAR_CSS }
			if CUSTOM_TITLEBAR() {
				div {
					id: "titlebar",
					"data-os": OS,
					// Drag the window by the bar; double-click toggles maximize (native feel).
					onmousedown: move |_| on_drag.drag(),
					ondoubleclick: move |_| on_double.toggle_maximized(),
					div { class: "drag-spacer" }
					ControlZone { controls: right, window }
				}
			}
		}
	}

	/// The right-hand control zone. Takes its own window handle so it needs no `PartialEq` on the
	/// (non-comparable) desktop context — the `controls` list drives memoization instead.
	#[component]
	fn ControlZone(controls: Vec<Control>, window: ReadSignal<DesktopContext>) -> Element {
		rsx! {
			div { class: "zone",
				for control in controls {
					ControlButton { control, window }
				}
			}
		}
	}

	#[component]
	fn ControlButton(control: Control, window: ReadSignal<DesktopContext>) -> Element {
		let (name, action): (&str, fn(&DesktopContext)) = match control {
			Control::Minimize => ("minimize", |w| w.set_minimized(true)),
			Control::Maximize => ("maximize", |w| w.toggle_maximized()),
			Control::Close => ("close", |w| w.close()),
		};

		rsx! {
			button {
				class: "ctl",
				"data-ctl": name,
				r#type: "button",
				// Don't let button interaction start a window drag / maximize.
				onmousedown: move |e| e.stop_propagation(),
				ondoubleclick: move |e| e.stop_propagation(),
				onclick: move |_| action(&window()),
				ControlGlyph { control }
			}
		}
	}

	/// Faithful thin-stroke caption glyphs.
	#[component]
	fn ControlGlyph(control: Control) -> Element {
		rsx! {
			svg {
				class: "glyph",
				view_box: "0 0 10 10",
				width: "10",
				height: "10",
				match control {
					Control::Minimize => rsx! {
					line {
						x1: "0",
						y1: "5",
						x2: "10",
						y2: "5",
					}
				},
					Control::Maximize => rsx! {
					rect {
						x: "0.5",
						y: "0.5",
						width: "9",
						height: "9",
					}
				},
					Control::Close => rsx! {
					line {
						x1: "0",
						y1: "0",
						x2: "10",
						y2: "10",
					}
					line {
						x1: "10",
						y1: "0",
						x2: "0",
						y2: "10",
					}
				},
				}
			}
		}
	}

	/// Switch the live window between native decorations and custom chrome.
	fn apply_chrome(window: &DesktopContext, custom: bool, variant: SidebarVariant) {
		#[cfg(target_os = "windows")]
		{
			// Borderless when custom; the native frame/title bar otherwise. The sidebar variant
			// only affects the macOS traffic-light inset, so it goes unused here.
			let _ = variant;
			window.set_decorations(!custom);
		}
		#[cfg(target_os = "macos")]
		{
			// Keep the native traffic lights either way; just (un)hide the titlebar chrome.
			use dioxus::desktop::tao::platform::macos::WindowExtMacOS;
			use dioxus::desktop::wry::WebViewExtMacOS;
			window.set_titlebar_transparent(custom);
			window.set_fullsize_content_view(custom);
			// Reposition the traffic lights for the current chrome (see `traffic_light_inset`).
			let _ = window.webview.set_traffic_light_inset(traffic_light_inset(custom, variant));
		}
	}

	/// The traffic-light inset for the current chrome. The macOS default is 20,20, which we restore
	/// for the native title bar. The custom titlebar drops them into the header; a docked sidebar is
	/// flush to the window edge while the floating card is inset by 0.5rem (8px) of padding, so the
	/// lights shift that far outward to stay aligned with it.
	#[cfg(target_os = "macos")]
	fn traffic_light_inset(custom: bool, variant: SidebarVariant) -> dioxus::desktop::LogicalPosition<f64> {
		use dioxus::desktop::LogicalPosition;

		if custom {
			let (x, y) = if variant == SidebarVariant::Floating { (20.0, 30.0) } else { (12.0, 22.0) };
			LogicalPosition::new(x, y)
		} else {
			LogicalPosition::new(20.0, 20.0)
		}
	}

	/// Re-apply the traffic-light inset after `AppKit` finishes relaying out the window. Toggling the
	/// full-size content view (switching to/from the custom title bar) kicks off an asynchronous
	/// relayout that resets the inset set synchronously in [`apply_chrome`], leaving the lights in
	/// the wrong place until the next change. Bouncing through an eval round-trip over a couple of
	/// animation frames yields the main run loop so the layout settles first.
	#[cfg(target_os = "macos")]
	fn reapply_inset_after_relayout(window: DesktopContext, custom: bool, variant: SidebarVariant) {
		use dioxus::desktop::wry::WebViewExtMacOS;

		spawn(async move {
			let mut eval =
				document::eval("requestAnimationFrame(() => requestAnimationFrame(() => dioxus.send(true)));");
			let _ = eval.recv::<bool>().await;
			let _ = window.webview.set_traffic_light_inset(traffic_light_inset(custom, variant));
		});
	}

	/// Right-hand window-control buttons for the current platform.
	#[cfg(target_os = "macos")]
	const fn controls() -> Vec<Control> {
		// Native traffic lights handle everything; draw no custom buttons.
		Vec::new()
	}

	#[cfg(target_os = "windows")]
	fn controls() -> Vec<Control> {
		vec![Control::Minimize, Control::Maximize, Control::Close]
	}
}
