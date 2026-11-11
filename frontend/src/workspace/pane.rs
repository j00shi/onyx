use dioxus::prelude::*;
use dioxus_icons::lucide::{Plus, X};

use super::model::{Axis, Node, NodeId, View};
use super::WORKSPACE;
use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::views::{Home, NewTab, Settings};

/// Root of the content workspace: renders the pane tree from the root node down.
#[component]
pub fn WorkspaceView() -> Element {
	let root = WORKSPACE.read().root();
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./pane.css") }
		PaneNode { id: root }
	}
}

/// Renders a single node: a [`LeafPane`] for a leaf, or a flexbox split that recurses into its
/// children. Splits aren't created until a later step, so only the leaf arm runs for now.
#[component]
fn PaneNode(id: NodeId) -> Element {
	let node = WORKSPACE.read().node(id).cloned();
	match node {
		Some(Node::Leaf { .. }) => rsx! {
			LeafPane { id }
		},
		Some(Node::Split { axis, children, fractions, .. }) => {
			let flow = match axis {
				Axis::Horizontal => "row",
				Axis::Vertical => "column",
			};
			rsx! {
				div { class: "pane-split", style: "flex-direction: {flow};",
					for (child , frac) in children.into_iter().zip(fractions) {
						div {
							key: "{child.0}",
							class: "pane-split-child",
							style: "flex: {frac} 1 0;",
							PaneNode { id: child }
						}
					}
				}
			}
		},
		None => rsx! {},
	}
}

/// A single tab group: its own tab bar plus the active tab's content below it.
#[component]
fn LeafPane(id: NodeId) -> Element {
	let node = WORKSPACE.read().node(id).cloned();
	let Some(Node::Leaf { tabs, active, .. }) = node else {
		return rsx! {};
	};

	let active_view = tabs.get(active).map(|t| t.view);

	rsx! {
		div { class: "leaf-pane",
			div { class: "leaf-tabbar", role: "tablist",
				div { class: "leaf-tablist",
					for (i , tab) in tabs.iter().enumerate() {
						// The tab is the clickable pill; the close button lives inside it so the whole
						// pill (not just the label) switches tabs. It's a div, not a button, so the
						// close button can nest inside it.
						div {
							key: "{tab.id.0}",
							class: "leaf-tab",
							role: "tab",
							tabindex: 0,
							"aria-selected": if i == active { "true" } else { "false" },
							onclick: move |_| WORKSPACE.write().set_active(id, i),
							onkeydown: move |e: KeyboardEvent| {
								let activate = match e.key() {
									Key::Enter => true,
									Key::Character(c) => c == " ",
									_ => false,
								};
								if activate {
									e.prevent_default();
									WORKSPACE.write().set_active(id, i);
								}
							},
							span { class: "leaf-tab-title", "{tab.view.title()}" }
							button {
								class: "leaf-tab-close",
								"aria-label": "Close tab",
								onclick: move |e: MouseEvent| {
									e.stop_propagation();
									WORKSPACE.write().close_tab(id, i);
								},
								X { size: "1rem" }
							}
						}
					}
				}
				Button {
					class: "leaf-tab-add",
					variant: ButtonVariant::Ghost,
					size: ButtonSize::IconSm,
					onclick: move |_| WORKSPACE.write().add_tab(id, View::NewTab),
					Plus { size: "1rem" }
				}
			}
			div { class: "leaf-content",
				if let Some(view) = active_view {
					{render_view(view)}
				}
			}
		}
	}
}

/// Map a [`View`] to its content element.
fn render_view(view: View) -> Element {
	match view {
		View::Home => rsx! {
			Home {}
		},
		View::NewTab => rsx! {
			NewTab {}
		},
		View::Settings => rsx! {
			Settings {}
		},
	}
}
