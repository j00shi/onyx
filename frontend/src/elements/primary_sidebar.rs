use dioxus::prelude::*;
use dioxus_icons::lucide::{
	Bell, Bookmark, Calendar, Folder, House, ListTodo, Mail, Search, Settings, Square, User, Vault,
};

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger};
use crate::components::separator::Separator;
use crate::components::sidebar::{
	use_sidebar, Sidebar, SidebarCollapsible, SidebarContent, SidebarGroup, SidebarHeader, SidebarInset,
	SidebarProvider, SidebarSide, SidebarTrigger, SidebarVariant,
};
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};
use crate::views::{SIDEBAR_SWAPPED, SIDEBAR_VARIANT};
use crate::Route;

/// Minimal and maximal width for the sidebar
const MIN_WIDTH: f64 = 200.0;
const MAX_WIDTH: f64 = 500.0;
/// Dragging the expanded sidebar narrower than `COLLAPSE_BELOW` collapses it; dragging the
/// collapsed sidebar wider than `EXPAND_ABOVE` re-expands it (gap needed to prevent flickering)
const COLLAPSE_BELOW: f64 = 120.0;
const EXPAND_ABOVE: f64 = 160.0;
/// Visible width of the collapsed edge. Floating adds 1rem of container
/// padding (`3rem + 1rem - 2px`); the docked sidebar is just the icon width (`3rem`).
const ICON_WIDTH_FLOATING: f64 = 62.0;
const ICON_WIDTH_SIDEBAR: f64 = 48.0;

/// The primary (left) sidebar: its own `SidebarProvider`, the resizable primary sidebar, and
/// the main content inset. `children` are rendered inside the inset so the surrounding frame
/// keeps ownership of its route `Outlet`.
#[component]
pub fn PrimarySidebar(children: Element) -> Element {
	let mut width = use_signal(|| 256.0_f64); // Primary sidebar width (px)
	let mut open = use_signal(|| true); // True when sidebar is open
	let mut dragging = use_signal(|| false); // True when handle is being dragged
	let mut start_x = use_signal(|| 0.0_f64); // Pointer x at drag start (px)
	let mut start_width = use_signal(|| 0.0_f64); // Sidebar width at drag start (px)

	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./primary_sidebar.css") }

		div { class: "primary-sidebar",
			SidebarProvider {
				class: if dragging() { "resizing" } else { "" }, // See reason for this in primary_sidebar.scss
				style: format!(
					"--dx-sidebar-width: {}px; --dx-sidebar-width-mobile: 18rem; --dx-sidebar-width-icon: 3rem",
					width(),
				),
				open: Some(open()),
				on_open_change: move |v| open.set(v),
				Sidebar {
					class: "sidebar",
					side: if SIDEBAR_SWAPPED() { SidebarSide::Right } else { SidebarSide::Left },
					variant: SIDEBAR_VARIANT(),
					collapsible: SidebarCollapsible::Icon,
					SidebarTabs {}
					div {
						class: "resize-handle",
						onpointerdown: move |e| {
							dragging.set(true);
							start_x.set(e.client_coordinates().x);
							// Anchor to the visible collapsed width (differs per variant)
							let icon_width = if SIDEBAR_VARIANT() == SidebarVariant::Floating {
								ICON_WIDTH_FLOATING
							} else {
								ICON_WIDTH_SIDEBAR
							};
							start_width.set(if open() { width() } else { icon_width });
						},
					}
				}
				SidebarInset { {children} }
			}
			if dragging() {
				div {
					class: "resize-overlay",
					onpointermove: move |e| {
						// When swapped the primary sidebar sits on the right, so dragging left widens it
						let direction = if SIDEBAR_SWAPPED() { -1.0_f64 } else { 1.0 };
						let desired = direction
							.mul_add(e.client_coordinates().x - start_x(), start_width()); // Drag past the minimum and keep going → collapse.
						if open() {
							if desired < COLLAPSE_BELOW {
								open.set(false);
							} else {
								width.set(desired.clamp(MIN_WIDTH, MAX_WIDTH));
							}
						} else if desired > EXPAND_ABOVE {
							open.set(true);
							width.set(desired.clamp(MIN_WIDTH, MAX_WIDTH));
						}
					},
					onpointerup: move |_| dragging.set(false),
					onpointercancel: move |_| dragging.set(false),
				}
			}
		}
	}
}

/// Tab list, ribbon, and panels. Split out so it renders inside `SidebarProvider` and can read
/// the sidebar context via `use_sidebar`.
#[component]
fn SidebarTabs() -> Element {
	let ctx = use_sidebar();

	rsx! {
		Tabs { class: "tabs", default_value: "home".to_string(),
			SidebarHeader { class: "header",
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
						TabTrigger {
							class: "tab",
							value: "home".to_string(),
							index: 0usize,
							title: "Home",
							House { size: "1rem" }
						}
						TabTrigger {
							class: "tab",
							value: "files".to_string(),
							index: 1usize,
							title: "Files",
							Folder { size: "1rem" }
						}
						TabTrigger {
							class: "tab",
							value: "search".to_string(),
							index: 2usize,
							title: "Search",
							Search { size: "1rem" }
						}
						TabTrigger {
							class: "tab",
							value: "bookmarks".to_string(),
							index: 3usize,
							title: "Bookmarks",
							Bookmark { size: "1rem" }
						}
					}
				}
			}
			SidebarContent { class: "content",
				SidebarGroup { class: "ribbon",
					div { class: "ribbon-top",
						Button {
							variant: ButtonVariant::Ghost,
							size: ButtonSize::Icon,
							title: "Tasks",
							ListTodo { size: "1.25rem" }
						}
						Button {
							variant: ButtonVariant::Ghost,
							size: ButtonSize::Icon,
							title: "Calendar",
							Calendar { size: "1.25rem" }
						}
						Button {
							variant: ButtonVariant::Ghost,
							size: ButtonSize::Icon,
							title: "Mail",
							Mail { size: "1.25rem" }
						}
						Button {
							variant: ButtonVariant::Ghost,
							size: ButtonSize::Icon,
							title: "Notifications",
							Bell { size: "1.25rem" }
						}
						// Example filler buttons
						for i in 0..10 {
							Button {
								variant: ButtonVariant::Ghost,
								size: ButtonSize::Icon,
								title: "Example {i + 1}",
								Square { size: "1.25rem" }
							}
						}
					}
					Separator { class: "ribbon-divider", decorative: true }
					div { class: "ribbon-bottom",
						SidebarTrigger { class: "sidebar-trigger" }
						DropdownMenu {
							DropdownMenuTrigger { class: "dropdown-trigger", title: "Vault",
								Vault { size: "1.25rem" }
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
						Button {
							variant: ButtonVariant::Ghost,
							size: ButtonSize::Icon,
							title: "Account",
							User { size: "1.25rem" }
						}
						Button {
							variant: ButtonVariant::Ghost,
							size: ButtonSize::Icon,
							title: "Settings",
							onclick: move |_| {
								navigator().push(Route::Settings {});
							},
							Settings { size: "1.25rem" }
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
