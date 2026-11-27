//! The plugin contract and the live plugin registry.

use dioxus::prelude::*;

use crate::api::App;

/// A plugin. Core plugin crates implement this directly; later a WASM/JS bridge type implements it
/// too, so the contract must not assume native Rust.
pub trait Plugin: 'static {
	/// Stable, namespaced id, e.g. `"example"`. Used to enable/disable the plugin later.
	fn id(&self) -> &str;

	/// Runs once when the plugin is enabled (Obsidian's `onload`). Register commands — and later
	/// ribbon items, views, settings and hotkeys — through `app`.
	fn setup(&mut self, app: App);

	/// Runs when the plugin is disabled (Obsidian's `onunload`). Defaults to a no-op; not exercised
	/// yet (nothing disables plugins), but defined now so the contract is complete.
	fn teardown(&mut self, _app: App) {}
}

/// Every enabled plugin, retained so [`Plugin::teardown`] can run later. Written once at startup
/// and never read reactively.
pub static PLUGINS: GlobalSignal<Vec<Box<dyn Plugin>>> = Signal::global(Vec::new);

/// Runs `setup` on each plugin and retains it. Must run inside the Dioxus runtime (it writes global
/// signals), so call it from a component's `use_hook`, not from a platform `launch`.
pub fn run_setup(mut plugins: Vec<Box<dyn Plugin>>) {
	let app = App::new();
	for plugin in &mut plugins {
		plugin.setup(app);
	}
	*PLUGINS.write() = plugins;
}
