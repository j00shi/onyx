//! Frontend-side plugin glue.
//!
//! Boots the app-shell plugin (privileged built-in commands that may touch frontend internals)
//! together with the core plugin crates. Called once from [`crate::App`]'s `use_hook`.

use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_icons::lucide::{Search, Settings};
use plugin_api::{run_setup, App, Command, KeyChord, Plugin, RibbonItemSpec, RibbonSection};

use crate::elements::{COMMAND_PALETTE_OPEN, PRIMARY_SIDEBAR_TOGGLE, SECONDARY_SIDEBAR_TOGGLE};
use crate::workspace::model::View;
use crate::workspace::WORKSPACE;

/// Runs every built-in and core plugin's `setup`. Must run inside the Dioxus runtime (it writes
/// global signals), so it is called from `App`'s `use_hook`.
pub fn bootstrap() {
	let plugins: Vec<Box<dyn Plugin>> = vec![Box::new(ShellPlugin), Box::new(example::ExamplePlugin)];
	run_setup(plugins);
}

/// The app shell expressed as a privileged built-in plugin: it registers core commands and, unlike
/// the `plugins/` crates, may reach into frontend state directly.
struct ShellPlugin;

impl Plugin for ShellPlugin {
	#[allow(clippy::unnecessary_literal_bound)]
	fn id(&self) -> &str {
		"app"
	}

	fn setup(&mut self, app: App) {
		app.add_command(Command {
			id: "app:open-settings".to_string(),
			name: "Open settings".to_string(),
			callback: Rc::new(|| {
				WORKSPACE.write().open_view(View::Settings);
			}),
			default_hotkey: Some(KeyChord::new(",").mod_key()),
		});

		app.add_command(Command {
			id: "app:open-command-palette".to_string(),
			name: "Open command palette".to_string(),
			callback: Rc::new(|| {
				*COMMAND_PALETTE_OPEN.write() = true;
			}),
			default_hotkey: Some(KeyChord::new("p").mod_key()),
		});

		app.add_command(Command {
			id: "app:toggle-primary-sidebar".to_string(),
			name: "Toggle primary sidebar".to_string(),
			callback: Rc::new(|| {
				if let Some(toggle) = *PRIMARY_SIDEBAR_TOGGLE.read() {
					toggle.call(());
				}
			}),
			default_hotkey: Some(KeyChord::new("b").mod_key()),
		});

		app.add_command(Command {
			id: "app:toggle-secondary-sidebar".to_string(),
			name: "Toggle secondary sidebar".to_string(),
			callback: Rc::new(|| {
				if let Some(toggle) = *SECONDARY_SIDEBAR_TOGGLE.read() {
					toggle.call(());
				}
			}),
			default_hotkey: Some(KeyChord::new("b").mod_key().shift()),
		});

		app.add_ribbon_item(RibbonItemSpec {
			icon: || rsx! { Search { size: "1.25rem" } },
			tooltip: "Command palette".to_string(),
			section: RibbonSection::Top,
			command: "app:open-command-palette".to_string(),
		});

		app.add_ribbon_item(RibbonItemSpec {
			icon: || rsx! { Settings { size: "1.25rem" } },
			tooltip: "Settings".to_string(),
			section: RibbonSection::Bottom,
			command: "app:open-settings".to_string(),
		});
	}
}
