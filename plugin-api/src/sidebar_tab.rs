//! Sidebar-tab contributions: plugins register sidebar panels here instead of the sidebar
//! hand-wiring them.

use dioxus::prelude::*;

/// A plugin-contributed sidebar tab. `icon`/`content` are builders because an `Element` can't be
/// made at setup time; they run in the sidebar's render scope.
#[derive(Clone)]
pub struct SidebarTabSpec {
	/// Stable id, used as the tab's value and to match trigger↔panel. Namespaced (e.g.
	/// "example:files") to avoid collisions.
	pub id: String,
	pub icon: fn() -> Element,
	pub label: String,
	pub content: fn() -> Element,
}

/// Contributed sidebar tabs, in registration order. Written by [`crate::App::add_sidebar_tab`].
pub static SIDEBAR_TABS: GlobalSignal<Vec<SidebarTabSpec>> = Signal::global(Vec::new);
