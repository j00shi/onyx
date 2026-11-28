use dioxus::prelude::*;
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
	// Sidebar tabs come from the plugin registry (`App::add_sidebar_tab`). Each spec's icon/content
	// builder runs here, in this render scope.
	let tab_specs = plugin_api::SIDEBAR_TABS.read();
	let tabs = tab_specs
		.iter()
		.map(|s| SidebarTab { id: s.id.clone(), icon: (s.icon)(), label: s.label.clone(), content: (s.content)() })
		.collect::<Vec<_>>();

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
