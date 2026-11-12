use dioxus::prelude::*;
use dioxus_icons::lucide::{Columns2, Plus, X};

use super::model::{Axis, DropZone, Edge, Node, NodeId, View};
use super::WORKSPACE;
use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger};
use crate::views::{Home, NewTab, Settings};

/// Where a dragged tab currently points: a pane body (edge → split, center → move to end) or a
/// specific insert slot in a pane's tab bar.
#[derive(Clone, Copy, PartialEq)]
enum DropTarget {
	Pane(NodeId, DropZone),
	Bar(NodeId, usize),
}

/// An in-progress drag of a tab out of its bar, tracked globally so any pane can be a drop target.
#[derive(Clone, PartialEq)]
struct TabDrag {
	/// Leaf the tab is being dragged from.
	source: NodeId,
	/// Index of the dragged tab within `source`.
	index: usize,
	/// Where the pointer currently points, if over a drop target.
	over: Option<DropTarget>,
}

/// The active cross-pane tab drag, or `None`. A within-bar reorder (see [`LeafPane`]) promotes to
/// this once the pointer leaves the tab bar.
static DRAG: GlobalSignal<Option<TabDrag>> = Signal::global(|| None);

/// Update the current drop target (no-op if unchanged or no drag is active).
fn set_over(target: DropTarget) {
	let Some(mut d) = DRAG() else {
		return;
	};
	if d.over != Some(target) {
		d.over = Some(target);
		*DRAG.write() = Some(d);
	}
}

/// Apply the current drop target to the workspace and end the drag.
fn commit_drag() {
	if let Some(TabDrag { source, index, over: Some(target) }) = DRAG() {
		match target {
			DropTarget::Pane(t, DropZone::Edge(edge)) => {
				WORKSPACE.write().move_tab_to_edge(source, index, t, edge);
			},
			DropTarget::Pane(t, DropZone::Center) => WORKSPACE.write().move_tab_to_leaf(source, index, t),
			DropTarget::Bar(dest, at) => WORKSPACE.write().move_tab_to_leaf_at(source, index, dest, at),
		}
	}
	*DRAG.write() = None;
}

/// Classify a pointer at `(x, y)` within a `w`×`h` pane into a drop zone: an edge band near each
/// side, or the center otherwise.
fn drop_zone(x: f64, y: f64, w: f64, h: f64) -> DropZone {
	if w <= 0.0 || h <= 0.0 {
		return DropZone::Center;
	}
	let (left, right, top, bottom) = (x, w - x, y, h - y);
	let nearest = left.min(right).min(top).min(bottom);
	if nearest > w.min(h) * 0.25 {
		DropZone::Center
	} else if left <= right && left <= top && left <= bottom {
		DropZone::Edge(Edge::Left)
	} else if right <= top && right <= bottom {
		DropZone::Edge(Edge::Right)
	} else if top <= bottom {
		DropZone::Edge(Edge::Top)
	} else {
		DropZone::Edge(Edge::Bottom)
	}
}

/// `data-zone` attribute value for a drop zone (drives the indicator's position in CSS).
const fn zone_attr(zone: DropZone) -> &'static str {
	match zone {
		DropZone::Center => "center",
		DropZone::Edge(Edge::Left) => "left",
		DropZone::Edge(Edge::Right) => "right",
		DropZone::Edge(Edge::Top) => "top",
		DropZone::Edge(Edge::Bottom) => "bottom",
	}
}

/// `data-insert` value for the tab at index `i`: a line on its left (`before`) when a dragged tab
/// would land at slot `i`, or on the right of the last tab (`after`) when appending.
const fn insert_mark(bar_insert: Option<usize>, i: usize, count: usize) -> &'static str {
	match bar_insert {
		Some(at) if at == i => "before",
		Some(at) if at == count && i + 1 == count => "after",
		_ => "",
	}
}

