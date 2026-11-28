//! The app-facing handle a plugin uses to contribute.

use dioxus::prelude::ReadableExt;

use crate::command::{invoke_command, Command, COMMANDS};
use crate::hotkey::HOTKEYS;
use crate::ribbon::{RibbonItemSpec, RIBBON_ITEMS};
use crate::sidebar_tab::{SidebarTabSpec, SIDEBAR_TABS};
use crate::view::{ViewId, ViewSpec, OPEN_VIEW, VIEWS};

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
	/// invoke it by id. A `default_hotkey` on the command seeds [`crate::HOTKEYS`].
	pub fn add_command(&self, command: Command) {
		if let Some(chord) = command.default_hotkey.clone() {
			HOTKEYS.write().insert(chord, command.id.clone());
		}
		COMMANDS.write().insert(command);
	}

	/// Runs another registered command by id. This is how a plugin triggers an app-shell action it
	/// cannot call directly.
	pub fn run_command(&self, id: &str) {
		invoke_command(id);
	}

	/// Contributes a button to the ribbon. Its `command` is invoked when the button is clicked.
	pub fn add_ribbon_item(&self, item: RibbonItemSpec) {
		RIBBON_ITEMS.write().push(item);
	}

	/// Contributes a tab to the primary sidebar. Its `content` builder renders when the tab is active.
	pub fn add_sidebar_tab(&self, tab: SidebarTabSpec) {
		SIDEBAR_TABS.write().push(tab);
	}

	/// Registers a view (a whole page) under `id`, so it can be opened in a pane by id.
	pub fn add_view(&self, id: ViewId, view: ViewSpec) {
		VIEWS.write().insert(id, view);
	}

	/// Opens a registered view in the focused pane, via the capability the frontend installed.
	pub fn open_view(&self, id: ViewId) {
		if let Some(open) = *OPEN_VIEW.read() {
			open.call(id);
		}
	}
}
