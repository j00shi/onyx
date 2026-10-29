use dioxus::prelude::*;
use palette::{encoding, Hsv, RgbHue};

use crate::components::color_picker::ColorPicker;
use crate::components::sidebar::SidebarVariant;
use crate::components::slider::Slider;
use crate::components::switch::Switch;

/// Sidebar variant shared with [`crate::views::Frame`].
pub static SIDEBAR_VARIANT: GlobalSignal<SidebarVariant> = Signal::global(|| SidebarVariant::Floating);
/// When true, the primary and secondary sidebars swap sides (left ↔ right).
pub static SIDEBAR_SWAPPED: GlobalSignal<bool> = Signal::global(|| false);
/// When true, the app draws its own window chrome (see [`crate::elements::TitleBar`]) instead of
/// the native OS title bar. Only meaningful on desktop; ignored on web/mobile.
pub static CUSTOM_TITLEBAR: GlobalSignal<bool> = Signal::global(|| true);
/// When true, the mobile sidebar drawer moves its tab bar below the content and enlarges its
/// ribbon buttons for easier one-handed reach. Only affects the sub-768px mobile sidebar.
pub static MOBILE_REACH_LAYOUT: GlobalSignal<bool> = Signal::global(|| false);
/// When true, the app uses the dark theme; otherwise the light theme. Applied as the
/// `data-theme` attribute on `<html>` by [`crate::views::Frame`] (see `_theme.scss`).
pub static DARK_MODE: GlobalSignal<bool> = Signal::global(|| true);
/// The application tint, in degrees. Drives the `--hue` custom property in `_theme.scss`.
pub static ACCENT_HUE: GlobalSignal<f64> = Signal::global(|| 200.0);
/// When true, [`crate::views::Frame`] paints the wallpaper behind the app chrome.
pub static SHOW_WALLPAPER: GlobalSignal<bool> = Signal::global(|| true);
/// When true, the window is translucent with a native backdrop blur.
///
/// macOS vibrancy, Windows 11 Mica/Acrylic; plain transparency on Linux. Applied as the
/// `data-window-effect` attribute on `<html>` by [`crate::views::Frame`] and acted on by
/// [`crate::elements::WindowEffect`]. Only meaningful on desktop; ignored on web/mobile.
pub static WINDOW_EFFECT: GlobalSignal<bool> = Signal::global(|| false);
/// Multiplier applied to the `--border-radius-*` custom properties in `_theme.scss`.
/// `1.0` keeps the designed radii; `0.0` squares every corner.
pub static BORDER_RADIUS_SCALE: GlobalSignal<f64> = Signal::global(|| 1.0);

#[component]
pub fn Settings() -> Element {
	// Full-saturation swatch so the picker visibly reflects the chosen hue; only the hue is
	// read back into `ACCENT_HUE`.
	let mut accent_color = use_signal(|| Hsv::<encoding::Srgb, f64>::new(RgbHue::new(ACCENT_HUE()), 1.0, 1.0));
	let mut color_picker_open = use_signal(|| false);

	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./settings.css") }
		div { id: "settings",
			h1 { class: "title", "Settings" }
			section { class: "setting",
				div { class: "label",
					span { class: "name", "Dark mode" }
					span { class: "description",
						"Switch between the dark and light themes."
					}
				}
				Switch {
					checked: Some(DARK_MODE()),
					on_checked_change: move |v: bool| *DARK_MODE.write() = v,
				}
			}
			section { class: "setting",
				div { class: "label",
					span { class: "name", "Accent color" }
					span { class: "description",
						"Tint the interface by shifting the hue of every color."
					}
				}
				ColorPicker {
					color: accent_color(),
					open: Some(color_picker_open()),
					on_open_change: move |v: bool| color_picker_open.set(v),
					on_color_change: move |c: Hsv<encoding::Srgb, f64>| {
						accent_color.set(c);
						*ACCENT_HUE.write() = c.hue.into_positive_degrees();
					},
				}
			}
			section { class: "setting",
				div { class: "label",
					span { class: "name", "Show wallpaper" }
					span { class: "description",
						"Paint the background wallpaper behind the app."
					}
				}
				Switch {
					checked: Some(SHOW_WALLPAPER()),
					on_checked_change: move |v: bool| *SHOW_WALLPAPER.write() = v,
				}
			}
			section { class: "setting",
				div { class: "label",
					span { class: "name", "Window blur" }
					span { class: "description",
						"Make the window translucent with a native backdrop blur. Desktop only."
					}
				}
				Switch {
					checked: Some(WINDOW_EFFECT()),
					on_checked_change: move |v: bool| *WINDOW_EFFECT.write() = v,
				}
			}
			section { class: "setting",
				div { class: "label",
					span { class: "name", "Corner roundness" }
					span { class: "description",
						"Scale the border radius of the interface, from square to extra round."
					}
				}
				Slider {
					label: "Corner roundness",
					horizontal: true,
					min: 0.0,
					max: 2.0,
					step: 0.25,
					value: BORDER_RADIUS_SCALE(),
					on_value_change: move |v: f64| *BORDER_RADIUS_SCALE.write() = v,
				}
			}
			section { class: "setting",
				div { class: "label",
					span { class: "name", "Floating sidebar" }
					span { class: "description",
						"Switch between the floating and docked sidebar styles."
					}
				}
				Switch {
					checked: Some(SIDEBAR_VARIANT() == SidebarVariant::Floating),
					on_checked_change: move |v: bool| {
						*SIDEBAR_VARIANT.write() = if v {
							SidebarVariant::Floating
						} else {
							SidebarVariant::Sidebar
						};
					},
				}
			}
			section { class: "setting",
				div { class: "label",
					span { class: "name", "Swap sidebars" }
					span { class: "description",
						"Move the primary sidebar to the right and the secondary to the left."
					}
				}
				Switch {
					checked: Some(SIDEBAR_SWAPPED()),
					on_checked_change: move |v: bool| *SIDEBAR_SWAPPED.write() = v,
				}
			}
			section { class: "setting",
				div { class: "label",
					span { class: "name", "Custom title bar" }
					span { class: "description",
						"Replace the native window title bar with Onyx's own window controls. Desktop only."
					}
				}
				Switch {
					checked: Some(CUSTOM_TITLEBAR()),
					on_checked_change: move |v: bool| *CUSTOM_TITLEBAR.write() = v,
				}
			}
			section { class: "setting",
				div { class: "label",
					span { class: "name", "Mobile reach layout" }
					span { class: "description",
						"On small screens, move the sidebar tabs below the content and enlarge its buttons for easier one-handed use. Mobile only."
					}
				}
				Switch {
					checked: Some(MOBILE_REACH_LAYOUT()),
					on_checked_change: move |v: bool| *MOBILE_REACH_LAYOUT.write() = v,
				}
			}
		}
	}
}
