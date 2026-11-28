//! Keyboard hotkeys. A [`KeyChord`] is the keyboard equivalent of a command id: the [`HOTKEYS`]
//! registry maps chords to the [`CommandId`](crate::CommandId) they run. A command may declare a
//! default chord ([`Command::default_hotkey`](crate::Command)), which seeds this registry on
//! registration; later (Step 8) the registry becomes user-editable for rebinding.

use std::collections::HashMap;

use dioxus::prelude::*;

use crate::command::CommandId;

/// A keyboard chord, e.g. Mod+Shift+B.
///
/// `primary` is the platform's primary modifier — Cmd on macOS, Ctrl everywhere else ("Mod" in
/// Obsidian terms) — so a binding is written once and works cross-platform. `key` is normalized to
/// lowercase (the pressed "B" and a Shift+B binding both store `"b"`; the Shift is carried by the
/// flag).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct KeyChord {
	/// The primary modifier: Cmd on macOS, Ctrl elsewhere.
	pub primary: bool,
	pub alt: bool,
	pub shift: bool,
	/// The non-modifier key, lowercased.
	pub key: String,
}

impl KeyChord {
	/// A chord for `key` with no modifiers. Chain [`mod_key`](Self::mod_key) / [`alt`](Self::alt) /
	/// [`shift`](Self::shift) to add them.
	pub fn new(key: impl Into<String>) -> Self {
		Self { primary: false, alt: false, shift: false, key: key.into().to_lowercase() }
	}

	/// Adds the primary modifier (Cmd/Ctrl).
	#[must_use]
	pub const fn mod_key(mut self) -> Self {
		self.primary = true;
		self
	}

	/// Adds Alt/Option.
	#[must_use]
	pub const fn alt(mut self) -> Self {
		self.alt = true;
		self
	}

	/// Adds Shift.
	#[must_use]
	pub const fn shift(mut self) -> Self {
		self.shift = true;
		self
	}

	/// Canonical identifier, e.g. `"mod+shift+b"`. Order is always `mod, alt, shift, key`. This is
	/// the exact string the JS keydown listener builds from an event, so the two must stay in sync.
	pub fn canonical(&self) -> String {
		let mut parts: Vec<&str> = Vec::new();
		if self.primary {
			parts.push("mod");
		}
		if self.alt {
			parts.push("alt");
		}
		if self.shift {
			parts.push("shift");
		}
		parts.push(&self.key);
		parts.join("+")
	}

	/// Human-readable label for display, rendered natively per platform: `"⌘⇧B"` on macOS,
	/// `"Ctrl+Shift+B"` elsewhere.
	#[must_use]
	pub fn display(&self, is_mac: bool) -> String {
		let key = self.key.to_uppercase();
		if is_mac {
			// Native macOS modifier order: ⌥ ⇧ ⌘, then the key.
			let mut s = String::new();
			if self.alt {
				s.push('⌥');
			}
			if self.shift {
				s.push('⇧');
			}
			if self.primary {
				s.push('⌘');
			}
			s.push_str(&key);
			s
		} else {
			let mut parts: Vec<&str> = Vec::new();
			if self.primary {
				parts.push("Ctrl");
			}
			if self.alt {
				parts.push("Alt");
			}
			if self.shift {
				parts.push("Shift");
			}
			parts.push(&key);
			parts.join("+")
		}
	}
}

/// Registered chords mapped to the command they invoke. Seeded from each command's
/// [`Command::default_hotkey`](crate::Command) when it is added; pushed to the global keydown
/// listener by the frontend.
pub static HOTKEYS: GlobalSignal<HashMap<KeyChord, CommandId>> = Signal::global(HashMap::new);

#[cfg(test)]
mod tests {
	use super::KeyChord;

	// `canonical()` must match the string the JS keydown listener builds (order: mod, alt, shift,
	// key; key lowercased), or no hotkey would ever resolve.
	#[test]
	fn canonical_matches_js_contract() {
		assert_eq!(KeyChord::new("P").mod_key().canonical(), "mod+p");
		assert_eq!(KeyChord::new(",").mod_key().canonical(), "mod+,");
		assert_eq!(KeyChord::new("b").mod_key().shift().canonical(), "mod+shift+b");
		assert_eq!(KeyChord::new("B").alt().shift().canonical(), "alt+shift+b");
		assert_eq!(KeyChord::new("x").canonical(), "x");
	}

	#[test]
	fn display_is_platform_native() {
		let chord = KeyChord::new("b").mod_key().shift();
		assert_eq!(chord.display(true), "⇧⌘B");
		assert_eq!(chord.display(false), "Ctrl+Shift+B");
		assert_eq!(KeyChord::new("p").mod_key().display(true), "⌘P");
		assert_eq!(KeyChord::new("p").mod_key().display(false), "Ctrl+P");
	}
}
