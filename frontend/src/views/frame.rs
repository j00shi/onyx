use dioxus::prelude::*;
use dioxus_icons::lucide::{
	Bell, Bookmark, Calendar, Folder, House, ListTodo, Mail, Search, Settings, Square, User, Vault,
};

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::sidebar::{
	use_sidebar, Sidebar, SidebarCollapsible, SidebarContent, SidebarGroup, SidebarHeader, SidebarInset,
	SidebarProvider, SidebarSide, SidebarTrigger, SidebarVariant,
};
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};

/// Minimal and maximal width for the sidebar
const MIN_WIDTH: f64 = 200.0;
const MAX_WIDTH: f64 = 500.0;
/// Dragging the expanded sidebar narrower than `COLLAPSE_BELOW` collapses it; dragging the
/// collapsed sidebar wider than `EXPAND_ABOVE` re-expands it (gap needed to prevent flickering)
const COLLAPSE_BELOW: f64 = 120.0;
const EXPAND_ABOVE: f64 = 160.0;
/// Visible width of the collapsed edge: `3rem + 1rem - 2px` (see frame.scss). TODO: What?
const ICON_WIDTH: f64 = 62.0;

#[component]
pub fn Frame(children: Element) -> Element {
	let mut width = use_signal(|| 256.0_f64); // Primary sidebar width (px)
	let mut open = use_signal(|| true); // True when sidebar is open
	let mut dragging = use_signal(|| false); // True when handle is being dragged
	let mut start_x = use_signal(|| 0.0_f64); // Pointer x at drag start (px)
	let mut start_width = use_signal(|| 0.0_f64); // Sidebar width at drag start (px)

	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./frame.css") }

		SidebarProvider {
			Sidebar {
				id: "sidebar-right",
				side: SidebarSide::Right,
				variant: SidebarVariant::Floating,
				collapsible: SidebarCollapsible::Icon,
				SidebarHeader { class: "header", SidebarTrigger {} }
			}
			SidebarProvider {
				class: if dragging() { "resizing" } else { "" }, // See reason for this in frame.scss
				style: format!(
					"--dx-sidebar-width: {}px; --dx-sidebar-width-mobile: 18rem; --dx-sidebar-width-icon: 3rem",
					width(),
				), // TODO: There a better way?
				// Control the open/close of the sidebar
				open: Some(open()),
				on_open_change: move |v| open.set(v),
				Sidebar {
					id: "sidebar-left",
					side: SidebarSide::Left,
					variant: SidebarVariant::Floating,
					collapsible: SidebarCollapsible::Icon,
					FrameTabs {}
					div {
						class: "resize-handle",
						onpointerdown: move |e| {
							dragging.set(true);
							start_x.set(e.client_coordinates().x);
							// Anchor to the visible width
							start_width.set(if open() { width() } else { ICON_WIDTH });
						},
					}
				}
				SidebarInset { id: "Frame", {children} }
			}
			if dragging() {
				div {
					class: "resize-overlay",
					onpointermove: move |e| {
						let desired = start_width() + (e.client_coordinates().x - start_x());
						if open() {
							// Drag past the minimum and keep going → collapse.
							if desired < COLLAPSE_BELOW {
								open.set(false);
							} else {
								width.set(desired.clamp(MIN_WIDTH, MAX_WIDTH));
							}
						} else if desired > EXPAND_ABOVE {
							// Drag the collapsed edge back out → expand.
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

#[component]
fn FrameTabs() -> Element {
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
						for i in 0..20 {
							Button {
								variant: ButtonVariant::Ghost,
								size: ButtonSize::Icon,
								title: "Example {i + 1}",
								Square { size: "1.25rem" }
							}
						}
					}
					div { class: "ribbon-bottom",
						SidebarTrigger { class: "sidebar-trigger" }
						Button {
							variant: ButtonVariant::Ghost,
							size: ButtonSize::Icon,
							title: "Account",
							User { size: "1.25rem" }
						}
						Button {
							variant: ButtonVariant::Ghost,
							size: ButtonSize::Icon,
							title: "Vault",
							Vault { size: "1.25rem" }
						}
						Button {
							variant: ButtonVariant::Ghost,
							size: ButtonSize::Icon,
							title: "Settings",
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
