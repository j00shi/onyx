use dioxus::prelude::*;
use dioxus_icons::lucide::{
	Bell, Bookmark, Calendar, Folder, House, ListTodo, Mail, Search, Settings, Square, User, Vault,
};

use crate::components::sidebar::SidebarSide;
use crate::elements::{
	BaseSidebar, Ribbon, RibbonItem, RibbonItemKind, RibbonMenuItem, SidebarTab, SidebarTabs, PRIMARY_SIDEBAR_CTX,
};
use crate::views::SIDEBAR_SWAPPED;
use crate::workspace::model::View;
use crate::workspace::WORKSPACE;

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
			register: move |ctx| *PRIMARY_SIDEBAR_CTX.write() = Some(ctx),
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

	// Placeholder ribbon content. Later these lists come from the plugin registry.
	let noop = EventHandler::new(|()| {});

	let mut ribbon_top = vec![
		RibbonItem {
			icon: rsx! { ListTodo { size: "1.25rem" } },
			tooltip: "I wonder how long I can type before this gets cut off".to_string(),
			kind: RibbonItemKind::Action(noop),
		},
		RibbonItem {
			icon: rsx! { Calendar { size: "1.25rem" } },
			tooltip: "Calendar".to_string(),
			kind: RibbonItemKind::Menu(vec![
				menu_item("Day", false),
				menu_item("Week", false),
				menu_item("Month", false),
				menu_item("Year", false),
				menu_item("Decade", false),
				menu_item("Century", false),
				menu_item("Millennium", false),
				menu_item("New event", true),
			]),
		},
		RibbonItem {
			icon: rsx! { Mail { size: "1.25rem" } },
			tooltip: "Mail".to_string(),
			kind: RibbonItemKind::Action(noop),
		},
		RibbonItem {
			icon: rsx! { Bell { size: "1.25rem" } },
			tooltip: "Notifications".to_string(),
			kind: RibbonItemKind::Action(noop),
		},
	];
	// Example filler buttons
	ribbon_top.extend((0..10).map(|i| RibbonItem {
		icon: rsx! { Square { size: "1.25rem" } },
		tooltip: format!("Example {}", i + 1),
		kind: RibbonItemKind::Action(noop),
	}));

	// Top-to-bottom visual order; the toggle is pinned below these by `Ribbon`.
	let mut ribbon_bottom = vec![
		RibbonItem {
			icon: rsx! { Settings { size: "1.25rem" } },
			tooltip: "Settings".to_string(),
			kind: RibbonItemKind::Action(EventHandler::new(|()| {
				WORKSPACE.write().open_view(View::Settings);
			})),
		},
		RibbonItem {
			icon: rsx! { User { size: "1.25rem" } },
			tooltip: "Account".to_string(),
			kind: RibbonItemKind::Action(noop),
		},
	];
	// Example filler buttons
	ribbon_bottom.extend((0..5).map(|i| RibbonItem {
		icon: rsx! { Square { size: "1.25rem" } },
		tooltip: format!("Example {}", i + 1),
		kind: RibbonItemKind::Action(noop),
	}));
	ribbon_bottom.push(RibbonItem {
		icon: rsx! { Vault { size: "1.25rem" } },
		tooltip: "Vault".to_string(),
		kind: RibbonItemKind::Menu(vec![
			menu_item("Personal", false),
			menu_item("Work", false),
			menu_item("University", false),
			menu_item("Monoversity", false),
			menu_item("IDK", false),
			menu_item("Cool vault", false),
			menu_item("Habmurger", false),
			menu_item("Manage Vaults", true),
		]),
	});

	rsx! {
		SidebarTabs { tabs,
			Ribbon { top: ribbon_top, bottom: ribbon_bottom }
		}
	}
}

/// Builds a placeholder dropdown row (no action wired yet).
fn menu_item(label: &str, separator_before: bool) -> RibbonMenuItem {
	RibbonMenuItem { label: label.to_string(), on_select: EventHandler::new(|()| {}), separator_before }
}
