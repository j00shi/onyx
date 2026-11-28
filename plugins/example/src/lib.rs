//! Example core plugin.
//!
//! It depends only on `plugin-api`, so it proves a plugin crate can contribute working behavior
//! through the public API alone — without touching `frontend` internals. Copy this crate as the
//! template for real core plugins (`file_explorer`, `markdown_editor`, …).

use std::rc::Rc;

use plugin_api::{App, Command, Plugin};

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
		app.add_command(Command {
			id: "example:open-settings".to_string(),
			name: "Example: open settings".to_string(),
			// Uses only the public `App` capability to trigger the app-shell command.
			callback: Rc::new(move || app.run_command("app:open-settings")),
			default_hotkey: None,
		});
	}
}
