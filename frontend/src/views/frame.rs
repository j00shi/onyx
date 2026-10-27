use dioxus::prelude::*;

use crate::components::sidebar::SidebarProvider;
use crate::elements::PrimarySidebar;
use crate::elements::SecondarySidebar;
use crate::elements::TitleBar;
use crate::views::SIDEBAR_SWAPPED;
use crate::Route;

#[component]
pub fn Frame() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./frame.css") }

		div { id: "frame", "data-swapped": SIDEBAR_SWAPPED(),
			TitleBar {}
			SidebarProvider {
				SecondarySidebar {}
				PrimarySidebar { Outlet::<Route> {} }
			}
		}
	}
}
