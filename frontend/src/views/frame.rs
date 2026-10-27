use dioxus::prelude::*;

use crate::components::sidebar::{
	Sidebar, SidebarCollapsible, SidebarHeader, SidebarProvider, SidebarSide, SidebarTrigger,
};
use crate::elements::PrimarySidebar;
use crate::elements::TitleBar;
use crate::views::{SIDEBAR_SWAPPED, SIDEBAR_VARIANT};
use crate::Route;

#[component]
pub fn Frame() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./frame.css") }

		div { id: "frame", "data-swapped": SIDEBAR_SWAPPED(),
			TitleBar {}
			SidebarProvider {
				Sidebar {
					class: "sidebar",
					side: if SIDEBAR_SWAPPED() { SidebarSide::Left } else { SidebarSide::Right },
					variant: SIDEBAR_VARIANT(),
					collapsible: SidebarCollapsible::Icon,
					SidebarHeader { class: "header", SidebarTrigger {} }
				}
				PrimarySidebar { Outlet::<Route> {} }
			}
		}
	}
}
