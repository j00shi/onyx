use dioxus::prelude::*;
use dioxus_icons::lucide::{
	ArrowLeft, ArrowRight, Bell, Bookmark, Calendar, Folder, House, ListTodo, Mail, Search, Settings, Square, User,
	Vault,
};

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::sidebar::{use_sidebar, SidebarContent, SidebarGroup, SidebarHeader, SidebarSide};
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};
use crate::components::tooltip::{Tooltip, TooltipContent, TooltipTrigger};
use crate::elements::{BaseSidebar, Ribbon, RibbonItem, RibbonItemKind, RibbonMenuItem, PRIMARY_SIDEBAR_CTX};
use crate::views::{CUSTOM_TITLEBAR, MOBILE_REACH_LAYOUT, SIDEBAR_SWAPPED};
use crate::workspace::model::View;
use crate::workspace::WORKSPACE;
use dioxus_primitives::ContentSide;

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
				SidebarTabs {}
			},
			{children}
		}
	}
}

/// Tab list, ribbon, and panels
#[component]
fn SidebarTabs() -> Element {
	let ctx = use_sidebar();

	// Tabs sit at the top of the sidebar, so tooltips normally drop below them. In the mobile
	// reach layout the tab bar moves to the bottom, so they flip up to stay on screen.
	let tab_tooltip_side =
		if MOBILE_REACH_LAYOUT() && (ctx.is_mobile)() { ContentSide::Top } else { ContentSide::Bottom };

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
		Tabs { class: "sidebar-body", default_value: "home".to_string(),
			SidebarHeader { class: "sidebar-header",
				// Traffic lights on macOS overlay this corner
				if cfg!(all(feature = "desktop", target_os = "macos")) && CUSTOM_TITLEBAR() {
					div { class: "controls-wrapper",
						Button {
							variant: ButtonVariant::Ghost,
							size: ButtonSize::IconSm,
							title: "Go back",
							ArrowLeft { size: "1rem" }
						}
						Button {
							variant: ButtonVariant::Ghost,
							size: ButtonSize::IconSm,
							title: "Go forward",
							ArrowRight { size: "1rem" }
						}
					}
				}
				// `SidebarHeader` only forwards global attributes, and a forwarded `onclick` on a
				// `TabTrigger` would be swallowed by the standard click handler. Instead the click
				// is caught via the (`display: contents`) wrapper, which fires for every tab,
				// including a re-click of the already active one.
				div {
					style: "display: contents",
					onclick: move |_| {
						if !(ctx.is_mobile)() && !ctx.open() {
							ctx.toggle();
						}
					},
					TabList { class: "tablist",
						Tooltip { class: "tab-tooltip",
							TooltipTrigger { class: "tab-slot",
								TabTrigger {
									class: "tab",
									value: "home".to_string(),
									index: 0usize,
									House { size: "1rem" }
								}
							}
							TooltipContent {
								class: "tooltip-content",
								side: tab_tooltip_side,
								"Home"
							}
						}
						Tooltip { class: "tab-tooltip",
							TooltipTrigger { class: "tab-slot",
								TabTrigger {
									class: "tab",
									value: "files".to_string(),
									index: 1usize,
									Folder { size: "1rem" }
								}
							}
							TooltipContent {
								class: "tooltip-content",
								side: tab_tooltip_side,
								"Files"
							}
						}
						Tooltip { class: "tab-tooltip",
							TooltipTrigger { class: "tab-slot",
								TabTrigger {
									class: "tab",
									value: "search".to_string(),
									index: 2usize,
									Search { size: "1rem" }
								}
							}
							TooltipContent {
								class: "tooltip-content",
								side: tab_tooltip_side,
								"Search"
							}
						}
						Tooltip { class: "tab-tooltip",
							TooltipTrigger { class: "tab-slot",
								TabTrigger {
									class: "tab",
									value: "bookmarks".to_string(),
									index: 3usize,
									Bookmark { size: "1rem" }
								}
							}
							TooltipContent {
								class: "tooltip-content",
								side: tab_tooltip_side,
								"Bookmarks"
							}
						}
					}
				}
			}
			SidebarContent { class: "sidebar-content",
				Ribbon { top: ribbon_top, bottom: ribbon_bottom }
				SidebarGroup { class: "panel",
					TabContent {
						class: "panel-tab-content",
						value: "home".to_string(),
						index: 0usize,
						p { "Lorem ipsum dolor sit amet consectetur adipisicing elit. Vitae velit ipsam praesentium nostrum enim ad neque, optio autem dolorum perferendis nemo eveniet obcaecati suscipit nihil ratione! Dolorum quos expedita assumenda? Lorem ipsum dolor sit amet consectetur adipisicing elit. Vitae velit ipsam praesentium nostrum enim ad neque, optio autem dolorum perferendis nemo eveniet obcaecati suscipit nihil ratione! Dolorum quos expedita assumenda? Lorem ipsum dolor sit amet consectetur adipisicing elit. Vitae velit ipsam praesentium nostrum enim ad neque, optio autem dolorum perferendis nemo eveniet obcaecati suscipit nihil ratione! Dolorum quos expedita assumenda? Lorem ipsum dolor sit amet consectetur adipisicing elit. Vitae velit ipsam praesentium nostrum enim ad neque, optio autem dolorum perferendis nemo eveniet obcaecati suscipit nihil ratione! Dolorum quos expedita assumenda?" }
					}
					TabContent {
						class: "panel-tab-content",
						value: "files".to_string(),
						index: 1usize,
						style: "background-color:rgba(10,10,10,0.5)",
						"Files tab content"
					}
					TabContent {
						class: "panel-tab-content",
						value: "search".to_string(),
						index: 2usize,
						"Search tab content"
					}
					TabContent {
						class: "panel-tab-content",
						value: "bookmarks".to_string(),
						index: 3usize,
						"Bookmarks tab content"
					}
				}
			}
		}
	}
}

/// Builds a placeholder dropdown row (no action wired yet).
fn menu_item(label: &str, separator_before: bool) -> RibbonMenuItem {
	RibbonMenuItem { label: label.to_string(), on_select: EventHandler::new(|()| {}), separator_before }
}
