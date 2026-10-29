use dioxus::prelude::*;

use crate::components::sidebar::{SidebarHeader, SidebarSide, SidebarTrigger};
use crate::elements::BaseSidebar;
use crate::views::SIDEBAR_SWAPPED;

/// The secondary sidebar. Docks on the right by default, or the left when the layout is swapped.
/// Shares its resizing and collapse/expand transitions with the primary via [`BaseSidebar`]; it
/// stays mostly empty, carrying only a toggle in its header.
#[component]
pub fn SecondarySidebar(children: Element) -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./secondary_sidebar.css") }

		BaseSidebar {
			name: "secondary".to_string(),
			side: if SIDEBAR_SWAPPED() { SidebarSide::Left } else { SidebarSide::Right },
			body: rsx! {
				div { class: "sidebar-body",
					SidebarHeader { class: "header", SidebarTrigger {} }
				}
			},
			{children}
		}
	}
}
