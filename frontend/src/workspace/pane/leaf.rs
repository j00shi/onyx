//! Tab groups: a pane's own tab bar (with drag-to-reorder and drag-out) plus the active tab's
//! content, which doubles as an edge/center drop target while a tab is being dragged.

use dioxus::prelude::*;
use dioxus_icons::lucide::{Columns2, Plus, X};

use super::super::model::{Edge, Node, NodeId, View};
use super::super::WORKSPACE;
use super::{
	commit_drag, drop_zone, set_hint, set_over, zone_rect, DropHint, DropTarget, TabDrag, DRAG, DRAG_THRESHOLD,
};
use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger};
use crate::views::{Home, NewTab, Settings};

/// A single tab group: its own tab bar plus the active tab's content below it.
#[component]
pub(super) fn LeafPane(id: NodeId) -> Element {
	let node = WORKSPACE.read().node(id).cloned();
	let Some(Node::Leaf { tabs, active, .. }) = node else {
		return rsx! {};
	};

	let active_view = tabs.get(active).map(|t| t.view);

	// A pending tab press: (index, start x, start y). Promoted to a drag once the pointer moves
	// past `DRAG_THRESHOLD`; `moved` then suppresses the click that would fire selection.
	let mut press = use_signal(|| None::<(usize, f64, f64)>);
	let mut moved = use_signal(|| false);
	// Size (px) of this pane's drop overlay, for classifying the pointer into a zone while dragging.
	let mut drop_size = use_signal(|| (0.0_f64, 0.0_f64));
	// Size (px) of a tab pill, for positioning the bar-insertion line.
	let mut tab_size = use_signal(|| (0.0_f64, 0.0_f64));

	let tab_count = tabs.len();
	// Index of the tab being dragged out of this leaf (dimmed while dragging), if any.
	let dragged = match DRAG() {
		Some(d) if d.source == id => Some(d.index),
		_ => None,
	};

	rsx! {
		div { class: "leaf-pane",
			div { class: "leaf-tabbar", role: "tablist",
				div {
					class: "leaf-tablist",
					// Over empty bar space mid-drag, target the end of this bar.
					onpointermove: move |e: PointerEvent| {
						if DRAG().is_some() {
							set_over(DropTarget::Bar(id, tab_count));
							let (_, th) = tab_size();
							let el = e.element_coordinates();
							let cl = e.client_coordinates();
							set_hint(DropHint { x: cl.x - 1.5, y: cl.y - el.y, w: 3.0, h: th, line: true });
						}
					},
					onpointerup: move |e: PointerEvent| {
						if DRAG().is_some() {
							e.stop_propagation();
							commit_drag();
						} else {
							press.set(None);
						}
					},
					// A fast drag can leave the bar before the move threshold registers on a tab;
					// promote here too so dragging a tab straight out still starts a drag.
					onpointerleave: move |_| {
						if DRAG().is_some() {
							return;
						}
						if let Some((idx, _, _)) = press() {
							*DRAG.write() = Some(TabDrag { source: id, index: idx, over: None });
							moved.set(true);
							press.set(None);
						}
					},
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
							"data-dragging": if dragged == Some(i) { "true" } else { "false" },
							onresize: move |e| {
								if let Ok(size) = e.get_border_box_size() {
									tab_size.set((size.width, size.height));
								}
							},
							onpointerdown: move |e: PointerEvent| {
								let c = e.client_coordinates();
								press.set(Some((i, c.x, c.y)));
								moved.set(false);
							},
							// Promote a pending press to a drag once it moves past the threshold, then mark
							// the slot before/after this tab and place the insertion line there.
							onpointermove: move |e: PointerEvent| {
								if DRAG().is_none() {
									let Some((idx, sx, sy)) = press() else {
										return;
									};
									let c = e.client_coordinates();
									if (c.x - sx).hypot(c.y - sy) <= DRAG_THRESHOLD {
										return;
									}
									*DRAG.write() = Some(TabDrag { source: id, index: idx, over: None });
									moved.set(true);
									press.set(None);
								}
								e.stop_propagation();
								let (tw, th) = tab_size();
								let el = e.element_coordinates();
								let cl = e.client_coordinates();
								let before = el.x < tw / 2.0;
								set_over(DropTarget::Bar(id, if before { i } else { i + 1 }));
								let left = cl.x - el.x;
								let line_x = if before { left } else { left + tw };
								set_hint(DropHint { x: line_x - 1.5, y: cl.y - el.y, w: 3.0, h: th, line: true });
							},
							onclick: move |_| {
								if moved() {
									moved.set(false);
									return;
								}
								WORKSPACE.write().set_active(id, i);
							},
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
				// Split this pane toward a side, duplicating the active tab's view into the new pane.
				DropdownMenu {
					DropdownMenuTrigger { class: "leaf-tab-split", "aria-label": "Split pane",
						Columns2 { size: "1rem" }
					}
					DropdownMenuContent { class: "leaf-split-menu",
						DropdownMenuItem::<String> {
							value: "right".to_string(),
							index: 0usize,
							on_select: move |_| split_toward(id, active_view, Edge::Right),
							"Split right"
						}
						DropdownMenuItem::<String> {
							value: "left".to_string(),
							index: 1usize,
							on_select: move |_| split_toward(id, active_view, Edge::Left),
							"Split left"
						}
						DropdownMenuItem::<String> {
							value: "down".to_string(),
							index: 2usize,
							on_select: move |_| split_toward(id, active_view, Edge::Bottom),
							"Split down"
						}
						DropdownMenuItem::<String> {
							value: "up".to_string(),
							index: 3usize,
							on_select: move |_| split_toward(id, active_view, Edge::Top),
							"Split up"
						}
					}
				}
			}
			div { class: "leaf-content",
				if let Some(view) = active_view {
					{render_view(view)}
				}
				// While a tab drag is in progress, the content area is a drop target: edges split the
				// pane, the center moves the tab in. The bar (above) handles slot insertion instead.
				if DRAG().is_some() {
					div {
						class: "pane-drop",
						onresize: move |e| {
							if let Ok(size) = e.get_border_box_size() {
								drop_size.set((size.width, size.height));
							}
						},
						onpointermove: move |e: PointerEvent| {
							let (w, h) = drop_size();
							let el = e.element_coordinates();
							let cl = e.client_coordinates();
							set_over(DropTarget::Pane(id, drop_zone(el.x, el.y, w, h)));
							set_hint(zone_rect(el.x, el.y, cl.x - el.x, cl.y - el.y, w, h));
						},
						onpointerup: move |e: PointerEvent| {
							e.stop_propagation();
							commit_drag();
						},
					}
				}
			}
		}
	}
}

/// Split `leaf` toward `edge`, duplicating `view` into the new pane. No-op if the leaf is empty.
fn split_toward(leaf: NodeId, view: Option<View>, edge: Edge) {
	if let Some(view) = view {
		WORKSPACE.write().split(leaf, edge, view);
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
