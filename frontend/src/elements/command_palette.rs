//! The command palette: a searchable modal over the registered [`plugin_api::COMMANDS`].
//!
//! Opening it is itself a command (`app:open-command-palette`, registered by the frontend
//! `ShellPlugin`), so the ribbon button and a future hotkey both route through `invoke_command`.
//! It renders `Element`s and reads frontend state, so it lives here rather than in a `plugins/`
//! crate; Step 6 later migrates it into a real plugin once the UI-contribution APIs exist.

use dioxus::prelude::*;

use crate::components::dialog::{Dialog, DialogTitle};
use crate::components::input::Input;
use crate::components::item::{Item, ItemGroup};

/// Whether the command palette modal is open. Set true by `app:open-command-palette`; set false
/// on Escape / outside-click (via the `Dialog`) or when a command is run.
pub static COMMAND_PALETTE_OPEN: GlobalSignal<bool> = Signal::global(|| false);

/// Closes the palette, then runs the command (which may open a view).
fn run_command(id: &str) {
	*COMMAND_PALETTE_OPEN.write() = false;
	plugin_api::invoke_command(id);
}

#[component]
pub fn CommandPalette() -> Element {
	let mut query = use_signal(String::new);
	let mut selected = use_signal(|| 0usize);

	// The palette's state outlives any single open (it lives in this always-mounted component),
	// so reset the field and selection each time it opens.
	use_effect(move || {
		if COMMAND_PALETTE_OPEN() {
			query.set(String::new());
			selected.set(0);
		}
	});

	// Clone `(id, name, hotkey)` out so no `COMMANDS` read borrow is held into the rsx or handlers.
	let q = query().to_lowercase();
	let matches: Vec<(String, String, Option<String>)> = plugin_api::COMMANDS
		.read()
		.iter()
		.filter(|command| command.name.to_lowercase().contains(&q))
		.map(|command| {
			let hotkey = command.default_hotkey.as_ref().map(|chord| chord.display(crate::hotkeys::IS_MAC()));
			(command.id.clone(), command.name.clone(), hotkey)
		})
		.collect();
	let match_count = matches.len();
	let matches_for_key = matches.clone();

	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./command_palette.css") }

		Dialog {
			class: "command-palette",
			open: COMMAND_PALETTE_OPEN(),
			on_open_change: move |open| *COMMAND_PALETTE_OPEN.write() = open,

			// Labels the modal for assistive tech; visually hidden via the scss.
			DialogTitle { class: "command-palette-title", "Command palette" }

			Input {
				class: "command-palette-input",
				value: "{query}",
				placeholder: "Run a command...",
				oninput: move |e: FormEvent| {
					query.set(e.value());
					selected.set(0);
				},
				onkeydown: move |e: KeyboardEvent| {
					match e.key() {
						Key::ArrowDown => {
							e.prevent_default();
							selected.set((selected() + 1).min(match_count.saturating_sub(1)));
						}
						Key::ArrowUp => {
							e.prevent_default();
							selected.set(selected().saturating_sub(1));
						}
						Key::Enter => {
							e.prevent_default();
							if let Some((id, _, _)) = matches_for_key.get(selected()) {
								run_command(id);
							}
						}
						_ => {}
					}
				},
			}

			ItemGroup { class: "command-palette-list",
				for (i, (id, name, hotkey)) in matches.iter().enumerate() {
					Item {
						key: "{id}",
						class: "command-palette-item",
						"data-selected": i == selected(),
						onclick: {
							let id = id.clone();
							move |_| run_command(&id)
						},
						span { class: "command-palette-item-name", "{name}" }
						if let Some(hotkey) = hotkey {
							kbd { class: "command-palette-hint", "{hotkey}" }
						}
					}
				}
			}
		}
	}
}
