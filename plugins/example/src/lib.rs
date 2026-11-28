//! Example core plugin.
//!
//! It depends only on `plugin-api`, so it proves a plugin crate can contribute working behavior
//! through the public API alone — without touching `frontend` internals. Copy this crate as the
//! template for real core plugins (`file_explorer`, `markdown_editor`, …).

use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_icons::lucide::{Bookmark, Folder, Puzzle, Search};
use plugin_api::{App, Command, Plugin, RibbonItemSpec, RibbonSection, SidebarTabSpec, ViewSpec};

/// A minimal demo plugin.
#[derive(Default)]
pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
	// The trait returns `&str` so dynamic (JS) plugins can hold their id as a field; this impl
	// returns a literal.
	#[allow(clippy::unnecessary_literal_bound)]
	fn id(&self) -> &str {
		"example"
	}

	fn setup(&mut self, app: App) {
		// Proves a plugin crate (depending only on `plugin-api`) can contribute a whole page.
		app.add_view(
			"example:view".to_string(),
			ViewSpec {
				title: "Example".to_string(),
				render: || {
					rsx! {
						div { class: "example-view",
							h1 { "Example plugin view" }
							p { "This whole page is rendered by the `example` plugin crate — it depends only on plugin-api." }
						}
					}
				},
			},
		);

		app.add_command(Command {
			id: "example:open-view".to_string(),
			name: "Example: open view".to_string(),
			// Opens the plugin's own view through the public `App` capability.
			callback: Rc::new(move || app.open_view("example:view".to_string())),
			default_hotkey: None,
		});

		app.add_ribbon_item(RibbonItemSpec {
			icon: || rsx! { Puzzle { size: "1.25rem" } },
			tooltip: "Example plugin".to_string(),
			section: RibbonSection::Top,
			command: "example:open-view".to_string(),
		});

		// Proves a plugin crate can also contribute sidebar panels. Real plugins (file explorer,
		// search, bookmarks) will replace these.
		app.add_sidebar_tab(SidebarTabSpec {
			id: "example:files".to_string(),
			icon: || rsx! { Folder { size: "1rem" } },
			label: "Files".to_string(),
			content: || rsx! { p { "Files tab — contributed by the example plugin." } },
		});

		app.add_sidebar_tab(SidebarTabSpec {
			id: "example:search".to_string(),
			icon: || rsx! { Search { size: "1rem" } },
			label: "Search".to_string(),
			content: || rsx! { p { "Search tab — contributed by the example plugin." } },
		});

		app.add_sidebar_tab(SidebarTabSpec {
			id: "example:bookmarks".to_string(),
			icon: || rsx! { Bookmark { size: "1rem" } },
			label: "Bookmarks".to_string(),
			content: || rsx! { p { "Bookmarks tab — contributed by the example plugin." } },
		});
	}
}
