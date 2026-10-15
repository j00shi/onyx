use crate::custom::Workspace;
use dioxus::prelude::*;

/// Full-screen Obsidian-compatible workspace page.
#[component]
pub fn Vault() -> Element {
	rsx! {
		Workspace {}
	}
}
