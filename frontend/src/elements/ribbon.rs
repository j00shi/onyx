use dioxus::prelude::*;
use dioxus_primitives::ContentSide;

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger};
use crate::components::separator::Separator;
use crate::components::sidebar::{SidebarGroup, SidebarTrigger};
use crate::components::tooltip::{Tooltip, TooltipContent, TooltipTrigger};
use crate::views::SIDEBAR_SWAPPED;

/// One button in the ribbon. The plugin API (later) hands over a list of these; for now the
/// primary sidebar builds them by hand.
#[derive(Clone, PartialEq)]
pub struct RibbonItem {
	/// Icon shown on the button, e.g. `rsx! { Mail { size: "1.25rem" } }` or an `img` for a custom
	/// image. Rendered as-is.
	pub icon: Element,
	/// Text shown when hovering the button.
	pub tooltip: String,
	/// Whether the button runs an action or opens a dropdown.
	pub kind: RibbonItemKind,
}

/// A ribbon button either fires an action or opens a dropdown menu.
#[derive(Clone, PartialEq)]
pub enum RibbonItemKind {
	/// Runs when the button is clicked.
	Action(EventHandler<()>),
	/// Opens a dropdown with these rows.
	Menu(Vec<RibbonMenuItem>),
}

/// One row inside a ribbon dropdown.
#[derive(Clone, PartialEq)]
pub struct RibbonMenuItem {
	/// Visible text of the row.
	pub label: String,
	/// Runs when the row is picked.
	pub on_select: EventHandler<()>,
	/// Draw a divider above this row.
	pub separator_before: bool,
}

/// Vertical ribbon of icon buttons, split into a `top` and `bottom` group with a divider between.
/// The sidebar toggle is built in and pinned to the very bottom. Both groups scroll independently.
///
/// `top` reads top-to-bottom. `bottom` also reads top-to-bottom; it's reversed internally because
/// `.ribbon-bottom` is `column-reverse` so its contents grow upward from the pinned toggle.
#[component]
pub fn Ribbon(top: Vec<RibbonItem>, bottom: Vec<RibbonItem>) -> Element {
	let tooltip_side = if SIDEBAR_SWAPPED() { ContentSide::Left } else { ContentSide::Right };

	// The dropdowns float in a fixed, non-clipped layer anchored to their trigger. Only one is open
	// at a time, keyed by the entry's unique id, so a ribbon scroll can close whichever is open.
	let mut open_menu = use_signal(|| None::<String>);

	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./ribbon.css") }

		SidebarGroup { class: "ribbon",
			div {
				class: "ribbon-top",
				onscroll: move |_| open_menu.set(None),
				for (i, item) in top.iter().enumerate() {
					RibbonEntry {
						key: "top-{i}",
						id: "top-{i}",
						item: item.clone(),
						side: tooltip_side,
						open_menu,
					}
				}
			}
			Separator { class: "ribbon-divider", decorative: true }
			div {
				class: "ribbon-bottom",
				onscroll: move |_| open_menu.set(None),
				// First DOM child, so `column-reverse` places it at the very bottom.
				Tooltip {
					// `tabindex: -1` keeps the wrapper out of the tab order so only the inner control
					// is a tab stop — otherwise keyboard users land on the trigger before the button.
					TooltipTrigger { tabindex: "-1",
						SidebarTrigger { class: "ribbon-button" }
					}
					TooltipContent {
						class: "tooltip-content",
						side: tooltip_side,
						"Toggle sidebar"
					}
				}
				// Reversed so the caller's top-to-bottom order survives `column-reverse`.
				for (i, item) in bottom.iter().enumerate().rev() {
					RibbonEntry {
						key: "bottom-{i}",
						id: "bottom-{i}",
						item: item.clone(),
						side: tooltip_side,
						open_menu,
					}
				}
			}
		}
	}
}

/// Renders a single [`RibbonItem`]: either a plain button or a dropdown.
#[component]
fn RibbonEntry(id: String, item: RibbonItem, side: ContentSide, open_menu: Signal<Option<String>>) -> Element {
	match &item.kind {
		RibbonItemKind::Action(on_click) => {
			let on_click = *on_click;
			rsx! {
				Tooltip {
					TooltipTrigger { tabindex: "-1",
						Button {
							variant: ButtonVariant::Ghost,
							size: ButtonSize::Icon,
							class: "ribbon-button",
							onclick: move |_| on_click.call(()),
							{item.icon.clone()}
						}
					}
					TooltipContent {
						class: "tooltip-content",
						side,
						{item.tooltip.clone()}
					}
				}
			}
		},
		RibbonItemKind::Menu(entries) => {
			// Each dropdown anchors its floating menu to its own trigger via a unique anchor name.
			let anchor = format!("--dd-{id}");
			let is_open = open_menu() == Some(id.clone());
			rsx! {
				DropdownMenu {
					open: Some(is_open),
					on_open_change: move |v| open_menu.set(if v { Some(id.clone()) } else { None }),
					Tooltip {
						TooltipTrigger { tabindex: "-1",
							DropdownMenuTrigger {
								class: "ribbon-button",
								style: "anchor-name: {anchor}",
								{item.icon.clone()}
							}
						}
						TooltipContent {
							class: "tooltip-content",
							side,
							{item.tooltip.clone()}
						}
					}
					DropdownMenuContent {
						class: "ribbon-dropdown-content",
						style: "position-anchor: {anchor}",
						for (i, entry) in entries.iter().enumerate() {
							if entry.separator_before {
								Separator { decorative: true }
							}
							{
								let on_select = entry.on_select;
								rsx! {
									DropdownMenuItem::<String> {
										key: "{i}",
										value: entry.label.clone(),
										index: i,
										class: "ribbon-dropdown-item",
										on_select: move |_| on_select.call(()),
										{entry.label.clone()}
									}
								}
							}
						}
					}
				}
			}
		},
	}
}
