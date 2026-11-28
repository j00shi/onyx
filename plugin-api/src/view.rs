//! View contributions: plugins register whole pages here instead of a fixed `View` enum.

use std::collections::HashMap;

use dioxus::prelude::*;

/// Addresses a registered view. Namespaced like command ids, e.g. "app:home".
pub type ViewId = String;

/// A plugin-contributed page. `render` is a builder because an `Element` can't be made at setup
/// time; it runs in the pane's render scope.
#[derive(Clone)]
pub struct ViewSpec {
	pub title: String,
	pub render: fn() -> Element,
}

/// Registered views by id. Written by [`crate::App::add_view`].
pub static VIEWS: GlobalSignal<HashMap<ViewId, ViewSpec>> = Signal::global(HashMap::new);

/// The "open this view in the focused pane" capability, installed by the frontend at bootstrap so
/// sandboxed plugins can open views via [`crate::App::open_view`] without touching the workspace.
pub static OPEN_VIEW: GlobalSignal<Option<Callback<ViewId>>> = Signal::global(|| None);
