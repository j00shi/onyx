use dioxus::prelude::*;

use crate::components::sidebar::SidebarVariant;
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

#[component]
pub fn Settings() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./settings.css") }
		div { id: "settings",
			h1 { class: "title", "Settings" }
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
