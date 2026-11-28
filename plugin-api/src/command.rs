//! The command registry. A command is the atomic unit of action: hotkeys, the command palette,
//! ribbon buttons and menu items all reference a command by id.

use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;

use crate::hotkey::KeyChord;

/// Identifier for a command, namespaced like `"app:open-settings"` or `"example:hello"`.
pub type CommandId = String;

/// One registered command.
#[derive(Clone)]
pub struct Command {
	/// Stable, namespaced id used to invoke the command.
	pub id: CommandId,
	/// Human-readable label shown in the command palette.
	pub name: String,
	/// Runs when the command is invoked. A plain `Rc<dyn Fn()>` so it stays runtime-agnostic — a
	/// future JS plugin's callback is a trampoline of the same type.
	pub callback: Rc<dyn Fn()>,
	/// Optional default hotkey. When set, registering the command seeds [`crate::HOTKEYS`].
	pub default_hotkey: Option<KeyChord>,
}

/// All registered commands, keyed by id.
#[derive(Default)]
pub struct CommandRegistry {
	by_id: HashMap<CommandId, Command>,
}

impl CommandRegistry {
	/// Adds a command, replacing any existing one with the same id.
	pub fn insert(&mut self, command: Command) {
		self.by_id.insert(command.id.clone(), command);
	}

	/// Looks up a command by id.
	pub fn get(&self, id: &str) -> Option<&Command> {
		self.by_id.get(id)
	}

	/// Iterates every registered command (e.g. for the command palette).
	pub fn iter(&self) -> impl Iterator<Item = &Command> {
		self.by_id.values()
	}
}

/// The live command registry. Written once at startup (and later as plugins load/unload); read
/// reactively by the future command palette.
pub static COMMANDS: GlobalSignal<CommandRegistry> = Signal::global(CommandRegistry::default);

/// Looks up a command by id and runs it. Safe to call from any UI event handler; an unknown id is
/// a silent no-op.
pub fn invoke_command(id: &str) {
	// Clone the callback out and drop the read borrow before calling, so the callback may itself
	// touch `COMMANDS` without a double borrow.
	let callback = COMMANDS.read().get(id).map(|command| command.callback.clone());
	if let Some(callback) = callback {
		callback();
	}
}
