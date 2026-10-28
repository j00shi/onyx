use dioxus::prelude::*;

use crate::components::sidebar::SidebarTrigger;
use crate::elements::PrimarySidebar;
use crate::elements::SecondarySidebar;
use crate::elements::TitleBar;
use crate::views::SIDEBAR_SWAPPED;
use crate::Route;

// Using a hardcoded const as background for now
const BACKGROUND: Asset = asset!("/assets/media/pexels-167691936-37955298.jpg");

#[component]
pub fn Frame() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./frame.css") }

		div {
			id: "frame",
			"data-swapped": SIDEBAR_SWAPPED(),
			style: "--frame-background: url('{BACKGROUND}');",
			TitleBar {}
			PrimarySidebar {
				SidebarTrigger { title: "Toggle primary" }
				SecondarySidebar {
					SidebarTrigger { title: "Toggle secondary" }
					Outlet::<Route> {}
				}
			}
		}
	}
}
