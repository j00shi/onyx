use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowLeft, ArrowRight};
use dioxus_primitives::ContentSide;

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::sidebar::{use_sidebar, SidebarContent, SidebarGroup, SidebarHeader};
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};
use crate::components::tooltip::{Tooltip, TooltipContent, TooltipTrigger};
use crate::views::{CUSTOM_TITLEBAR, MOBILE_REACH_LAYOUT};

/// One tab in a sidebar. The plugin API (later) hands over a list of these; for now the primary
/// sidebar builds them by hand.
#[derive(Clone, PartialEq)]
pub struct SidebarTab {
	/// Stable id, used as the tab's value and to match trigger↔panel. Plugin tabs should namespace
	/// this (e.g. `"myplugin:files"`) to avoid collisions.
	pub id: String,
	/// Icon shown in the tab bar, e.g. `rsx! { House { size: "1rem" } }`. Rendered as-is.
	pub icon: Element,
	/// Tooltip shown on hover; also the accessible label.
	pub label: String,
	/// Panel content rendered when this tab is active.
	pub content: Element,
}

/// Sidebar tab bar and panels. Takes a list of [`SidebarTab`]s and renders their triggers in the
/// header and their panels in the content area, deriving the tab index from the list order.
///
/// `children` are dropped into the content area before the panels — the primary sidebar passes its
/// [`Ribbon`](super::Ribbon) there.
#[component]
pub fn SidebarTabs(tabs: Vec<SidebarTab>, #[props(default)] default: Option<String>, children: Element) -> Element {
	let ctx = use_sidebar();

	// Tabs sit at the top of the sidebar, so tooltips normally drop below them. In the mobile
	// reach layout the tab bar moves to the bottom, so they flip up to stay on screen.
	let tab_tooltip_side =
		if MOBILE_REACH_LAYOUT() && (ctx.is_mobile)() { ContentSide::Top } else { ContentSide::Bottom };

	// Open on the first tab unless the caller picks one.
	let default_value = default.or_else(|| tabs.first().map(|t| t.id.clone())).unwrap_or_default();

	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./sidebar_tabs.css") }

		// The tab bar is a horizontal row, so Left/Right (not the vertical default) navigate it.
		// Tabs use a roving tabindex: Tab reaches the active tab, arrows move between them, Enter
		// activates.
		Tabs { class: "sidebar-body", horizontal: true, default_value,
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
						for (i, tab) in tabs.iter().enumerate() {
							Tooltip { key: "{tab.id}", class: "tab-tooltip",
								// `tabindex: -1` keeps the wrapper out of the tab order so focus lands on
								// the tab button itself — otherwise its roving tabindex is bypassed and
								// Enter/Space never reaches the button.
								TooltipTrigger { class: "tab-slot", tabindex: "-1",
									TabTrigger {
										class: "tab",
										value: tab.id.clone(),
										index: i,
										{tab.icon.clone()}
									}
								}
								TooltipContent {
									class: "tooltip-content",
									side: tab_tooltip_side,
									{tab.label.clone()}
								}
							}
						}
					}
				}
			}
			SidebarContent { class: "sidebar-content",
				{children}
				SidebarGroup { class: "panel",
					for (i, tab) in tabs.iter().enumerate() {
						TabContent {
							key: "{tab.id}",
							class: "panel-tab-content",
							value: tab.id.clone(),
							index: i,
							{tab.content.clone()}
						}
					}
				}
			}
		}
	}
}
