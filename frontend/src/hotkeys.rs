//! Global keyboard hotkeys.
//!
//! One `window` keydown listener (capture phase) resolves a keypress to a registered
//! [`plugin_api::HOTKEYS`] binding and runs its command. It lives here (frontend app-shell glue)
//! rather than in `plugin-api`, mirroring how [`crate::elements::CommandPalette`] consumes the
//! `plugin-api` registries.
//!
//! The match happens in JS because `preventDefault` must be called synchronously inside the DOM
//! handler — we can't round-trip to Rust first. So Rust pushes the current binding set to JS
//! (whenever [`plugin_api::HOTKEYS`] changes); JS matches, suppresses the event, and sends back the
//! command id for Rust to invoke. Running in the *capture* phase and calling
//! `stopImmediatePropagation` on a match lets our bindings supersede the vendored sidebar's own
//! `window` keydown listener (which otherwise toggles both sidebars on Cmd+B).

use dioxus::prelude::*;

/// Whether we're on macOS. Detected once; used only to render native hotkey glyphs (⌘ vs Ctrl).
pub static IS_MAC: GlobalSignal<bool> = Signal::global(|| false);

/// The capture-phase keydown listener. Keeps a `canonical -> command_id` map (fed from Rust),
/// suppresses matched chords, and posts the matched id back.
const LISTENER_JS: &str = r"
	let binds = new Map();
	function canon(e) {
		let p = [];
		if (e.metaKey || e.ctrlKey) p.push('mod');
		if (e.altKey) p.push('alt');
		if (e.shiftKey) p.push('shift');
		p.push(e.key.toLowerCase());
		return p.join('+');
	}
	function handler(e) {
		const id = binds.get(canon(e));
		if (id) {
			e.preventDefault();
			e.stopImmediatePropagation();
			dioxus.send(id);
		}
	}
	window.addEventListener('keydown', handler, true);
	while (true) {
		binds = new Map(await dioxus.recv());
	}
";

/// Mounts the global hotkey listener. Call once from a component that lives for the app's lifetime
/// (`Frame`); no teardown is needed because it never unmounts.
pub fn use_global_hotkeys() {
	// Detect the platform once for native hotkey-glyph rendering.
	use_hook(|| {
		spawn(async move {
			let mut eval =
				document::eval("dioxus.send(/mac|iphone|ipad/i.test(navigator.platform || navigator.userAgent));");
			if let Ok(is_mac) = eval.recv::<bool>().await {
				*IS_MAC.write() = is_mac;
			}
		});
	});

	// Set up the keydown listener once and drain matched command ids forever. `Eval` is `Copy`, so
	// the same handle is also used by the push effect below.
	let eval = use_hook(|| {
		let mut eval = document::eval(LISTENER_JS);
		spawn(async move {
			while let Ok(id) = eval.recv::<String>().await {
				plugin_api::invoke_command(&id);
			}
		});
		eval
	});

	// Push the current bindings as `[canonical, id]` pairs whenever the registry changes.
	use_effect(move || {
		let pairs: Vec<(String, String)> =
			plugin_api::HOTKEYS.read().iter().map(|(chord, id)| (chord.canonical(), id.clone())).collect();
		let _ = eval.send(pairs);
	});
}