/// Root of the content workspace: renders the pane tree from the root node down.
#[component]
pub fn WorkspaceView() -> Element {
	let root = WORKSPACE.read().root();
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./pane.css") }
		div {
			class: "workspace",
			// Releasing a tab drag anywhere that isn't a pane drop target cancels it (pane overlays
			// stop propagation so their own release commits instead).
			onpointerup: move |_| {
				if DRAG().is_some() {
					*DRAG.write() = None;
				}
			},
			PaneNode { id: root }
		}
	}
}

/// Renders a single node: a [`LeafPane`] for a leaf, or a [`SplitPane`] for a split. Only one of
/// the two arms is reachable per node.
#[component]
fn PaneNode(id: NodeId) -> Element {
	let node = WORKSPACE.read().node(id).cloned();
	match node {
		Some(Node::Leaf { .. }) => rsx! {
			LeafPane { id }
		},
		Some(Node::Split { .. }) => rsx! {
			SplitPane { id }
		},
		None => rsx! {},
	}
}

/// Minimum pixels a pane can be squeezed to while dragging a divider.
const MIN_PANE_PX: f64 = 100.0;

/// Live state while a divider is dragged. Fractions are kept local during the drag and only
/// committed to `WORKSPACE` on release, so a pointer-move doesn't re-render the whole tree.
#[derive(Clone, PartialEq)]
struct DividerDrag {
	/// Divider sits between child `index` and `index + 1`.
	index: usize,
	/// Pointer position (px) along the axis at drag start.
	start: f64,
	/// Fractions at drag start (the base the drag is applied to).
	base: Vec<f32>,
	/// Fractions to render during the drag.
	live: Vec<f32>,
}

/// Shift `delta_px` of the container between adjacent panes `a` and `b`, clamping each to
/// `MIN_PANE_PX`. Returns the new `(a, b)` fractions (their sum is preserved).
#[allow(clippy::cast_possible_truncation)] // fractions are f32; sub-f32 precision is irrelevant here
fn resize_pair(a: f32, b: f32, delta_px: f64, container_px: f64) -> (f32, f32) {
	let total = a + b;
	let delta = (delta_px / container_px) as f32;
	let min = (MIN_PANE_PX / container_px) as f32;
	let hi = total - min;
	let new_a = if min <= hi { (a + delta).clamp(min, hi) } else { total / 2.0 };
	(new_a, total - new_a)
}

