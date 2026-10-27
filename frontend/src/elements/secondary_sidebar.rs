use dioxus::prelude::*;

use crate::components::sidebar::{Sidebar, SidebarCollapsible, SidebarHeader, SidebarSide, SidebarTrigger};
use crate::views::{SIDEBAR_SWAPPED, SIDEBAR_VARIANT};

/// The secondary sidebar. Docks on the right by default, or the left when the layout is swapped
#[component]
pub fn SecondarySidebar() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./secondary_sidebar.css") }

		Sidebar {
			class: "secondary-sidebar",
			side: if SIDEBAR_SWAPPED() { SidebarSide::Left } else { SidebarSide::Right },
			variant: SIDEBAR_VARIANT(),
			collapsible: SidebarCollapsible::Icon,
			SidebarHeader { class: "header", SidebarTrigger {} }
		}
	}
}
