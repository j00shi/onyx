//! Frontend-side plugin glue.
//!
//! Boots the app-shell plugin (privileged built-in commands that may touch frontend internals)
//! together with the core plugin crates. Called once from [`crate::App`]'s `use_hook`.

use std::rc::Rc;

use plugin_api::{run_setup, App, Command, Plugin};

use crate::elements::COMMAND_PALETTE_OPEN;
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
		});

		app.add_command(Command {
			id: "app:open-command-palette".to_string(),
			name: "Open command palette".to_string(),
			callback: Rc::new(|| {
				*COMMAND_PALETTE_OPEN.write() = true;
			}),
		});
	}
}
