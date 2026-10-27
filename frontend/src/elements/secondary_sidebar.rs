use dioxus::prelude::*;

use crate::components::sidebar::{
	Sidebar, SidebarCollapsible, SidebarHeader, SidebarInset, SidebarProvider, SidebarSide, SidebarTrigger,
};
use crate::views::{SIDEBAR_SWAPPED, SIDEBAR_VARIANT};

/// The secondary sidebar. Docks on the right by default, or the left when the layout is swapped.
#[component]
pub fn SecondarySidebar(children: Element) -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./secondary_sidebar.css") }

		div { class: "secondary-sidebar-wrapper",
			SidebarProvider {
				Sidebar {
					class: "secondary-sidebar",
					side: if SIDEBAR_SWAPPED() { SidebarSide::Left } else { SidebarSide::Right },
					variant: SIDEBAR_VARIANT(),
					collapsible: SidebarCollapsible::Icon,
					SidebarHeader { class: "header", SidebarTrigger {} }
				}
				SidebarInset { {children} }
			}
		}
	}
}
