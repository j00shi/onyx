use dioxus::prelude::*;
use dioxus_icons::lucide::{Bookmark, Folder, House, Search};
use plugin_api::RibbonSection;

use crate::components::sidebar::SidebarSide;
use crate::elements::{
	BaseSidebar, Ribbon, RibbonItem, RibbonItemKind, SidebarTab, SidebarTabs, PRIMARY_SIDEBAR_TOGGLE,
};
use crate::views::SIDEBAR_SWAPPED;

/// The primary sidebar. Docks on the left by default, or the right when the layout is swapped.
/// Resizing and the collapse/expand transitions come from [`BaseSidebar`]; the tab ribbon and
/// panels are added here.
#[component]
pub fn PrimarySidebar(children: Element) -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./primary_sidebar.css") }

		BaseSidebar {
			name: "primary".to_string(),
			side: if SIDEBAR_SWAPPED() { SidebarSide::Right } else { SidebarSide::Left },
			register: move |toggle| *PRIMARY_SIDEBAR_TOGGLE.write() = Some(toggle),
			body: rsx! {
				PrimaryTabs {}
			},
			{children}
		}
	}
}

/// Builds the primary sidebar's concrete tabs and ribbon and hands them to [`SidebarTabs`].
#[component]
fn PrimaryTabs() -> Element {
	// Placeholder tabs. Later these come from the plugin registry.
	let tabs = vec![
		SidebarTab {
			id: "home".to_string(),
			icon: rsx! { House { size: "1rem" } },
			label: "Home".to_string(),
			content: rsx! {
				p { "Lorem ipsum dolor sit amet consectetur adipisicing elit. Vitae velit ipsam praesentium nostrum enim ad neque, optio autem dolorum perferendis nemo eveniet obcaecati suscipit nihil ratione! Dolorum quos expedita assumenda? Lorem ipsum dolor sit amet consectetur adipisicing elit. Vitae velit ipsam praesentium nostrum enim ad neque, optio autem dolorum perferendis nemo eveniet obcaecati suscipit nihil ratione! Dolorum quos expedita assumenda? Lorem ipsum dolor sit amet consectetur adipisicing elit. Vitae velit ipsam praesentium nostrum enim ad neque, optio autem dolorum perferendis nemo eveniet obcaecati suscipit nihil ratione! Dolorum quos expedita assumenda? Lorem ipsum dolor sit amet consectetur adipisicing elit. Vitae velit ipsam praesentium nostrum enim ad neque, optio autem dolorum perferendis nemo eveniet obcaecati suscipit nihil ratione! Dolorum quos expedita assumenda?" }
			},
		},
		SidebarTab {
			id: "files".to_string(),
			icon: rsx! { Folder { size: "1rem" } },
			label: "Files".to_string(),
			content: rsx! { "Files tab content" },
		},
		SidebarTab {
			id: "search".to_string(),
			icon: rsx! { Search { size: "1rem" } },
			label: "Search".to_string(),
			content: rsx! { "Search tab content" },
		},
		SidebarTab {
			id: "bookmarks".to_string(),
			icon: rsx! { Bookmark { size: "1rem" } },
			label: "Bookmarks".to_string(),
			content: rsx! { "Bookmarks tab content" },
		},
	];

	// Ribbon buttons come from the plugin registry (`App::add_ribbon_item`). Each spec's icon
	// builder runs here, and its command id becomes the click handler, in this render scope.
	let specs = plugin_api::RIBBON_ITEMS.read();
	let build = |section: RibbonSection| {
		specs
			.iter()
			.filter(|s| s.section == section)
			.map(|s| {
				let command = s.command.clone();
				RibbonItem {
					icon: (s.icon)(),
					tooltip: s.tooltip.clone(),
					kind: RibbonItemKind::Action(EventHandler::new(move |()| plugin_api::invoke_command(&command))),
				}
			})
			.collect::<Vec<_>>()
	};
	let ribbon_top = build(RibbonSection::Top);
	let ribbon_bottom = build(RibbonSection::Bottom);

	rsx! {
		SidebarTabs { tabs,
			Ribbon { top: ribbon_top, bottom: ribbon_bottom }
		}
	}
}
