use dioxus::prelude::*;

use dioxus_icons::lucide::Wrench;
use dioxus_primitives::ContentSide;

use crate::components::{
	separator::Separator,
	sidebar::{
		Sidebar, SidebarCollapsible, SidebarContent, SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuItem,
		SidebarProvider, SidebarRail, SidebarSide, SidebarTrigger, SidebarVariant,
	},
	tooltip::{Tooltip, TooltipContent, TooltipTrigger},
};

#[derive(Clone, PartialEq)]
struct Extension {
	name: &'static str,
	url: &'static str,
}

const EXTENSIONS: &[Extension] = &[
	Extension { name: "Files", url: "#" },
	Extension { name: "Search", url: "#" },
	Extension { name: "Marketplace", url: "#" },
];

#[component]
pub fn Frame(children: Element) -> Element {
	rsx! {
		// document::Link { rel: "stylesheet", href: asset!("./frame.css") }
		div { id: "Frame",
			SidebarProvider {
				Sidebar {
					side: SidebarSide::Left,
					variant: SidebarVariant::Floating,
					collapsible: SidebarCollapsible::Icon,
					div { class: "primary-sidebar",
						SidebarHeader { SidebarTrigger {} }
						SidebarContent {
							NavExtensions { extensions: EXTENSIONS }
						}
						SidebarRail {}
					}
				}
				SidebarInset {
					div { class: "sidebar-header",
						div { class: "tab-bar" }
						Separator {}
					}
					div { class: "sidebar-content", {children} }
				}
			}
		}
	}
}

#[component]
fn NavExtensions(extensions: &'static [Extension]) -> Element {
	rsx! {
		SidebarMenu {
			for extension in extensions.iter() {
				SidebarMenuItem { key: "{extension.name}",
					Tooltip {
						TooltipTrigger { Wrench {} }
						TooltipContent { side: ContentSide::Right,
							span { {extension.name} }
						}
					}
				}
			}
		}
	}
}
