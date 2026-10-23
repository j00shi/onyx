use dioxus::prelude::*;

use crate::components::sidebar::SidebarVariant;
use crate::components::switch::Switch;

/// Sidebar variant shared with [`crate::views::Frame`].
pub static SIDEBAR_VARIANT: GlobalSignal<SidebarVariant> = Signal::global(|| SidebarVariant::Floating);
/// When true, the primary and secondary sidebars swap sides (left ↔ right).
pub static SIDEBAR_SWAPPED: GlobalSignal<bool> = Signal::global(|| false);

#[component]
pub fn Settings() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./settings.css") }
		div { id: "Settings", class: "settings",
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
		}
	}
}
