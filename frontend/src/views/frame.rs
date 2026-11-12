use dioxus::prelude::*;

use crate::components::sidebar::SidebarVariant;
use crate::elements::PrimarySidebar;
use crate::elements::SecondarySidebar;
use crate::elements::TitleBar;
use crate::platform::WindowEffect;
use crate::views::ACCENT_HUE;
use crate::views::BORDER_RADIUS_SCALE;
use crate::views::DARK_MODE;
use crate::views::DURATION_SCALE;
use crate::views::SHOW_WALLPAPER;
use crate::views::SIDEBAR_SWAPPED;
use crate::views::SIDEBAR_VARIANT;
use crate::views::SPACING_SCALE;
use crate::views::WINDOW_EFFECT;
use crate::workspace::WorkspaceView;

// Using a hardcoded const as background for now
const BACKGROUND: Asset = asset!("/assets/media/pexels-167691936-37955298.jpg");

#[component]
pub fn Frame() -> Element {
	use_effect(move || {
		let theme = if DARK_MODE() { "dark" } else { "light" };
		let hue = ACCENT_HUE();
		let scale = BORDER_RADIUS_SCALE();
		let spacing = SPACING_SCALE();
		let duration = DURATION_SCALE();
		let effect = WINDOW_EFFECT();
		let js = format!(
			"const r = document.documentElement; \
			 r.setAttribute('data-theme', '{theme}'); \
			 r.setAttribute('data-window-effect', '{effect}'); \
			 r.style.setProperty('--hue', '{hue}deg'); \
			 r.style.setProperty('--border-radius-sm', 'calc(0.5rem * {scale})'); \
			 r.style.setProperty('--border-radius-md', 'calc(1rem * {scale})'); \
			 r.style.setProperty('--border-radius-lg', 'calc(1.5rem * {scale})'); \
			 r.style.setProperty('--space-sm', 'calc(0.25rem * {spacing})'); \
			 r.style.setProperty('--space-md', 'calc(0.5rem * {spacing})'); \
			 r.style.setProperty('--space-lg', 'calc(1rem * {spacing})'); \
			 r.style.setProperty('--duration-fast', 'calc(0.1s * {duration})'); \
			 r.style.setProperty('--duration-medium', 'calc(0.2s * {duration})'); \
			 r.style.setProperty('--duration-slow', 'calc(0.5s * {duration})'); \
			 r.style.setProperty('--delay', 'calc(0.5s * {duration})');"
		);
		_ = document::eval(&js);
	});

	let frame_background = if SHOW_WALLPAPER() { format!("url('{BACKGROUND}')") } else { "none".to_string() };

	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./frame.css") }

		div {
			id: "frame",
			"data-swapped": SIDEBAR_SWAPPED(),
			style: "--frame-background: {frame_background};",
			TitleBar {}
			WindowEffect {}
			PrimarySidebar {
				SecondarySidebar {
					div {
						id: "content",
						// Lets the content line its tab bar up with the sidebar header, which sits lower
						// when floating (see `#content` in frame.scss).
						"data-sidebar-variant": if SIDEBAR_VARIANT() == SidebarVariant::Floating { "floating" } else { "sidebar" },
						WorkspaceView {}
					}
				}
			}
		}
	}
}
