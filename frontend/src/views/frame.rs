use dioxus::prelude::*;

use crate::components::sidebar::SidebarTrigger;
use crate::elements::PrimarySidebar;
use crate::elements::SecondarySidebar;
use crate::elements::TitleBar;
use crate::elements::WindowEffect;
use crate::views::ACCENT_HUE;
use crate::views::BORDER_RADIUS_SCALE;
use crate::views::DARK_MODE;
use crate::views::SHOW_WALLPAPER;
use crate::views::SIDEBAR_SWAPPED;
use crate::views::WINDOW_EFFECT;
use crate::Route;

// Using a hardcoded const as background for now
const BACKGROUND: Asset = asset!("/assets/media/pexels-167691936-37955298.jpg");

#[component]
pub fn Frame() -> Element {
	use_effect(move || {
		let theme = if DARK_MODE() { "dark" } else { "light" };
		let hue = ACCENT_HUE();
		let scale = BORDER_RADIUS_SCALE();
		let effect = WINDOW_EFFECT();
		let js = format!(
			"const r = document.documentElement; \
			 r.setAttribute('data-theme', '{theme}'); \
			 r.setAttribute('data-window-effect', '{effect}'); \
			 r.style.setProperty('--hue', '{hue}deg'); \
			 r.style.setProperty('--border-radius-sm', 'calc(0.5rem * {scale})'); \
			 r.style.setProperty('--border-radius-md', 'calc(1rem * {scale})'); \
			 r.style.setProperty('--border-radius-lg', 'calc(1.5rem * {scale})');"
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
				SidebarTrigger { title: "Toggle primary" }
				SecondarySidebar {
					SidebarTrigger { title: "Toggle secondary" }
					div { id: "content", Outlet::<Route> {} }
				}
			}
		}
	}
}
