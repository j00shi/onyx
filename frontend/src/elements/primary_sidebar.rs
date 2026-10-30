use dioxus::prelude::*;
use dioxus_icons::lucide::{
	ArrowLeft, ArrowRight, Bell, Bookmark, Calendar, Folder, House, ListTodo, Mail, Search, Settings, Square, User,
	Vault,
};

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger};
use crate::components::separator::Separator;
use crate::components::sidebar::{
	use_sidebar, SidebarContent, SidebarGroup, SidebarHeader, SidebarSide, SidebarTrigger,
};
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};
use crate::components::tooltip::{Tooltip, TooltipContent, TooltipTrigger};
use crate::elements::BaseSidebar;
use crate::views::{CUSTOM_TITLEBAR, MOBILE_REACH_LAYOUT, SIDEBAR_SWAPPED};
use crate::Route;
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
	let ribbon_tooltip_side = if SIDEBAR_SWAPPED() { ContentSide::Left } else { ContentSide::Right };

	rsx! {
		Tabs { class: "sidebar-body tabs", default_value: "home".to_string(),
			SidebarHeader { class: "header",
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
								class: "tab-tooltip-content",
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
								class: "tab-tooltip-content",
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
								class: "tab-tooltip-content",
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
								class: "tab-tooltip-content",
								side: tab_tooltip_side,
								"Bookmarks"
							}
						}
					}
				}
			}
			SidebarContent { class: "content",
				SidebarGroup { class: "ribbon",
					div { class: "ribbon-top",
						Tooltip {
							TooltipTrigger {
								Button {
									variant: ButtonVariant::Ghost,
									size: ButtonSize::Icon,
									ListTodo { size: "1.25rem" }
								}
							}
							TooltipContent {
								class: "tab-tooltip-content",
								side: ribbon_tooltip_side,
								"Tasks"
							}
						}
						Tooltip {
							TooltipTrigger {
								Button {
									variant: ButtonVariant::Ghost,
									size: ButtonSize::Icon,
									Calendar { size: "1.25rem" }
								}
							}
							TooltipContent {
								class: "tab-tooltip-content",
								side: ribbon_tooltip_side,
								"Calendar"
							}
						}
						Tooltip {
							TooltipTrigger {
								Button {
									variant: ButtonVariant::Ghost,
									size: ButtonSize::Icon,
									Mail { size: "1.25rem" }
								}
							}
							TooltipContent {
								class: "tab-tooltip-content",
								side: ribbon_tooltip_side,
								"Mail"
							}
						}
						Tooltip {
							TooltipTrigger {
								Button {
									variant: ButtonVariant::Ghost,
									size: ButtonSize::Icon,
									Bell { size: "1.25rem" }
								}
							}
							TooltipContent {
								class: "tab-tooltip-content",
								side: ribbon_tooltip_side,
								"Notifications"
							}
						}
						// Example filler buttons
						for i in 0..10 {
							Tooltip {
								TooltipTrigger {
									Button {
										variant: ButtonVariant::Ghost,
										size: ButtonSize::Icon,
										Square { size: "1.25rem" }
									}
								}
								TooltipContent {
									class: "tab-tooltip-content",
									side: ribbon_tooltip_side,
									"Example {i + 1}"
								}
							}
						}
					}
					Separator { class: "ribbon-divider", decorative: true }
					div { class: "ribbon-bottom",
						Tooltip {
							TooltipTrigger {
								SidebarTrigger { class: "sidebar-trigger" }
							}
							TooltipContent {
								class: "tab-tooltip-content",
								side: ribbon_tooltip_side,
								"Toggle sidebar"
							}
						}
						DropdownMenu {
							Tooltip {
								TooltipTrigger {
									DropdownMenuTrigger { class: "dropdown-trigger",
										Vault { size: "1.25rem" }
									}
								}
								TooltipContent {
									class: "tab-tooltip-content",
									side: ribbon_tooltip_side,
									"Vault"
								}
							}
							DropdownMenuContent { class: "dropdown-content",
								DropdownMenuItem::<String> {
									value: "personal".to_string(),
									index: 0usize,
									"Personal"
								}
								DropdownMenuItem::<String> { value: "work".to_string(), index: 1usize, "Work" }
								DropdownMenuItem::<String> {
									value: "university".to_string(),
									index: 2usize,
									"University"
								}
								Separator { decorative: true }
								DropdownMenuItem::<String> {
									value: "manage".to_string(),
									index: 3usize,
									"Manage Vaults"
								}
							}
						}
						Tooltip {
							TooltipTrigger {
								Button {
									variant: ButtonVariant::Ghost,
									size: ButtonSize::Icon,
									User { size: "1.25rem" }
								}
							}
							TooltipContent {
								class: "tab-tooltip-content",
								side: ribbon_tooltip_side,
								"Account"
							}
						}
						Tooltip {
							TooltipTrigger {
								Button {
									variant: ButtonVariant::Ghost,
									size: ButtonSize::Icon,
									onclick: move |_| {
										navigator().push(Route::Settings {});
									},
									Settings { size: "1.25rem" }
								}
							}
							TooltipContent {
								class: "tab-tooltip-content",
								side: ribbon_tooltip_side,
								"Settings"
							}
						}
					}
				}
				SidebarGroup { class: "panel",
					TabContent {
						class: "tab-content",
						value: "home".to_string(),
						index: 0usize,
						"Home tab content"
					}
					TabContent {
						class: "tab-content",
						value: "files".to_string(),
						index: 1usize,
						"Files tab content"
					}
					TabContent {
						class: "tab-content",
						value: "search".to_string(),
						index: 2usize,
						"Search tab content"
					}
					TabContent {
						class: "tab-content",
						value: "bookmarks".to_string(),
						index: 3usize,
						"Bookmarks tab content"
					}
				}
			}
		}
	}
}
