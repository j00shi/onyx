use dioxus::prelude::*;
use dioxus_icons::lucide::{Bell, Bookmark, Calendar, Folder, House, ListTodo, Mail, Search, Settings};

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::sidebar::{
	Sidebar, SidebarCollapsible, SidebarContent, SidebarGroup, SidebarHeader, SidebarInset, SidebarProvider,
	SidebarRail, SidebarSide, SidebarTrigger, SidebarVariant,
};
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};

#[component]
pub fn Frame(children: Element) -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./frame.css") }

		SidebarProvider {
			Sidebar {
				id: "sidebar-right",
				side: SidebarSide::Right,
				variant: SidebarVariant::Floating,
				collapsible: SidebarCollapsible::Icon,
				SidebarHeader { class: "header", SidebarTrigger {} }
				SidebarRail {}
			}
			SidebarProvider {
				Sidebar {
					id: "sidebar-left",
					side: SidebarSide::Left,
					variant: SidebarVariant::Floating,
					collapsible: SidebarCollapsible::Icon,
					Tabs { class: "tabs", default_value: "home".to_string(),
						SidebarHeader { class: "header",
							TabList { class: "tablist",
								TabTrigger {
									class: "tab",
									value: "home".to_string(),
									index: 0usize,
									House { size: "1rem" }
								}
								TabTrigger {
									class: "tab",
									value: "files".to_string(),
									index: 1usize,
									Folder { size: "1rem" }
								}
								TabTrigger {
									class: "tab",
									value: "search".to_string(),
									index: 2usize,
									Search { size: "1rem" }
								}
								TabTrigger {
									class: "tab",
									value: "bookmarks".to_string(),
									index: 3usize,
									Bookmark { size: "1rem" }
								}
							}
						}
						SidebarContent { class: "content",
							SidebarGroup { class: "ribbon",
								SidebarTrigger { class: "sidebar-trigger" }
								Button {
									variant: ButtonVariant::Ghost,
									size: ButtonSize::Icon,
									ListTodo { size: "1.25rem" }
								}
								Button {
									variant: ButtonVariant::Ghost,
									size: ButtonSize::Icon,
									Calendar { size: "1.25rem" }
								}
								Button {
									variant: ButtonVariant::Ghost,
									size: ButtonSize::Icon,
									Mail { size: "1.25rem" }
								}
								Button {
									variant: ButtonVariant::Ghost,
									size: ButtonSize::Icon,
									Bell { size: "1.25rem" }
								}
								Button {
									variant: ButtonVariant::Ghost,
									size: ButtonSize::Icon,
									Settings { size: "1.25rem" }
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
					SidebarRail {}
				}
				SidebarInset { id: "Frame", {children} }
			}
		}
	}
}