/// A split container: lays its children along one axis with a draggable divider between each pair.
#[component]
fn SplitPane(id: NodeId) -> Element {
	let node = WORKSPACE.read().node(id).cloned();
	let Some(Node::Split { axis, children, fractions, .. }) = node else {
		return rsx! {};
	};

	// Container size along the axis (px), kept current via the ResizeObserver; drives px→fraction.
	let mut container_px = use_signal(|| 0.0_f64);
	let mut drag = use_signal(|| None::<DividerDrag>);

	let horizontal = axis == Axis::Horizontal;
	let flow = if horizontal { "row" } else { "column" };
	let axis_attr = if horizontal { "horizontal" } else { "vertical" };

	// Render the live fractions mid-drag, otherwise the model's.
	let fracs = drag.read().as_ref().map_or_else(|| fractions.clone(), |d| d.live.clone());

	rsx! {
		div {
			class: "pane-split",
			style: "flex-direction: {flow};",
			onresize: move |e| {
				if let Ok(size) = e.get_border_box_size() {
					container_px.set(if horizontal { size.width } else { size.height });
				}
			},
			for (i , child) in children.iter().enumerate() {
				div {
					key: "{child.0}",
					class: "pane-split-child",
					style: "flex: {fracs.get(i).copied().unwrap_or(0.0)} 1 0;",
					PaneNode { id: *child }
				}
				if i + 1 < children.len() {
					div {
						class: "pane-divider",
						"data-axis": axis_attr,
						onpointerdown: move |e| {
							let p = if horizontal { e.client_coordinates().x } else { e.client_coordinates().y };
							if let Some(Node::Split { fractions, .. }) = WORKSPACE.read().node(id).cloned() {
								drag.set(Some(DividerDrag { index: i, start: p, base: fractions.clone(), live: fractions }));
							}
						},
					}
				}
			}
		}
		if drag.read().is_some() {
			div {
				class: "resize-overlay",
				"data-axis": axis_attr,
				onpointermove: move |e| {
					let s = container_px();
					let Some(mut d) = drag() else {
						return;
					};
					if s <= 0.0 {
						return;
					}
					let p = if horizontal { e.client_coordinates().x } else { e.client_coordinates().y };
					if let (Some(&a), Some(&b)) = (d.base.get(d.index), d.base.get(d.index + 1)) {
						let (na, nb) = resize_pair(a, b, p - d.start, s);
						if let Some(x) = d.live.get_mut(d.index) {
							*x = na;
						}
						if let Some(x) = d.live.get_mut(d.index + 1) {
							*x = nb;
						}
						drag.set(Some(d));
					}
				},
				onpointerup: move |_| {
					if let Some(d) = drag() {
						WORKSPACE.write().set_fractions(id, d.live);
					}
					drag.set(None);
				},
				onpointercancel: move |_| drag.set(None),
			}
		}
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

	// Pointer-drag reorder within this bar: `dragging` tracks the dragged tab's current index;
	// `moved` suppresses the click that would otherwise fire selection at the end of a drag.
	let mut dragging = use_signal(|| None::<usize>);
	let mut moved = use_signal(|| false);
	// Size (px) of this pane's drop overlay, for classifying the pointer into a zone while dragging.
	let mut drop_size = use_signal(|| (0.0_f64, 0.0_f64));

	let tab_count = tabs.len();
	// Insertion slot to mark while a cross-pane drag hovers this bar.
	let bar_insert = match DRAG().and_then(|d| d.over) {
		Some(DropTarget::Bar(leaf, at)) if leaf == id => Some(at),
		_ => None,
	};

	rsx! {
		div { class: "leaf-pane",
			div { class: "leaf-tabbar", role: "tablist",
				div {
					class: "leaf-tablist",
					// Over empty bar space mid-drag, target the end of this bar.
					onpointermove: move |_| {
						if DRAG().is_some() {
							set_over(DropTarget::Bar(id, tab_count));
						}
					},
					onpointerup: move |e: PointerEvent| {
						if DRAG().is_some() {
							e.stop_propagation();
							commit_drag();
						} else {
							dragging.set(None);
						}
					},
					// Leaving the bar mid-reorder hands off to a cross-pane drag (drop on a pane edge
					// to split, its center to append, or a bar to insert at a slot).
					onpointerleave: move |_| {
						if let Some(idx) = dragging() {
							*DRAG.write() = Some(TabDrag { source: id, index: idx, over: None });
							dragging.set(None);
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
							"data-dragging": if dragging() == Some(i) { "true" } else { "false" },
							"data-insert": insert_mark(bar_insert, i, tab_count),
							onpointerdown: move |_| {
								dragging.set(Some(i));
								moved.set(false);
							},
							// Mid cross-pane drag, hovering a tab targets its slot for insertion.
							onpointermove: move |e: PointerEvent| {
								if DRAG().is_some() {
									e.stop_propagation();
									set_over(DropTarget::Bar(id, i));
								}
							},
							// Hovering another tab mid-reorder slots the dragged tab into its place.
							onpointerenter: move |_| {
								if let Some(from) = dragging() {
									if from != i {
										WORKSPACE.write().reorder_tab(id, from, i);
										dragging.set(Some(i));
										moved.set(true);
									}
								}
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
						onpointermove: move |e| {
							let (w, h) = drop_size();
							let p = e.element_coordinates();
							set_over(DropTarget::Pane(id, drop_zone(p.x, p.y, w, h)));
						},
						onpointerup: move |e: PointerEvent| {
							e.stop_propagation();
							commit_drag();
						},
						// Highlight the zone the drop would land in.
						if let Some(DropTarget::Pane(over_id, zone)) = DRAG().and_then(|d| d.over) {
							if over_id == id {
								div { class: "pane-drop-indicator", "data-zone": zone_attr(zone) }
							}
						}
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
