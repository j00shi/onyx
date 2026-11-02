//! Native window backdrop: a translucent system blur behind the whole window.
//!
//! Desktop-only, macOS + Windows. Follows [`crate::views::WINDOW_EFFECT`]: when enabled the window
//! gets a real system backdrop (macOS vibrancy / Windows 11 Mica); when disabled it is cleared and
//! the page paints its normal opaque background again (see `data-window-effect` in `main.scss`).
//!
//! On Linux, web, and mobile this compiles to nothing. Linux has no `window-vibrancy` support, so
//! there the effect is plain window transparency (the window is always launched transparent, and
//! the compositor draws any blur) toggled purely by the CSS background.
//! TODO: Implement this better maybe?

use dioxus::prelude::*;

/// Applies/removes the native window backdrop as [`crate::views::WINDOW_EFFECT`] changes. Renders
/// nothing. Only active on a macOS/Windows desktop build.
#[component]
pub fn WindowEffect() -> Element {
	#[cfg(all(feature = "desktop", any(target_os = "macos", target_os = "windows")))]
	{
		desktop::window_effect()
	}
	#[cfg(not(all(feature = "desktop", any(target_os = "macos", target_os = "windows"))))]
	{
		rsx! {}
	}
}

#[cfg(all(feature = "desktop", any(target_os = "macos", target_os = "windows")))]
mod desktop {
	#[cfg(target_os = "windows")]
	use dioxus::desktop::use_wry_event_handler;
	use dioxus::desktop::{use_window, DesktopContext};
	use dioxus::prelude::*;

	use crate::views::{DARK_MODE, WINDOW_EFFECT};

	pub(super) fn window_effect() -> Element {
		let window = use_window();

		// Re-apply the backdrop whenever the toggle flips. Dark mode is read too: Windows Mica wants
		// to know the theme, so the backdrop re-applies when the theme changes while it's on.
		use_effect({
			// Clone only where the resize handler below also needs the window (Windows); elsewhere
			// the effect is the sole owner and can take it by move.
			#[cfg(target_os = "windows")]
			let window = window.clone();
			move || {
				let on = WINDOW_EFFECT();
				let dark = DARK_MODE();
				apply_effect(&window, on, dark);
			}
		});

		// Windows' DWM caches the extended-frame glass, so after a resize the newly exposed area can
		// keep showing the window's default (white) surface until the backdrop is re-applied for the
		// new size. Re-assert it on every resize while the effect is on. (macOS vibrancy tracks the
		// window on its own, so this is Windows-only.)
		#[cfg(target_os = "windows")]
		use_wry_event_handler(move |event, _| {
			use dioxus::desktop::tao::event::{Event, WindowEvent};
			if let Event::WindowEvent { event: WindowEvent::Resized(_), .. } = event {
				if WINDOW_EFFECT() {
					apply_effect(&window, true, DARK_MODE());
				}
			}
		});

		rsx! {}
	}

	/// Apply or clear the platform's native backdrop on the live window. Errors (e.g. an unsupported
	/// OS version) are ignored like the other live-window tweaks in `titlebar.rs` — the window simply
	/// stays as-is. The window is always transparent, and the page is opaque while the effect is off,
	/// so a failed/absent backdrop just reads as a normal opaque window (this also sidesteps the
	/// Windows "white window until resize" transparency bug, Dioxus #4886).
	fn apply_effect(window: &DesktopContext, on: bool, dark: bool) {
		// `window.window` is an `Arc<tao::window::Window>`; `&Window` implements `HasWindowHandle`.
		let handle = window.window.as_ref();

		#[cfg(target_os = "macos")]
		{
			use window_vibrancy::{apply_vibrancy, clear_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

			// macOS vibrancy follows the system appearance on its own, so the theme isn't needed here.
			let _ = dark;
			if on {
				let _ = apply_vibrancy(
					handle,
					NSVisualEffectMaterial::UnderWindowBackground,
					Some(NSVisualEffectState::FollowsWindowActiveState),
					None,
				);
			} else {
				let _ = clear_vibrancy(handle);
			}
		}

		#[cfg(target_os = "windows")]
		{
			use dioxus::desktop::tao::platform::windows::WindowExtWindows;
			use window_vibrancy::{apply_mica, clear_mica};
			use windows_sys::Win32::{
				Foundation::HWND, Graphics::Dwm::DwmExtendFrameIntoClientArea, UI::Controls::MARGINS,
			};

			let hwnd = handle.hwnd() as HWND;
			if on {
				let _ = apply_mica(handle, Some(dark));
				// `apply_mica` only sets the DWM backdrop attribute. Our window is borderless and
				// transparent, so the system backdrop has no frame to paint on and the client area
				// composites to black. Extend the frame across the whole window (the -1 "sheet of
				// glass" margins) so the Mica backdrop actually fills it.
				let margins = MARGINS { cxLeftWidth: -1, cxRightWidth: -1, cyTopHeight: -1, cyBottomHeight: -1 };
				unsafe {
					let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);
				}
			} else {
				let _ = clear_mica(handle);
				// Collapse the extended frame so the opaque page paints normally again.
				let margins = MARGINS { cxLeftWidth: 0, cxRightWidth: 0, cyTopHeight: 0, cyBottomHeight: 0 };
				unsafe {
					let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);
				}
			}
		}
	}
}
