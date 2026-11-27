//! The app-facing handle a plugin uses to contribute.

use crate::command::{invoke_command, Command, COMMANDS};

/// Handle passed to a plugin's [`crate::Plugin::setup`].
///
/// It is `Copy` and carries no state today (it delegates to the global registries); the type exists
/// so new contribution methods — and later a per-vault scope — can be added without changing
/// `setup`'s signature.
#[derive(Clone, Copy)]
pub struct App {}

impl App {
	/// Creates a handle. Crate-private so only [`crate::run_setup`] mints one; later this carries
	/// the active vault's scope.
	pub(crate) const fn new() -> Self {
		Self {}
	}

	/// Registers a command so hotkeys, the command palette, ribbon buttons and menu items can
	/// invoke it by id.
	pub fn add_command(&self, command: Command) {
		COMMANDS.write().insert(command);
	}

	/// Runs another registered command by id. This is how a plugin triggers an app-shell action it
	/// cannot call directly.
	pub fn run_command(&self, id: &str) {
		invoke_command(id);
	}
}
